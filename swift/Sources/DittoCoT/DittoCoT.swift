import Foundation
import DittoSwift
import DittoCoTCore

/// Main DittoCoT integration module
public class DittoCoT {
    
    // MARK: - Properties
    
    public let ditto: Ditto
    private let converter: CoTDocumentConverter
    public let collectionName: String
    
    // MARK: - Initialization
    
    public init(
        ditto: Ditto,
        peerKey: String? = nil,
        collectionName: String = "cot_events"
    ) {
        self.ditto = ditto
        self.collectionName = collectionName
        
        // Use provided peer key or generate from Ditto
        let key = peerKey ?? ditto.siteID.description
        self.converter = CoTDocumentConverter(peerKey: key)
    }
    
    // MARK: - Document Operations
    
    /// Insert a CoT event into Ditto
    public func insert(_ event: CoTEvent) async throws -> String {
        let conversionResult = converter.convert(event)
        
        switch conversionResult {
        case .success(let document):
            let collection = ditto.store.collection(collectionName)
            let dittoDoc = document.toDittoDocument()
            
            return try await withCheckedThrowingContinuation { continuation in
                do {
                    let docID = try collection.upsert(dittoDoc)
                    continuation.resume(returning: docID.description)
                } catch {
                    continuation.resume(throwing: error)
                }
            }
            
        case .failure(let error):
            throw error
        }
    }
    
    /// Insert multiple CoT events into Ditto
    public func insertBatch(_ events: [CoTEvent]) async throws -> [String] {
        var documentIDs: [String] = []
        
        for event in events {
            let docID = try await insert(event)
            documentIDs.append(docID)
        }
        
        return documentIDs
    }
    
    /// Update an existing CoT event in Ditto
    public func update(_ event: CoTEvent) async throws -> String {
        let uid = event.uid
        
        let conversionResult = converter.convert(event)
        
        switch conversionResult {
        case .success(let document):
            let collection = ditto.store.collection(collectionName)
            let dittoDoc = document.toDittoDocument()
            
            return try await withCheckedThrowingContinuation { continuation in
                do {
                    // Update document counter for existing documents
                    let query = collection.find("_id == $0", args: ["_id": uid])
                    let existingDocs = query.exec()
                    
                    var updatedDoc = dittoDoc
                    if let existing = existingDocs.first,
                       let counter = existing.value["_c"] as? Int64 {
                        updatedDoc["_c"] = counter + 1
                    }
                    
                    let docID = try collection.upsert(updatedDoc)
                    continuation.resume(returning: docID.description)
                } catch {
                    continuation.resume(throwing: error)
                }
            }
            
        case .failure(let error):
            throw error
        }
    }
    
    /// Remove a CoT event from Ditto (soft delete)
    public func remove(uid: String) async throws {
        let collection = ditto.store.collection(collectionName)
        
        // For now, we'll use a simpler approach - just upsert with _r = true
        let deleteDoc: [String: Any?] = [
            "_id": uid,
            "_r": true
        ]
        
        try await withCheckedThrowingContinuation { (continuation: CheckedContinuation<Void, Error>) in
            do {
                let _ = try collection.upsert(deleteDoc)
                continuation.resume(returning: ())
            } catch {
                continuation.resume(throwing: error)
            }
        }
    }
    
    // MARK: - Query Operations
    
    /// Find CoT events by type
    public func findByType(_ type: String) -> [DittoSwift.DittoDocument] {
        let collection = ditto.store.collection(collectionName)
        return collection.find("w == $0 && _r == false", args: ["w": type]).exec()
    }
    
    /// Find CoT events by callsign
    public func findByCallsign(_ callsign: String) -> [DittoSwift.DittoDocument] {
        let collection = ditto.store.collection(collectionName)
        return collection.find("e == $0 && _r == false", args: ["e": callsign]).exec()
    }
    
    /// Find all active CoT events
    public func findAll() -> [DittoSwift.DittoDocument] {
        let collection = ditto.store.collection(collectionName)
        return collection.find("_r == false", args: [:]).exec()
    }
    
    /// Find CoT events within a time range
    public func findByTimeRange(from: Date, to: Date) -> [DittoSwift.DittoDocument] {
        let fromMillis = from.timeIntervalSince1970 * 1000
        let toMillis = to.timeIntervalSince1970 * 1000
        
        let collection = ditto.store.collection(collectionName)
        return collection.find("b >= $0 && b <= $1 && _r == false", 
                                args: ["fromMillis": fromMillis, "toMillis": toMillis]).exec()
    }
}