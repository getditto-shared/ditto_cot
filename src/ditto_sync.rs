use crate::cot_events::CotEvent;
use crate::ditto::{
    self, ChatDocument, CommonFields, DittoDocument, EmergencyDocument, LocationDocument,
};
use crate::error::CotError;
use chrono::Utc;
use dittolive_ditto::prelude::*;
use serde::de::DeserializeOwned;
use serde::Serialize;
use uuid::Uuid;

/// Inserts a CoT event into Ditto after transforming it to Ditto's format
///
/// This function transforms the CoT event into the appropriate Ditto document type
/// and inserts it into the corresponding collection.
pub async fn insert_cot_event(
    ditto: &Ditto,
    event: &CotEvent,
    peer_key: &str,
) -> Result<(), CotError> {
    // Transform the CoT event to a Ditto document
    let ditto_doc = ditto::cot_to_document(event, peer_key);

    // Determine the collection name based on the document type
    let (collection_name, doc_value) = match ditto_doc {
        DittoDocument::Chat(chat) => ("cot_chat".to_string(), serde_json::to_value(chat)?),
        DittoDocument::Location(loc) => ("cot_location".to_string(), serde_json::to_value(loc)?),
        DittoDocument::Emergency(emergency) => (
            "cot_emergency".to_string(),
            serde_json::to_value(emergency)?,
        ),
        DittoDocument::Generic(gen) => ("cot_generic".to_string(), serde_json::to_value(gen)?),
    };

    // Insert the document
    insert_document(ditto, &collection_name, &doc_value).await
}

/// Inserts a document into the specified collection
async fn insert_document(
    ditto: &Ditto,
    collection: &str,
    document: &impl Serialize,
) -> Result<(), CotError> {
    let store = ditto.store();
    let query = format!(
        "INSERT INTO {} DOCUMENTS {}",
        collection,
        serde_json::to_string(document).map_err(|e| CotError::Format(e.to_string()))?
    );

    store
        .execute_v2(&query)
        .await
        .map_err(|e| CotError::Format(e.to_string()))?;

    Ok(())
}

