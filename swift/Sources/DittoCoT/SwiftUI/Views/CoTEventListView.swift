import SwiftUI
import CoreLocation

/// Main SwiftUI view for displaying CoT events in a list
@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
public struct CoTEventListView: View {
    @StateObject private var viewModel: CoTEventViewModel
    @State private var showingFilters = false
    @State private var selectedEvent: CoTEventModel?
    
    public init(observable: CoTObservable) {
        self._viewModel = StateObject(wrappedValue: CoTEventViewModel(observable: observable))
    }
    
    public var body: some View {
        NavigationView {
            VStack {
                // Search bar
                SearchBar(text: $viewModel.searchText)
                
                // Event list
                if viewModel.isLoading {
                    ProgressView("Loading events...")
                        .frame(maxWidth: .infinity, maxHeight: .infinity)
                } else if viewModel.filteredEvents.isEmpty {
                    EmptyEventsView()
                } else {
                    List {
                        ForEach(EventCategory.allCases, id: \.self) { category in
                            if let events = viewModel.eventsByCategory[category], !events.isEmpty {
                                Section(header: CategoryHeader(category: category)) {
                                    ForEach(events) { event in
                                        CoTEventRow(event: event)
                                            .onTapGesture {
                                                selectedEvent = event
                                            }
                                            .swipeActions(edge: .trailing, allowsFullSwipe: false) {
                                                Button("Delete", role: .destructive) {
                                                    Task {
                                                        try? await viewModel.deleteEvent(event)
                                                    }
                                                }
                                            }
                                    }
                                }
                            }
                        }
                    }
                    .refreshable {
                        viewModel.refreshEvents()
                    }
                }
            }
            .navigationTitle("CoT Events")
            .toolbar {
                #if os(iOS)
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button("Filters") {
                        showingFilters = true
                    }
                }
                
                ToolbarItem(placement: .navigationBarLeading) {
                    ConnectionStatusView(isConnected: viewModel.isConnected)
                }
                #else
                ToolbarItem(placement: .primaryAction) {
                    Button("Filters") {
                        showingFilters = true
                    }
                }
                
                ToolbarItem(placement: .status) {
                    ConnectionStatusView(isConnected: viewModel.isConnected)
                }
                #endif
            }
            .sheet(isPresented: $showingFilters) {
                FilterView(viewModel: viewModel)
            }
            .sheet(item: $selectedEvent) { event in
                CoTEventDetailView(event: event)
            }
            .alert("Error", isPresented: .constant(viewModel.error != nil)) {
                Button("OK") {
                    // Error is automatically cleared
                }
            } message: {
                Text(viewModel.error?.localizedDescription ?? "Unknown error")
            }
        }
    }
}

// MARK: - Supporting Views

@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
struct SearchBar: View {
    @Binding var text: String
    
    var body: some View {
        HStack {
            Image(systemName: "magnifyingglass")
                .foregroundColor(.secondary)
            
            TextField("Search events...", text: $text)
                .textFieldStyle(RoundedBorderTextFieldStyle())
        }
        .padding(.horizontal)
    }
}

@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
struct EmptyEventsView: View {
    var body: some View {
        VStack(spacing: 16) {
            Image(systemName: "antenna.radiowaves.left.and.right")
                .font(.system(size: 60))
                .foregroundColor(.secondary)
            
            Text("No Events")
                .font(.title2)
                .fontWeight(.semibold)
            
            Text("Events will appear here when they are received")
                .font(.body)
                .foregroundColor(.secondary)
                .multilineTextAlignment(.center)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
    }
}

@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
struct CategoryHeader: View {
    let category: EventCategory
    
    var body: some View {
        HStack {
            Image(systemName: category.systemImage)
                .foregroundColor(.accentColor)
            Text(category.displayName)
                .font(.headline)
        }
    }
}

// ConnectionStatusView moved to PresenceGraphView.swift to avoid duplication

@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
struct CoTEventRow: View {
    let event: CoTEventModel
    
