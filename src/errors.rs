use std::io;
use serde_json;
use zmq;

error_chain!{
    foreign_links {
        serde_json::Error, SerdeError;
        io::Error, IoError;
        zmq::Error, ZmqError;
    }

    errors {
        UnknownMessageError
        EmptyMsgError
        ParseMsgError(t: String)
        KernelError(t: String)
        MessageDecodeError(t: String)
    }
}

