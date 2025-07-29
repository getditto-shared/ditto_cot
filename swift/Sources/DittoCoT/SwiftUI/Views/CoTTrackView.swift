import SwiftUI
import CoreLocation
import DittoSwift

/// SwiftUI view for displaying CoT tracks
@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
public struct CoTTrackView: View {
    @ObservedObject private var trackObservable: TrackObservable
    @State private var selectedTrack: DittoDocument?
    
    public init(trackObservable: TrackObservable) {
        self.trackObservable = trackObservable
    }
    
    public var body: some View {
        VStack {
            trackHeader
            
            if trackObservable.trackEvents.isEmpty {
                emptyTrackView
            } else {
                trackListView
            }
        }
        .navigationTitle("Tracks")
        #if os(iOS)
        .navigationBarTitleDisplayMode(.inline)
        #endif
        .onAppear {
            trackObservable.refreshTracks()
        }
    }
    
    private var trackHeader: some View {
        HStack {
            Text("Tracks")
                .font(.title2)
                .fontWeight(.semibold)
            
            Spacer()
            
            Text("\(trackObservable.trackCount) active")
                .font(.caption)
                .foregroundColor(.secondary)
                .padding(.horizontal, 8)
                .padding(.vertical, 4)
                .background(Color.secondary.opacity(0.2))
                .cornerRadius(8)
        }
        .padding(.horizontal)
    }
    
    private var emptyTrackView: some View {
        VStack {
            Spacer()
            VStack(spacing: 16) {
                Image(systemName: "location.slash")
                    .font(.system(size: 60))
                    .foregroundColor(.secondary)
                
                Text("No tracks available")
                    .font(.headline)
                    .foregroundColor(.secondary)
                
                Text("Tracks will appear here when entities are being tracked")
                    .font(.caption)
                    .foregroundColor(.secondary)
                    .multilineTextAlignment(.center)
                    .padding(.horizontal, 40)
            }
            Spacer()
        }
    }
    
    private var trackListView: some View {
        ScrollView {
            LazyVStack(spacing: 8) {
                ForEach(trackObservable.activeTracks, id: \.id) { track in
                    TrackRow(track: track, isSelected: selectedTrack?.id == track.id)
                        .onTapGesture {
                            selectedTrack = track
                        }
                }
            }
            .padding(.horizontal)
        }
    }
}

@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
struct TrackRow: View {
    let track: DittoDocument
    let isSelected: Bool
    
    private var callsign: String {
        track.value["e"] as? String ?? "Unknown"
    }
    
    private var type: String {
        track.value["w"] as? String ?? "unknown"
    }
    
    private var location: CLLocationCoordinate2D? {
        guard let lat = track.value["j"] as? Double,
              let lon = track.value["l"] as? Double else { return nil }
        return CLLocationCoordinate2D(latitude: lat, longitude: lon)
    }
    
    private var timestamp: Date {
        if let bValue = track.value["b"] as? Double {
            return Date(timeIntervalSince1970: bValue / 1000.0)
        }
        return Date()
    }
    
    private var trackIcon: String {
        if type.contains("a-f-A") { return "airplane" }
        if type.contains("a-f-S") { return "ferry" }
        if type.contains("a-f-G") { return "car" }
        if type.contains("a-h") { return "exclamationmark.triangle" }
        return "location"
    }
    
    private var trackColor: Color {
        if type.contains("a-f") { return .blue }
        if type.contains("a-h") { return .red }
        if type.contains("a-n") { return .green }
        if type.contains("a-u") { return .orange }
        return .gray
    }
    
    var body: some View {
        HStack(spacing: 12) {
            // Icon
            Image(systemName: trackIcon)
                .font(.title2)
                .foregroundColor(trackColor)
                .frame(width: 40)
            
            // Info
            VStack(alignment: .leading, spacing: 4) {
                Text(callsign)
                    .font(.headline)
                
                if let location = location {
                    Text(String(format: "%.6f, %.6f", location.latitude, location.longitude))
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
                
                Text(timestamp, style: .relative)
                    .font(.caption2)
                    .foregroundColor(.secondary)
            }
            
            Spacer()
            
            // Type badge
            Text(type)
                .font(.caption2)
                .foregroundColor(.secondary)
                .padding(.horizontal, 8)
                .padding(.vertical, 4)
                .background(Color.secondary.opacity(0.1))
                .cornerRadius(4)
        }
        .padding()
        .background(isSelected ? Color.accentColor.opacity(0.1) : Color.secondary.opacity(0.05))
        .cornerRadius(8)
    }
}

// MARK: - Preview
#if DEBUG
@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
struct CoTTrackView_Previews: PreviewProvider {
    static var previews: some View {
        NavigationView {
            Text("Preview not available")
        }
    }
}
#endif