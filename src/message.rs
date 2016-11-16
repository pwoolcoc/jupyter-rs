use zmq;
use serde_json;
use std::default::Default;

use msg_type::MsgType;
use errors::*;

pub enum Content {
    KernelInfoRequest,
    KernelInfoReply(KernelInfoReply),
    CommOpenRequest,
    CommOpenReply,
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
}

#[derive(Serialize)]
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

#[derive(Serialize)]
struct LanguageInfo {
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

#[derive(Serialize)]
struct HelpLinks {
    text: String,
    url: String,
}
