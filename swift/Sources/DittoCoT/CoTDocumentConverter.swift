import Foundation
import DittoSwift
import DittoCoTCore

/// Converts CoT events to Ditto documents
public class CoTDocumentConverter {
    
    // MARK: - Properties
    
    private let peerKey: String
    private let defaultQos: String
    private let defaultOpex: String
    private let defaultAccess: String
    private let defaultCaveat: String
    private let defaultReleasableTo: String
    
    // MARK: - Initialization
    
    public init(
        peerKey: String,
        defaultQos: String = "i-i-i",
        defaultOpex: String = "s-s-s",
        defaultAccess: String = "Unspecified",
        defaultCaveat: String = "None",
        defaultReleasableTo: String = "USA"
    ) {
        self.peerKey = peerKey
        self.defaultQos = defaultQos
        self.defaultOpex = defaultOpex
        self.defaultAccess = defaultAccess
        self.defaultCaveat = defaultCaveat
        self.defaultReleasableTo = defaultReleasableTo
    }
    
    // MARK: - Conversion Methods
    
    /// Converts a CoT event to an appropriate Ditto document
    public func convert(_ event: CoTEvent) -> Result<DittoDocumentProtocol, ConversionError> {
        // Extract common fields
        let uid = event.uid
        
        let timeMillis = event.time.timeIntervalSince1970 * 1000
        let callsign = extractCallsign(from: event) ?? uid
        
        // Determine document type and convert
        if event.type.hasPrefix("b-t-f") {
            return convertToChatDocument(event, uid: uid, timeMillis: timeMillis, callsign: callsign)
        } else if isMapItemEvent(event) {
            return convertToMapItemDocument(event, uid: uid, timeMillis: timeMillis, callsign: callsign)
        } else {
            return convertToGenericDocument(event, uid: uid, timeMillis: timeMillis, callsign: callsign)
        }
    }
    
    // MARK: - Private Conversion Methods
    
    private func convertToChatDocument(
        _ event: CoTEvent,
        uid: String,
        timeMillis: Double,
        callsign: String
    ) -> Result<DittoDocumentProtocol, ConversionError> {
        // Extract chat-specific fields from detail
        var chatFrom: String?
        var room: String?
        var roomId: String?
        var message: String?
        
        if let detail = event.detail {
            // Look for chat element in detail
            if let chatValue = detail.getValue(at: "chat") {
                if case .object(let chatDict) = chatValue {
                    chatFrom = chatDict["from"]?.stringValue
                    room = chatDict["room"]?.stringValue
                    roomId = chatDict["roomId"]?.stringValue
                    message = chatDict["msg"]?.stringValue
                }
            }
        }
        
        let chatDoc = ChatDocument(
            _id: uid,
            a: peerKey,
            b: timeMillis,
            d: uid,
            _c: 0,
            _r: false,
            _v: 2,
            e: callsign,
            authorCallsign: chatFrom,
            authorType: event.type,
            authorUid: uid,
            g: event.version,
            h: event.point.ce,
            i: event.point.hae,
            j: event.point.lat,
            k: event.point.le,
            l: event.point.lon,
            location: formatLocation(event.point),
            message: message,
            n: event.start.timeIntervalSince1970.milliseconds,
            o: event.stale.timeIntervalSince1970.milliseconds,
            p: event.how,
            parent: roomId,
            q: event.access ?? defaultAccess,
            r: convertDetailToRField(event.detail),
            room: room,
            roomId: roomId,
            s: event.opex ?? defaultOpex,
            source: nil, // No source field in CoTEvent
            t: event.qos ?? defaultQos,
            time: event.time.ISO8601Format(),
            u: defaultCaveat,
            v: defaultReleasableTo,
            w: event.type
        )
        
        return .success(chatDoc)
    }
    
    private func convertToMapItemDocument(
        _ event: CoTEvent,
        uid: String,
        timeMillis: Double,
        callsign: String
    ) -> Result<DittoDocumentProtocol, ConversionError> {
        let mapDoc = MapItemDocument(
            _id: uid,
            a: peerKey,
            b: timeMillis,
            d: uid,
            _c: 0,
            _r: false,
            _v: 2,
            e: callsign,
            c: extractName(from: event),
            f: true, // Default visibility
            g: event.version,
            h: event.point.ce,
            i: event.point.hae,
            j: event.point.lat,
            k: event.point.le,
            l: event.point.lon,
            n: event.start.timeIntervalSince1970.milliseconds,
            o: event.stale.timeIntervalSince1970.milliseconds,
            p: event.how,
            q: event.access ?? defaultAccess,
            r: convertDetailToRField(event.detail),
            s: event.opex ?? defaultOpex,
            source: nil, // No source field in CoTEvent
            t: event.qos ?? defaultQos,
            u: defaultCaveat,
            v: defaultReleasableTo,
            w: event.type
        )
        
        return .success(mapDoc)
    }
    
