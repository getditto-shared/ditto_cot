import SwiftUI
import DittoSwift
import DittoCoT
import DittoCoTCore
import CoreLocation
#if os(macOS)
import AppKit
#endif

/// Example SwiftUI app demonstrating Ditto CoT integration
@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
@main
struct CoTExampleApp: App {
    @StateObject private var appEnvironment = AppEnvironment()
    
    var body: some Scene {
        WindowGroup {
            ContentView()
                .environmentObject(appEnvironment)
                .cotBinding(appEnvironment.cotBinding)
        }
    }
}

@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
class AppEnvironment: NSObject, ObservableObject {
    let ditto: Ditto
    let dittoCoT: DittoCoT
    let observable: CoTObservable
    let trackObservable: TrackObservable
    let cotBinding: CoTBinding
    @Published var userCallsign: String = "USER-1"
    @Published var isTracking: Bool = false
    
    // Track state
    private var trackTimer: Timer?
    private var currentTrackPosition = CLLocationCoordinate2D(latitude: 37.7749, longitude: -122.4194)
    private var currentSpeed: Double = 10.0 // knots
    private var currentCourse: Double = 45.0 // degrees
    
    // Location manager for getting actual device location
    private let locationManager = CLLocationManager()
    @Published var currentLocation: CLLocation?
    @Published var locationAuthorizationStatus: CLAuthorizationStatus = .notDetermined
    
    override init() {
        // Load environment variables
        let environment = EnvironmentLoader.loadEnvironment()
        
        // Initialize Ditto with SharedKey identity using environment variables
        let ditto: Ditto
        do {
            let appId = try EnvironmentLoader.requireEnvironmentVariable("DITTO_APP_ID", from: environment)
            let sharedKey = try EnvironmentLoader.requireEnvironmentVariable("DITTO_SHARED_KEY", from: environment)
            let licenseToken = try EnvironmentLoader.requireEnvironmentVariable("DITTO_LICENSE_TOKEN", from: environment)
            
            // Create unique persistence directory for example app with timestamp to avoid conflicts
            let documentsPath = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask).first!
            let timestamp = Int(Date().timeIntervalSince1970)
            let exampleAppPersistenceDir = documentsPath.appendingPathComponent("DittoCoTExample_\(timestamp)")
            
            do {
                print("🔧 Initializing Ditto with App ID: \(appId)")
                print("🔧 Using persistence directory: \(exampleAppPersistenceDir.path)")
                
                ditto = Ditto(
                    identity: .sharedKey(
                        appID: appId,
                        sharedKey: sharedKey,
                        siteID: UInt64.random(in: 1...UInt64.max) // Generate random site ID for this instance
                    ),
                    persistenceDirectory: exampleAppPersistenceDir
                )
                print("✅ Ditto instance created successfully")
                
                // Activate Ditto with license token
                print("🔧 Setting license token...")
                try ditto.setOfflineOnlyLicenseToken(licenseToken)
                print("✅ License token set successfully")
            } catch {
                print("❌ Failed to initialize Ditto: \(error)")
                print("❌ Error details: \(error.localizedDescription)")
                throw error
            }
            
            print("Ditto initialized with App ID: \(appId)")
            print("Ditto activated with license token")
        } catch {
            // Environment variables not configured - exit with helpful message
            print("❌ Environment variables not configured: \(error)")
            print("Please configure your .env file with valid Ditto credentials:")
            print("1. Copy .env.example to .env")
            print("2. Get credentials from https://portal.ditto.live")
            print("3. Fill in DITTO_APP_ID, DITTO_SHARED_KEY, and DITTO_LICENSE_TOKEN")
            
            fatalError("Ditto credentials required. Please configure .env file.")
        }
        
        self.ditto = ditto
        
        // Initialize CoT integration
        self.dittoCoT = DittoCoT(ditto: ditto)
        self.observable = CoTObservable(dittoCoT: dittoCoT)
        self.trackObservable = TrackObservable(ditto: ditto)
        self.cotBinding = CoTBinding(observable: observable)
        
        super.init()
        
