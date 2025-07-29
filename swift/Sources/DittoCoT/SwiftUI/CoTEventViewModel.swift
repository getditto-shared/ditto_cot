import SwiftUI
import Combine
import CoreLocation
import DittoSwift
import DittoCoTCore

/// SwiftUI view model for managing CoT events with reactive updates
@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
@MainActor
public class CoTEventViewModel: ObservableObject {
    
    // MARK: - Published Properties
    
    @Published public private(set) var events: [CoTEventModel] = []
    @Published public private(set) var chatMessages: [ChatMessageModel] = []
    @Published public private(set) var locationUpdates: [LocationUpdateModel] = []
    @Published public private(set) var emergencyEvents: [EmergencyEventModel] = []
    @Published public private(set) var isConnected: Bool = false
    @Published public private(set) var error: Error?
    @Published public private(set) var isLoading: Bool = false
    
    // MARK: - Filter Properties
    
    @Published public var selectedCallsigns: Set<String> = []
    @Published public var selectedEventTypes: Set<String> = []
    @Published public var timeRangeFilter: TimeInterval = 3600 // 1 hour default
    @Published public var searchText: String = ""
    
    // MARK: - Private Properties
    
    private let observable: CoTObservable
    private var cancellables = Set<AnyCancellable>()
    
    // MARK: - Computed Properties
    
    /// Filtered events based on current filter settings
    public var filteredEvents: [CoTEventModel] {
        events.filter { event in
            // Callsign filter
            if !selectedCallsigns.isEmpty && !selectedCallsigns.contains(event.callsign) {
                return false
            }
            
            // Event type filter
            if !selectedEventTypes.isEmpty && !selectedEventTypes.contains(event.type) {
                return false
            }
            
            // Time range filter
            let timeThreshold = Date().timeIntervalSince1970 - timeRangeFilter
            if event.timestamp.timeIntervalSince1970 < timeThreshold {
                return false
            }
            
            // Search text filter
            if !searchText.isEmpty {
                let searchLower = searchText.lowercased()
                return event.callsign.lowercased().contains(searchLower) ||
                       event.type.lowercased().contains(searchLower) ||
                       (event.remarks?.lowercased().contains(searchLower) ?? false)
            }
            
            return true
        }
    }
    
    /// All unique callsigns in current events
    public var availableCallsigns: [String] {
        Array(Set(events.map(\.callsign))).sorted()
    }
    
    /// All unique event types in current events
    public var availableEventTypes: [String] {
        Array(Set(events.map(\.type))).sorted()
    }
    
    /// Events grouped by category for organized display
    public var eventsByCategory: [EventCategory: [CoTEventModel]] {
        Dictionary(grouping: filteredEvents) { $0.eventCategory }
    }
    
    /// Recent chat messages (last 50)
    public var recentChatMessages: [ChatMessageModel] {
        Array(chatMessages.suffix(50))
    }
    
    /// Active emergency events only
    public var activeEmergencyEvents: [EmergencyEventModel] {
        emergencyEvents.filter(\.isActive)
    }
    
    // MARK: - Initialization
    
    public init(observable: CoTObservable) {
        self.observable = observable
        setupObservables()
    }
    
    // MARK: - Public Methods
    
    /// Refresh all events
    public func refreshEvents() {
        Task {
            isLoading = true
            observable.refreshAll()
            isLoading = false
        }
    }
    
    /// Send a chat message
    public func sendChatMessage(
        message: String,
        room: String = "Ditto",
        callsign: String
    ) async throws {
        let chatDocumentId = "chat-\(UUID().uuidString)"
        let chatDocument: [String: Any] = [
            "_id": chatDocumentId,
            "_c": 0,
            "_r": false,
            "_v": 2,
            "a": observable.dittoCoT.ditto.siteID.description,
            "msg": message,
            "room": room,
            "roomId": "ChatContact-\(room)",
            "parent": "RootContactGroup",
            "e": callsign,
            "d": chatDocumentId,
            "b": Int64(Date().timeIntervalSince1970 * 1000)
        ]
        
        let chatCollection = observable.dittoCoT.ditto.store.collection("chat")
        let docId = try chatCollection.upsert(chatDocument)
        print("🟢 CHAT SENT: \(message) -> \(docId)")
        
        // Force refresh to see the message immediately
        observable.refreshAll()
    }
    
    /// Send a location update
    public func sendLocationUpdate(
        callsign: String,
        location: CLLocationCoordinate2D,
        altitude: Double = 0,
        accuracy: Double = 10
    ) async throws {
        let locationEvent = try CoTEventBuilder()
            .uid("\(callsign)-\(Date().timeIntervalSince1970)")
            .type("a-f-G-U-C")
            .how("m-g")
            .point(CoTPoint(
                lat: location.latitude,
                lon: location.longitude,
                hae: altitude,
                ce: accuracy,
                le: accuracy
            ))
            .detail(CoTDetail([
                "contact": [
                    "callsign": callsign
                ]
            ]))
            .build()
        
        _ = try await observable.insert(locationEvent)
    }
    
