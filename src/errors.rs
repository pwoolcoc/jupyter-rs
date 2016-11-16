use std::io;
use serde_json;

error_chain!{
    foreign_links {
        serde_json::Error, SerdeError;
        io::Error, IoError;
    }

    errors {
        KernelError(t: String)
        MessageDecodeError(t: String)
    }
}

