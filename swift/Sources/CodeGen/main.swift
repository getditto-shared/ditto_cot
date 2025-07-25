import ArgumentParser
import Foundation

@main
struct DittoCoTCodeGen: ParsableCommand {
    static let configuration = CommandConfiguration(
        commandName: "ditto-cot-codegen",
        abstract: "Generate Swift types from JSON schemas for Ditto CoT library"
    )
    
    @Option(name: .shortAndLong, help: "Path to the schema directory")
    var schemaPath: String = "../../schema"
    
    @Option(name: .shortAndLong, help: "Output directory for generated Swift files")
    var outputPath: String = "../DittoCoTCore/Generated"
    
    func run() throws {
        let fileManager = FileManager.default
        
        // Ensure output directory exists
        try fileManager.createDirectory(atPath: outputPath, withIntermediateDirectories: true)
        
        // Read and parse schemas
        let schemas = try loadSchemas(from: schemaPath)
        
        // Generate Swift types
        let generator = SwiftTypeGenerator(schemas: schemas)
        let generatedCode = try generator.generate()
        
        // Write generated files
        for (filename, content) in generatedCode {
            let filePath = "\(outputPath)/\(filename)"
            try content.write(toFile: filePath, atomically: true, encoding: .utf8)
            print("Generated: \(filePath)")
        }
        
        print("Code generation completed successfully!")
    }
    
    private func loadSchemas(from path: String) throws -> [String: Any] {
        var schemas: [String: Any] = [:]
        
        let schemaFiles = ["common", "api", "chat", "file", "mapitem", "generic", "ditto"]
        
        for schemaName in schemaFiles {
            let schemaPath = "\(path)/\(schemaName).schema.json"
            let data = try Data(contentsOf: URL(fileURLWithPath: schemaPath))
            let json = try JSONSerialization.jsonObject(with: data) as! [String: Any]
            schemas[schemaName] = json
        }
        
        return schemas
    }
}

// Swift Type Generator
class SwiftTypeGenerator {
    private let schemas: [String: Any]
    
    init(schemas: [String: Any]) {
        self.schemas = schemas
    }
    
    func generate() throws -> [String: String] {
        var files: [String: String] = [:]
        
        // Generate JSON helper types
        files["JSONValue.swift"] = generateJSONValue()
        
        // Generate base protocol
        files["DittoDocument.swift"] = generateBaseProtocol()
        
        // Generate document types
        files["DocumentTypes.swift"] = generateDocumentTypes()
        
        // Generate union type
        files["DittoCoTDocument.swift"] = generateUnionType()
        
        return files
    }
    
    private func generateJSONValue() -> String {
        return """
        // Generated file - do not edit
        import Foundation
        
        /// A type that can represent any JSON value
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
                if value == nil {
                    self = .null
                } else if let bool = value as? Bool {
                    self = .bool(bool)
                } else if let int = value as? Int {
                    self = .number(Double(int))
                } else if let double = value as? Double {
                    self = .number(double)
                } else if let string = value as? String {
                    self = .string(string)
                } else if let array = value as? [Any] {
                    self = .array(array.map { JSONValue($0) })
                } else if let dict = value as? [String: Any] {
                    self = .object(dict.mapValues { JSONValue($0) })
                } else {
                    self = .null
                }
            }
        }
        """
    }
    
    private func generateBaseProtocol() -> String {
        return """
        // Generated file - do not edit
        import Foundation
        
        /// Base protocol for all Ditto CoT documents
        public protocol DittoDocument: Codable {
            var _id: String { get }
            var _c: Int { get }
            var _v: Int { get }
            var _r: Bool { get }
            var type: String { get }
        }
        
        /// Extension to provide default implementations
        extension DittoDocument {
            public var _v: Int { 2 }
        }
        """
    }
    
    private func generateDocumentTypes() -> String {
        guard let commonSchema = schemas["common"] as? [String: Any],
              let commonProperties = commonSchema["properties"] as? [String: Any] else {
            return "// Error: common schema not found"
        }
        
        var code = """
        // Generated file - do not edit
        import Foundation
        
        """
        
        let documentTypes = ["api", "chat", "file", "mapitem", "generic"]
        
        for typeName in documentTypes {
            guard let schema = schemas[typeName] as? [String: Any] else { continue }
            
            let structName = typeName.capitalized + "Document"
            let title = (schema["title"] as? String) ?? typeName.capitalized
            
            code += """
            
            /// \(title) document type
            public struct \(structName): DittoDocument {
                public let type = "\(typeName)"
                
                // Ditto system fields
                public var _id: String
                public var _c: Int
                public var _v: Int { 2 }
                public var _r: Bool
            
            """
            
            // Add common fields
            for (key, propertyData) in commonProperties.sorted(by: { $0.key < $1.key }) {
                if !key.hasPrefix("_"), let property = propertyData as? [String: Any] {
                    let swiftType = mapToSwiftType(property)
                    let defaultValue = getDefaultValue(from: property)
                    code += "    public var \(key): \(swiftType)"
                    if let defaultValue = defaultValue {
                        code += " = \(defaultValue)"
                    } else if key == "r" && swiftType == "JSONValue" {
                        // Special case: r field should default to empty object
                        code += " = JSONValue([:])"
                    }
                    code += "\n"
                }
            }
            
            // Add type-specific fields from allOf
            if let allOf = schema["allOf"] as? [[String: Any]] {
                for ref in allOf {
                    if let properties = ref["properties"] as? [String: Any] {
                        for (key, propertyData) in properties.sorted(by: { $0.key < $1.key }) {
                            if key != "@type", let property = propertyData as? [String: Any] {
                                let swiftType = mapToSwiftType(property)
                                let defaultValue = getDefaultValue(from: property)
                                code += "    public var \(key): \(swiftType)"
                                if let defaultValue = defaultValue {
                                    code += " = \(defaultValue)"
                                }
                                code += "\n"
                            }
                        }
                    }
                }
            }
            
            code += """
            }
            """
        }
        
        return code
    }
    
