use crate::model::FlatCotEvent;
use crate::error::CotError;
use dittolive_ditto::prelude::*;

pub async fn insert_flat_cot_event(ditto: &Ditto, event: &FlatCotEvent) -> Result<(), CotError> {
    let store = ditto.store();
    let event_json = serde_json::to_value(event).map_err(|e| CotError::Format(e.to_string()))?;

    // For execute_v2, we need to include the parameters in the query string
    let query = format!(
        "INSERT INTO cot_events DOCUMENTS {}",
        event_json
    );
    store.execute_v2(&query).await.map_err(|e| CotError::Format(e.to_string()))?;

    Ok(())
}

pub async fn get_flat_cot_events(ditto: &Ditto) -> Result<Vec<FlatCotEvent>, CotError> {
    let store = ditto.store();
    let result = store.execute_v2("SELECT * FROM cot_events").await
        .map_err(|e| CotError::Format(e.to_string()))?;

    let events = result.iter()
        .map(|item| item.deserialize_value::<FlatCotEvent>()
             .map_err(|e| CotError::Format(e.to_string())))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(events)
}