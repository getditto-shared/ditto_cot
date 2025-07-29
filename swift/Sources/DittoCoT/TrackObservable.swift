import Foundation
import DittoSwift
import Combine

/// Dedicated observable for track events only - completely isolated from other collections
@available(iOS 13.0, macOS 10.15, watchOS 6.0, tvOS 13.0, *)
public class TrackObservable: ObservableObject {
    
    // MARK: - Properties
    
    private let ditto: Ditto
    private var subscription: DittoSubscription?
    private var liveQuery: DittoLiveQuery?
    
    @Published public private(set) var trackEvents: [DittoDocument] = []
    @Published public private(set) var isConnected: Bool = false
    @Published public private(set) var error: Error?
    
    // MARK: - Initialization
    
    public init(ditto: Ditto) {
        self.ditto = ditto
        print("🎯 TrackObservable: Initializing for track collection only")
    }
    
    // MARK: - Public Methods
    
    public func startObserving() {
        print("🎯 TrackObservable: Starting track observation...")
        setupTrackSubscription()
        setupTrackLiveQuery()
        print("🎯 TrackObservable: Track observation started")
    }
    
    public func stopObserving() {
        print("🎯 TrackObservable: Stopping track observation...")
        liveQuery?.stop()
        subscription = nil
        liveQuery = nil
        trackEvents = []
        print("🎯 TrackObservable: Track observation stopped")
    }
    
    public func refreshTracks() {
        print("🎯 TrackObservable: Manual refresh requested")
        let collection = ditto.store.collection("track")
        let currentDocs = collection.find("_r != true").exec()
        
        DispatchQueue.main.async { [weak self] in
            self?.trackEvents = currentDocs
            print("🎯 TrackObservable: Manual refresh complete - \(currentDocs.count) track events")
        }
    }
    
    // MARK: - Private Methods
    
    private func setupTrackSubscription() {
        print("🎯 TrackObservable: Setting up track subscription...")
        let collection = ditto.store.collection("track")
        subscription = collection.find("_r != true").subscribe()
        print("🎯 TrackObservable: Track subscription created")
    }
    
    private func setupTrackLiveQuery() {
        print("🎯 TrackObservable: Setting up track live query...")
        let collection = ditto.store.collection("track")
        
        liveQuery = collection.find("_r != true").observeLocal { [weak self] docs, event in
            guard let self = self else { return }
            
            DispatchQueue.main.async {
                let beforeCount = self.trackEvents.count
                
                // Handle the live query event
                print("🎯 TrackObservable: Live query event - \(docs.count) track events")
                self.trackEvents = docs
                self.isConnected = true
                
                let afterCount = docs.count
                let change = afterCount - beforeCount
                
                if beforeCount == 0 {
                    print("🎯 TrackObservable: Initial load - \(afterCount) track events")
                } else if change > 0 {
                    print("🎯 TrackObservable: +\(change) new track(s)")
                } else if change < 0 {
                    print("🎯 TrackObservable: \(abs(change)) track(s) removed")
                } else {
                    print("🎯 TrackObservable: Track updated (same count)")
                }
                
                self.error = nil
            }
        }
        
        print("🎯 TrackObservable: Track live query setup complete")
    }
}

// MARK: - Track Event Helpers

@available(iOS 13.0, macOS 10.15, watchOS 6.0, tvOS 13.0, *)
extension TrackObservable {
    
    /// Get all active track events
    public var activeTracks: [DittoDocument] {
        return trackEvents.filter { doc in
            if let removed = doc.value["_r"] as? Bool, removed {
                return false
            }
            return true
        }
    }
    
    /// Get track count
    public var trackCount: Int {
        return activeTracks.count
    }
    
    /// Get tracks by callsign
    public func tracks(for callsign: String) -> [DittoDocument] {
        return activeTracks.filter { doc in
            if let docCallsign = doc.value["e"] as? String {
                return docCallsign == callsign
            }
            return false
        }
    }
}