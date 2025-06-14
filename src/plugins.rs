use crate::plugin::{DetailPlugin};
use crate::model::FlatCotEvent;
use std::collections::HashMap;
use serde_json::json;
use serde_json::Value;

pub struct GroupPlugin;

impl DetailPlugin for GroupPlugin {
    fn matches(&self, tag: &str) -> bool {
        tag == "__group"
    }

    fn parse(&self, attributes: &HashMap<String, String>) -> Option<(String, Value)> {
        attributes.get("name").map(|name| ("group_name".to_string(), json!(name)))
    }

    fn enrich_flat(&self, flat: &mut FlatCotEvent, key: &str, val: &Value) {
        if key == "group_name" {
            flat.group_name = val.as_str().map(|s| s.to_string());
        }
    }
}