import Foundation
import Combine
import DittoSwift
import DittoCoTCore

public typealias DittoDoc = DittoSwift.DittoDocument

/// Observable wrapper for CoT events with Combine integration
@available(iOS 13.0, macOS 10.15, watchOS 6.0, tvOS 13.0, *)
public class CoTObservable: ObservableObject {
    
    // MARK: - Published Properties
    
    @Published public private(set) var events: [DittoDoc] = []
    @Published public private(set) var isConnected: Bool = false
    @Published public private(set) var error: Error?
    @Published public private(set) var connectedPeers: [DittoPeer] = []
    
    // MARK: - Private Properties
    
    private let dittoCoT: DittoCoT
    private var cancellables = Set<AnyCancellable>()
    private var subscription: DittoSubscription?
    private var liveQuery: DittoLiveQuery?
    private var presenceObserver: Any? // Use Any for now due to API differences
    
    // MARK: - Publishers
    
    /// Publisher that emits when new CoT events are added
    public lazy var newEventsPublisher: AnyPublisher<[DittoDoc], Never> = {
        $events
            .dropFirst() // Skip initial empty state
            .removeDuplicates { old, new in
                old.count == new.count && old.allSatisfy { oldEvent in
                    new.contains { newEvent in
                        oldEvent.id == newEvent.id
                    }
                }
            }
            .eraseToAnyPublisher()
    }()
    
    /// Publisher that emits connection status changes
    public var connectionPublisher: AnyPublisher<Bool, Never> {
        $isConnected.eraseToAnyPublisher()
    }
    
    /// Publisher that emits errors
    public var errorPublisher: AnyPublisher<Error, Never> {
        $error
            .compactMap { $0 }
            .eraseToAnyPublisher()
    }
    
    // MARK: - Initialization
    
    public init(dittoCoT: DittoCoT) {
        self.dittoCoT = dittoCoT
        setupLiveQueries()
        setupPresenceObserver()
        setupConnectivityMonitoring()
    }
    
    deinit {
        cleanup()
    }
    
    // MARK: - Query Methods
    
    /// Refresh events by type
    public func refreshByType(_ type: String) {
        events = dittoCoT.findByType(type)
    }
    
    /// Refresh events by callsign
    public func refreshByCallsign(_ callsign: String) {
        events = dittoCoT.findByCallsign(callsign)
    }
    
    /// Refresh all active events
    public func refreshAll() {
        let foundEvents = dittoCoT.findAll()
        print("🔄 Manual refresh found \(foundEvents.count) events")
        events = foundEvents
    }
    
    /// Refresh events within a time range
    public func refreshByTimeRange(from: Date, to: Date) {
        events = dittoCoT.findByTimeRange(from: from, to: to)
    }
    
    // MARK: - Event Operations
    
    /// Insert a new CoT event
    public func insert(_ event: CoTEvent) async throws -> String {
        do {
            print("📤 Inserting CoT event: \(event.uid)")
            print("   Type: \(event.type)")
            print("   Detail: \(event.detail?.description ?? "none")")
            
            let docID = try await dittoCoT.insert(event)
            print("✅ Successfully inserted event with ID: \(docID)")
            
            await MainActor.run {
                self.error = nil
                // Don't refresh manually - let live query handle it
                print("🔄 Event inserted, live query should update automatically")
            }
            return docID
        } catch {
            print("❌ Failed to insert event: \(error)")
            await MainActor.run {
                self.error = error
            }
            throw error
        }
    }
    
    /// Update an existing CoT event
    public func update(_ event: CoTEvent) async throws -> String {
        do {
            let docID = try await dittoCoT.update(event)
            await MainActor.run {
                self.error = nil
                // Refresh to show updated event
                self.refreshAll()
            }
            return docID
        } catch {
            await MainActor.run {
                self.error = error
            }
            throw error
        }
    }
    
    /// Remove a CoT event
    public func remove(uid: String) async throws {
        do {
            try await dittoCoT.remove(uid: uid)
            await MainActor.run {
                self.error = nil
                // Refresh to hide removed event
                self.refreshAll()
            }
        } catch {
            await MainActor.run {
                self.error = error
            }
            throw error
        }
    }
}

// MARK: - Convenience Publishers

@available(iOS 13.0, macOS 10.15, watchOS 6.0, tvOS 13.0, *)
extension CoTObservable {
    
    /// Publisher for chat messages only
    public var chatMessagesPublisher: AnyPublisher<[DittoDoc], Never> {
        $events
            .map { docs in
                docs.filter { doc in
                    if let type = doc.value["w"] as? String {
                        return type.hasPrefix("b-t-f")
                    }
                    return false
                }
            }
            .eraseToAnyPublisher()
    }
    
