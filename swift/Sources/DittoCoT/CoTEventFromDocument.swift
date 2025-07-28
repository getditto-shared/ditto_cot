import Foundation
import DittoSwift
import DittoCoTCore

/// Extension to convert Ditto documents back to CoT events
extension CoTEvent {
    
    /// Initialize a CoT event from a generic DittoDocument
    public init?(from document: DittoSwift.DittoDocument) {
        // Try to determine the document type and convert appropriately
        guard let uid = document.value["_id"] as? String else {
            print("❌ CoTEvent conversion failed: missing _id")
            return nil
        }
        
        // Detect document type based on fields (following Java implementation pattern)
        if let msg = document.value["msg"] as? String, 
           let _ = document.value["room"] as? String {
            // This is a chat document using chat schema
            guard let chatEvent = Self.fromChatDocument(document, uid: uid, message: msg) else {
                return nil
            }
            self = chatEvent
            return
        }
        // Type field (w) may be missing or empty - use default if needed
        let type = (document.value["w"] as? String)?.isEmpty == false 
            ? document.value["w"] as! String 
            : "a-f-G-U-C"  // Default to friendly unit type if missing
        // Time field handling: 'n' field is the START timestamp in microseconds
        let timestampMicros: Double
        
        // Use 'n' field as the timestamp (start time)
        if let nValue = document.value["n"] as? Double, nValue > 0 {
            timestampMicros = nValue
            print("📍 Document \(uid) using start time 'n': \(nValue)")
        } else if let bValue = document.value["b"] {
            // 'b' field could be Int64, Int, or Double
            if let intValue = bValue as? Int64 {
                timestampMicros = Double(intValue)
            } else if let intValue = bValue as? Int {
                timestampMicros = Double(intValue)
            } else if let doubleValue = bValue as? Double {
                timestampMicros = doubleValue
            } else {
                print("⚠️ Document \(uid) 'b' field has unexpected type: \(Swift.type(of: bValue))")
                timestampMicros = 0
            }
            print("⚠️ Document \(uid) missing 'n' field, using 'b': \(timestampMicros)")
        } else {
            print("❌ Document \(uid) has NO timestamp fields!")
            print("   Available fields: \(Array(document.value.keys).sorted())")
            print("   'b' field: \(String(describing: document.value["b"])) (type: \(Swift.type(of: document.value["b"])))")
            print("   'n' field: \(String(describing: document.value["n"])) (type: \(Swift.type(of: document.value["n"])))")
            // This should never happen - but don't use current time
            timestampMicros = 0
        }
        // Stale time - use fallback chain: o -> c -> timestamp + 5 minutes
        let staleMillis: Double
        if let staleValue = document.value["o"] as? Double, staleValue > 0 {
            staleMillis = staleValue
        } else if let cValue = document.value["c"] as? Double, cValue > 0 {
            staleMillis = cValue
        } else {
            staleMillis = (timestampMicros / 1000.0) + (5 * 60 * 1000) // Convert to millis then add 5 minutes
        }
        // Location fields with defaults (schema shows they can be 0.0)
        let lat = document.value["j"] as? Double ?? 0.0
        let lon = document.value["l"] as? Double ?? 0.0
        
        // Note: Don't skip (0,0) coordinates - some mapitem documents may have placeholder coordinates
        // or be intended for display regardless of location
        
        // Callsign might be empty, missing, or null - use uid as fallback
        let callsignValue = document.value["e"] as? String
        let callsign = (callsignValue?.isEmpty == false) ? callsignValue! : uid
        
        let timestamp: Date
        if timestampMicros > 0 {
            timestamp = Date(timeIntervalSince1970: timestampMicros / 1_000_000.0) // Convert microseconds to seconds
            print("🕐 Document \(uid) timestamp: \(timestamp) (from micros: \(timestampMicros))")
        } else {
            // If we couldn't get a timestamp, fail the conversion
            print("❌ Cannot create event without valid timestamp for document \(uid)")
            return nil
        }
        let staleTime = Date(timeIntervalSince1970: staleMillis / 1000.0)
        let how = document.value["p"] as? String ?? "h-g-i-g-o"
        
        // Optional location fields - handle missing values gracefully
        let hae = document.value["i"] as? Double ?? 0.0
        let ce = document.value["h"] as? Double ?? 9999999.0  // Large value for unknown accuracy
        let le = document.value["k"] as? Double ?? 9999999.0
        
        // Build detail section based on type
        var detailDict: [String: Any] = [
            "contact": ["callsign": callsign]
        ]
        
        // Handle chat messages
        if type.hasPrefix("b-t-f"), let rField = document.value["r"] as? [String: Any] {
            if let message = rField["message"] as? String, !message.isEmpty {
                detailDict["chat"] = [
                    "from": callsign,
                    "room": rField["room"] as? String ?? "All Chat Rooms",
                    "msg": message
                ]
            }
        }
        
        // Handle remarks - try multiple possible formats
        if let rField = document.value["r"] as? [String: Any] {
            // Try structured remarks first
            if let remarksDict = rField["remarks"] as? [String: Any],
               let remarksText = remarksDict["_text"] as? String, !remarksText.isEmpty {
                detailDict["remarks"] = remarksText
            }
            // Try direct string remarks
            else if let remarksText = rField["remarks"] as? String, !remarksText.isEmpty {
                detailDict["remarks"] = remarksText
            }
        }
        
        do {
            let builder = CoTEventBuilder()
                .uid(uid)
                .type(type)
                .time(timestamp)
                .how(how)
                .point(CoTPoint(
                    lat: lat,
                    lon: lon,
                    hae: hae,
                    ce: ce,
                    le: le
                ))
                .detail(CoTDetail(detailDict))
            
            self = try builder
                .validity(start: timestamp, stale: staleTime)
                .build()
        } catch {
            return nil
        }
    }
    
