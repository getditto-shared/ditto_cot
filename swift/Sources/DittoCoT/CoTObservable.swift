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
    
    // MARK: - Private Properties
    
    private let dittoCoT: DittoCoT
    private var cancellables = Set<AnyCancellable>()
    
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
        self.isConnected = true // Simplified connection status
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
        events = dittoCoT.findAll()
    }
    
    /// Refresh events within a time range
    public func refreshByTimeRange(from: Date, to: Date) {
        events = dittoCoT.findByTimeRange(from: from, to: to)
    }
    
    // MARK: - Event Operations
    
    /// Insert a new CoT event
    public func insert(_ event: CoTEvent) async throws -> String {
        do {
            let docID = try await dittoCoT.insert(event)
            await MainActor.run {
                self.error = nil
                // Refresh to show new event
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
}