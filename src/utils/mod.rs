pub mod error;

use std::collections::BTreeMap;
use serde_json::Value;

pub struct Metadata {
    contents: BTreeMap<String, Value>,
}

impl Metadata {
    pub fn new() -> Metadata {
        Metadata { contents: BTreeMap::new() }
    }

    pub fn string(mut self, key: &str, value: &str) -> Metadata {
        self.contents.insert(key.to_string(), json!(value.to_string()));
        self
    }

    pub fn number(mut self, key: &str, value: isize) -> Metadata {
        self.contents.insert(key.to_string(), json!(value));
        self
    }

    pub fn boolean(mut self, key: &str, value: bool) -> Metadata {
        self.contents.insert(key.to_string(), json!(value));
        self
    }

    pub fn build(self) -> BTreeMap<String, Value> {
        self.contents
    }
}
