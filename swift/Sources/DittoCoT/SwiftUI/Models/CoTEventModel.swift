import Foundation
import CoreLocation
import DittoSwift
import DittoCoTCore

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
    public let rawDocumentData: [String: Any]? // Store raw document for JSON display
    
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
        remarks: String? = nil,
        rawDocumentData: [String: Any]? = nil
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
        self.rawDocumentData = rawDocumentData
        self.isStale = Date() > staleTime
        self.eventCategory = EventCategory.from(type: type)
    }
}

extension CoTEventModel {
    /// Create from Ditto document
    public init?(from document: DittoSwift.DittoDocument) {
        // Convert document to CoTEvent first
        guard let cotEvent = CoTEvent(from: document) else {
            print("❌ CoTEventModel: Failed to convert document \(document.id) to CoTEvent")
            // Log the document type to debug
            if let type = document.value["w"] as? String {
                print("   Document type: \(type)")
            }
            // Check if it's a chat document with msg/room fields
            if document.value["msg"] != nil && document.value["room"] != nil {
                print("   This is a chat document but failed CoTEvent conversion")
            }
            return nil
        }
        
        // Store raw document data for JSON display
        let rawData = document.value as [String: Any]
        
        // Now create model from CoTEvent with raw document data
        self.init(from: cotEvent, rawDocumentData: rawData)
    }
    
    /// Create from CoT Event
    public init(from event: CoTEvent, rawDocumentData: [String: Any]? = nil) {
        let location = CLLocationCoordinate2D(
            latitude: event.point.lat,
            longitude: event.point.lon
        )
        
        // Extract remarks from detail
        var remarks: String?
        if let detailDict = event.detail?.toDict(),
           let remarksValue = detailDict["remarks"] {
            if let remarksStr = remarksValue as? String {
                remarks = remarksStr
            } else if let remarksDict = remarksValue as? [String: Any],
                      let remarksText = remarksDict["_text"] as? String {
                remarks = remarksText
            }
        }
        
        self.init(
            uid: event.uid,
            type: event.type,
            callsign: Self.extractCallsign(from: event) ?? event.uid,
            timestamp: event.time,
            location: location,
            altitude: event.point.hae > 0 ? event.point.hae : nil,
            accuracy: event.point.ce > 0 ? event.point.ce : nil,
            staleTime: event.stale,
            how: event.how,
            remarks: remarks,
            rawDocumentData: rawDocumentData
        )
    }
    
    private static func extractCallsign(from event: CoTEvent) -> String? {
        guard let detailDict = event.detail?.toDict() else { return nil }
        
        // Check contact callsign
        if let contact = detailDict["contact"] as? [String: Any],
           let callsign = contact["callsign"] as? String {
            return callsign
        }
        
        // Check chat from field
        if let chat = detailDict["chat"] as? [String: Any],
           let from = chat["from"] as? String {
            return from
        }
        
        return nil
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
        // Note: Not hashing rawDocumentData as it's for display only and could be complex
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
               // Note: Not comparing rawDocumentData as it's for display only
    }
    
