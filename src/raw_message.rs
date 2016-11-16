use std::collections::HashMap;
use serde_json;
use serde_json::Value;

use msg_type::MsgType;

#[derive(Serialize, Deserialize)]
pub struct RawMessage {
    header: Option<Header>,
    parent_header: Option<Header>,
    metadata: Value,
    content: Value,
}

#[derive(Serialize, Deserialize)]
pub struct Header {
    msg_id: String,
    username: String,
    session: String,
    pub msg_type: MsgType,
    version: String,
}

impl RawMessage {
    pub fn from_map(map: HashMap<String, String>) -> RawMessage {
        let header: Option<Header> = match map.get("header") {
            Some(h) => serde_json::from_str(h).ok(),
            None => None,
        };
        let parent_header: Option<Header> = match map.get("parent_header") {
            Some(h) => serde_json::from_str(h).ok(),
            None => None,
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
}
