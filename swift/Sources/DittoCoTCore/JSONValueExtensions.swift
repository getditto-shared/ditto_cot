import Foundation

/// Extensions for JSONValue to provide convenience accessors
/// This file provides the stringValue and other convenience properties
/// needed by CoTDetail.swift without being overwritten by code generation.
extension JSONValue {
    /// Extract string value if this is a string JSONValue
    public var stringValue: String? {
        switch self {
        case .string(let value):
            return value
        default:
            return nil
        }
    }
    
    /// Extract number value if this is a number JSONValue
    public var numberValue: Double? {
        switch self {
        case .number(let value):
            return value
        default:
            return nil
        }
    }
    
    /// Extract boolean value if this is a bool JSONValue
    public var boolValue: Bool? {
        switch self {
        case .bool(let value):
            return value
        default:
            return nil
        }
    }
    
    /// Extract array value if this is an array JSONValue
    public var arrayValue: [JSONValue]? {
        switch self {
        case .array(let value):
            return value
        default:
            return nil
        }
    }
    
    /// Extract object value if this is an object JSONValue
    public var objectValue: [String: JSONValue]? {
        switch self {
        case .object(let value):
            return value
        default:
            return nil
        }
    }
}