    /// Send an emergency beacon
    public func sendEmergencyBeacon(
        callsign: String,
        location: CLLocationCoordinate2D,
        emergencyType: EmergencyType = .general
    ) async throws {
        let emergencyEvent = try CoTEventBuilder()
            .uid("\(callsign)-emergency-\(Date().timeIntervalSince1970)")
            .type("b-a-o-tbl")
            .how("h-e")
            .point(CoTPoint(
                lat: location.latitude,
                lon: location.longitude
            ))
            .detail(CoTDetail([
                "emergency": [
                    "type": emergencyType.rawValue,
                    "callsign": callsign
                ],
                "contact": [
                    "callsign": callsign
                ]
            ]))
            .build()
        
        _ = try await observable.insert(emergencyEvent)
    }
    
    /// Delete an event
    public func deleteEvent(_ event: CoTEventModel) async throws {
        try await observable.remove(uid: event.uid)
    }
    
    // MARK: - Filter Methods
    
    public func toggleCallsignFilter(_ callsign: String) {
        if selectedCallsigns.contains(callsign) {
            selectedCallsigns.remove(callsign)
        } else {
            selectedCallsigns.insert(callsign)
        }
    }
    
    public func toggleEventTypeFilter(_ type: String) {
        if selectedEventTypes.contains(type) {
            selectedEventTypes.remove(type)
        } else {
            selectedEventTypes.insert(type)
        }
    }
    
    public func clearAllFilters() {
        selectedCallsigns.removeAll()
        selectedEventTypes.removeAll()
        searchText = ""
        timeRangeFilter = 3600
    }
    
    // MARK: - Private Methods
    