    /// Publisher for location updates only
    public var locationUpdatesPublisher: AnyPublisher<[DittoDoc], Never> {
        $events
            .map { docs in
                docs.filter { doc in
                    if let type = doc.value["w"] as? String {
                        return type.hasPrefix("a-f") || type.hasPrefix("a-u")
                    }
                    return false
                }
            }
            .eraseToAnyPublisher()
    }
    
    /// Publisher for emergency events
    public var emergencyEventsPublisher: AnyPublisher<[DittoDoc], Never> {
        $events
            .map { docs in
                docs.filter { doc in
                    if let type = doc.value["w"] as? String {
                        return type.contains("emergency") || type.contains("911")
                    }
                    return false
                }
            }
            .eraseToAnyPublisher()
    }
    
    /// Publisher for connected peers
    public var peersPublisher: AnyPublisher<[DittoPeer], Never> {
        $connectedPeers.eraseToAnyPublisher()
    }
}

// MARK: - Live Subscriptions and Observers

@available(iOS 13.0, macOS 10.15, watchOS 6.0, tvOS 13.0, *)
extension CoTObservable {
    
    private func setupLiveQueries() {
        let collection = dittoCoT.ditto.store.collection(dittoCoT.collectionName)
        
        print("🔍 Setting up live queries for collection: \(dittoCoT.collectionName)")
        
        // Set up subscription for the collection using proper DQL syntax
        subscription = collection.find("_r != true").subscribe()
        print("📡 Subscription created for active events")
        
        // Set up live query to observe changes (both local and remote)
        liveQuery = collection.find("_r != true").observeLocal(eventHandler: { [weak self] docs, event in
            print("📥 Live query received update: \(docs.count) documents")
            print("📋 Event type: \(event)")
            
            // Log each document for debugging
            for (index, doc) in docs.enumerated() {
                print("  Document \(index): \(doc.id)")
                if let type = doc.value["w"] as? String {
                    print("    Type: \(type)")
                }
                if let callsign = doc.value["e"] as? String {
                    print("    Callsign: \(callsign)")
                }
            }
            
            DispatchQueue.main.async {
                self?.events = docs
                self?.error = nil
                print("✅ Updated events array with \(docs.count) events")
            }
        })
        
        // Also do an immediate query to see what's already there
        let currentDocs = collection.find("_r != true").exec()
        print("🗂️ Current documents in collection: \(currentDocs.count)")
        DispatchQueue.main.async {
            self.events = currentDocs
        }
    }
    
    private func setupPresenceObserver() {
        print("👥 Setting up presence observer")
        
        // Try to use Ditto's presence API
        presenceObserver = dittoCoT.ditto.presence.observe { [weak self] graph in
            print("👥 Presence graph updated:")
            print("   Local peer: \(graph.localPeer)")
            print("   Remote peers: \(graph.remotePeers.count)")
            
            DispatchQueue.main.async {
                self?.connectedPeers = Array(graph.remotePeers)
                self?.isConnected = !graph.remotePeers.isEmpty
                print("✅ Updated presence: \(graph.remotePeers.count) peers, connected: \(self?.isConnected ?? false)")
            }
        }
        
        // Also set up a timer for basic connectivity monitoring
        Timer.publish(every: 5.0, on: .main, in: .common)
            .autoconnect()
            .sink { [weak self] _ in
                self?.updateConnectivityStatus()
            }
            .store(in: &cancellables)
    }
    
    private func setupConnectivityMonitoring() {
        // Monitor transport conditions
        Timer.publish(every: 5.0, on: .main, in: .common)
            .autoconnect()
            .sink { [weak self] _ in
                // Update connectivity status based on Ditto's state
                self?.updateConnectivityStatus()
            }
            .store(in: &cancellables)
    }
    
    private func updateConnectivityStatus() {
        // Check if we have any active peers or local connectivity
        let hasActivePeers = !connectedPeers.isEmpty
        // Simplified connectivity check - assume connected if we have events
        let hasRecentActivity = !events.isEmpty
        
        isConnected = hasActivePeers || hasRecentActivity
    }
    
    private func cleanup() {
        print("🧹 Cleaning up subscriptions and observers")
        subscription?.cancel()
        liveQuery?.stop()
        // presenceObserver cleanup will be handled by cancellables
        presenceObserver = nil
        cancellables.removeAll()
    }
    
    /// Start all subscriptions and observers
    public func startObserving() {
        if subscription == nil {
            setupLiveQueries()
        }
        setupPresenceObserver()
    }
    
    /// Stop all subscriptions and observers
    public func stopObserving() {
        cleanup()
    }
}