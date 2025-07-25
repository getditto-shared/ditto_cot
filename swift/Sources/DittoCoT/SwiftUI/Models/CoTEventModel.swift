import Foundation
import CoreLocation
import DittoSwift

/// UI-friendly model for CoT events
public struct CoTEventModel: Identifiable, Hashable, Equatable {
    public let id: String
    public let uid: String
    public let type: String
    public let callsign: String
    public let timestamp: Date
    public let location: CLLocationCoordinate2D
    public let altitude: Double?
    public let accuracy: Double?
    public let staleTime: Date
    public let how: String
    public let remarks: String?
    public let isStale: Bool
    public let eventCategory: EventCategory
    
    public init(
        uid: String,
        type: String,
        callsign: String, 
        timestamp: Date,
        location: CLLocationCoordinate2D,
        altitude: Double? = nil,
        accuracy: Double? = nil,
        staleTime: Date,
        how: String,
        remarks: String? = nil
    ) {
        self.id = uid
        self.uid = uid
        self.type = type
        self.callsign = callsign
        self.timestamp = timestamp
        self.location = location
        self.altitude = altitude
        self.accuracy = accuracy
        self.staleTime = staleTime
        self.how = how
        self.remarks = remarks
        self.isStale = Date() > staleTime
        self.eventCategory = EventCategory.from(type: type)
    }
}

extension CoTEventModel {
    /// Create from Ditto document
    public init?(from document: DittoSwift.DittoDocument) {
        guard let uid = document.value["_id"] as? String,
              let type = document.value["w"] as? String,
              let callsign = document.value["e"] as? String,
              let timestampMillis = document.value["b"] as? Double,
              let lat = document.value["j"] as? Double,
              let lon = document.value["l"] as? Double,
              let staleMillis = document.value["o"] as? Double,
              let how = document.value["p"] as? String else {
            return nil
        }
        
        let timestamp = Date(timeIntervalSince1970: timestampMillis / 1000)
        let staleTime = Date(timeIntervalSince1970: staleMillis / 1000)
        let location = CLLocationCoordinate2D(latitude: lat, longitude: lon)
        
        // Extract optional fields
        let altitude = document.value["i"] as? Double
        let accuracy = document.value["h"] as? Double
        
        // Extract remarks from r field
        var remarks: String?
        if let rField = document.value["r"] as? [String: Any],
           let remarksDict = rField["remarks"] as? [String: Any],
           let remarksText = remarksDict["_text"] as? String {
            remarks = remarksText
        }
        
        self.init(
            uid: uid,
            type: type,
            callsign: callsign,
            timestamp: timestamp,
            location: location,
            altitude: altitude,
            accuracy: accuracy,
            staleTime: staleTime,
            how: how,
            remarks: remarks
        )
    }
    
    // MARK: - Hashable & Equatable Implementation
    
    public func hash(into hasher: inout Hasher) {
        hasher.combine(id)
        hasher.combine(uid)
        hasher.combine(type)
        hasher.combine(callsign)
        hasher.combine(timestamp)
        hasher.combine(location.latitude)
        hasher.combine(location.longitude)
        hasher.combine(altitude)
        hasher.combine(accuracy)
        hasher.combine(staleTime)
        hasher.combine(how)
        hasher.combine(remarks)
        hasher.combine(isStale)
        hasher.combine(eventCategory)
    }
    
    public static func == (lhs: CoTEventModel, rhs: CoTEventModel) -> Bool {
        return lhs.id == rhs.id &&
               lhs.uid == rhs.uid &&
               lhs.type == rhs.type &&
               lhs.callsign == rhs.callsign &&
               lhs.timestamp == rhs.timestamp &&
               lhs.location.latitude == rhs.location.latitude &&
               lhs.location.longitude == rhs.location.longitude &&
               lhs.altitude == rhs.altitude &&
               lhs.accuracy == rhs.accuracy &&
               lhs.staleTime == rhs.staleTime &&
               lhs.how == rhs.how &&
               lhs.remarks == rhs.remarks &&
               lhs.isStale == rhs.isStale &&
               lhs.eventCategory == rhs.eventCategory
    }
}

