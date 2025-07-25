import Foundation
import XMLCoder

/// Represents a geographic point in the Cursor-on-Target system
/// Following the CoT 2.0 specification for point elements
public struct CoTPoint: Codable, Equatable {
    /// Latitude in decimal degrees (WGS84)
    /// Range: -90.0 to 90.0
    public let lat: Double
    
    /// Longitude in decimal degrees (WGS84)
    /// Range: -180.0 to 180.0
    public let lon: Double
    
    /// Height above ellipsoid (meters)
    /// Positive values indicate above Mean Sea Level
    public let hae: Double
    
    /// Circular Error (meters) - horizontal uncertainty
    /// Default is 999999.0 indicating "unknown"
    public let ce: Double
    
    /// Linear Error (meters) - vertical uncertainty
    /// Default is 999999.0 indicating "unknown"
    public let le: Double
    
    /// Initializes a CoT point with all parameters
    /// - Parameters:
    ///   - lat: Latitude in decimal degrees (-90 to 90)
    ///   - lon: Longitude in decimal degrees (-180 to 180)
    ///   - hae: Height above ellipsoid in meters
    ///   - ce: Circular error in meters (default: 999999.0)
    ///   - le: Linear error in meters (default: 999999.0)
    public init(lat: Double, lon: Double, hae: Double = 0.0, ce: Double = 999999.0, le: Double = 999999.0) {
        precondition((-90...90).contains(lat), "Latitude must be between -90 and 90 degrees")
        precondition((-180...180).contains(lon), "Longitude must be between -180 and 180 degrees")
        precondition(ce >= 0, "Circular error must be non-negative")
        precondition(le >= 0, "Linear error must be non-negative")
        
        self.lat = lat
        self.lon = lon
        self.hae = hae
        self.ce = ce
        self.le = le
    }
}

// MARK: - CustomStringConvertible
extension CoTPoint: CustomStringConvertible {
    public var description: String {
        "CoTPoint(lat: \(lat), lon: \(lon), hae: \(hae), ce: \(ce), le: \(le))"
    }
}

// MARK: - XML Coding
extension CoTPoint {
    enum CodingKeys: String, CodingKey {
        case lat
        case lon
        case hae
        case ce
        case le
    }
}

// MARK: - XMLCoder Support
extension CoTPoint: DynamicNodeEncoding {
    public static func nodeEncoding(for key: CodingKey) -> XMLEncoder.NodeEncoding {
        // All point properties are XML attributes, not elements
        return .attribute
    }
}

extension CoTPoint: DynamicNodeDecoding {
    public static func nodeDecoding(for key: CodingKey) -> XMLDecoder.NodeDecoding {
        // All point properties are XML attributes, not elements
        return .attribute
    }
}