use std::default::Default;

use nom::*;
// use serde::{Serialize, Deserialize};
// use serde_json;

pub struct Header {
    msg_type: Option<String>,
}

pub struct Metadata;
pub struct Content;

pub struct RawMessage {
    header: Header,
    parent_header: Header,
    metadata: Metadata,
    content: Content,
}

impl RawMessage {
    pub fn msg_type(&self) -> Option<&str> {
        self.header.msg_type.as_ref().map(|s| &s[..])
    }
}

impl Default for RawMessage {
    fn default() -> RawMessage {
        RawMessage {
            header: Header { msg_type: None },
            parent_header: Header { msg_type: None },
            content: Content,
            metadata: Metadata,
        }
    }
}
