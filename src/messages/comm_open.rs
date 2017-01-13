use message::Content;
use errors::*;

pub fn parse(content: Option<&str>) -> Result<Option<Content>> {
    Ok(Some(Content::CommOpenReply))
}
