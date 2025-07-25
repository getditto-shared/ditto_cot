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
        room: String = "All Chat Rooms",
        callsign: String
    ) async throws {
        let chatEvent = try CoTEventBuilder()
            .uid(UUID().uuidString)
            .type("b-t-f")
            .how("h-e")
            .point(CoTPoint(lat: 0, lon: 0)) // Chat doesn't require real location
            .detail(CoTDetail([
                "chat": [
                    "from": callsign,
                    "room": room,
                    "msg": message
                ]
            ]))
            .build()
        
        _ = try await observable.insert(chatEvent)
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
        
        // Events updates
        observable.$events
            .receive(on: DispatchQueue.main)
            .map { docs in
                docs.compactMap { CoTEventModel(from: $0) }
                    .sorted { $0.timestamp > $1.timestamp } // Most recent first
            }
            .assign(to: \.events, on: self)
            .store(in: &cancellables)
        
        // Chat messages
        observable.chatMessagesPublisher
            .receive(on: DispatchQueue.main)
            .map { docs in
                docs.compactMap { ChatMessageModel(from: $0) }
                    .sorted { $0.timestamp < $1.timestamp } // Chronological order
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