        // Start Ditto sync
        do {
            try ditto.startSync()
            print("Ditto sync started successfully")
            
            // Start observing CoT events
            observable.startObserving()
            print("CoT event observation started")
            
            // Start observing track events
            trackObservable.startObserving()
            print("Track observation started")
            
            // Perform initial refresh
            observable.refreshAll()
            trackObservable.refreshTracks()
            print("Initial data refresh completed")
            
            // Setup location manager
            setupLocationManager()
            
            // Don't send debug track automatically - wait for user to interact with map or start tracking
        } catch {
            print("Failed to start Ditto sync: \(error)")
        }
    }
    
    // MARK: - Tracking Methods
    
    func startTracking() {
        guard !isTracking else { return }
        
        // Check if we have location permission and actual location data
        #if os(macOS)
        guard locationAuthorizationStatus == .authorizedAlways else {
            print("⚠️ Cannot start tracking - location permission not granted")
            print("⚠️ Please allow location access first by visiting the Map tab")
            return
        }
        #else
        guard locationAuthorizationStatus == .authorizedWhenInUse || locationAuthorizationStatus == .authorizedAlways else {
            print("⚠️ Cannot start tracking - location permission not granted")
            print("⚠️ Please allow location access first by visiting the Map tab")
            return
        }
        #endif
        
        guard currentLocation != nil else {
            print("⚠️ Cannot start tracking - no current location available")
            print("⚠️ Please allow location access and wait for GPS fix by visiting the Map tab")
            return
        }
        
        print("🎯 Starting track updates every 10 seconds with actual device location...")
        isTracking = true
        
        // Send initial track
        sendTrackUpdate()
        
        // Schedule updates every 10 seconds
        trackTimer = Timer.scheduledTimer(withTimeInterval: 10.0, repeats: true) { _ in
            self.sendTrackUpdate()
        }
    }
    
    func stopTracking() {
        print("🛑 Stopping track updates")
        isTracking = false
        trackTimer?.invalidate()
        trackTimer = nil
    }
    
    private func sendTrackUpdate() {
        Task {
            do {
                // Only use actual device location - no fallback to simulated
                guard let currentLoc = currentLocation else {
                    print("⚠️ No current location available - skipping track update")
                    print("⚠️ Please ensure location services are enabled and GPS has a fix")
                    return
                }
                
                let trackPosition = currentLoc.coordinate
                print("📍 Using actual device location: \(trackPosition.latitude), \(trackPosition.longitude)")
                
                // Use consistent UID for this track
                let trackUID = "usv-track-\(userCallsign)"
                
                print("📍 Sending track update for \(trackUID)")
                print("   Position: \(trackPosition.latitude), \(trackPosition.longitude)")
                print("   Speed: \(currentSpeed) knots, Course: \(currentCourse)°")
                
                let event = try CoTEventBuilder()
                    .uid(trackUID)
                    .type("a-f-S-X-M")  // Friendly surface vessel
                    .how("m-g")
                    .point(CoTPoint(
                        lat: trackPosition.latitude,
                        lon: trackPosition.longitude,
                        hae: currentLocation?.altitude ?? 0.0
                    ))
                    .detail(CoTDetail([
                        "contact": ["callsign": "USV-\(userCallsign)"],
                        "track": [
                            "speed": currentSpeed,
                            "course": currentCourse
                        ]
                    ]))
                    .build()
                
                // This will update the existing document if it exists
                _ = try await observable.insert(event)
                print("✅ Track update sent successfully")
            } catch {
                print("❌ Failed to send track update: \(error)")
            }
        }
    }
    
    private func updateTrackPosition() {
        // Convert course to radians
        let courseRadians = currentCourse * .pi / 180.0
        
        // Calculate distance traveled in 10 seconds (in nautical miles)
        let distanceNM = (currentSpeed * 10.0) / 3600.0
        
        // Convert to degrees (approximately)
        let latChange = (distanceNM / 60.0) * cos(courseRadians)
        let lonChange = (distanceNM / 60.0) * sin(courseRadians) / cos(currentTrackPosition.latitude * .pi / 180.0)
        
        // Update position
        currentTrackPosition.latitude += latChange
        currentTrackPosition.longitude += lonChange
        
        // Add some variation to speed and course
        currentSpeed += Double.random(in: -1...1)
        currentSpeed = max(5, min(15, currentSpeed)) // Keep between 5-15 knots
        
        currentCourse += Double.random(in: -5...5)
        if currentCourse < 0 { currentCourse += 360 }
        if currentCourse >= 360 { currentCourse -= 360 }
    }
    
    // Setup location manager to get device location
    private func setupLocationManager() {
        locationManager.delegate = self
        locationManager.desiredAccuracy = kCLLocationAccuracyBest
        
        // Initialize authorization status
        locationAuthorizationStatus = locationManager.authorizationStatus
        
        // Request location permissions  
        switch locationAuthorizationStatus {
        case .notDetermined:
            locationManager.requestWhenInUseAuthorization()
        case .authorizedWhenInUse, .authorizedAlways:
            locationManager.startUpdatingLocation()
        default:
            print("⚠️ Location access not authorized - using simulated location")
        }
    }
    
    // Debug method to send test track on startup
    private func sendDebugTrackOnStartup() {
        print("🚀 SENDING DEBUG TRACK ON STARTUP")
        Task {
            do {
                let event = try CoTEventBuilder()
                    .uid("debug-startup-track")
                    .type("a-f-S-X-M")  // Surface vessel
                    .how("m-g")
                    .point(CoTPoint(
                        lat: 37.7749,
                        lon: -122.4194,
                        hae: 0.0
                    ))
                    .detail(CoTDetail([
                        "contact": ["callsign": "DEBUG-TRACK"]
                    ]))
                    .build()
                
                print("🚀 Inserting debug track event: \(event.uid)")
                _ = try await observable.insert(event)
                print("🚀 Debug track sent successfully!")
            } catch {
                print("❌ Failed to send debug track: \(error)")
            }
        }
    }
}

