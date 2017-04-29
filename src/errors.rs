use std::io;
use serde_json;

error_chain!{
    foreign_links {
        serde_json::Error, SerdeError;
        io::Error, IoError;
    }

    errors {
        UnknownMessageError
        EmptyMsgError
        ParseMsgError(t: String)
        KernelError(t: String)
        MessageDecodeError(t: String)
    }
}

