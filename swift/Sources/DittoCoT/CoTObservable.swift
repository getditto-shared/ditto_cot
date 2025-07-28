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
    private var additionalSubscriptions: [DittoSubscription] = []
    private var liveQuery: DittoLiveQuery?
    private var additionalLiveQueries: [DittoLiveQuery] = []
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
        print("🚀 CoTObservable: Initializing with dittoCoT")
        self.dittoCoT = dittoCoT
        print("🚀 CoTObservable: Setting up live queries...")
        setupLiveQueries()
        print("🚀 CoTObservable: Setting up presence observer...")
        setupPresenceObserver()
        print("🚀 CoTObservable: Setting up connectivity monitoring...")
        setupConnectivityMonitoring()
        print("🚀 CoTObservable: Initialization complete")
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
        print("🔄 Manual refresh found \(foundEvents.count) events from main collection")
        
        // Don't overwrite - just trigger the live queries to refresh
        // The live queries will handle updating the events array
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
                    // Check for chat type OR chat document schema (msg + room fields)
                    if let type = doc.value["w"] as? String, type.hasPrefix("b-t-f") {
                        return true
                    }
                    // Also check for chat document schema (documents with message/msg and room fields)
                    if (doc.value["message"] != nil || doc.value["msg"] != nil) && doc.value["room"] != nil {
                        return true
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
        
        print("🔍 [\(Date())] Setting up live queries for collection: \(dittoCoT.collectionName)")
        
        // Check what collections exist in the store
        print("🗂️ Checking all collections in Ditto store...")
        // Note: Ditto doesn't have a direct API to list all collections, but we can check our known collection
        
        // IMPORTANT: Set up subscription FIRST to sync remote changes into local DB
        print("📡 Creating subscription:")
        print("   Collection: '\(dittoCoT.collectionName)'")
        print("   DQL Query: '_r != true'")
        subscription = collection.find("_r != true").subscribe()
        print("📡 Primary subscription created - this will sync remote changes into local database")
        
        // Also subscribe to other common collection names that might be used by different clients
        let alternativeCollections = ["chat", "mapitem", "track", "api", "file"]
        print("📡 Creating additional subscriptions to alternative collections...")
        
        for collectionName in alternativeCollections {
            let altCollection = dittoCoT.ditto.store.collection(collectionName)
            let altSub = altCollection.find("_r != true").subscribe()
            additionalSubscriptions.append(altSub)
            print("   ✅ Subscribed to '\(collectionName)'")
            
            // Also check if there are any existing documents
            let existingDocs = altCollection.find("_r != true").exec()
            if existingDocs.count > 0 {
                print("   🔍 Found \(existingDocs.count) existing documents in '\(collectionName)'")
                for doc in existingDocs {
                    print("      • \(doc.id)")
                }
            }
            
            // Set up live query for this collection too
            let altLiveQuery = altCollection.find("_r != true").observeLocal { [weak self] docs, event in
                print("🔥 LIVE QUERY TRIGGERED for '\(collectionName)'!")
                print("   📥 Received \(docs.count) documents")
                print("   📋 Event type: \(event)")
                
                // Filter out stale events
                let now = Date().timeIntervalSince1970 * 1000
                let activeEvents = docs.filter { doc in
                    if let staleMillis = (doc.value["o"] ?? doc.value["c"]) as? Double {
                        let isActive = staleMillis > now
                        if !isActive {
                            print("   ⏰ Filtering out stale: \(doc.id)")
                        }
                        return isActive
                    }
                    return true
                }
                
                if activeEvents.count > 0 {
                    print("   ✅ \(activeEvents.count) active events in '\(collectionName)'")
                    // Merge these events into our main events array
                    DispatchQueue.main.async {
                        // Add to existing events (avoiding duplicates)
                        let currentIds = Set(self?.events.map { $0.id } ?? [])
                        let newEvents = activeEvents.filter { !currentIds.contains($0.id) }
                        if newEvents.count > 0 {
                            self?.events.append(contentsOf: newEvents)
                            print("   ➕ Added \(newEvents.count) new events from '\(collectionName)'")
                        }
                    }
                }
            }
            additionalLiveQueries.append(altLiveQuery)
            print("   ✅ Live query observer added for '\(collectionName)'")
        }
        
        // Verify subscription is active
        if subscription != nil {
            print("✅ Primary subscription is active and syncing")
        } else {
            print("❌ Primary subscription failed to create")
        }
        
        // Set up live query to observe ALL changes in the local database (including those synced from remote)
        print("🔧 Setting up live query observer...")
        liveQuery = collection.find("_r != true").observeLocal { [weak self] docs, event in
            print("🔥 LIVE QUERY CALLBACK TRIGGERED!")
            print("📥 Live query received update: \(docs.count) documents")
            print("📋 Event type: \(event)")
            print("   This includes both local changes AND remote changes synced by subscription")
            
            // Log each document for debugging
            for (index, doc) in docs.enumerated() {
                print("  Document \(index): \(doc.id)")
                if let type = doc.value["w"] as? String {
                    print("    Type: \(type)")
                } else {
                    print("    Type: MISSING 'w' field")
                }
                if let callsign = doc.value["e"] as? String {
                    print("    Callsign: \(callsign)")
                } else {
                    print("    Callsign: MISSING 'e' field")
                }
                // Check if it's a chat or mapitem document
                if doc.value["message"] != nil || doc.value["msg"] != nil {
                    print("    This is a CHAT document")
                }
                if doc.value["c"] != nil { // mapitem has 'c' field for name
                    print("    This is a MAPITEM document")
                }
                // Show first few fields
                print("    Fields: \(Array(doc.value.keys).prefix(10).sorted())")
            }
            
            // Filter out stale events
            let now = Date().timeIntervalSince1970 * 1000 // Convert to milliseconds
            let activeEvents = docs.filter { doc in
                // Check if document has stale time (field 'o' for cot_events, 'c' for other collections)
                if let staleMillis = (doc.value["o"] ?? doc.value["c"]) as? Double {
                    let isActive = staleMillis > now
                    if !isActive {
                        print("    ⏰ Filtering out stale event: \(doc.id) (stale: \(Date(timeIntervalSince1970: staleMillis/1000)))")
                    }
                    return isActive
                }
                // If no stale time, include it
                return true
            }
            
            print("✅ Filtered to \(activeEvents.count) active events (from \(docs.count) total)")
            
            // Log first few events for debugging
            for (index, event) in activeEvents.prefix(5).enumerated() {
                print("   📄 Active Event \(index): \(event.id)")
                if let type = event.value["w"] as? String {
                    print("      Type: \(type)")
                }
                if let callsign = event.value["e"] as? String {
                    print("      Callsign: \(callsign)")
                }
                if let bField = event.value["b"] {
                    print("      b field: \(bField) (type: \(Swift.type(of: bField)))")
                }
                
                // Check if it's being filtered out somewhere
                if event.value["msg"] != nil {
                    print("      This is a CHAT document with msg field")
                }
            }
            
            DispatchQueue.main.async {
                self?.events = activeEvents
                self?.error = nil
                print("✅ Updated CoTObservable.events array with \(activeEvents.count) active events")
                print("   Publishing to subscribers...")
            }
        }
        print("🔧 Live query observer setup complete")
        
        // Also do an immediate query to see what's already there
        let currentDocs = collection.find("_r != true").exec()
        print("🗂️ Current documents in collection: \(currentDocs.count)")
        
        // Filter out stale events
        let now = Date().timeIntervalSince1970 * 1000
        let activeEvents = currentDocs.filter { doc in
            if let staleMillis = (doc.value["o"] ?? doc.value["c"]) as? Double {
                return staleMillis > now
            }
            return true
        }
        
        print("🗂️ Active documents after filtering: \(activeEvents.count)")
        DispatchQueue.main.async {
            self.events = activeEvents
        }
    }
    
    private func setupPresenceObserver() {
        print("👥 Setting up presence observer")
        
        // Try to use Ditto's presence API
        presenceObserver = dittoCoT.ditto.presence.observe { [weak self] graph in
            let remotePeers = Array(graph.remotePeers)
            
            // Only log if peer count changes
            if remotePeers.count != self?.connectedPeers.count {
                print("👥 Presence updated: \(remotePeers.count) remote peer(s)")
                for peer in remotePeers {
                    let connectionType = peer.connections.first?.type
                    let connectionStr = connectionType != nil ? String(describing: connectionType!) : "unknown"
                    print("   • \(peer.deviceName) via \(connectionStr)")
                }
            }
            
            DispatchQueue.main.async {
                self?.connectedPeers = remotePeers
                self?.isConnected = !remotePeers.isEmpty
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
        additionalSubscriptions.forEach { $0.cancel() }
        additionalSubscriptions.removeAll()
        liveQuery?.stop()
        additionalLiveQueries.forEach { $0.stop() }
        additionalLiveQueries.removeAll()
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