// MARK: - CLLocationManagerDelegate
@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
extension AppEnvironment: CLLocationManagerDelegate {
    func locationManager(_ manager: CLLocationManager, didUpdateLocations locations: [CLLocation]) {
        if let location = locations.last {
            DispatchQueue.main.async {
                self.currentLocation = location
                print("📍 Updated device location: \(location.coordinate.latitude), \(location.coordinate.longitude)")
                
                // Update current track position for first time setup
                if self.currentTrackPosition.latitude == 37.7749 && self.currentTrackPosition.longitude == -122.4194 {
                    self.currentTrackPosition = location.coordinate
                    print("📍 Set initial track position to device location")
                }
            }
        }
    }
    
    func locationManager(_ manager: CLLocationManager, didChangeAuthorization status: CLAuthorizationStatus) {
        DispatchQueue.main.async {
            self.locationAuthorizationStatus = status
            switch status {
            case .authorizedWhenInUse, .authorizedAlways:
                self.locationManager.startUpdatingLocation()
                print("📍 Location access authorized - starting location updates")
            case .denied, .restricted:
                print("⚠️ Location access denied - will use simulated location")
            case .notDetermined:
                print("📍 Location authorization not determined")
            @unknown default:
                print("📍 Unknown location authorization status")
            }
        }
    }
    
    func locationManager(_ manager: CLLocationManager, didFailWithError error: Error) {
        print("❌ Location manager error: \(error)")
    }
}

@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
struct ContentView: View {
    @EnvironmentObject private var appEnvironment: AppEnvironment
    @Environment(\.cotBinding) private var cotBinding
    @State private var selectedTab = 0
    
    var body: some View {
        TabView(selection: $selectedTab) {
            // Events tab
            CoTEventListView(observable: appEnvironment.observable)
                .tabItem {
                    Image(systemName: "list.bullet")
                    Text("Events")
                }
                .badge(cotBinding?.eventCount ?? 0)
                .tag(0)
            
            // Map tab
            CoTMapView(observable: appEnvironment.observable, trackObservable: appEnvironment.trackObservable)
                .tabItem {
                    Image(systemName: "map")
                    Text("Map")
                }
                .tag(1)
            
            // Chat tab
            CoTChatView(observable: appEnvironment.observable, callsign: $appEnvironment.userCallsign)
                .tabItem {
                    Image(systemName: "message")
                    Text("Chat")
                }
                .badge(cotBinding?.chatMessageCount ?? 0)
                .tag(2)
            
            // Tracks tab
            CoTTrackView(trackObservable: appEnvironment.trackObservable)
                .tabItem {
                    Image(systemName: "location.circle")
                    Text("Tracks")
                }
                .badge(appEnvironment.trackObservable.trackCount)
                .tag(3)
            
            // Dashboard tab
            DashboardView()
                .tabItem {
                    Image(systemName: "chart.bar")
                    Text("Dashboard")
                }
                .tag(4)
            
            // Debug/Presence tab
            PresenceDebugView(observable: appEnvironment.observable)
                .navigationTitle("Debug")
                .tabItem {
                    Image(systemName: "network")
                    Text("Presence")
                }
                .badge(appEnvironment.observable.connectedPeers.count)
                .tag(5)
        }
        .overlay(alignment: .top) {
            // Emergency alert banner
            if let cotBinding = cotBinding, cotBinding.emergencyCount > 0 {
                EmergencyBanner(count: cotBinding.emergencyCount)
                    .transition(.move(edge: .top))
                    .animation(.easeInOut, value: cotBinding.emergencyCount)
            }
        }
    }
}

