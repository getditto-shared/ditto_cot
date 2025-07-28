import SwiftUI
import MapKit
import CoreLocation

/// SwiftUI view for displaying CoT events on a map
@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
public struct CoTMapView: View {
    @StateObject private var viewModel: CoTEventViewModel
    @StateObject private var locationManager = LocationManager()
    @State private var region = MKCoordinateRegion(
        center: CLLocationCoordinate2D(latitude: 37.7749, longitude: -122.4194), // Default to SF
        span: MKCoordinateSpan(latitudeDelta: 0.1, longitudeDelta: 0.1)
    )
    @State private var selectedEvent: CoTEventModel?
    @State private var showingLocationSheet = false
    @State private var isFullScreen = false // Start normal, allow full screen toggle
    
    public init(observable: CoTObservable) {
        self._viewModel = StateObject(wrappedValue: CoTEventViewModel(observable: observable))
    }
    
    public var body: some View {
        ZStack {
            // Map - full screen
            Map(coordinateRegion: $region, annotationItems: viewModel.filteredEvents) { event in
                MapAnnotation(coordinate: event.location) {
                    EventAnnotation(event: event) {
                        selectedEvent = event
                    }
                }
            }
            .ignoresSafeArea(.container, edges: isFullScreen ? .all : .bottom) // Respect tab bar unless full screen
            
            // Overlay controls
            VStack {
                // Top controls
                HStack {
                    // Full screen toggle (top left)
                    Button(action: { isFullScreen.toggle() }) {
                        Image(systemName: isFullScreen ? "arrow.down.right.and.arrow.up.left" : "arrow.up.left.and.arrow.down.right")
                            .foregroundColor(.white)
                            .frame(width: 44, height: 44)
                            .background(Color.black.opacity(0.7))
                            .cornerRadius(8)
                    }
                    
                    Spacer()
                    
                    // Right side controls
                    VStack(spacing: 12) {
                        // Zoom in
                        Button(action: zoomIn) {
                            Image(systemName: "plus")
                                .foregroundColor(.white)
                                .frame(width: 44, height: 44)
                                .background(Color.black.opacity(0.7))
                                .cornerRadius(8)
                        }
                        
                        // Zoom out
                        Button(action: zoomOut) {
                            Image(systemName: "minus")
                                .foregroundColor(.white)
                                .frame(width: 44, height: 44)
                                .background(Color.black.opacity(0.7))
                                .cornerRadius(8)
                        }
                        
                        // Center on user location
                        Button(action: centerOnUserLocation) {
                            Image(systemName: "location.fill")
                                .foregroundColor(.white)
                                .frame(width: 44, height: 44)
                                .background(Color.blue.opacity(0.8))
                                .cornerRadius(8)
                        }
                        
                        // Send location update
                        Button(action: { showingLocationSheet = true }) {
                            Image(systemName: "plus.circle.fill")
                                .foregroundColor(.white)
                                .frame(width: 44, height: 44)
                                .background(Color.green.opacity(0.8))
                                .cornerRadius(8)
                        }
                        
                        // Refresh events
                        Button(action: { viewModel.refreshEvents() }) {
                            Image(systemName: "arrow.clockwise")
                                .foregroundColor(.white)
                                .frame(width: 44, height: 44)
                                .background(Color.orange.opacity(0.8))
                                .cornerRadius(8)
                        }
                    }
                }
                .padding(.top, isFullScreen ? 50 : 20)
                .padding(.horizontal)
                
                Spacer()
                
                // Bottom status bar
                HStack {
                    Text("\(viewModel.filteredEvents.count) events")
                        .padding(8)
                        .background(Color.black.opacity(0.7))
                        .foregroundColor(.white)
                        .cornerRadius(8)
                    
                    Spacer()
                    
                    ConnectionStatusView(isConnected: viewModel.isConnected)
                        .padding(8)
                        .background(Color.black.opacity(0.7))
                        .cornerRadius(8)
                }
                .padding(.bottom, isFullScreen ? 20 : 40)
                .padding(.horizontal)
            }
        }
        .navigationTitle(isFullScreen ? "" : "CoT Map")
        #if os(iOS)
        .navigationBarHidden(isFullScreen)
        .navigationBarTitleDisplayMode(.inline)
        #endif
        #if os(macOS)
        .modifier(MacOSToolbarModifier(isFullScreen: isFullScreen))
        #endif
        .onAppear {
            viewModel.refreshEvents()
            locationManager.requestLocationPermission()
        }
        .onChange(of: locationManager.location) { location in
            if let location = location {
                // Automatically center on user location when first received
                withAnimation(.easeInOut(duration: 0.8)) {
                    region.center = location.coordinate
                    // Also adjust zoom level to a good default for local area
                    region.span = MKCoordinateSpan(latitudeDelta: 0.01, longitudeDelta: 0.01)
                }
            }
        }
        .sheet(item: $selectedEvent) { event in
            CoTEventDetailView(event: event)
        }
        .sheet(isPresented: $showingLocationSheet) {
            SendLocationSheet(
                viewModel: viewModel,
                currentLocation: locationManager.location?.coordinate
            )
        }
    }
    
