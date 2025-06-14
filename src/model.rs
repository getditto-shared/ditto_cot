use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlatCotEvent {
    pub uid: String,
    pub type_: String,
    pub time: String,
    pub start: String,
    pub stale: String,
    pub how: String,
    pub lat: f64,
    pub lon: f64,
    pub hae: f64,
    pub ce: f64,
    pub le: f64,

    pub callsign: Option<String>,
    pub group_name: Option<String>,

    pub detail_extra: HashMap<String, Value>,
}