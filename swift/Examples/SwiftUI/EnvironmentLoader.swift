import Foundation

/// Simple utility to load environment variables from a .env file
struct EnvironmentLoader {
    
    /// Load environment variables from a .env file in the specified directory
    /// - Parameter directory: The directory containing the .env file (defaults to current working directory)
    /// - Returns: Dictionary of environment variables
    static func loadEnvironment(from directory: String? = nil) -> [String: String] {
        let workingDirectory = directory ?? FileManager.default.currentDirectoryPath
        let envPath = URL(fileURLWithPath: workingDirectory).appendingPathComponent(".env").path
        
        var environment: [String: String] = [:]
        
        // First, load from actual environment variables
        environment = ProcessInfo.processInfo.environment
        
        // Then, load from .env file (if it exists) and override
        guard let envContent = try? String(contentsOfFile: envPath, encoding: .utf8) else {
            print("Warning: .env file not found at \(envPath)")
            print("Please copy .env.example to .env and configure your Ditto credentials")
            return environment
        }
        
        let lines = envContent.components(separatedBy: .newlines)
        
        for line in lines {
            let trimmedLine = line.trimmingCharacters(in: .whitespaces)
            
            // Skip empty lines and comments
            if trimmedLine.isEmpty || trimmedLine.hasPrefix("#") {
                continue
            }
            
            // Parse KEY=VALUE format
            let components = trimmedLine.split(separator: "=", maxSplits: 1)
            if components.count == 2 {
                let key = String(components[0]).trimmingCharacters(in: .whitespaces)
                let value = String(components[1]).trimmingCharacters(in: .whitespaces)
                environment[key] = value
            }
        }
        
        return environment
    }
    
    /// Get a required environment variable, throwing an error if not found
    /// - Parameter key: The environment variable key
    /// - Returns: The environment variable value
    /// - Throws: EnvironmentError if the variable is not found or empty
    static func requireEnvironmentVariable(_ key: String, from environment: [String: String]? = nil) throws -> String {
        let env = environment ?? loadEnvironment()
        
        guard let value = env[key], !value.isEmpty else {
            throw EnvironmentError.missingRequiredVariable(key)
        }
        
        return value
    }
    
    /// Get an optional environment variable
    /// - Parameters:
    ///   - key: The environment variable key
    ///   - defaultValue: Default value if not found
    ///   - environment: Optional pre-loaded environment dictionary
    /// - Returns: The environment variable value or the default value
    static func getEnvironmentVariable(_ key: String, defaultValue: String = "", from environment: [String: String]? = nil) -> String {
        let env = environment ?? loadEnvironment()
        return env[key] ?? defaultValue
    }
}

/// Errors that can occur when loading environment variables
enum EnvironmentError: LocalizedError {
    case missingRequiredVariable(String)
    
    var errorDescription: String? {
        switch self {
        case .missingRequiredVariable(let key):
            return "Required environment variable '\(key)' is missing or empty. Please check your .env file."
        }
    }
}