/// Chat message model for UI
public struct ChatMessageModel: Identifiable, Hashable {
    public let id: String
    public let from: String
    public let message: String
    public let room: String
    public let timestamp: Date
    public let isFromCurrentUser: Bool
    
    public init(
        id: String,
        from: String,
        message: String,
        room: String,
        timestamp: Date,
        isFromCurrentUser: Bool = false
    ) {
        self.id = id
        self.from = from
        self.message = message
        self.room = room
        self.timestamp = timestamp
        self.isFromCurrentUser = isFromCurrentUser
    }
}

extension ChatMessageModel {
    /// Create from Ditto document
    public init?(from document: DittoSwift.DittoDocument) {
        guard let uid = document.value["_id"] as? String,
              let timestampMillis = document.value["b"] as? Double,
              let rField = document.value["r"] as? [String: Any],
              let chatDict = rField["chat"] as? [String: Any],
              let from = chatDict["from"] as? String,
              let message = chatDict["msg"] as? String else {
            return nil
        }
        
        let room = chatDict["room"] as? String ?? "All Chat Rooms"
        let timestamp = Date(timeIntervalSince1970: timestampMillis / 1000)
        
        self.init(
            id: uid,
            from: from,
            message: message,
            room: room,
            timestamp: timestamp
        )
    }
}

/// Location update model for UI
public struct LocationUpdateModel: Identifiable, Hashable, Equatable {
    public let id: String
    public let callsign: String
    public let location: CLLocationCoordinate2D
    public let altitude: Double?
    public let accuracy: Double?
    public let timestamp: Date
    public let speed: Double?
    public let course: Double?
    
    public init(
        id: String,
        callsign: String,
        location: CLLocationCoordinate2D,
        altitude: Double? = nil,
        accuracy: Double? = nil,
        timestamp: Date,
        speed: Double? = nil,
        course: Double? = nil
    ) {
        self.id = id
        self.callsign = callsign
        self.location = location
        self.altitude = altitude
        self.accuracy = accuracy
        self.timestamp = timestamp
        self.speed = speed
        self.course = course
    }
}

extension LocationUpdateModel {
    /// Create from Ditto document
    public init?(from document: DittoSwift.DittoDocument) {
        guard let uid = document.value["_id"] as? String,
              let callsign = document.value["e"] as? String,
              let lat = document.value["j"] as? Double,
              let lon = document.value["l"] as? Double,
              let timestampMillis = document.value["b"] as? Double else {
            return nil
        }
        
        let location = CLLocationCoordinate2D(latitude: lat, longitude: lon)
        let timestamp = Date(timeIntervalSince1970: timestampMillis / 1000)
        let altitude = document.value["i"] as? Double
        let accuracy = document.value["h"] as? Double
        
        // Extract speed and course from r field
        var speed: Double?
        var course: Double?
        if let rField = document.value["r"] as? [String: Any],
           let trackDict = rField["track"] as? [String: Any] {
            speed = trackDict["speed"] as? Double
            course = trackDict["course"] as? Double
        }
        
        self.init(
            id: uid,
            callsign: callsign,
            location: location,
            altitude: altitude,
            accuracy: accuracy,
            timestamp: timestamp,
            speed: speed,
            course: course
        )
    }
    
    // MARK: - Hashable & Equatable Implementation
    
    public func hash(into hasher: inout Hasher) {
        hasher.combine(id)
        hasher.combine(callsign)
        hasher.combine(location.latitude)
        hasher.combine(location.longitude)
        hasher.combine(altitude)
        hasher.combine(accuracy)
        hasher.combine(timestamp)
        hasher.combine(speed)
        hasher.combine(course)
    }
    
    public static func == (lhs: LocationUpdateModel, rhs: LocationUpdateModel) -> Bool {
        return lhs.id == rhs.id &&
               lhs.callsign == rhs.callsign &&
               lhs.location.latitude == rhs.location.latitude &&
               lhs.location.longitude == rhs.location.longitude &&
               lhs.altitude == rhs.altitude &&
               lhs.accuracy == rhs.accuracy &&
               lhs.timestamp == rhs.timestamp &&
               lhs.speed == rhs.speed &&
               lhs.course == rhs.course
    }
}

