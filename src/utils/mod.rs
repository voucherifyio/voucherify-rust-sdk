use std::collections::BTreeMap;
use serde_json::{Value, Number};

pub struct Metadata {
    contents: BTreeMap<String, Value>,
}

impl Metadata {
    pub fn new() -> Metadata {
        Metadata {
            contents: BTreeMap::new(),
        }
    }

    pub fn string(mut self, key: &str, value: &str) -> Metadata {
        self.contents.insert(key.to_string(), Value::String(value.to_string()));
        self
    }

    pub fn number(mut self, key: &str, value: usize) -> Metadata {
        self.contents.insert(key.to_string(), Value::Number(Number::from_f64(value as f64).unwrap()));
        self
    }

    pub fn boolean(mut self, key: &str, value: bool) -> Metadata {
        self.contents.insert(key.to_string(), Value::Bool(value));
        self
    }

    pub fn build(self) -> BTreeMap<String, Value> {
        self.contents
    }
}