/// Retrieves documents from a specific Ditto collection
///
/// # Type Parameters
/// - `T`: The type to deserialize the documents into
///
/// # Parameters
/// - `ditto`: The Ditto instance to use
/// - `collection`: The name of the collection to query
/// - `query`: Optional WHERE clause (without the WHERE keyword)
///
/// # Returns
/// A vector of deserialized documents, or an error if the operation fails
pub async fn get_documents<T: DeserializeOwned>(
    ditto: &Ditto,
    collection: &str,
    query: Option<&str>,
) -> Result<Vec<T>, CotError> {
    let store = ditto.store();
    let query_str = match query {
        Some(q) => format!("SELECT * FROM {} WHERE {}", collection, q),
        None => format!("SELECT * FROM {}", collection),
    };

    let result = store
        .execute_v2(&query_str)
        .await
        .map_err(|e| CotError::Format(e.to_string()))?;

    let items = result
        .iter()
        .map(|item| {
            item.deserialize_value::<T>()
                .map_err(|e| CotError::Format(e.to_string()))
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(items)
}

/// Retrieves a single document by ID from a collection
///
/// # Type Parameters
/// - `T`: The type to deserialize the document into
///
/// # Parameters
/// - `ditto`: The Ditto instance to use
/// - `collection`: The name of the collection to query
/// - `id`: The ID of the document to retrieve
///
/// # Returns
/// The deserialized document, or None if not found
pub async fn get_document<T: DeserializeOwned>(
    ditto: &Ditto,
    collection: &str,
    id: &str,
) -> Result<Option<T>, CotError> {
    let query = format!("_id = '{}'", id);
    let mut results = get_documents::<T>(ditto, collection, Some(&query)).await?;
    Ok(results.pop())
}

/// Updates a document in a Ditto collection
///
/// # Parameters
/// - `ditto`: The Ditto instance to use
/// - `collection`: The name of the collection containing the document
/// - `doc_id`: The ID of the document to update
/// - `updates`: A value containing the fields to update
///
/// # Returns
/// `Ok(())` if the update was successful, or an error if it failed
pub async fn update_document<T: Serialize>(
    ditto: &Ditto,
    collection: &str,
    doc_id: &str,
    updates: &T,
) -> Result<(), CotError> {
    let store = ditto.store();
    let updates_json =
        serde_json::to_value(updates).map_err(|e| CotError::Format(e.to_string()))?;

    // Convert the updates to a JSON string, removing the outer braces
    let updates_str = updates_json.to_string();
    let updates_content = updates_str.trim_start_matches('{').trim_end_matches('}');

    let query = format!(
        "UPDATE {} SET {} WHERE _id = '{}'",
        collection, updates_content, doc_id
    );

    store
        .execute_v2(&query)
        .await
        .map_err(|e| CotError::Format(e.to_string()))?;

    Ok(())
}

/// Deletes a document from a Ditto collection
///
/// # Parameters
/// - `ditto`: The Ditto instance to use
/// - `collection`: The name of the collection containing the document
/// - `doc_id`: The ID of the document to delete
///
/// # Returns
/// `Ok(())` if the deletion was successful, or an error if it failed
pub async fn delete_document(
    ditto: &Ditto,
    collection: &str,
    doc_id: &str,
) -> Result<(), CotError> {
    let store = ditto.store();
    let query = format!("DELETE FROM {} WHERE _id = '{}'", collection, doc_id);

    store
        .execute_v2(&query)
        .await
        .map_err(|e| CotError::Format(e.to_string()))?;

    Ok(())
}

/// Helper function to create a new location document
pub async fn create_location(
    ditto: &Ditto,
    event: &CotEvent,
    peer_key: &str,
) -> Result<(), CotError> {
    let point = event.point();
    let location = LocationDocument {
        common: CommonFields {
            id: event.uid().to_string(),
            counter: 0,
            version: 2,
            deleted: false,
            peer_key: peer_key.to_string(),
            timestamp: Utc::now().timestamp_millis(),
            author_uid: event.uid().to_string(),
            author_callsign: event.callsign().unwrap_or("unknown").to_string(),
            version_str: "".to_string(),
            ce: point.ce,
        },
        location: ditto::schema::Location {
            latitude: point.lat,
            longitude: point.lon,
            altitude: point.hae,
            circular_error: point.ce,
            speed: 0.0,
            course: 0.0,
        },
        location_type: event.event_type().to_string(),
        metadata: None,
    };

    insert_document(ditto, "cot_location", &location).await
}

/// Helper function to create a new chat document
pub async fn create_chat(ditto: &Ditto, event: &CotEvent, peer_key: &str) -> Result<(), CotError> {
    if let Some(chat_doc) = ditto::transform_chat_event(event, peer_key) {
        insert_document(ditto, "cot_chat", &chat_doc).await
    } else {
        // If chat transformation fails, create a generic chat document
        let chat = ChatDocument {
            common: CommonFields {
                id: Uuid::new_v4().to_string(),
                counter: 0,
                version: 2,
                deleted: false,
                peer_key: peer_key.to_string(),
                timestamp: Utc::now().timestamp_millis(),
                author_uid: event.uid().to_string(),
                author_callsign: event.callsign().unwrap_or("unknown").to_string(),
                version_str: "".to_string(),
                ce: event.point().ce,
            },
            message: "Chat message".to_string(),
            room: "default".to_string(),
            parent: None,
            room_id: "room_default".to_string(),
            author_callsign: event.callsign().unwrap_or("unknown").to_string(),
            author_uid: event.uid().to_string(),
            author_type: "user".to_string(),
            time: Utc::now().to_rfc3339(),
            location: Some(format!(
                "{},{},{}",
                event.point().lat,
                event.point().lon,
                event.point().hae
            )),
        };
        insert_document(ditto, "cot_chat", &chat).await
    }
}

/// Helper function to create a new emergency document
pub async fn create_emergency(
    ditto: &Ditto,
    event: &CotEvent,
    peer_key: &str,
) -> Result<(), CotError> {
    let point = event.point();
    let emergency = EmergencyDocument {
        common: CommonFields {
            id: event.uid().to_string(),
            counter: 0,
            version: 2,
            deleted: false,
            peer_key: peer_key.to_string(),
            timestamp: Utc::now().timestamp_millis(),
            author_uid: event.uid().to_string(),
            author_callsign: event.callsign().unwrap_or("unknown").to_string(),
            version_str: "".to_string(),
            ce: point.ce,
        },
        emergency_type: event.detail.get("type").cloned().unwrap_or_default(),
        status: event
            .detail
            .get("status")
            .cloned()
            .unwrap_or_else(|| "active".to_string()),
        location: ditto::schema::Location {
            latitude: point.lat,
            longitude: point.lon,
            altitude: point.hae,
            circular_error: point.ce,
            speed: 0.0,
            course: 0.0,
        },
        details: event
            .detail
            .get("message")
            .cloned()
            .map(|msg| serde_json::json!({ "message": msg })),
    };
    insert_document(ditto, "cot_emergency", &emergency).await
}