    /// Format the raw document data as pretty-printed JSON
    public var formattedJSON: String {
        guard let rawData = rawDocumentData else {
            return "No raw document data available"
        }
        
        do {
            let jsonData = try JSONSerialization.data(withJSONObject: rawData, options: [.prettyPrinted, .sortedKeys])
            return String(data: jsonData, encoding: .utf8) ?? "Failed to format JSON"
        } catch {
            return "Failed to serialize JSON: \(error.localizedDescription)"
        }
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
        guard let uid = document.value["_id"] as? String else {
            return nil
        }
        
        // Try direct chat document conversion first (for documents like from "Liquid")
        // Schema shows message field can be "message" OR "msg"
        let message = document.value["message"] as? String ?? document.value["msg"] as? String
        let room = document.value["room"] as? String
        
        if let message = message, let room = room {
            print("📱 Converting chat document \(uid) directly: \(message)")
            print("📱 FULL CHAT DOCUMENT STRUCTURE:")
            print("📱 Document ID: \(uid)")
            print("📱 All fields: \(Array(document.value.keys).sorted())")
            for (key, value) in document.value.sorted(by: { $0.key < $1.key }) {
                print("📱   \(key): \(type(of: value)) = \(value)")
            }
            print("📱 END DOCUMENT STRUCTURE")
            
            let from = document.value["e"] as? String ?? "Unknown"
            
            // Use the 'time' field from chat document schema (ISO8601 string)
            let timestamp: Date
            if let timeString = document.value["time"] as? String {
                let formatter = ISO8601DateFormatter()
                formatter.formatOptions = [.withInternetDateTime, .withFractionalSeconds]
                
                if let parsedDate = formatter.date(from: timeString) {
                    timestamp = parsedDate
                    print("📱 Chat \(uid) using 'time' field: \(timeString) -> \(timestamp)")
                } else {
                    // Try without fractional seconds
                    formatter.formatOptions = [.withInternetDateTime]
                    if let parsedDate = formatter.date(from: timeString) {
                        timestamp = parsedDate
                        print("📱 Chat \(uid) using 'time' field (no fractionals): \(timeString) -> \(timestamp)")
                    } else {
                        print("⚠️ Chat \(uid) couldn't parse 'time' field: \(timeString)")
                        timestamp = Date()
                    }
                }
            } else if let bValue = document.value["b"] {
                // 'b' field could be Int64, Int, or Double - convert to milliseconds
                let bMillis: Double
                if let intValue = bValue as? Int64 {
                    bMillis = Double(intValue)
                } else if let intValue = bValue as? Int {
                    bMillis = Double(intValue)
                } else if let doubleValue = bValue as? Double {
                    bMillis = doubleValue
                } else {
                    print("⚠️ Chat \(uid) 'b' field has unexpected type: \(Swift.type(of: bValue))")
                    bMillis = 0
                }
                
                if bMillis > 0 {
                    // 'b' field is in milliseconds based on actual documents
                    timestamp = Date(timeIntervalSince1970: bMillis / 1000.0)
                    print("📱 Chat \(uid) using 'b' field: \(bMillis) ms -> \(timestamp)")
                } else {
                    timestamp = Date()
                    print("📱 Chat \(uid) 'b' field is 0 or negative: \(bMillis)")
                }
            } else if let nValue = document.value["n"] {
                // 'n' field could be Int64, Int, or Double
                let nNumeric: Double
                if let intValue = nValue as? Int64 {
                    nNumeric = Double(intValue)
                } else if let intValue = nValue as? Int {
                    nNumeric = Double(intValue)
                } else if let doubleValue = nValue as? Double {
                    nNumeric = doubleValue
                } else {
                    print("⚠️ Chat \(uid) 'n' field has unexpected type: \(Swift.type(of: nValue))")
                    nNumeric = 0
                }
                
                if nNumeric > 0 {
                    // Check magnitude to determine units
                    if nNumeric > 1_000_000_000_000 { // Likely microseconds
                        timestamp = Date(timeIntervalSince1970: nNumeric / 1_000_000.0)
                        print("📱 Chat \(uid) using 'n' field as microseconds: \(nNumeric) -> \(timestamp)")
                    } else { // Likely milliseconds
                        timestamp = Date(timeIntervalSince1970: nNumeric / 1000.0)
                        print("📱 Chat \(uid) using 'n' field as milliseconds: \(nNumeric) -> \(timestamp)")
                    }
                } else {
                    timestamp = Date()
                    print("📱 Chat \(uid) 'n' field is 0 or negative: \(nNumeric)")
                }
            } else {
                print("❌ Chat \(uid) has NO timestamp fields!")
                print("   Available fields: \(Array(document.value.keys).sorted())")
                print("   'b' field: \(String(describing: document.value["b"])) (type: \(Swift.type(of: document.value["b"])))")
                print("   'n' field: \(String(describing: document.value["n"])) (type: \(Swift.type(of: document.value["n"])))")
                timestamp = Date()
            }
            
            self.init(
                id: uid,
                from: from,
                message: message,
                room: room,
                timestamp: timestamp
            )
            return
        }
        
        // Fallback: Try converting to CoT event first (for proper CoT documents)
        guard let cotEvent = CoTEvent(from: document),
              cotEvent.type.hasPrefix("b-t-f") else {
            print("❌ Failed to convert chat document \(uid): not a chat type")
            return nil
        }
        
        // Extract chat details from event
        guard let detailDict = cotEvent.detail?.toDict(),
              let chatDict = detailDict["chat"] as? [String: Any],
              let from = chatDict["from"] as? String,
              let message = chatDict["msg"] as? String else {
            print("❌ Failed to convert chat document \(uid): missing chat details")
            return nil
        }
        
        let roomName = chatDict["room"] as? String ?? "All Chat Rooms"
        
        self.init(
            id: cotEvent.uid,
            from: from,
            message: message,
            room: roomName,
            timestamp: cotEvent.time
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
              let lon = document.value["l"] as? Double else {
            return nil
        }
        
        // Get timestamp - prefer 'n' field (start time), fallback to 'b'
        let timestampMicros: Double
        if let nValue = document.value["n"] as? Double, nValue > 0 {
            timestampMicros = nValue
        } else if let bValue = document.value["b"] as? Double, bValue > 0 {
            timestampMicros = bValue
        } else {
            return nil // Can't create location update without timestamp
        }
        
        let location = CLLocationCoordinate2D(latitude: lat, longitude: lon)
        let timestamp = Date(timeIntervalSince1970: timestampMicros / 1_000_000.0) // Convert microseconds to seconds
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
              let timestampMicros = document.value["b"] as? Double,
              let type = document.value["w"] as? String,
              type.contains("emergency") || type.contains("911") else {
            return nil
        }
        
        let location = CLLocationCoordinate2D(latitude: lat, longitude: lon)
        let timestamp = Date(timeIntervalSince1970: timestampMicros / 1_000_000.0) // Convert microseconds to seconds
        
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