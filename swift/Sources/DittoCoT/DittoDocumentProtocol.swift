import Foundation
import DittoSwift

/// Protocol defining the structure for Ditto documents
public protocol DittoDocumentProtocol {
    var _id: String { get }
    var a: String { get } // Ditto peer key string
    var b: Double { get } // Millis since epoch
    var d: String { get } // TAK UID of author
    var _c: Int64 { get } // Document counter (updates)
    var _r: Bool { get } // Soft-delete flag
    var _v: Int64 { get } // Schema version (2)
    var e: String { get } // Callsign of author
    
    func toDittoDocument() -> [String: Any?]
}

// MARK: - Document Types

/// MapItem document structure for location updates
public struct MapItemDocument: DittoDocumentProtocol, Codable {
    public let _id: String
    public let a: String // Ditto peer key
    public let b: Double // Time in millis
    public let d: String // TAK UID of author
    public let _c: Int64 // Document counter
    public let _r: Bool // Soft-delete flag
    public let _v: Int64 // Schema version (2)
    public let e: String // Callsign of author
    
    // Optional fields
    public let c: String? // Name or title
    public let f: Bool? // Visibility flag
    public let g: String // Version
    public let h: Double? // CE
    public let i: Double? // HAE
    public let j: Double? // LAT
    public let k: Double? // LE
    public let l: Double? // LON
    public let n: Double? // Start
    public let o: Double? // Stale
    public let p: String // How
    public let q: String // Access
    public let r: [String: RValue] // Detail fields
    public let s: String // Opex
    public let source: String?
    public let t: String // Qos
    public let u: String // Caveat
    public let v: String // Releasable to
    public let w: String // Type
    
    public func toDittoDocument() -> [String: Any?] {
        var doc: [String: Any?] = [
            "_id": _id,
            "a": a,
            "b": b,
            "d": d,
            "_c": _c,
            "_r": _r,
            "_v": _v,
            "e": e,
            "g": g,
            "p": p,
            "q": q,
            "s": s,
            "t": t,
            "u": u,
            "v": v,
            "w": w
        ]
        
        // Add optional fields
        if let c = c { doc["c"] = c }
        if let f = f { doc["f"] = f }
        if let h = h { doc["h"] = h }
        if let i = i { doc["i"] = i }
        if let j = j { doc["j"] = j }
        if let k = k { doc["k"] = k }
        if let l = l { doc["l"] = l }
        if let n = n { doc["n"] = n }
        if let o = o { doc["o"] = o }
        if let source = source { doc["source"] = source }
        
        // Convert r field
        if !r.isEmpty {
            doc["r"] = r.mapValues { $0.toAny() }
        }
        
        return doc
    }
}

/// Chat document structure
public struct ChatDocument: DittoDocumentProtocol, Codable {
    public let _id: String
    public let a: String
    public let b: Double
    public let d: String
    public let _c: Int64
    public let _r: Bool
    public let _v: Int64
    public let e: String
    
    // Chat-specific fields
    public let authorCallsign: String?
    public let authorType: String?
    public let authorUid: String?
    public let g: String
    public let h: Double?
    public let i: Double?
    public let j: Double?
    public let k: Double?
    public let l: Double?
    public let location: String?
    public let message: String?
    public let n: Double?
    public let o: Double?
    public let p: String
    public let parent: String?
    public let q: String
    public let r: [String: RValue]
    public let room: String?
    public let roomId: String?
    public let s: String
    public let source: String?
    public let t: String
    public let time: String?
    public let u: String
    public let v: String
    public let w: String
    
    public func toDittoDocument() -> [String: Any?] {
        var doc: [String: Any?] = [
            "_id": _id,
            "a": a,
            "b": b,
            "d": d,
            "_c": _c,
            "_r": _r,
            "_v": _v,
            "e": e,
            "g": g,
            "p": p,
            "q": q,
            "s": s,
            "t": t,
            "u": u,
            "v": v,
            "w": w
        ]
        
        // Add optional fields
        if let authorCallsign = authorCallsign { doc["authorCallsign"] = authorCallsign }
        if let authorType = authorType { doc["authorType"] = authorType }
        if let authorUid = authorUid { doc["authorUid"] = authorUid }
        if let h = h { doc["h"] = h }
        if let i = i { doc["i"] = i }
        if let j = j { doc["j"] = j }
        if let k = k { doc["k"] = k }
        if let l = l { doc["l"] = l }
        if let location = location { doc["location"] = location }
        if let message = message { doc["message"] = message }
        if let n = n { doc["n"] = n }
        if let o = o { doc["o"] = o }
        if let parent = parent { doc["parent"] = parent }
        if let room = room { doc["room"] = room }
        if let roomId = roomId { doc["roomId"] = roomId }
        if let source = source { doc["source"] = source }
        if let time = time { doc["time"] = time }
        
        // Convert r field
        if !r.isEmpty {
            doc["r"] = r.mapValues { $0.toAny() }
        }
        
        return doc
    }
}