/// Emergency event model for UI
public struct EmergencyEventModel: Identifiable, Hashable, Equatable {
    public let id: String
    public let callsign: String
    public let emergencyType: String
    public let location: CLLocationCoordinate2D
    public let timestamp: Date
    public let isActive: Bool
    
    public init(
        id: String,
        callsign: String,
        emergencyType: String,
        location: CLLocationCoordinate2D,
        timestamp: Date,
        isActive: Bool = true
    ) {
        self.id = id
        self.callsign = callsign
        self.emergencyType = emergencyType
        self.location = location
        self.timestamp = timestamp
        self.isActive = isActive
    }
}

extension EmergencyEventModel {
    /// Create from Ditto document
    public init?(from document: DittoSwift.DittoDocument) {
        guard let uid = document.value["_id"] as? String,
              let callsign = document.value["e"] as? String,
              let lat = document.value["j"] as? Double,
              let lon = document.value["l"] as? Double,
              let timestampMillis = document.value["b"] as? Double,
              let type = document.value["w"] as? String,
              type.contains("emergency") || type.contains("911") else {
            return nil
        }
        
        let location = CLLocationCoordinate2D(latitude: lat, longitude: lon)
        let timestamp = Date(timeIntervalSince1970: timestampMillis / 1000)
        
        // Extract emergency type from r field
        var emergencyType = "general"
        if let rField = document.value["r"] as? [String: Any],
           let emergencyDict = rField["emergency"] as? [String: Any],
           let type = emergencyDict["type"] as? String {
            emergencyType = type
        }
        
        let isActive = !(document.value["_r"] as? Bool ?? false)
        
        self.init(
            id: uid,
            callsign: callsign,
            emergencyType: emergencyType,
            location: location,
            timestamp: timestamp,
            isActive: isActive
        )
    }
    
    // MARK: - Hashable & Equatable Implementation
    
    public func hash(into hasher: inout Hasher) {
        hasher.combine(id)
        hasher.combine(callsign)
        hasher.combine(emergencyType)
        hasher.combine(location.latitude)
        hasher.combine(location.longitude)
        hasher.combine(timestamp)
        hasher.combine(isActive)
    }
    
    public static func == (lhs: EmergencyEventModel, rhs: EmergencyEventModel) -> Bool {
        return lhs.id == rhs.id &&
               lhs.callsign == rhs.callsign &&
               lhs.emergencyType == rhs.emergencyType &&
               lhs.location.latitude == rhs.location.latitude &&
               lhs.location.longitude == rhs.location.longitude &&
               lhs.timestamp == rhs.timestamp &&
               lhs.isActive == rhs.isActive
    }
}

/// Event category for UI organization
public enum EventCategory: String, CaseIterable, Identifiable {
    case friendly = "friendly"
    case hostile = "hostile" 
    case neutral = "neutral"
    case unknown = "unknown"
    case chat = "chat"
    case emergency = "emergency"
    
    public var id: String { rawValue }
    
    public var displayName: String {
        switch self {
        case .friendly: return "Friendly"
        case .hostile: return "Hostile"
        case .neutral: return "Neutral"
        case .unknown: return "Unknown"
        case .chat: return "Chat"
        case .emergency: return "Emergency"
        }
    }
    
    public var systemImage: String {
        switch self {
        case .friendly: return "checkmark.shield.fill"
        case .hostile: return "xmark.shield.fill"
        case .neutral: return "minus.circle.fill"
        case .unknown: return "questionmark.circle.fill" 
        case .chat: return "message.fill"
        case .emergency: return "exclamationmark.triangle.fill"
        }
    }
    
    static func from(type: String) -> EventCategory {
        if type.hasPrefix("b-t-f") {
            return .chat
        } else if type.contains("emergency") || type.contains("911") {
            return .emergency
        } else if type.hasPrefix("a-f") {
            return .friendly
        } else if type.hasPrefix("a-h") {
            return .hostile
        } else if type.hasPrefix("a-n") {
            return .neutral
        } else {
            return .unknown
        }
    }
}