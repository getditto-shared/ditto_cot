import Foundation
import XMLCoder

// MARK: - XML Serializable Protocol
public protocol XMLSerializable {
    func toXML(prettyPrint: Bool) throws -> String
    static func fromXML(_ xml: String) throws -> Self
}

/// Represents a Cursor-on-Target (CoT) event
/// Conforms to the CoT 2.0 specification
public struct CoTEvent: Codable, Equatable {
    
    // MARK: - Required Attributes
    
    /// CoT version (must be >= 2.0)
    public let version: String
    
    /// Globally unique identifier for this event
    public let uid: String
    
    /// Hierarchical event type (e.g., "a-f-G-U-C")
    /// Format: component1(-component2)*
    public let type: String
    
    /// Time the event was generated
    public let time: Date
    
    /// Start time of event validity interval
    public let start: Date
    
    /// End time of event validity interval (when event becomes stale)
    public let stale: Date
    
    /// Method of event generation (e.g., "m-g" for manual GPS)
    public let how: String
    
    // MARK: - Required Elements
    
    /// Geographic location of the event
    public let point: CoTPoint
    
    // MARK: - Optional Attributes
    
    /// Access control field (e.g., "unrestricted", "nato", "coalition")
    public let access: String?
    
    /// Quality of Service: priority-overtaking-assurance (e.g., "1-r-c")
    public let qos: String?
    
    /// Operational exercise indicator: o=operations, e=exercise, s=simulation
    public let opex: String?
    
    /// Message handling caveat (e.g., "FOUO", "UNCLASSIFIED")
    public let caveat: String?
    
    /// Release authorization (e.g., "USA", "NATO")
    public let releasableTo: String?
    
    // MARK: - Optional Elements
    
    /// Additional event details
    public let detail: CoTDetail?
    
    // MARK: - Initialization
    
    /// Initialize a CoT event with all parameters
    /// - Parameters:
    ///   - version: CoT version (default: "2.0")
    ///   - uid: Unique identifier
    ///   - type: Event type
    ///   - time: Event generation time
    ///   - start: Validity start time
    ///   - stale: Validity end time
    ///   - how: Generation method
    ///   - point: Geographic location
    ///   - access: Access control
    ///   - qos: Quality of service
    ///   - opex: Operational exercise indicator
    ///   - caveat: Message caveat
    ///   - releasableTo: Release authorization
    ///   - detail: Additional details
    public init(
        version: String = "2.0",
        uid: String,
        type: String,
        time: Date,
        start: Date,
        stale: Date,
        how: String,
        point: CoTPoint,
        access: String? = nil,
        qos: String? = nil,
        opex: String? = nil,
        caveat: String? = nil,
        releasableTo: String? = nil,
        detail: CoTDetail? = nil
    ) {
        self.version = version
        self.uid = uid
        self.type = type
        self.time = time
        self.start = start
        self.stale = stale
        self.how = how
        self.point = point
        self.access = access
        self.qos = qos
        self.opex = opex
        self.caveat = caveat
        self.releasableTo = releasableTo
        self.detail = detail
    }
    
    // MARK: - Builder
    
    /// Creates a new event builder
    public static func builder() -> CoTEventBuilder {
        CoTEventBuilder()
    }
    
    /// Create a blue force tracking event with minimal required parameters
    public static func blueForceTrack(uid: String, at point: CoTPoint, callsign: String? = nil) -> CoTEvent {
        var builder = CoTEvent.builder()
            .uid(uid)
            .blueForceTrack()
            .point(point)
            .validFor(300) // 5 minutes default
        
        if let callsign = callsign {
            builder = builder.callsign(callsign)
        }
        
        return try! builder.build()
    }
    
    /// Create an emergency event with minimal required parameters
    public static func emergency(uid: String, at point: CoTPoint, message: String? = nil) -> CoTEvent {
        var builder = CoTEvent.builder()
            .uid(uid)
            .emergency()
            .point(point)
            .validFor(3600) // 1 hour for emergencies
        
        if let message = message {
            builder = builder.remarks(message)
        }
        
        return try! builder.build()
    }
}