@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
struct DashboardView: View {
    @Environment(\.cotBinding) private var cotBinding
    @EnvironmentObject private var appEnvironment: AppEnvironment
    
    var body: some View {
        ScrollView {
            LazyVStack(spacing: 16) {
                // Status cards
                HStack(spacing: 16) {
                    StatusCard(
                        title: "Events",
                        value: "\(cotBinding?.eventCount ?? 0)",
                        icon: "list.bullet",
                        color: .blue
                    )
                    
                    StatusCard(
                        title: "Messages",
                        value: "\(cotBinding?.chatMessageCount ?? 0)",
                        icon: "message",
                        color: .green
                    )
                }
                
                HStack(spacing: 16) {
                    StatusCard(
                        title: "Emergencies",
                        value: "\(cotBinding?.emergencyCount ?? 0)",
                        icon: "exclamationmark.triangle",
                        color: .red
                    )
                    
                    StatusCard(
                        title: "Active Users",
                        value: "\(cotBinding?.activeCallsigns.count ?? 0)",
                        icon: "person.2",
                        color: .orange
                    )
                }
                
                // Connection status
                ConnectionStatusCard(
                    health: cotBinding?.connectionHealth ?? .unknown,
                    lastUpdate: cotBinding?.lastEventTime
                )
                
                // Tracking status
                if appEnvironment.isTracking {
                    TrackingStatusCard()
                }
                
                // Callsign input
                CallsignCard(callsign: $appEnvironment.userCallsign)
                
                // Active callsigns list
                if let cotBinding = cotBinding, !cotBinding.activeCallsigns.isEmpty {
                    ActiveCallsignsCard(callsigns: Array(cotBinding.activeCallsigns))
                }
                
                // Presence graph
                PresenceGraphView(observable: appEnvironment.observable)
                
                // Quick actions
                QuickActionsCard(appEnvironment: appEnvironment)
            }
            .padding()
        }
        .navigationTitle("Dashboard")
        .refreshable {
            appEnvironment.observable.refreshAll()
        }
    }
}

@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
struct StatusCard: View {
    let title: String
    let value: String
    let icon: String
    let color: Color
    
    var body: some View {
        VStack(spacing: 8) {
            Image(systemName: icon)
                .font(.title2)
                .foregroundColor(color)
            
            Text(value)
                .font(.title)
                .fontWeight(.bold)
            
            Text(title)
                .font(.caption)
                .foregroundColor(.secondary)
        }
        .frame(maxWidth: .infinity)
        .padding()
        #if os(iOS)
        .background(Color(.systemBackground))
        #else
        .background(Color(NSColor.windowBackgroundColor))
        #endif
        .cornerRadius(12)
        .shadow(radius: 2)
    }
}

@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
struct ConnectionStatusCard: View {
    let health: ConnectionHealth
    let lastUpdate: Date?
    
    var body: some View {
        HStack {
            Image(systemName: connectionIcon)
                .foregroundColor(connectionColor)
                .font(.title2)
            
            VStack(alignment: .leading, spacing: 4) {
                Text("Connection")
                    .font(.headline)
                
                Text(health.displayName)
                    .font(.body)
                    .foregroundColor(connectionColor)
                
                if let lastUpdate = lastUpdate {
                    Text("Last update: \(lastUpdate, style: .relative)")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
            }
            
            Spacer()
        }
        .padding()
        #if os(iOS)
        .background(Color(.systemBackground))
        #else
        .background(Color(NSColor.windowBackgroundColor))
        #endif
        .cornerRadius(12)
        .shadow(radius: 2)
    }
    
    private var connectionIcon: String {
        switch health {
        case .excellent, .good: return "wifi"
        case .poor: return "wifi.exclamationmark"
        case .disconnected: return "wifi.slash"
        case .unknown: return "questionmark.circle"
        }
    }
    
    private var connectionColor: Color {
        switch health {
        case .excellent: return .green
        case .good: return .blue
        case .poor: return .orange
        case .disconnected: return .red
        case .unknown: return .gray
        }
    }
}

@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
struct TrackingStatusCard: View {
    @EnvironmentObject private var appEnvironment: AppEnvironment
    @State private var animationPhase = 0.0
    