    private func generateUnionType() -> String {
        return """
        // Generated file - do not edit
        import Foundation
        
        /// Union type representing any CoT document
        public enum DittoCoTDocument: Codable {
            case api(ApiDocument)
            case chat(ChatDocument)
            case file(FileDocument)
            case mapitem(MapitemDocument)
            case generic(GenericDocument)
            
            private enum CodingKeys: String, CodingKey {
                case type = "@type"
            }
            
            public init(from decoder: Decoder) throws {
                let container = try decoder.container(keyedBy: CodingKeys.self)
                let type = try container.decode(String.self, forKey: .type)
                
                switch type {
                case "api":
                    self = .api(try ApiDocument(from: decoder))
                case "chat":
                    self = .chat(try ChatDocument(from: decoder))
                case "file":
                    self = .file(try FileDocument(from: decoder))
                case "mapitem":
                    self = .mapitem(try MapitemDocument(from: decoder))
                case "generic":
                    self = .generic(try GenericDocument(from: decoder))
                default:
                    throw DecodingError.dataCorruptedError(
                        forKey: .type,
                        in: container,
                        debugDescription: "Unknown document type: \\(type)"
                    )
                }
            }
            
            public func encode(to encoder: Encoder) throws {
                switch self {
                case .api(let document):
                    try document.encode(to: encoder)
                case .chat(let document):
                    try document.encode(to: encoder)
                case .file(let document):
                    try document.encode(to: encoder)
                case .mapitem(let document):
                    try document.encode(to: encoder)
                case .generic(let document):
                    try document.encode(to: encoder)
                }
            }
        }
        """
    }
    
    private func mapToSwiftType(_ property: [String: Any]) -> String {
        if let type = property["type"] as? String {
            switch type {
            case "string":
                return "String"
            case "integer":
                return "Int"
            case "number":
                return "Double"
            case "boolean":
                return "Bool"
            case "object":
                if property["additionalProperties"] != nil {
                    return "JSONValue"
                }
                return "JSONValue"
            default:
                return "JSONValue"
            }
        } else if let typeArray = property["type"] as? [String] {
            // Handle union types like ["string", "null"]
            if typeArray.contains("null") {
                let nonNullTypes = typeArray.filter { $0 != "null" }
                if let firstType = nonNullTypes.first {
                    let mockProperty = ["type": firstType]
                    return mapToSwiftType(mockProperty) + "?"
                }
            }
        }
        return "JSONValue"
    }
    
    private func getDefaultValue(from property: [String: Any]) -> String? {
        if let constValue = property["const"] {
            return valueToSwiftLiteral(constValue, forType: mapToSwiftType(property))
        }
        if let defaultValue = property["default"] {
            return valueToSwiftLiteral(defaultValue, forType: mapToSwiftType(property))
        }
        return nil
    }
    
    private func valueToSwiftLiteral(_ value: Any, forType type: String) -> String {
        if type == "JSONValue" {
            return "JSONValue(\(valueToAnyLiteral(value)))"
        }
        
        switch value {
        case let string as String:
            return "\"\(string)\""
        case let int as Int:
            return "\(int)"
        case let double as Double:
            if double.truncatingRemainder(dividingBy: 1) == 0 {
                return "\(Int(double))"
            }
            return "\(double)"
        case let bool as Bool:
            return "\(bool)"
        default:
            return "nil"
        }
    }
    
    private func valueToAnyLiteral(_ value: Any) -> String {
        switch value {
        case let string as String:
            return "\"\(string)\""
        case let int as Int:
            return "\(int)"
        case let double as Double:
            if double.truncatingRemainder(dividingBy: 1) == 0 {
                return "\(Int(double))"
            }
            return "\(double)"
        case let bool as Bool:
            return "\(bool)"
        case let dict as [String: Any]:
            let pairs = dict.map { key, value in
                "\"\(key)\": \(valueToAnyLiteral(value))"
            }.joined(separator: ", ")
            return "[\(pairs)]"
        case let array as [Any]:
            let items = array.map { valueToAnyLiteral($0) }.joined(separator: ", ")
            return "[\(items)]"
        default:
            return "nil"
        }
    }
}