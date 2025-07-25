import SwiftUI
import Combine
import DittoSwift
import DittoCoTCore

/// SwiftUI-specific binding utilities for CoT events
@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
public class CoTBinding: ObservableObject {
    
    // MARK: - Reactive Properties
    
    /// Real-time count of active events
    @Published public private(set) var eventCount: Int = 0
    
    /// Real-time count of chat messages
    @Published public private(set) var chatMessageCount: Int = 0
    
    /// Real-time count of emergency events
    @Published public private(set) var emergencyCount: Int = 0
    
    /// Most recent event timestamp
    @Published public private(set) var lastEventTime: Date?
    
    /// Connection health status
    @Published public private(set) var connectionHealth: ConnectionHealth = .unknown
    
    /// Active callsigns
    @Published public private(set) var activeCallsigns: Set<String> = []
    
    // MARK: - Private Properties
    
    private let observable: CoTObservable
    private var cancellables = Set<AnyCancellable>()
    
    // MARK: - Initialization
    
    public init(observable: CoTObservable) {
        self.observable = observable
        setupBindings()
    }
    
    // MARK: - Computed Properties
    
    /// Formatted event count string for UI display
    public var eventCountText: String {
        eventCount == 1 ? "1 event" : "\(eventCount) events"
    }
    
    /// Formatted chat count string for UI display
    public var chatCountText: String {
        chatMessageCount == 1 ? "1 message" : "\(chatMessageCount) messages"
    }
    
    /// Formatted emergency count string for UI display
    public var emergencyCountText: String {
        emergencyCount == 0 ? "No emergencies" : 
        emergencyCount == 1 ? "1 emergency" : "\(emergencyCount) emergencies"
    }
    
    /// Connection status icon name
    public var connectionIcon: String {
        switch connectionHealth {
        case .excellent: return "wifi"
        case .good: return "wifi"
        case .poor: return "wifi.exclamationmark"
        case .disconnected: return "wifi.slash"
        case .unknown: return "questionmark.circle"
        }
    }
    
    /// Connection status color
    public var connectionColor: Color {
        switch connectionHealth {
        case .excellent: return .green
        case .good: return .blue
        case .poor: return .orange
        case .disconnected: return .red
        case .unknown: return .gray
        }
    }
    
    // MARK: - Binding Methods
    
    /// Create a binding for filtering events by callsign
    public func callsignFilterBinding(for callsign: String) -> Binding<Bool> {
        Binding(
            get: { [weak self] in
                // This would integrate with a filter system
                self?.activeCallsigns.contains(callsign) ?? false
            },
            set: { [weak self] isSelected in
                if isSelected {
                    self?.activeCallsigns.insert(callsign)
                } else {
                    self?.activeCallsigns.remove(callsign)
                }
            }
        )
    }
    
    /// Create a binding for real-time event updates
    public func eventPublisher(for eventType: String) -> AnyPublisher<[CoTEventModel], Never> {
        observable.$events
            .map { documents in
                documents.compactMap { CoTEventModel(from: $0) }
                    .filter { $0.type == eventType }
            }
            .eraseToAnyPublisher()
    }
    
    /// Create a publisher for location updates of a specific callsign
    public func locationPublisher(for callsign: String) -> AnyPublisher<LocationUpdateModel?, Never> {
        observable.locationUpdatesPublisher
            .map { documents in
                documents.compactMap { LocationUpdateModel(from: $0) }
                    .first { $0.callsign == callsign }
            }
            .eraseToAnyPublisher()
    }
    
    // MARK: - Private Methods
    