    var body: some View {
        HStack {
            Image(systemName: "dot.radiowaves.left.and.right")
                .foregroundColor(.green)
                .font(.title2)
            
            VStack(alignment: .leading, spacing: 4) {
                Text("Tracking Active")
                    .font(.headline)
                
                Text("USV-\(appEnvironment.userCallsign)")
                    .font(.body)
                    .foregroundColor(.secondary)
                
                Text("Sending position every 10 seconds")
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
            
            Spacer()
            
            Button("Stop") {
                appEnvironment.stopTracking()
            }
            .buttonStyle(.bordered)
            .foregroundColor(.red)
        }
        .padding()
        #if os(iOS)
        .background(Color(.systemBackground))
        #else
        .background(Color(NSColor.windowBackgroundColor))
        #endif
        .cornerRadius(12)
        .shadow(radius: 2)
        .overlay(
            RoundedRectangle(cornerRadius: 12)
                .stroke(Color.green.opacity(0.5), lineWidth: 2)
        )
    }
}

@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
struct CallsignCard: View {
    @Binding var callsign: String
    @State private var editingCallsign: String = ""
    @State private var isEditing: Bool = false
    
    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack {
                Text("Your Callsign")
                    .font(.headline)
                
                Spacer()
                
                if !isEditing {
                    Button("Edit") {
                        editingCallsign = callsign
                        isEditing = true
                    }
                    .buttonStyle(.bordered)
                }
            }
            
            if isEditing {
                HStack {
                    TextField("Enter callsign", text: $editingCallsign)
                        .textFieldStyle(.roundedBorder)
                        .onSubmit {
                            saveCallsign()
                        }
                    
                    Button("Cancel") {
                        isEditing = false
                        editingCallsign = ""
                    }
                    .buttonStyle(.bordered)
                    
                    Button("Save") {
                        saveCallsign()
                    }
                    .buttonStyle(.borderedProminent)
                    .disabled(editingCallsign.trimmingCharacters(in: .whitespaces).isEmpty)
                }
            } else {
                Text(callsign)
                    .font(.title2)
                    .fontWeight(.semibold)
                    .foregroundColor(.accentColor)
            }
        }
        .frame(maxWidth: .infinity, alignment: .leading)
        .padding()
        #if os(iOS)
        .background(Color(.systemBackground))
        #else
        .background(Color(NSColor.windowBackgroundColor))
        #endif
        .cornerRadius(12)
        .shadow(radius: 2)
    }
    
    private func saveCallsign() {
        let trimmed = editingCallsign.trimmingCharacters(in: .whitespaces)
        if !trimmed.isEmpty {
            callsign = trimmed
            isEditing = false
            editingCallsign = ""
        }
    }
}

@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
struct ActiveCallsignsCard: View {
    let callsigns: [String]
    
    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("Active Callsigns")
                .font(.headline)
            
            LazyVGrid(columns: Array(repeating: GridItem(.flexible()), count: 3), spacing: 8) {
                ForEach(callsigns.prefix(9), id: \.self) { callsign in
                    Text(callsign)
                        .font(.caption)
                        .padding(.horizontal, 8)
                        .padding(.vertical, 4)
                        .background(Color.blue.opacity(0.2))
                        .cornerRadius(8)
                }
            }
            
            if callsigns.count > 9 {
                Text("and \(callsigns.count - 9) more...")
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
        }
        .frame(maxWidth: .infinity, alignment: .leading)
        .padding()
        #if os(iOS)
        .background(Color(.systemBackground))
        #else
        .background(Color(NSColor.windowBackgroundColor))
        #endif
        .cornerRadius(12)
        .shadow(radius: 2)
    }
}

@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
struct QuickActionsCard: View {
    let appEnvironment: AppEnvironment
    
    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("Quick Actions")
                .font(.headline)
            
