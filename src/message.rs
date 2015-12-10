use nom::*;
use std::default::Default;

use super::{Result, Error};
use raw_message::RawMessage;

pub enum Message {
    KernelInfoRequest,
    KernelInfoReply(KernelInfoReply),
}

impl Message {
    pub fn from_raw(msg: RawMessage) -> Result<Message> {
        let msg_type = msg.msg_type();

        match msg_type.as_ref().map(|s| &s[..]) {
            Some("kernel_info_request") => Ok(Message::KernelInfoRequest),
            Some("kernel_info_reply") => Ok(Message::KernelInfoReply(Default::default())),
            Some(m) => Err(Error::MessageDecodeError(format!("Unknown message type {}", m))),
            None => Err(Error::MessageDecodeError(format!("Malformed message; Messages are expected to have a msg_type in their \
                            header"))),
        }
    }

    pub fn to_raw(self) -> RawMessage {
        Default::default()
    }
}

pub struct KernelInfoReply {
    protocol_version: String,
    implementation: String,
    implementation_version: String,
    language_info: LanguageInfo,
    banner: String,
    help_links: Vec<HelpLinks>,
}

impl Default for KernelInfoReply {
    fn default() -> KernelInfoReply {
        KernelInfoReply {
            protocol_version: "".into(),
            implementation: "".into(),
            implementation_version: "".into(),
            language_info: Default::default(),
            banner: "".into(),
            help_links: vec![],
        }
    }
}

// Helper structs

pub struct LanguageInfo {
    name: String,
    version: String,
    mimetype: String,
    file_extension: String,
    pygments_lexer: String,
    codemirror_mode: String,
    nbconvert_exporter: String,
}

impl Default for LanguageInfo {
    fn default() -> LanguageInfo {
        LanguageInfo {
            name: "".into(),
            version: "".into(),
            mimetype: "".into(),
            file_extension: "".into(),
            pygments_lexer: "".into(),
            codemirror_mode: "".into(),
            nbconvert_exporter: "".into(),
        }
    }
}

pub struct HelpLinks {
    text: String,
    url: String,
}