// MARK: - CustomStringConvertible
extension CoTEvent: CustomStringConvertible {
    public var description: String {
        """
        CoTEvent(
            uid: \(uid),
            type: \(type),
            time: \(ISO8601DateFormatter().string(from: time)),
            point: \(point)
        )
        """
    }
}

// MARK: - XML Serialization
extension CoTEvent: XMLSerializable {
    
    /// Convert the event to XML string
    /// - Parameter prettyPrint: Whether to format the XML with indentation
    /// - Returns: XML string representation
    /// - Throws: Encoding errors
    public func toXML(prettyPrint: Bool = false) throws -> String {
        let encoder = XMLEncoder()
        encoder.outputFormatting = prettyPrint ? [.prettyPrinted] : []
        
        // Configure date formatting to match CoT specification (ISO8601)
        let dateFormatter = ISO8601DateFormatter()
        dateFormatter.formatOptions = [.withInternetDateTime, .withFractionalSeconds]
        encoder.dateEncodingStrategy = .custom { date, encoder in
            var container = encoder.singleValueContainer()
            try container.encode(dateFormatter.string(from: date))
        }
        
        // Create a simplified version for XML serialization
        let xmlEvent = XMLCoTEvent(from: self)
        let data = try encoder.encode(xmlEvent, withRootKey: "event")
        return String(data: data, encoding: .utf8) ?? ""
    }
    
    /// Create a CoT event from XML string
    /// - Parameter xml: XML string representation
    /// - Returns: Parsed CoT event
    /// - Throws: Decoding errors
    public static func fromXML(_ xml: String) throws -> CoTEvent {
        guard let data = xml.data(using: .utf8) else {
            throw CoTXMLError.invalidXMLString
        }
        
        let decoder = XMLDecoder()
        
        // Configure date parsing to handle CoT format
        let dateFormatter = ISO8601DateFormatter()
        dateFormatter.formatOptions = [.withInternetDateTime, .withFractionalSeconds]
        decoder.dateDecodingStrategy = .custom { decoder in
            let container = try decoder.singleValueContainer()
            let dateString = try container.decode(String.self)
            
            if let date = dateFormatter.date(from: dateString) {
                return date
            }
            
            // Fallback to basic ISO8601 format
            dateFormatter.formatOptions = [.withInternetDateTime]
            if let date = dateFormatter.date(from: dateString) {
                return date
            }
            
            throw DecodingError.dataCorruptedError(in: container, debugDescription: "Invalid date format: \(dateString)")
        }
        
        let xmlEvent = try decoder.decode(XMLCoTEvent.self, from: data)
        return xmlEvent.toCoTEvent()
    }
}

// MARK: - XML Coding Keys
extension CoTEvent {
    enum CodingKeys: String, CodingKey {
        case version
        case uid
        case type
        case time
        case start
        case stale
        case how
        case point
        case access
        case qos
        case opex
        case caveat
        case releasableTo = "releasableto"
        case detail
    }
}


// MARK: - XML Wrapper for Serialization
struct XMLCoTEvent: Codable {
    let version: String
    let uid: String
    let type: String
    let time: Date
    let start: Date
    let stale: Date
    let how: String
    let point: CoTPoint
    let access: String?
    let qos: String?
    let opex: String?
    let caveat: String?
    let releasableTo: String?
    // Skip detail for now - XML detail handling is complex
    
    init(from event: CoTEvent) {
        self.version = event.version
        self.uid = event.uid
        self.type = event.type
        self.time = event.time
        self.start = event.start
        self.stale = event.stale
        self.how = event.how
        self.point = event.point
        self.access = event.access
        self.qos = event.qos
        self.opex = event.opex
        self.caveat = event.caveat
        self.releasableTo = event.releasableTo
        // Note: detail is omitted for now due to XML complexity
    }
    