/// Generic document structure
public struct GenericDocument: DittoDocumentProtocol, Codable {
    public let _id: String
    public let a: String
    public let b: Double
    public let d: String
    public let _c: Int64
    public let _r: Bool
    public let _v: Int64
    public let e: String
    public let g: String
    public let h: Double?
    public let i: Double?
    public let j: Double?
    public let k: Double?
    public let l: Double?
    public let n: Double?
    public let o: Double?
    public let p: String
    public let q: String
    public let r: [String: RValue]
    public let s: String
    public let source: String?
    public let t: String
    public let u: String
    public let v: String
    public let w: String
    
    public func toDittoDocument() -> [String: Any?] {
        var doc: [String: Any?] = [
            "_id": _id,
            "a": a,
            "b": b,
            "d": d,
            "_c": _c,
            "_r": _r,
            "_v": _v,
            "e": e,
            "g": g,
            "p": p,
            "q": q,
            "s": s,
            "t": t,
            "u": u,
            "v": v,
            "w": w
        ]
        
        // Add optional fields
        if let h = h { doc["h"] = h }
        if let i = i { doc["i"] = i }
        if let j = j { doc["j"] = j }
        if let k = k { doc["k"] = k }
        if let l = l { doc["l"] = l }
        if let n = n { doc["n"] = n }
        if let o = o { doc["o"] = o }
        if let source = source { doc["source"] = source }
        
        // Convert r field
        if !r.isEmpty {
            doc["r"] = r.mapValues { $0.toAny() }
        }
        
        return doc
    }
}

// MARK: - RValue Type

/// Dynamic value type for the 'r' field in documents
public enum RValue: Codable {
    case null
    case boolean(Bool)
    case number(Double)
    case string(String)
    case object([String: Any])
    case array([Any])
    
    public init(from decoder: Decoder) throws {
        let container = try decoder.singleValueContainer()
        
        if container.decodeNil() {
            self = .null
        } else if let bool = try? container.decode(Bool.self) {
            self = .boolean(bool)
        } else if let number = try? container.decode(Double.self) {
            self = .number(number)
        } else if let string = try? container.decode(String.self) {
            self = .string(string)
        } else if let array = try? container.decode([JSONAny].self) {
            self = .array(array.map { $0.value })
        } else if let object = try? container.decode([String: JSONAny].self) {
            self = .object(object.mapValues { $0.value })
        } else {
            throw DecodingError.dataCorruptedError(
                in: container,
                debugDescription: "Cannot decode RValue"
            )
        }
    }
    
    public func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()
        
        switch self {
        case .null:
            try container.encodeNil()
        case .boolean(let value):
            try container.encode(value)
        case .number(let value):
            try container.encode(value)
        case .string(let value):
            try container.encode(value)
        case .object(let value):
            let jsonAny = value.mapValues { JSONAny($0) }
            try container.encode(jsonAny)
        case .array(let value):
            let jsonAny = value.map { JSONAny($0) }
            try container.encode(jsonAny)
        }
    }
    
    func toAny() -> Any? {
        switch self {
        case .null:
            return nil
        case .boolean(let value):
            return value
        case .number(let value):
            return value
        case .string(let value):
            return value
        case .object(let value):
            return value
        case .array(let value):
            return value
        }
    }
}

// MARK: - Helper Types

private struct JSONAny: Codable {
    let value: Any
    
    init(_ value: Any) {
        self.value = value
    }
    
    init(from decoder: Decoder) throws {
        let container = try decoder.singleValueContainer()
        
        if container.decodeNil() {
            value = NSNull()
        } else if let bool = try? container.decode(Bool.self) {
            value = bool
        } else if let int = try? container.decode(Int.self) {
            value = int
        } else if let double = try? container.decode(Double.self) {
            value = double
        } else if let string = try? container.decode(String.self) {
            value = string
        } else if let array = try? container.decode([JSONAny].self) {
            value = array.map { $0.value }
        } else if let dict = try? container.decode([String: JSONAny].self) {
            value = dict.mapValues { $0.value }
        } else {
            throw DecodingError.dataCorruptedError(
                in: container,
                debugDescription: "Cannot decode JSONAny"
            )
        }
    }
    
    func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()
        
        switch value {
        case is NSNull:
            try container.encodeNil()
        case let bool as Bool:
            try container.encode(bool)
        case let int as Int:
            try container.encode(int)
        case let double as Double:
            try container.encode(double)
        case let string as String:
            try container.encode(string)
        case let array as [Any]:
            try container.encode(array.map { JSONAny($0) })
        case let dict as [String: Any]:
            try container.encode(dict.mapValues { JSONAny($0) })
        default:
            throw EncodingError.invalidValue(
                value,
                EncodingError.Context(
                    codingPath: [],
                    debugDescription: "Cannot encode value"
                )
            )
        }
    }
}