use crate::cot_events::CotEvent;
use crate::ditto::{self, DittoDocument};
use crate::error::CotError;
use dittolive_ditto::prelude::*;
use serde::de::DeserializeOwned;
use serde::Serialize;

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
        DittoDocument::MapItem(loc) => ("cot_location".to_string(), serde_json::to_value(loc)?),
        DittoDocument::Api(emergency) => (
            "cot_emergency".to_string(),
            serde_json::to_value(emergency)?,
        ),
        DittoDocument::File(gen) => ("cot_generic".to_string(), serde_json::to_value(gen)?),
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
    let _point = event.point();
    // Use the codegen transform_location_event to produce a MapItem
    let location = ditto::transform_location_event(event, peer_key);
    insert_document(ditto, "cot_location", &location).await
}

/// Helper function to create a new chat document
pub async fn create_chat(ditto: &Ditto, event: &CotEvent, peer_key: &str) -> Result<(), CotError> {
    if let Some(chat_doc) = ditto::transform_chat_event(event, peer_key) {
        insert_document(ditto, "cot_chat", &chat_doc).await
    } else {
        // If chat transformation fails, do not insert a Chat document
        // Optionally, insert a generic File document instead, or handle error as needed
        Ok(())
    }
}

/// Helper function to create a new emergency document
pub async fn create_emergency(
    ditto: &Ditto,
    event: &CotEvent,
    peer_key: &str,
) -> Result<(), CotError> {
    let _point = event.point();
    // Use the codegen transform_emergency_event to produce an Api
    let emergency = ditto::transform_emergency_event(event, peer_key);
    insert_document(ditto, "cot_emergency", &emergency).await
}