            VStack(spacing: 8) {
                Button("Send Test Location") {
                    sendTestLocation()
                }
                .buttonStyle(.borderedProminent)
                
                // Tracking toggle with location status
                VStack(spacing: 4) {
                    HStack {
                        Button(appEnvironment.isTracking ? "Stop Tracking" : "Start USV Tracking") {
                            if appEnvironment.isTracking {
                                appEnvironment.stopTracking()
                            } else {
                                appEnvironment.startTracking()
                            }
                        }
                        .buttonStyle(.bordered)
                        .foregroundColor(appEnvironment.isTracking ? .red : .blue)
                        
                        if appEnvironment.isTracking {
                            Image(systemName: "dot.radiowaves.left.and.right")
                                .foregroundColor(.green)
                        }
                    }
                    
                    // Show location status
                    if !appEnvironment.isTracking {
                        #if os(macOS)
                        let hasPermission = appEnvironment.locationAuthorizationStatus == .authorizedAlways
                        #else
                        let hasPermission = appEnvironment.locationAuthorizationStatus == .authorizedWhenInUse || appEnvironment.locationAuthorizationStatus == .authorizedAlways
                        #endif
                        let hasLocation = appEnvironment.currentLocation != nil
                        
                        if !hasPermission || !hasLocation {
                            Text(!hasPermission ? "Visit Map tab to enable location" : "Waiting for GPS fix...")
                                .font(.caption)
                                .foregroundColor(.orange)
                        } else {
                            Text("Ready to track")
                                .font(.caption)
                                .foregroundColor(.green)
                        }
                    }
                }
                
                Button("Send Test Chat Message") {
                    sendTestChatMessage()
                }
                .buttonStyle(.bordered)
                
                Button("Refresh All Events") {
                    print("🔄 REFRESH BUTTON CLICKED!")
                    appEnvironment.observable.refreshAll()
                    appEnvironment.trackObservable.refreshTracks()
                }
                .buttonStyle(.bordered)
            }
        }
        .frame(maxWidth: .infinity, alignment: .leading)
        .padding()
        #if os(iOS)
        .background(Color(.systemBackground))
        #else
        .background(Color(NSColor.windowBackgroundColor))
        #endif
        .cornerRadius(12)
        .shadow(radius: 2)
    }
    
    private func sendTestLocation() {
        print("🚀 TEST LOCATION BUTTON CLICKED!")
        Task {
            do {
                print("📍 Building test location event...")
                
                // Use actual location if available, otherwise use SF coordinates
                let (lat, lon): (Double, Double)
                if let currentLoc = appEnvironment.currentLocation {
                    lat = currentLoc.coordinate.latitude + Double.random(in: -0.001...0.001) // Small random offset
                    lon = currentLoc.coordinate.longitude + Double.random(in: -0.001...0.001)
                    print("📍 Using actual device location with small offset")
                } else {
                    lat = 37.7749 + Double.random(in: -0.01...0.01)
                    lon = -122.4194 + Double.random(in: -0.01...0.01)
                    print("📍 Using SF coordinates - no device location available")
                }
                
                let event = try CoTEventBuilder()
                    .uid("test-\(UUID().uuidString)")
                    .type("a-f-G-U-C")
                    .how("m-g")
                    .point(CoTPoint(lat: lat, lon: lon))
                    .detail(CoTDetail([
                        "contact": ["callsign": appEnvironment.userCallsign]
                    ]))
                    .build()
                
                print("📍 Inserting test location event: \(event.uid)")
                _ = try await appEnvironment.observable.insert(event)
                print("📍 Test location sent successfully!")
            } catch {
                print("❌ Failed to send test location: \(error)")
            }
        }
    }
    
    private func sendTestChatMessage() {
        print("💬 TEST CHAT BUTTON CLICKED!")
        Task {
            do {
                print("💬 Sending simple ATAK-style test chat...")
                let viewModel = CoTEventViewModel(observable: appEnvironment.observable)
                try await viewModel.sendChatMessage(
                    message: "Hello from the example app!",
                    room: "Ditto",
                    callsign: appEnvironment.userCallsign
                )
                print("💬 Test chat sent successfully!")
            } catch {
                print("❌ Failed to send test chat: \(error)")
            }
        }
    }
}

@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
struct EmergencyBanner: View {
    let count: Int
    
    var body: some View {
        HStack {
            Image(systemName: "exclamationmark.triangle.fill")
                .foregroundColor(.white)
            
            Text("\(count) emergency event\(count == 1 ? "" : "s") active")
                .font(.headline)
                .foregroundColor(.white)
            
            Spacer()
        }
        .padding()
        .background(Color.red)
        .cornerRadius(8)
        .padding(.horizontal)
        .shadow(radius: 4)
    }
}