    func toCoTEvent() -> CoTEvent {
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
            detail: nil // Detail not supported in basic XML
        )
    }
    
    enum CodingKeys: String, CodingKey {
        case version
        case uid
        case type
        case time
        case start
        case stale
        case how
        case point
        case access
        case qos
        case opex
        case caveat
        case releasableTo = "releasableto"
    }
}

// MARK: - XMLCoder Support for XMLCoTEvent
extension XMLCoTEvent: DynamicNodeEncoding {
    static func nodeEncoding(for key: CodingKey) -> XMLEncoder.NodeEncoding {
        switch key {
        case CodingKeys.point:
            return .element
        default:
            return .attribute
        }
    }
}

extension XMLCoTEvent: DynamicNodeDecoding {
    static func nodeDecoding(for key: CodingKey) -> XMLDecoder.NodeDecoding {
        switch key {
        case CodingKeys.point:
            return .element
        default:
            return .attribute
        }
    }
}

// MARK: - Validation Error Types
public enum CoTValidationError: LocalizedError {
    case invalidVersion(String)
    case invalidType(String)
    case invalidTimeOrdering(String)
    case invalidCoordinate(String)
    
    public var errorDescription: String? {
        switch self {
        case .invalidVersion(let version):
            return "Invalid CoT version: \(version)"
        case .invalidType(let type):
            return "Invalid event type format: \(type)"
        case .invalidTimeOrdering(let message):
            return "Invalid time ordering: \(message)"
        case .invalidCoordinate(let message):
            return "Invalid coordinate: \(message)"
        }
    }
}

// MARK: - XML Error Types
public enum CoTXMLError: LocalizedError {
    case invalidXMLString
    case encodingFailed(Error)
    case decodingFailed(Error)
    
    public var errorDescription: String? {
        switch self {
        case .invalidXMLString:
            return "Invalid XML string - unable to convert to data"
        case .encodingFailed(let error):
            return "XML encoding failed: \(error.localizedDescription)"
        case .decodingFailed(let error):
            return "XML decoding failed: \(error.localizedDescription)"
        }
    }
}

// MARK: - Helper Methods
public extension CoTEvent {
    
    /// Check if the event is currently valid (between start and stale times)
    var isValid: Bool {
        let now = Date()
        return now >= start && now <= stale
    }
    
    /// Check if the event has become stale
    var isStale: Bool {
        Date() > stale
    }
    
    /// Get the event type components as an array
    /// e.g., "a-f-G-U-C" returns ["a", "f", "G", "U", "C"]
    var typeComponents: [String] {
        type.split(separator: "-").map(String.init)
    }
    
    /// Get the primary type component (first component)
    var primaryType: String? {
        typeComponents.first
    }
    
    /// Check if this is an atom type (starts with "a")
    var isAtom: Bool {
        primaryType == "a"
    }
    
    /// Get the callsign from detail if available
    var callsign: String? {
        detail?.callsign
    }
    
    /// Validate the event against CoT specification
    var validationResult: Result<Void, CoTValidationError> {
        // Check version
        guard version.starts(with: "2.") else {
            return .failure(.invalidVersion(version))
        }
        
        // Check type format
        guard type.range(of: #"^\w+(-\w+)*$"#, options: .regularExpression) != nil else {
            return .failure(.invalidType(type))
        }
        
        // Check time ordering
        guard start <= stale else {
            return .failure(.invalidTimeOrdering("start must be <= stale"))
        }
        
        // Check coordinates
        guard (-90...90).contains(point.lat) else {
            return .failure(.invalidCoordinate("latitude out of range: \(point.lat)"))
        }
        
        guard (-180...180).contains(point.lon) else {
            return .failure(.invalidCoordinate("longitude out of range: \(point.lon)"))
        }
        
        return .success(())
    }
    
    /// Check if the event is valid according to CoT specification
    var isValidEvent: Bool {
        validationResult.isSuccess
    }
}

// MARK: - Result Extension
private extension Result where Success == Void {
    var isSuccess: Bool {
        switch self {
        case .success:
            return true
        case .failure:
            return false
        }
    }
}