    /// Convert a chat document to CoT event (following Java implementation pattern)
    private static func fromChatDocument(_ document: DittoSwift.DittoDocument, uid: String, message: String) -> CoTEvent? {
        // Extract fields using the actual chat document schema
        let room = document.value["room"] as? String ?? "All Chat Rooms"
        let callsign = document.value["e"] as? String ?? uid // Fallback to uid if no callsign
        
        // Use 'n' field (start time) first, then 'b' field as fallback
        let timestamp: Date
        if let nValue = document.value["n"] {
            // 'n' field could be Int64, Int, or Double
            let nMicros: Double
            if let intValue = nValue as? Int64 {
                nMicros = Double(intValue)
            } else if let intValue = nValue as? Int {
                nMicros = Double(intValue)
            } else if let doubleValue = nValue as? Double {
                nMicros = doubleValue
            } else {
                print("⚠️ Chat \(uid) 'n' field has unexpected type: \(Swift.type(of: nValue))")
                nMicros = 0
            }
            
            if nMicros > 0 {
                timestamp = Date(timeIntervalSince1970: nMicros / 1_000_000.0) // Convert microseconds to seconds
                print("💬 Chat \(uid) using start time 'n': \(nMicros) micros -> \(timestamp)")
            } else {
                timestamp = Date(timeIntervalSince1970: 0)
            }
        } else if let bValue = document.value["b"] {
            // 'b' field could be Int64, Int, or Double - this is in milliseconds
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
                timestamp = Date(timeIntervalSince1970: bMillis / 1000.0) // Convert milliseconds to seconds
                print("💬 Chat \(uid) using 'b' field: \(bMillis) millis -> \(timestamp)")
            } else {
                timestamp = Date(timeIntervalSince1970: 0)
            }
        } else {
            print("❌ Chat \(uid) has NO timestamp fields!")
            print("   Available fields: \(Array(document.value.keys).sorted())")
            print("   'b' field: \(String(describing: document.value["b"])) (type: \(Swift.type(of: document.value["b"])))")
            print("   'n' field: \(String(describing: document.value["n"])) (type: \(Swift.type(of: document.value["n"])))")
            timestamp = Date(timeIntervalSince1970: 0) // Use epoch instead of current time
        }
        
        do {
            let builder = CoTEventBuilder()
                .uid(uid)
                .type("b-t-f") // Chat message type
                .time(timestamp)
                .how("h-e") // Human entry
                .point(CoTPoint(lat: 0, lon: 0)) // Chat messages have no location
                .detail(CoTDetail([
                    "chat": [
                        "from": callsign,
                        "room": room,
                        "msg": message
                    ],
                    "contact": ["callsign": callsign]
                ]))
            
            return try builder
                .validity(start: timestamp, stale: timestamp.addingTimeInterval(300)) // 5 min validity
                .build()
        } catch {
            print("❌ Failed to build chat CoT event: \(error)")
            return nil
        }
    }
}