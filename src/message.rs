use zmq;
use serde_json;
use serde_json::builder::ObjectBuilder;
use std::default::Default;

use msg_type::MsgType;
use errors::*;

#[derive(Debug, Serialize, PartialEq)]
pub enum Content {
    KernelInfoRequest,
    KernelInfoReply(KernelInfoReply),
    CommOpenRequest,
    CommOpenReply,
}

impl Content {
    pub fn reply(&self) -> Result<Content> {
        Ok(Content::KernelInfoReply(Default::default()))
    }
}

pub struct Metadata;

#[derive(Serialize, Deserialize)]
pub struct Header {
    msg_id: String,
    username: String,
    session: String,
    pub msg_type: MsgType,
    version: String,
}

macro_rules! opt_res {
    ($e:expr) => {{
        match $e {
            Some(val) => {
                val
            },
            None => {
                return Err(ErrorKind::EmptyMsgError.into())
            }
        }
    }}
}

fn msg(router: &mut zmq::Socket) -> Result<zmq::Message> {
    let mut msg = zmq::Message::new()?;
    let _ = router.recv(&mut msg, 0)?;
    Ok(msg)
}

pub struct Message {
    identity: String,
    hmac: String,
    header: Option<Header>,
    parent_header: Option<Header>,
    metadata: Option<Metadata>,
    content: Option<Content>,
}

impl Message {
    pub fn from_socket(router: &mut zmq::Socket) -> Result<Message> {
        debug!("waiting on message");

        let identity = msg(router)?;
        let identity = identity.as_str().unwrap_or("");

        let delim = msg(router)?;
        let delim = delim.as_str();
        let delim = opt_res!(delim);
        if delim != "<IDS|MSG>" {
            return Err(ErrorKind::ParseMsgError(
                        format!("Expected delimiter <IDS|MSG>, got {}", delim)).into());
        }

        let hmac = msg(router)?;
        let hmac = opt_res!(hmac.as_str());

        let header = msg(router)?;
        let header = opt_res!(header.as_str());
        let header: Option<Header> = serde_json::from_str(header).ok();

        let parent_header = msg(router)?;
        let parent_header = opt_res!(parent_header.as_str());
        let parent_header: Option<Header> = serde_json::from_str(parent_header).ok();

        let metadata = msg(router)?;
        let metadata = opt_res!(metadata.as_str());

        let content = msg(router)?;
        let content = Message::parse_content(&header, content.as_str())?;

        debug!("msg_type: {:?}", header.as_ref().map(|h| h.msg_type));
        debug!("content: {:?}", content.as_ref());
        debug!("hmac: {:?}", &hmac);
        Ok(Message {
            identity: identity.into(),
            hmac: hmac.into(),
            header: header,
            parent_header: parent_header,
            metadata: Some(Metadata),
            content: content,
        })
    }

    fn parse_content(header: &Option<Header>, content: Option<&str>) -> Result<Option<Content>> {
        if let &Some(ref header) = header {
            let msg_type = header.msg_type;
            msg_type.parse(content)
        } else{
            Ok(None)
        }
    }

    pub fn reply(&self) -> Result<Reply> {
        Ok(Reply::KernelInfoReply(Default::default()))
    }
}

#[derive(Debug, PartialEq)]
pub enum Reply {
    KernelInfoReply(KernelInfoReply),
}

impl Reply {
    pub fn to_json(self) -> serde_json::Value {
        match self {
            Reply::KernelInfoReply(k) => {
                let objs: Vec<serde_json::Value> = k.help_links.iter().map(|h| {
                    ObjectBuilder::new().insert("text", h.text.clone())
                                        .insert("url", h.url.clone())
                                        .build()
                }).collect();

                ObjectBuilder::new()
                                .insert("status", "ok")
                                .insert("protocol_version", k.protocol_version.clone())
                                .insert("implementation", k.implementation.clone())
                                .insert("implementation_version", k.implementation_version.clone())
                                .insert_object("language_info", |o| {
                                    o.insert("name", k.language_info.name.clone())
                                     .insert("version", k.language_info.version.clone())
                                     .insert("mimetype", k.language_info.mimetype.clone())
                                     .insert("file_extension", k.language_info.file_extension.clone())
                                     .insert("pygments_lexer", k.language_info.pygments_lexer.clone())
                                     .insert("codemirror_mode", k.language_info.codemirror_mode.clone())
                                     .insert("nbconvert_exporter", k.language_info.nbconvert_exporter.clone())
                                })
                                .insert("banner", k.banner.clone())
                                .insert("help_links", objs)
                                .build()
            },
        }
    }
}

#[derive(Serialize, Debug, PartialEq)]
pub struct KernelInfoReply {
    pub protocol_version: String,
    pub implementation: String,
    pub implementation_version: String,
    pub language_info: LanguageInfo,
    pub banner: String,
    pub help_links: Vec<HelpLinks>,
}

impl Default for KernelInfoReply {
    fn default() -> KernelInfoReply {
        KernelInfoReply {
            protocol_version: "5.1".into(),
            implementation: "rust".into(),
            implementation_version: "0.1.0".into(),
            language_info: Default::default(),
            banner: "Welcome to rust!".into(),
            help_links: vec!["https://doc.rust-lang.org".into()],
        }
    }
}

// Helper structs

#[derive(Serialize, Debug, PartialEq)]
pub struct LanguageInfo {
    pub name: String,
    pub version: String,
    pub mimetype: String,
    pub file_extension: String,
    pub pygments_lexer: String,
    pub codemirror_mode: String,
    pub nbconvert_exporter: String,
}

impl Default for LanguageInfo {
    fn default() -> LanguageInfo {
        LanguageInfo {
            name: "rust".into(),
            version: "1.14.0-nightly".into(),
            mimetype: "application/rust".into(),
            file_extension: "rs".into(),
            pygments_lexer: "rust".into(),
            codemirror_mode: "rust".into(),
            nbconvert_exporter: "".into(),
        }
    }
}

#[derive(Serialize, Debug, PartialEq)]
pub struct HelpLinks {
    pub text: String,
    pub url: String,
}

impl<'a> From<&'a str> for HelpLinks {
    fn from(s: &'a str) -> HelpLinks {
        HelpLinks {
            text: s.into(),
            url: s.into(),
        }
    }
}