    private func convertToGenericDocument(
        _ event: CoTEvent,
        uid: String,
        timeMillis: Double,
        callsign: String
    ) -> Result<DittoDocumentProtocol, ConversionError> {
        let genericDoc = GenericDocument(
            _id: uid,
            a: peerKey,
            b: timeMillis,
            d: uid,
            _c: 0,
            _r: false,
            _v: 2,
            e: callsign,
            g: event.version,
            h: event.point.ce,
            i: event.point.hae,
            j: event.point.lat,
            k: event.point.le,
            l: event.point.lon,
            n: event.start.timeIntervalSince1970.milliseconds,
            o: event.stale.timeIntervalSince1970.milliseconds,
            p: event.how,
            q: event.access ?? defaultAccess,
            r: convertDetailToRField(event.detail),
            s: event.opex ?? defaultOpex,
            source: nil, // No source field in CoTEvent
            t: event.qos ?? defaultQos,
            u: defaultCaveat,
            v: defaultReleasableTo,
            w: event.type
        )
        
        return .success(genericDoc)
    }
    
    // MARK: - Helper Methods
    
    private func extractCallsign(from event: CoTEvent) -> String? {
        // Try to extract from contact element in detail
        if let detail = event.detail {
            // Check contact callsign
            if let callsign = detail.callsign {
                return callsign
            }
            
            // For chat messages, check the chat element
            if let chatValue = detail.getValue(at: "chat"),
               case .object(let chatDict) = chatValue,
               let from = chatDict["from"]?.stringValue {
                return from
            }
        }
        return nil
    }
    
    private func extractName(from event: CoTEvent) -> String? {
        // Extract name from contact or other detail elements
        return event.detail?.callsign
    }
    
    private func isMapItemEvent(_ event: CoTEvent) -> Bool {
        // Check if this is a map item event based on type
        let type = event.type
        
        // Common map item prefixes
        let mapItemPrefixes = ["a-", "a-f", "a-h", "a-n", "a-u"]
        return mapItemPrefixes.contains { type.hasPrefix($0) }
    }
    
    private func formatLocation(_ point: CoTPoint) -> String {
        return "\(point.lat),\(point.lon)"
    }
    
    private func convertDetailToRField(_ detail: CoTDetail?) -> [String: RValue] {
        guard let detail = detail else { return [:] }
        
        // Convert the detail JSONValue to RValue format
        return convertJSONValueToRField(detail.value)
    }
    
    private func convertJSONValueToRField(_ jsonValue: JSONValue) -> [String: RValue] {
        guard case .object(let dict) = jsonValue else { return [:] }
        
        return dict.compactMapValues { value in
            convertJSONValueToRValue(value)
        }
    }
    
    private func convertJSONValueToRValue(_ jsonValue: JSONValue) -> RValue {
        switch jsonValue {
        case .null:
            return .null
        case .bool(let value):
            return .boolean(value)
        case .number(let value):
            return .number(value)
        case .string(let value):
            return .string(value)
        case .array(let values):
            let converted = values.compactMap { convertJSONValueToRValue($0).toAny() }
            return .array(converted)
        case .object(let dict):
            let converted = dict.compactMapValues { convertJSONValueToRValue($0).toAny() }
            return .object(converted)
        }
    }
}

// MARK: - Supporting Types

public enum ConversionError: Error, LocalizedError {
    case missingRequiredField(String)
    case invalidEventType
    case conversionFailed(String)
    
    public var errorDescription: String? {
        switch self {
        case .missingRequiredField(let field):
            return "Missing required field: \(field)"
        case .invalidEventType:
            return "Invalid or unsupported event type"
        case .conversionFailed(let reason):
            return "Conversion failed: \(reason)"
        }
    }
}

// MARK: - Extensions

private extension TimeInterval {
    var milliseconds: Double {
        return self * 1000
    }
}

private extension JSONValue {
    var stringValue: String? {
        if case .string(let value) = self {
            return value
        }
        return nil
    }
}