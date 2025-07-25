import SwiftUI
import DittoSwift

/// SwiftUI view for displaying the Ditto presence graph
@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
public struct PresenceGraphView: View {
    @ObservedObject private var observable: CoTObservable
    @State private var isExpanded = false
    
    public init(observable: CoTObservable) {
        self.observable = observable
    }
    
    public var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack {
                Image(systemName: "network")
                    .foregroundColor(.blue)
                    .font(.title2)
                
                Text("Presence Graph")
                    .font(.headline)
                
                Spacer()
                
                Text("\(observable.connectedPeers.count) peers")
                    .font(.caption)
                    .foregroundColor(.secondary)
                
                Button(action: { isExpanded.toggle() }) {
                    Image(systemName: isExpanded ? "chevron.up" : "chevron.down")
                        .foregroundColor(.secondary)
                }
            }
            
            if isExpanded {
                LazyVStack(alignment: .leading, spacing: 8) {
                    if observable.connectedPeers.isEmpty {
                        HStack {
                            Image(systemName: "person.slash")
                                .foregroundColor(.orange)
                            Text("No peers connected")
                                .foregroundColor(.secondary)
                        }
                        .padding(.vertical, 4)
                    } else {
                        ForEach(Array(observable.connectedPeers.enumerated()), id: \.offset) { index, peer in
                            PeerRow(peer: peer)
                        }
                    }
                }
                .transition(.opacity.combined(with: .move(edge: .top)))
            }
        }
        .padding()
        #if os(iOS)
        .background(Color(.systemBackground))
        #else
        .background(Color(NSColor.windowBackgroundColor))
        #endif
        .cornerRadius(12)
        .shadow(radius: 2)
        .animation(.easeInOut(duration: 0.3), value: isExpanded)
    }
}

@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
struct PeerRow: View {
    let peer: DittoPeer
    
    var body: some View {
        HStack(spacing: 12) {
            // Peer status indicator
            Circle()
                .fill(connectionColor)
                .frame(width: 8, height: 8)
            
            VStack(alignment: .leading, spacing: 2) {
                HStack {
                    Text(peerDisplayName)
                        .font(.subheadline)
                        .fontWeight(.medium)
                    
                    Spacer()
                    
                    Text(connectionTypeText)
                        .font(.caption)
                        .foregroundColor(.secondary)
                        .padding(.horizontal, 6)
                        .padding(.vertical, 2)
                        .background(Color.secondary.opacity(0.1))
                        .cornerRadius(4)
                }
                
                Text("Peer ID: \(String(describing: peer).prefix(12))...")
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
        }
        .padding(.vertical, 4)
    }
    
    private var peerDisplayName: String {
        // Try to extract a meaningful name, fallback to peer type
        if !peer.deviceName.isEmpty {
            return peer.deviceName
        }
        return "Peer \(String(describing: peer).prefix(8))"
    }
    
    private var connectionColor: Color {
        // Simplified color coding since exact distance API may vary
        return .blue // Default to connected color
    }
    
    private var connectionTypeText: String {
        // Simplified connection type since exact API may vary
        return "Connected"
    }
}

@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
public struct ConnectionStatusView: View {
    let isConnected: Bool
    
    public init(isConnected: Bool) {
        self.isConnected = isConnected
    }
    
    public var body: some View {
        HStack(spacing: 6) {
            Circle()
                .fill(isConnected ? Color.green : Color.red)
                .frame(width: 8, height: 8)
            
            Text(isConnected ? "Connected" : "Disconnected")
                .font(.caption)
                .foregroundColor(isConnected ? .green : .red)
        }
    }
}

// MARK: - Presence Graph Debug View

@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
public struct PresenceDebugView: View {
    @ObservedObject private var observable: CoTObservable
    
    public init(observable: CoTObservable) {
        self.observable = observable
    }
    
    public var body: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("Ditto Presence Debug")
                .font(.title2)
                .fontWeight(.bold)
            
            // Connection Status
            HStack {
                Text("Status:")
                    .fontWeight(.medium)
                Spacer()
                ConnectionStatusView(isConnected: observable.isConnected)
            }
            
            // Peer Count
            HStack {
                Text("Connected Peers:")
                    .fontWeight(.medium)
                Spacer()
                Text("\(observable.connectedPeers.count)")
                    .fontWeight(.semibold)
                    .foregroundColor(.blue)
            }
            
            // Event Count
            HStack {
                Text("Total Events:")
                    .fontWeight(.medium)
                Spacer()
                Text("\(observable.events.count)")
                    .fontWeight(.semibold)
                    .foregroundColor(.green)
            }
            
            Divider()
            
            // Presence Graph
            PresenceGraphView(observable: observable)
            
            // Quick Actions
            VStack(spacing: 8) {
                Text("Debug Actions")
                    .font(.headline)
                
                HStack(spacing: 12) {
                    Button("Refresh Data") {
                        observable.refreshAll()
                    }
                    .buttonStyle(.bordered)
                    
                    Button("Restart Observing") {
                        observable.stopObserving()
                        observable.startObserving()
                    }
                    .buttonStyle(.bordered)
                }
            }
        }
        .padding()
        #if os(iOS)
        .background(Color(.systemGroupedBackground))
        #else
        .background(Color(NSColor.controlBackgroundColor))
        #endif
        .cornerRadius(12)
    }
}