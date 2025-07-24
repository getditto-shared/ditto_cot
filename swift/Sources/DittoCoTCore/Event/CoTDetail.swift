import Foundation
import XMLCoder

/// Represents the detail element of a CoT event
/// This is a flexible container that can hold any structured data
public struct CoTDetail: Codable, Equatable {
    /// The raw detail data stored as a flexible JSON-compatible structure
    private let storage: JSONValue
    
    /// Initialize with a JSONValue
    public init(_ value: JSONValue) {
        self.storage = value
    }
    
    /// Initialize with a dictionary
    public init(_ dictionary: [String: Any]) {
        self.storage = JSONValue(dictionary)
    }
    
    /// Initialize an empty detail
    public init() {
        self.storage = .object([:])
    }
    
    /// Access the underlying JSONValue storage
    public var value: JSONValue {
        storage
    }
    
    /// Get a value at a specific key path
    /// - Parameter keyPath: Dot-separated key path (e.g., "contact.callsign")
    /// - Returns: The value at the key path, or nil if not found
    public func getValue(at keyPath: String) -> JSONValue? {
        let keys = keyPath.split(separator: ".").map(String.init)
        var current = storage
        
        for key in keys {
            guard case .object(let dict) = current,
                  let next = dict[key] else {
                return nil
            }
            current = next
        }
        
        return current
    }
    
    /// Set a value at a specific key path
    /// - Parameters:
    ///   - keyPath: Dot-separated key path (e.g., "contact.callsign")
    ///   - value: The value to set
    /// - Returns: A new CoTDetail with the value set
    public func settingValue(_ value: JSONValue, at keyPath: String) -> CoTDetail {
        let keys = keyPath.split(separator: ".").map(String.init)
        
        func setValue(in json: JSONValue, keys: ArraySlice<String>, value: JSONValue) -> JSONValue {
            guard let key = keys.first else { return value }
            
            let remainingKeys = keys.dropFirst()
            
            if case .object(var dict) = json {
                if remainingKeys.isEmpty {
                    dict[key] = value
                } else {
                    let existing = dict[key] ?? .object([:])
                    dict[key] = setValue(in: existing, keys: remainingKeys, value: value)
                }
                return .object(dict)
            } else if keys.count == 1 {
                // If we're at a non-object, replace it with an object containing our value
                return .object([key: value])
            } else {
                // Create nested structure
                var dict: [String: JSONValue] = [:]
                dict[key] = setValue(in: .object([:]), keys: remainingKeys, value: value)
                return .object(dict)
            }
        }
        
        let newStorage = setValue(in: storage, keys: ArraySlice(keys), value: value)
        return CoTDetail(newStorage)
    }
    
    // MARK: - Common CoT Detail Properties
    
    /// Get or set the contact callsign
    public var callsign: String? {
        getValue(at: "contact.callsign")?.stringValue
    }
    
    /// Get or set the remarks text
    public var remarks: String? {
        getValue(at: "remarks")?.stringValue
    }
    
    /// Get or set the color (ARGB hex format)
    public var color: String? {
        getValue(at: "color.argb")?.stringValue
    }
}

// MARK: - Codable
extension CoTDetail {
    public init(from decoder: Decoder) throws {
        let container = try decoder.singleValueContainer()
        self.storage = try container.decode(JSONValue.self)
    }
    
    public func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()
        try container.encode(storage)
    }
}

// MARK: - XML Coding Support
extension CoTDetail {
    /// Convert the detail to a dictionary for XML encoding
    var xmlDictionary: [String: Any] {
        storage.xmlDictionary
    }
    
    /// Create CoTDetail from XML dictionary
    init(xmlDictionary: [String: Any]) {
        self.storage = JSONValue(xmlDictionary)
    }
}

// MARK: - JSONValue XML Support
private extension JSONValue {
    var xmlDictionary: [String: Any] {
        switch self {
        case .null:
            return [:]
        case .bool(let value):
            return ["value": value]
        case .number(let value):
            return ["value": value]
        case .string(let value):
            return ["value": value]
        case .array(let values):
            let mappedValues = values.enumerated().map { (index, value) in
                ("item_\(index)", value.xmlDictionary)
            }
            return Dictionary(mappedValues) { _, new in new }
        case .object(let dict):
            return dict.mapValues { $0.xmlDictionary }
        }
    }
    
    init(_ xmlDict: [String: Any]) {
        // Simple conversion - this could be enhanced
        if let singleValue = xmlDict["value"] {
            if let bool = singleValue as? Bool {
                self = .bool(bool)
            } else if let number = singleValue as? Double {
                self = .number(number)
            } else if let int = singleValue as? Int {
                self = .number(Double(int))
            } else if let string = singleValue as? String {
                self = .string(string)
            } else {
                self = .string("\(singleValue)")
            }
        } else {
            let converted = xmlDict.compactMapValues { value -> JSONValue? in
                if let dict = value as? [String: Any] {
                    return JSONValue(dict)
                } else if let array = value as? [Any] {
                    return .array(array.map { JSONValue($0) })
                } else if let string = value as? String {
                    return .string(string)
                } else if let int = value as? Int {
                    return .number(Double(int))
                } else if let double = value as? Double {
                    return .number(double)
                } else if let bool = value as? Bool {
                    return .bool(bool)
                } else {
                    return .string("\(value)")
                }
            }
            self = .object(converted)
        }
    }
}

// MARK: - CustomStringConvertible
extension CoTDetail: CustomStringConvertible {
    public var description: String {
        "CoTDetail(\(storage))"
    }
}

// MARK: - JSONValue Helpers
private extension JSONValue {
    var stringValue: String? {
        if case .string(let value) = self {
            return value
        }
        return nil
    }
}