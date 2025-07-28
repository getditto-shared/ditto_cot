import Foundation

/// A flexible JSON-compatible value type for handling dynamic CoT event details
public enum JSONValue: Codable, Equatable {
    case null
    case bool(Bool)
    case number(Double)
    case string(String)
    case array([JSONValue])
    case object([String: JSONValue])
    
    public init(from decoder: Decoder) throws {
        let container = try decoder.singleValueContainer()
        
        if container.decodeNil() {
            self = .null
        } else if let bool = try? container.decode(Bool.self) {
            self = .bool(bool)
        } else if let number = try? container.decode(Double.self) {
            self = .number(number)
        } else if let string = try? container.decode(String.self) {
            self = .string(string)
        } else if let array = try? container.decode([JSONValue].self) {
            self = .array(array)
        } else if let object = try? container.decode([String: JSONValue].self) {
            self = .object(object)
        } else {
            throw DecodingError.dataCorruptedError(in: container, debugDescription: "Cannot decode JSONValue")
        }
    }
    
    public func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()
        
        switch self {
        case .null:
            try container.encodeNil()
        case .bool(let value):
            try container.encode(value)
        case .number(let value):
            try container.encode(value)
        case .string(let value):
            try container.encode(value)
        case .array(let value):
            try container.encode(value)
        case .object(let value):
            try container.encode(value)
        }
    }
}

/// Convenience extension for creating JSONValue from common Swift types
extension JSONValue {
    public init(_ value: Any?) {
        switch value {
        case nil:
            self = .null
        case let bool as Bool:
            self = .bool(bool)
        case let int as Int:
            self = .number(Double(int))
        case let double as Double:
            self = .number(double)
        case let float as Float:
            self = .number(Double(float))
        case let string as String:
            self = .string(string)
        case let array as [Any?]:
            self = .array(array.map(JSONValue.init))
        case let dict as [String: Any?]:
            self = .object(dict.compactMapValues { $0 == nil ? .null : JSONValue($0) })
        default:
            // Fallback for unknown types - convert to string
            self = .string(String(describing: value))
        }
    }
    
    /// Extract the underlying Swift value
    public var swiftValue: Any? {
        switch self {
        case .null:
            return nil
        case .bool(let value):
            return value
        case .number(let value):
            return value
        case .string(let value):
            return value
        case .array(let value):
            return value.map { $0.swiftValue }
        case .object(let value):
            return value.mapValues { $0.swiftValue }
        }
    }
    
    /// Convenience accessor for string values
    public var stringValue: String? {
        if case .string(let value) = self {
            return value
        }
        return nil
    }
    
    /// Convenience accessor for number values
    public var numberValue: Double? {
        if case .number(let value) = self {
            return value
        }
        return nil
    }
    
    /// Convenience accessor for boolean values
    public var boolValue: Bool? {
        if case .bool(let value) = self {
            return value
        }
        return nil
    }
    
    /// Convenience accessor for array values
    public var arrayValue: [JSONValue]? {
        if case .array(let value) = self {
            return value
        }
        return nil
    }
    
    /// Convenience accessor for object values
    public var objectValue: [String: JSONValue]? {
        if case .object(let value) = self {
            return value
        }
        return nil
    }
}