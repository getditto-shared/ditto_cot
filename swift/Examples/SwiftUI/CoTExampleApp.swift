import SwiftUI
import DittoSwift
import DittoCoT
import DittoCoTCore
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
class AppEnvironment: ObservableObject {
    let ditto: Ditto
    let dittoCoT: DittoCoT
    let observable: CoTObservable
    let cotBinding: CoTBinding
    
    init() {
        // Load environment variables
        let environment = EnvironmentLoader.loadEnvironment()
        
        // Initialize Ditto with SharedKey identity using environment variables
        do {
            let appId = try EnvironmentLoader.requireEnvironmentVariable("DITTO_APP_ID", from: environment)
            let sharedKey = try EnvironmentLoader.requireEnvironmentVariable("DITTO_SHARED_KEY", from: environment)
            let licenseToken = try EnvironmentLoader.requireEnvironmentVariable("DITTO_LICENSE_TOKEN", from: environment)
            
            // Create unique persistence directory for example app
            let documentsPath = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask).first!
            let exampleAppPersistenceDir = documentsPath.appendingPathComponent("DittoCoTExample")
            
            let ditto = Ditto(
                identity: .sharedKey(
                    appID: appId,
                    sharedKey: sharedKey,
                    siteID: UInt64.random(in: 1...UInt64.max) // Generate random site ID for this instance
                ),
                persistenceDirectory: exampleAppPersistenceDir
            )
            
            // Activate Ditto with license token
            try ditto.setOfflineOnlyLicenseToken(licenseToken)
            
            self.ditto = ditto
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
        
        // Initialize CoT integration
        self.dittoCoT = DittoCoT(ditto: ditto)
        self.observable = CoTObservable(dittoCoT: dittoCoT)
        self.cotBinding = CoTBinding(observable: observable)
        
        // Start Ditto sync
        do {
            try ditto.startSync()
            print("Ditto sync started successfully")
            
            // Start observing CoT events
            observable.startObserving()
            print("CoT event observation started")
            
            // Perform initial refresh
            observable.refreshAll()
            print("Initial data refresh completed")
        } catch {
            print("Failed to start Ditto sync: \(error)")
        }
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
            NavigationView {
                CoTEventListView(observable: appEnvironment.observable)
            }
            .tabItem {
                Image(systemName: "list.bullet")
                Text("Events")
            }
            .badge(cotBinding?.eventCount ?? 0)
            .tag(0)
            
            // Map tab
            NavigationView {
                CoTMapView(observable: appEnvironment.observable)
            }
            .tabItem {
                Image(systemName: "map")
                Text("Map")
            }
            .tag(1)
            
            // Chat tab
            NavigationView {
                CoTChatView(observable: appEnvironment.observable)
            }
            .tabItem {
                Image(systemName: "message")
                Text("Chat")
            }
            .badge(cotBinding?.chatMessageCount ?? 0)
            .tag(2)
            
            // Dashboard tab
            NavigationView {
                DashboardView()
            }
            .tabItem {
                Image(systemName: "chart.bar")
                Text("Dashboard")
            }
            .tag(3)
            
            // Debug/Presence tab
            NavigationView {
                PresenceDebugView(observable: appEnvironment.observable)
                    .navigationTitle("Debug")
            }
            .tabItem {
                Image(systemName: "network")
                Text("Presence")
            }
            .badge(appEnvironment.observable.connectedPeers.count)
            .tag(4)
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
                
                Button("Send Test Chat Message") {
                    sendTestChatMessage()
                }
                .buttonStyle(.bordered)
                
                Button("Refresh All Events") {
                    appEnvironment.observable.refreshAll()
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
        Task {
            do {
                _ = try await appEnvironment.observable.insert(
                    CoTEventBuilder()
                        .uid("test-\(UUID().uuidString)")
                        .type("a-f-G-U-C")
                        .how("m-g")
                        .point(CoTPoint(
                            lat: 37.7749 + Double.random(in: -0.01...0.01),
                            lon: -122.4194 + Double.random(in: -0.01...0.01)
                        ))
                        .detail(CoTDetail([
                            "contact": ["callsign": "TEST-USER"]
                        ]))
                        .build()
                )
            } catch {
                print("Failed to send test location: \(error)")
            }
        }
    }
    
    private func sendTestChatMessage() {
        Task {
            do {
                _ = try await appEnvironment.observable.insert(
                    CoTEventBuilder()
                        .uid("chat-\(UUID().uuidString)")
                        .type("b-t-f")
                        .how("h-e")
                        .point(CoTPoint(lat: 0, lon: 0))
                        .detail(CoTDetail([
                            "chat": [
                                "from": "TEST-USER",
                                "room": "All Chat Rooms",
                                "msg": "Hello from the example app!"
                            ]
                        ]))
                        .build()
                )
            } catch {
                print("Failed to send test chat: \(error)")
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