use std::default::Default;

use nom::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json;
use serde_json::Value;


pub struct RawMessage {
    header: Value,
    parent_header: Value,
    metadata: Value,
    content: Value,
}

impl RawMessage {
    pub fn new() -> RawMessage {
        Default::default()
    }

    pub fn from_map(map: HashMap<String, String>) -> RawMessage {
        let header: Value = match map.get("header") {
            Some(h) => serde_json::from_str(h).unwrap(),
            None => Value::Null,
        };
        let parent_header: Value = match map.get("parent_header") {
            Some(h) => serde_json::from_str(h).unwrap(),
            None => Value::Null,
        };
        let metadata: Value = match map.get("metadata") {
            Some(h) => serde_json::from_str(h).unwrap(),
            None => Value::Null,
        };
        let content: Value = match map.get("content") {
            Some(h) => serde_json::from_str(h).unwrap(),
            None => Value::Null,
        };
        RawMessage {
            header: header,
            parent_header: parent_header,
            metadata: metadata,
            content: content,
        }
    }

    pub fn msg_type(&self) -> Option<&str> {
        self.header.as_object()
                   .and_then(|obj| obj.get("msg_type"))
                   .and_then(|m| m.as_string())
                   .map(|s| &s[..])
    }
}

impl Default for RawMessage {
    fn default() -> RawMessage {
        RawMessage {
            header: Value::Null,
            parent_header: Value::Null,
            metadata: Value::Null,
            content: Value::Null,
        }
    }
}