    private func centerOnUserLocation() {
        if let location = locationManager.location {
            withAnimation(.easeInOut(duration: 0.5)) {
                region.center = location.coordinate
            }
        } else {
            locationManager.requestLocationPermission()
        }
    }
    
    private func zoomIn() {
        withAnimation(.easeInOut(duration: 0.3)) {
            let currentSpan = region.span
            region.span = MKCoordinateSpan(
                latitudeDelta: max(currentSpan.latitudeDelta * 0.5, 0.001), // Min zoom limit
                longitudeDelta: max(currentSpan.longitudeDelta * 0.5, 0.001)
            )
        }
    }
    
    private func zoomOut() {
        withAnimation(.easeInOut(duration: 0.3)) {
            let currentSpan = region.span
            region.span = MKCoordinateSpan(
                latitudeDelta: min(currentSpan.latitudeDelta * 2.0, 180.0), // Max zoom limit
                longitudeDelta: min(currentSpan.longitudeDelta * 2.0, 180.0)
            )
        }
    }
}

@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
struct EventAnnotation: View {
    let event: CoTEventModel
    let onTap: () -> Void
    
    var body: some View {
        Button(action: onTap) {
            ZStack {
                // Background circle
                Circle()
                    .fill(backgroundColor)
                    .frame(width: 32, height: 32)
                    .overlay(
                        Circle()
                            .stroke(Color.white, lineWidth: 2)
                    )
                
                // Icon
                Image(systemName: event.eventCategory.systemImage)
                    .foregroundColor(.white)
                    .font(.system(size: 14, weight: .bold))
            }
            .opacity(event.isStale ? 0.6 : 1.0)
            .scaleEffect(event.isStale ? 0.8 : 1.0)
        }
        .buttonStyle(PlainButtonStyle())
    }
    
    private var backgroundColor: Color {
        if event.isStale {
            return .gray
        }
        
        switch event.eventCategory {
        case .friendly: return .green
        case .hostile: return .red
        case .neutral: return .yellow
        case .unknown: return .blue
        case .chat: return .purple
        case .emergency: return .orange
        }
    }
}

@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
struct SendLocationSheet: View {
    @ObservedObject var viewModel: CoTEventViewModel
    let currentLocation: CLLocationCoordinate2D?
    @Environment(\.dismiss) private var dismiss
    
    @State private var callsign = "USER-1"
    @State private var customLat = ""
    @State private var customLon = ""
    @State private var useCurrentLocation = true
    @State private var isLoading = false
    