    private func setupObservables() {
        // Connection status
        observable.connectionPublisher
            .receive(on: DispatchQueue.main)
            .assign(to: \.isConnected, on: self)
            .store(in: &cancellables)
        
        // Error handling
        observable.errorPublisher
            .receive(on: DispatchQueue.main)
            .sink { [weak self] error in
                self?.error = error
            }
            .store(in: &cancellables)
        
        // Events updates - Show ALL documents as raw events
        observable.$events
            .receive(on: DispatchQueue.main)
            .map { docs in
                print("📊 CoTEventViewModel: Received \(docs.count) documents from observable")
                print("   Document IDs: \(docs.map { $0.id })")
                let models = docs.compactMap { doc -> CoTEventModel? in
                    // Create a raw event model for ALL documents
                    guard let uid = doc.value["_id"] as? String else {
                        print("❌ Document missing _id: \(doc.id)")
                        return nil
                    }
                    
                    // Extract basic fields that all documents should have
                    let type = doc.value["w"] as? String ?? "unknown"
                    let callsign = doc.value["e"] as? String ?? doc.value["authorCallsign"] as? String ?? uid
                    
                    // Debug: Show what timestamp fields exist
                    print("🔍 Document \(uid) timestamp fields:")
                    if let b = doc.value["b"] { print("   b: \(b) (type: \(Swift.type(of: b)))") }
                    if let n = doc.value["n"] { print("   n: \(n) (type: \(Swift.type(of: n)))") }
                    if let time = doc.value["time"] { print("   time: \(time) (type: \(Swift.type(of: time)))") }
                    
                    // Get timestamp - try multiple fields
                    let timestamp: Date
                    if let timeString = doc.value["time"] as? String {
                        let formatter = ISO8601DateFormatter()
                        formatter.formatOptions = [.withInternetDateTime, .withFractionalSeconds]
                        timestamp = formatter.date(from: timeString) ?? Date()
                    } else if let bValue = doc.value["b"] {
                        // 'b' field could be Int64, Int, or Double
                        let bMillis: Double
                        if let intValue = bValue as? Int64 {
                            bMillis = Double(intValue)
                        } else if let intValue = bValue as? Int {
                            bMillis = Double(intValue)
                        } else if let doubleValue = bValue as? Double {
                            bMillis = doubleValue
                        } else {
                            print("⚠️ Could not convert 'b' field to number: \(bValue)")
                            bMillis = 0
                        }
                        
                        if bMillis > 0 {
                            timestamp = Date(timeIntervalSince1970: bMillis / 1000.0)
                            print("📊 Using 'b' field timestamp: \(bMillis) ms -> \(timestamp)")
                        } else {
                            timestamp = Date()
                        }
                    } else if let nValue = doc.value["n"] {
                        // 'n' field could be Int64, Int, or Double
                        let nNumeric: Double
                        if let intValue = nValue as? Int64 {
                            nNumeric = Double(intValue)
                        } else if let intValue = nValue as? Int {
                            nNumeric = Double(intValue)
                        } else if let doubleValue = nValue as? Double {
                            nNumeric = doubleValue
                        } else {
                            nNumeric = 0
                        }
                        
                        if nNumeric > 0 {
                            // Check magnitude to determine units
                            if nNumeric > 1_000_000_000_000 { // Very large number, treat as microseconds for backward compatibility
                                timestamp = Date(timeIntervalSince1970: nNumeric / 1_000_000.0)
                            } else { // Likely milliseconds
                                timestamp = Date(timeIntervalSince1970: nNumeric / 1000.0)
                            }
                        } else {
                            timestamp = Date()
                        }
                    } else {
                        print("⚠️ Document \(uid) has NO valid timestamp fields! Using current time as fallback")
                        timestamp = Date()
                    }
                    
                    // Get location or use default
                    let lat = doc.value["j"] as? Double ?? 0.0
                    let lon = doc.value["l"] as? Double ?? 0.0
                    let location = CLLocationCoordinate2D(latitude: lat, longitude: lon)
                    
                    // Get stale time
                    let staleTime: Date
                    if let oValue = doc.value["o"] as? Double, oValue > 0 {
                        // Check magnitude to determine units
                        if oValue > 1_000_000_000_000 { // Very large number, treat as microseconds for backward compatibility
                            staleTime = Date(timeIntervalSince1970: oValue / 1_000_000.0)
                        } else { // Likely milliseconds
                            staleTime = Date(timeIntervalSince1970: oValue / 1000.0)
                        }
                    } else {
                        staleTime = timestamp.addingTimeInterval(300) // 5 minutes default
                    }
                    
                    // Extract any remarks or message
                    let remarks = doc.value["message"] as? String ?? 
                                 doc.value["msg"] as? String ?? 
                                 doc.value["remarks"] as? String
                    
                    // Create raw event model with document data
                    let model = CoTEventModel(
                        uid: uid,
                        type: type,
                        callsign: callsign,
                        timestamp: timestamp,
                        location: location,
                        altitude: doc.value["i"] as? Double,
                        accuracy: doc.value["h"] as? Double,
                        staleTime: staleTime,
                        how: doc.value["p"] as? String ?? "h-g-i-g-o",
                        remarks: remarks,
                        rawDocumentData: doc.value as [String: Any]
                    )
                    
                    print("✅ Created raw event model for \(type) document: \(uid)")
                    print("   Timestamp: \(timestamp) (\(timestamp.timeIntervalSince1970))")
                    print("   Type: \(type), Callsign: \(callsign)")
                    return model
                }
                print("✅ Successfully converted \(models.count) documents to raw event models")
                return models.sorted { $0.timestamp > $1.timestamp } // Most recent first
            }
            .assign(to: \.events, on: self)
            .store(in: &cancellables)
        
        // Chat messages
        observable.chatMessagesPublisher
            .receive(on: DispatchQueue.main)
            .map { docs in
                print("💬 CoTEventViewModel: Processing \(docs.count) potential chat documents...")
                let chatModels = docs.compactMap { doc -> ChatMessageModel? in
                    if let model = ChatMessageModel(from: doc) {
                        print("   ✅ Converted chat document: \(doc.id) -> \(model.from): \(model.message)")
                        print("   ✅ Chat message timestamp: \(model.timestamp)")
                        return model
                    } else {
                        print("   ❌ Failed to convert chat document: \(doc.id)")
                        print("      FULL CHAT DOCUMENT FIELDS:")
                        for (key, value) in doc.value.sorted(by: { $0.key < $1.key }) {
                            print("        \(key): \(type(of: value)) = \(value)")
                        }
                        return nil
                    }
                }
                print("💬 Successfully converted \(chatModels.count) chat messages")
                return chatModels.sorted { $0.timestamp < $1.timestamp } // Chronological order
            }
            .assign(to: \.chatMessages, on: self)
            .store(in: &cancellables)
        
        // Location updates
        observable.locationUpdatesPublisher
            .receive(on: DispatchQueue.main)
            .map { docs in
                docs.compactMap { LocationUpdateModel(from: $0) }
                    .sorted { $0.timestamp > $1.timestamp } // Most recent first
            }
            .assign(to: \.locationUpdates, on: self)
            .store(in: &cancellables)
        
        // Emergency events
        observable.emergencyEventsPublisher
            .receive(on: DispatchQueue.main)
            .map { docs in
                docs.compactMap { EmergencyEventModel(from: $0) }
                    .sorted { $0.timestamp > $1.timestamp } // Most recent first
            }
            .assign(to: \.emergencyEvents, on: self)
            .store(in: &cancellables)
    }
}

// MARK: - Supporting Types

public enum EmergencyType: String, CaseIterable, Identifiable {
    case general = "general"
    case medical = "medical"
    case fire = "fire"
    case police = "police"
    case rescue = "rescue"
    
    public var id: String { rawValue }
    
    public var displayName: String {
        switch self {
        case .general: return "General Emergency"
        case .medical: return "Medical Emergency"
        case .fire: return "Fire Emergency"
        case .police: return "Police Emergency"
        case .rescue: return "Rescue Emergency"
        }
    }
    
    public var systemImage: String {
        switch self {
        case .general: return "exclamationmark.triangle.fill"
        case .medical: return "cross.fill"
        case .fire: return "flame.fill"
        case .police: return "shield.fill"
        case .rescue: return "lifepreserver.fill"
        }
    }
}