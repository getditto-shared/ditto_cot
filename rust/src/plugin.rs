use crate::model::FlatCotEvent;
use std::collections::HashMap;
use serde_json::Value;

pub trait DetailPlugin {
    fn matches(&self, tag: &str) -> bool;
    fn parse(&self, attributes: &HashMap<String, String>) -> Option<(String, Value)>;
    fn enrich_flat(&self, flat: &mut FlatCotEvent, key: &str, val: &Value);
}

pub struct PluginRegistry {
    plugins: Vec<Box<dyn DetailPlugin + Send + Sync>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self { plugins: Vec::new() }
    }

    pub fn register(&mut self, plugin: Box<dyn DetailPlugin + Send + Sync>) {
        self.plugins.push(plugin);
    }

    pub fn handle(&self, tag: &str, attrs: &HashMap<String, String>, flat: &mut FlatCotEvent) {
        for plugin in &self.plugins {
            if plugin.matches(tag) {
                if let Some((key, value)) = plugin.parse(attrs) {
                    plugin.enrich_flat(flat, &key, &value);
                }
            }
        }
    }
}