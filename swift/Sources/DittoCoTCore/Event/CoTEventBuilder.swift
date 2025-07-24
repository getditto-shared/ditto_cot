import Foundation

/// Builder for creating CoT events with a fluent API
/// Following Swift conventions with @discardableResult for method chaining
public class CoTEventBuilder {
    
    // MARK: - Properties
    
    private var version: String = "2.0"
    private var uid: String?
    private var type: String?
    private var time: Date = Date()
    private var start: Date = Date()
    private var stale: Date = Date().addingTimeInterval(300) // 5 minutes default
    private var how: String = "h-g-i-g-o" // human-gps-input-generic-other
    private var point: CoTPoint?
    private var access: String?
    private var qos: String?
    private var opex: String?
    private var caveat: String?
    private var releasableTo: String?
    private var detail: CoTDetail?
    
    // MARK: - Initialization
    
    public init() {}
    
    // MARK: - Required Field Setters
    
    /// Set the unique identifier
    @discardableResult
    public func uid(_ uid: String) -> Self {
        self.uid = uid
        return self
    }
    
    /// Set the event type (e.g., "a-f-G-U-C")
    @discardableResult
    public func type(_ type: String) -> Self {
        self.type = type
        return self
    }
    
    /// Set the event type using components
    @discardableResult
    public func type(components: String...) -> Self {
        self.type = components.joined(separator: "-")
        return self
    }
    
    /// Set the geographic location
    @discardableResult
    public func point(lat: Double, lon: Double, hae: Double = 0.0) -> Self {
        self.point = CoTPoint(lat: lat, lon: lon, hae: hae)
        return self
    }
    
    /// Set the geographic location with error values
    @discardableResult
    public func point(lat: Double, lon: Double, hae: Double = 0.0, ce: Double, le: Double) -> Self {
        self.point = CoTPoint(lat: lat, lon: lon, hae: hae, ce: ce, le: le)
        return self
    }
    
    /// Set the geographic location using a CoTPoint
    @discardableResult
    public func point(_ point: CoTPoint) -> Self {
        self.point = point
        return self
    }
    
    // MARK: - Time Field Setters
    
    /// Set the event generation time
    @discardableResult
    public func time(_ time: Date) -> Self {
        self.time = time
        return self
    }
    
    /// Set the validity interval
    @discardableResult
    public func validity(start: Date, stale: Date) -> Self {
        self.start = start
        self.stale = stale
        return self
    }
    
    /// Set validity duration from now
    @discardableResult
    public func validFor(_ duration: TimeInterval) -> Self {
        let now = Date()
        self.start = now
        self.stale = now.addingTimeInterval(duration)
        return self
    }
    
    /// Set how the event was generated
    @discardableResult
    public func how(_ how: String) -> Self {
        self.how = how
        return self
    }
    
    // MARK: - Optional Field Setters
    
    /// Set the access control field
    @discardableResult
    public func access(_ access: String) -> Self {
        self.access = access
        return self
    }
    
    /// Set quality of service
    @discardableResult
    public func qos(priority: Int = 1, overtaking: Character = "r", assurance: Character = "c") -> Self {
        self.qos = "\(priority)-\(overtaking)-\(assurance)"
        return self
    }
    
    /// Set operational exercise indicator
    @discardableResult
    public func opex(_ opex: String) -> Self {
        self.opex = opex
        return self
    }
    
    /// Set as operational event
    @discardableResult
    public func operational() -> Self {
        self.opex = "o"
        return self
    }
    
    /// Set as exercise event
    @discardableResult
    public func exercise(_ name: String? = nil) -> Self {
        self.opex = name.map { "e-\($0)" } ?? "e"
        return self
    }
    
    /// Set as simulation event
    @discardableResult
    public func simulation(_ name: String? = nil) -> Self {
        self.opex = name.map { "s-\($0)" } ?? "s"
        return self
    }
    
    /// Set message caveat
    @discardableResult
    public func caveat(_ caveat: String) -> Self {
        self.caveat = caveat
        return self
    }
    
    /// Set release authorization
    @discardableResult
    public func releasableTo(_ releasableTo: String) -> Self {
        self.releasableTo = releasableTo
        return self
    }
    
    // MARK: - Detail Setters
    