    var body: some View {
        NavigationStack {
            Form {
                Section(header: Text("Callsign")) {
                    TextField("Enter callsign", text: $callsign)
                        #if os(iOS)
                        .autocapitalization(.allCharacters)
                        #endif
                }
                
                Section(header: Text("Location")) {
                    Toggle("Use Current Location", isOn: $useCurrentLocation)
                    
                    if !useCurrentLocation {
                        HStack {
                            Text("Latitude")
                            TextField("0.0", text: $customLat)
                                #if os(iOS)
                                .keyboardType(.decimalPad)
                                #endif
                                .textFieldStyle(RoundedBorderTextFieldStyle())
                        }
                        
                        HStack {
                            Text("Longitude") 
                            TextField("0.0", text: $customLon)
                                #if os(iOS)
                                .keyboardType(.decimalPad)
                                #endif
                                .textFieldStyle(RoundedBorderTextFieldStyle())
                        }
                    }
                }
                
                if useCurrentLocation && currentLocation == nil {
                    Section {
                        Text("Location permission required")
                            .foregroundColor(.secondary)
                    }
                }
            }
            .navigationTitle("Send Location")
            #if os(iOS)
            .navigationBarTitleDisplayMode(.inline)
            #endif
            .toolbar {
                #if os(iOS)
                ToolbarItem(placement: .navigationBarLeading) {
                    Button("Cancel") {
                        dismiss()
                    }
                }
                
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button("Send") {
                        sendLocationUpdate()
                    }
                    .disabled(isLoading || !canSendLocation)
                }
                #else
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") {
                        dismiss()
                    }
                }
                
                ToolbarItem(placement: .confirmationAction) {
                    Button("Send") {
                        sendLocationUpdate()
                    }
                    .disabled(isLoading || !canSendLocation)
                }
                #endif
            }
        }
    }
    
    private var canSendLocation: Bool {
        guard !callsign.isEmpty else { return false }
        
        if useCurrentLocation {
            return currentLocation != nil
        } else {
            return !customLat.isEmpty && !customLon.isEmpty
        }
    }
    
    private func sendLocationUpdate() {
        guard let location = targetLocation else { return }
        
        isLoading = true
        
        Task {
            do {
                try await viewModel.sendLocationUpdate(
                    callsign: callsign,
                    location: location
                )
                
                await MainActor.run {
                    dismiss()
                }
            } catch {
                // Handle error
                print("Failed to send location: \(error)")
            }
            
            await MainActor.run {
                isLoading = false
            }
        }
    }
    
    private var targetLocation: CLLocationCoordinate2D? {
        if useCurrentLocation {
            return currentLocation
        } else {
            guard let lat = Double(customLat),
                  let lon = Double(customLon) else {
                return nil
            }
            return CLLocationCoordinate2D(latitude: lat, longitude: lon)
        }
    }
}

// MARK: - Location Manager

@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
class LocationManager: NSObject, ObservableObject, CLLocationManagerDelegate {
    private let manager = CLLocationManager()
    
    @Published var location: CLLocation?
    @Published var authorizationStatus: CLAuthorizationStatus = .notDetermined
    
    override init() {
        super.init()
        manager.delegate = self
        manager.desiredAccuracy = kCLLocationAccuracyBest
    }
    
    func requestLocationPermission() {
        switch authorizationStatus {
        case .notDetermined:
            manager.requestWhenInUseAuthorization()
        case .denied, .restricted:
            // Could show alert to go to settings
            break
        case .authorizedWhenInUse, .authorizedAlways:
            manager.startUpdatingLocation()
        @unknown default:
            break
        }
    }
    
    // MARK: - CLLocationManagerDelegate
    
    func locationManager(_ manager: CLLocationManager, didUpdateLocations locations: [CLLocation]) {
        location = locations.last
    }
    
    func locationManager(_ manager: CLLocationManager, didChangeAuthorization status: CLAuthorizationStatus) {
        authorizationStatus = status
        
        switch status {
        case .authorizedWhenInUse, .authorizedAlways:
            manager.startUpdatingLocation()
        case .denied, .restricted:
            manager.stopUpdatingLocation()
        case .notDetermined:
            break
        @unknown default:
            break
        }
    }
    
    func locationManager(_ manager: CLLocationManager, didFailWithError error: Error) {
        print("Location error: \(error)")
    }
}

// MARK: - macOS Compatibility

#if os(macOS)
@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
struct MacOSToolbarModifier: ViewModifier {
    let isFullScreen: Bool
    
    func body(content: Content) -> some View {
        if #available(macOS 13.0, *) {
            content.toolbar(isFullScreen ? .hidden : .visible)
        } else {
            content
        }
    }
}
#endif