    var body: some View {
        VStack(alignment: .leading, spacing: 4) {
            HStack {
                Image(systemName: event.eventCategory.systemImage)
                    .foregroundColor(.accentColor)
                    .frame(width: 20)
                
                Text(event.callsign)
                    .font(.headline)
                    .fontWeight(.semibold)
                
                Spacer()
                
                if event.isStale {
                    Text("STALE")
                        .font(.caption)
                        .padding(.horizontal, 6)
                        .padding(.vertical, 2)
                        .background(Color.red)
                        .foregroundColor(.white)
                        .cornerRadius(4)
                }
                
                Text(event.timestamp, style: .time)
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
            
            HStack {
                Text(event.type)
                    .font(.caption)
                    .padding(.horizontal, 6)
                    .padding(.vertical, 2)
                    .background(Color.secondary.opacity(0.2))
                    .cornerRadius(4)
                
                Spacer()
                
                Text(coordinateString(event.location))
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
            
            if let remarks = event.remarks {
                Text(remarks)
                    .font(.caption)
                    .foregroundColor(.secondary)
                    .lineLimit(2)
            }
        }
        .padding(.vertical, 2)
    }
    
    private func coordinateString(_ coordinate: CLLocationCoordinate2D) -> String {
        String(format: "%.4f°, %.4f°", coordinate.latitude, coordinate.longitude)
    }
}

@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
struct FilterView: View {
    @ObservedObject var viewModel: CoTEventViewModel
    @Environment(\.dismiss) private var dismiss
    
    var body: some View {
        NavigationView {
            Form {
                Section("Time Range") {
                    Picker("Time Range", selection: $viewModel.timeRangeFilter) {
                        Text("Last 15 minutes").tag(TimeInterval(900))
                        Text("Last hour").tag(TimeInterval(3600))
                        Text("Last 6 hours").tag(TimeInterval(21600))
                        Text("Last 24 hours").tag(TimeInterval(86400))
                        Text("All time").tag(TimeInterval.greatestFiniteMagnitude)
                    }
                    .pickerStyle(.menu)
                }
                
                Section("Callsigns") {
                    ForEach(viewModel.availableCallsigns, id: \.self) { callsign in
                        Toggle(callsign, isOn: Binding(
                            get: { viewModel.selectedCallsigns.contains(callsign) },
                            set: { _ in viewModel.toggleCallsignFilter(callsign) }
                        ))
                    }
                }
                
                Section("Event Types") {
                    ForEach(viewModel.availableEventTypes, id: \.self) { type in
                        Toggle(type, isOn: Binding(
                            get: { viewModel.selectedEventTypes.contains(type) },
                            set: { _ in viewModel.toggleEventTypeFilter(type) }
                        ))
                    }
                }
            }
            .navigationTitle("Filters")
            #if os(iOS)
            .navigationBarTitleDisplayMode(.inline)
            #endif
            .toolbar {
                #if os(iOS)
                ToolbarItem(placement: .navigationBarLeading) {
                    Button("Clear All") {
                        viewModel.clearAllFilters()
                    }
                }
                
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button("Done") {
                        dismiss()
                    }
                }
                #else
                ToolbarItem(placement: .cancellationAction) {
                    Button("Clear All") {
                        viewModel.clearAllFilters()
                    }
                }
                
                ToolbarItem(placement: .confirmationAction) {
                    Button("Done") {
                        dismiss()
                    }
                }
                #endif
            }
        }
    }
}

@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
struct CoTEventDetailView: View {
    let event: CoTEventModel
    @Environment(\.dismiss) private var dismiss
    
    var body: some View {
        NavigationView {
            ScrollView {
                VStack(alignment: .leading, spacing: 16) {
                    // Header
                    VStack(alignment: .leading, spacing: 8) {
                        HStack {
                            Image(systemName: event.eventCategory.systemImage)
                                .font(.title)
                                .foregroundColor(.accentColor)
                            
                            Text(event.callsign)
                                .font(.title)
                                .fontWeight(.bold)
                            
                            Spacer()
                            
                            if event.isStale {
                                Text("STALE")
                                    .font(.caption)
                                    .padding(.horizontal, 8)
                                    .padding(.vertical, 4)
                                    .background(Color.red)
                                    .foregroundColor(.white)
                                    .cornerRadius(8)
                            }
                        }
                        
                        Text(event.type)
                            .font(.headline)
                            .foregroundColor(.secondary)
                    }
                    
                    Divider()
                    
                    // Details
                    DetailRow(label: "UID", value: event.uid)
                    DetailRow(label: "Timestamp", value: event.timestamp.formatted())
                    DetailRow(label: "Stale Time", value: event.staleTime.formatted())
                    DetailRow(label: "How", value: event.how)
                    DetailRow(label: "Location", value: coordinateString(event.location))
                    
                    if let altitude = event.altitude {
                        DetailRow(label: "Altitude", value: "\(altitude) m")
                    }
                    
                    if let accuracy = event.accuracy {
                        DetailRow(label: "Accuracy", value: "\(accuracy) m")
                    }
                    
                    if let remarks = event.remarks {
                        VStack(alignment: .leading, spacing: 4) {
                            Text("Remarks")
                                .font(.headline)
                            Text(remarks)
                                .font(.body)
                        }
                    }
                }
                .padding()
            }
            .navigationTitle("Event Details")
            #if os(iOS)
            .navigationBarTitleDisplayMode(.inline)
            #endif
            .toolbar {
                #if os(iOS)
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button("Done") {
                        dismiss()
                    }
                }
                #else
                ToolbarItem(placement: .confirmationAction) {
                    Button("Done") {
                        dismiss()
                    }
                }
                #endif
            }
        }
    }
    
    private func coordinateString(_ coordinate: CLLocationCoordinate2D) -> String {
        String(format: "%.6f°, %.6f°", coordinate.latitude, coordinate.longitude)
    }
}

@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
struct DetailRow: View {
    let label: String
    let value: String
    
    var body: some View {
        HStack {
            Text(label)
                .font(.headline)
                .frame(width: 100, alignment: .leading)
            
            Text(value)
                .font(.body)
                .textSelection(.enabled)
            
            Spacer()
        }
    }
}