    private func setupBindings() {
        // Event count binding
        observable.$events
            .map { $0.count }
            .receive(on: DispatchQueue.main)
            .assign(to: \.eventCount, on: self)
            .store(in: &cancellables)
        
        // Chat message count binding
        observable.chatMessagesPublisher
            .map { $0.count }
            .receive(on: DispatchQueue.main)
            .assign(to: \.chatMessageCount, on: self)
            .store(in: &cancellables)
        
        // Emergency count binding
        observable.emergencyEventsPublisher
            .map { $0.count }
            .receive(on: DispatchQueue.main)
            .assign(to: \.emergencyCount, on: self)
            .store(in: &cancellables)
        
        // Last event time binding
        observable.$events
            .compactMap { documents in
                documents.compactMap { CoTEventModel(from: $0) }
                    .map(\.timestamp)
                    .max()
            }
            .receive(on: DispatchQueue.main)
            .assign(to: \.lastEventTime, on: self)
            .store(in: &cancellables)
        
        // Active callsigns binding
        observable.$events
            .map { documents in
                Set(documents.compactMap { CoTEventModel(from: $0) }
                    .map(\.callsign))
            }
            .receive(on: DispatchQueue.main)
            .assign(to: \.activeCallsigns, on: self)
            .store(in: &cancellables)
        
        // Connection health monitoring
        setupConnectionHealthMonitoring()
    }
    
    private func setupConnectionHealthMonitoring() {
        // Combine connection status with event flow to determine health
        Publishers.CombineLatest(
            observable.connectionPublisher,
            observable.$events.map { _ in Date() }
        )
        .debounce(for: .seconds(2), scheduler: DispatchQueue.main)
        .map { isConnected, _ in
            if !isConnected {
                return ConnectionHealth.disconnected
            }
            
            // Could add more sophisticated health checks here
            // For now, connected = good
            return ConnectionHealth.good
        }
        .receive(on: DispatchQueue.main)
        .assign(to: \.connectionHealth, on: self)
        .store(in: &cancellables)
    }
}

// MARK: - Supporting Types

public enum ConnectionHealth: String, CaseIterable {
    case excellent = "excellent"
    case good = "good" 
    case poor = "poor"
    case disconnected = "disconnected"
    case unknown = "unknown"
    
    public var displayName: String {
        switch self {
        case .excellent: return "Excellent"
        case .good: return "Good"
        case .poor: return "Poor"
        case .disconnected: return "Disconnected"
        case .unknown: return "Unknown"
        }
    }
}

// MARK: - SwiftUI Environment

@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
public struct CoTBindingKey: EnvironmentKey {
    public static let defaultValue: CoTBinding? = nil
}

@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
public extension EnvironmentValues {
    var cotBinding: CoTBinding? {
        get { self[CoTBindingKey.self] }
        set { self[CoTBindingKey.self] = newValue }
    }
}

// MARK: - View Modifiers

@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
public extension View {
    /// Inject CoT binding into the environment
    func cotBinding(_ binding: CoTBinding) -> some View {
        environment(\.cotBinding, binding)
    }
    
    /// Auto-refresh when events change
    func cotAutoRefresh<T: Equatable>(
        on keyPath: KeyPath<CoTBinding, T>,
        binding: CoTBinding,
        perform action: @escaping () -> Void
    ) -> some View {
        onReceive(binding.publisher(for: keyPath).removeDuplicates()) { _ in
            action()
        }
    }
}

// MARK: - Publisher Extensions

@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
extension CoTBinding {
    /// Create a publisher for any property
    public func publisher<T>(for keyPath: KeyPath<CoTBinding, T>) -> AnyPublisher<T, Never> {
        // For now, return a simple empty publisher - this needs more sophisticated implementation
        return Empty<T, Never>()
            .eraseToAnyPublisher()
    }
}

// MARK: - Convenience Bindings

@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
public extension CoTBinding {
    
    /// Binding for search text with real-time filtering
    func searchBinding(
        initialValue: String = "",
        onUpdate: @escaping (String) -> Void = { _ in }
    ) -> Binding<String> {
        Binding(
            get: { initialValue },
            set: { newValue in
                onUpdate(newValue)
            }
        )
    }
    
    /// Binding for time range selection
    func timeRangeBinding(
        initialValue: TimeInterval = 3600,
        onUpdate: @escaping (TimeInterval) -> Void = { _ in }
    ) -> Binding<TimeInterval> {
        Binding(
            get: { initialValue },
            set: { newValue in
                onUpdate(newValue)
            }
        )
    }
    
    /// Binding for emergency alert visibility
    var showEmergencyAlert: Binding<Bool> {
        Binding(
            get: { [weak self] in
                (self?.emergencyCount ?? 0) > 0
            },
            set: { _ in
                // Emergency alerts are read-only from the data
            }
        )
    }
}