    /// Set the detail object
    @discardableResult
    public func detail(_ detail: CoTDetail) -> Self {
        self.detail = detail
        return self
    }
    
    /// Set a callsign in the detail
    @discardableResult
    public func callsign(_ callsign: String) -> Self {
        let currentDetail = self.detail ?? CoTDetail()
        self.detail = currentDetail.settingValue(.string(callsign), at: "contact.callsign")
        return self
    }
    
    /// Set remarks in the detail
    @discardableResult
    public func remarks(_ remarks: String) -> Self {
        let currentDetail = self.detail ?? CoTDetail()
        self.detail = currentDetail.settingValue(.string(remarks), at: "remarks")
        return self
    }
    
    /// Set color in the detail (ARGB hex format)
    @discardableResult
    public func color(_ argb: String) -> Self {
        let currentDetail = self.detail ?? CoTDetail()
        self.detail = currentDetail.settingValue(.string(argb), at: "color.argb")
        return self
    }
    
    /// Add a custom detail value
    @discardableResult
    public func detailValue(_ value: Any, at keyPath: String) -> Self {
        let currentDetail = self.detail ?? CoTDetail()
        self.detail = currentDetail.settingValue(JSONValue(value), at: keyPath)
        return self
    }
    
    // MARK: - Build
    
    /// Build the CoT event
    /// - Throws: `CoTEventBuilderError` if required fields are missing
    public func build() throws -> CoTEvent {
        guard let uid = uid else {
            throw CoTEventBuilderError.missingRequiredField("uid")
        }
        
        guard let type = type else {
            throw CoTEventBuilderError.missingRequiredField("type")
        }
        
        guard let point = point else {
            throw CoTEventBuilderError.missingRequiredField("point")
        }
        
        // Validate type format
        guard type.range(of: #"^\w+(-\w+)*$"#, options: .regularExpression) != nil else {
            throw CoTEventBuilderError.invalidFormat("type", "Must match pattern: word(-word)*")
        }
        
        // Validate QoS format if present
        if let qos = qos {
            guard qos.range(of: #"^\d-\w-\w$"#, options: .regularExpression) != nil else {
                throw CoTEventBuilderError.invalidFormat("qos", "Must match pattern: digit-word-word")
            }
        }
        
        return CoTEvent(
            version: version,
            uid: uid,
            type: type,
            time: time,
            start: start,
            stale: stale,
            how: how,
            point: point,
            access: access,
            qos: qos,
            opex: opex,
            caveat: caveat,
            releasableTo: releasableTo,
            detail: detail
        )
    }
    
    /// Build the CoT event as a Result
    /// - Returns: Result containing either the built event or the build error
    public func buildResult() -> Result<CoTEvent, CoTEventBuilderError> {
        do {
            let event = try build()
            return .success(event)
        } catch let error as CoTEventBuilderError {
            return .failure(error)
        } catch {
            return .failure(.invalidFormat("unknown", error.localizedDescription))
        }
    }
}

// MARK: - Builder Errors

/// Errors that can occur when building a CoT event
public enum CoTEventBuilderError: LocalizedError {
    case missingRequiredField(String)
    case invalidFormat(String, String)
    
    public var errorDescription: String? {
        switch self {
        case .missingRequiredField(let field):
            return "Missing required field: \(field)"
        case .invalidFormat(let field, let format):
            return "Invalid format for field '\(field)': \(format)"
        }
    }
}

// MARK: - Convenience Extensions

public extension CoTEventBuilder {
    
    /// Create a blue force (friendly) tracking event
    @discardableResult
    func blueForceTrack() -> Self {
        type("a-f-G")
    }
    
    /// Create a hostile force tracking event
    @discardableResult
    func hostileTrack() -> Self {
        type("a-h-G")
    }
    
    /// Create a neutral force tracking event
    @discardableResult
    func neutralTrack() -> Self {
        type("a-n-G")
    }
    
    /// Create an unknown force tracking event
    @discardableResult
    func unknownTrack() -> Self {
        type("a-u-G")
    }
    
    /// Create an emergency/911 event
    @discardableResult
    func emergency() -> Self {
        type("b-a-o-tbl")
            .qos(priority: 9, overtaking: "r", assurance: "g")
    }
}