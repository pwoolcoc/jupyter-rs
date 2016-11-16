use zmq;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use std::collections::HashMap;
use serde_json;

use errors::{Result, ErrorKind};
use raw_message::{RawMessage, Header};

pub struct Shell {
    transport: String,
    addr: String,
    port: u32,
}

#[derive(Debug, Clone, PartialEq)]
enum RecvState {
    Start,
    GetDelim,
    GetHMAC,
    GetHeader,
    GetParentHeader,
    GetMetadata,
    GetContent,
    Finish,
}

impl RecvState {
    fn get_raw_message(&mut self, router: &mut zmq::Socket) -> Result<RawMessage> {
        let mut map = HashMap::new();
        let mut header: Option<Header>;
        'getmessage: loop {
            let mut msg = zmq::Message::new().unwrap();
            match router.recv(&mut msg, 0) {
                Ok(()) => {
                    match msg.as_str() {
                        Some(m) => {
                            if *self == RecvState::Start && m == "<IDS|MSG>" {
                                // Skip the identity
                                *self = RecvState::GetDelim;
                            }

                            match *self {
                                RecvState::Start => {
                                    debug!("got identity:");
                                    map.insert("identity".to_owned(), m.to_owned());
                                    *self = RecvState::GetDelim;
                                },
                                RecvState::GetDelim => {
                                    debug!("got delimiter:");
                                    map.insert("delimiter".to_owned(), m.to_owned());
                                    *self = RecvState::GetHMAC;
                                },
                                RecvState::GetHMAC => {
                                    debug!("got hmac:");
                                    map.insert("hmac".to_owned(), m.to_owned());
                                    *self = RecvState::GetHeader;
                                },
                                RecvState::GetHeader => {
                                    debug!("got header:");
                                    header = serde_json::from_str(m).ok();
                                    debug!("msg_type: {:?}", header.as_ref().map(|h| h.msg_type));
                                    map.insert("header".to_owned(), m.to_owned());
                                    *self = RecvState::GetParentHeader;
                                },
                                RecvState::GetParentHeader => {
                                    debug!("got parent header:");
                                    map.insert("parent_header".to_owned(), m.to_owned());
                                    *self = RecvState::GetMetadata;
                                },
                                RecvState::GetMetadata => {
                                    debug!("got metadata:");
                                    map.insert("metadata".to_owned(), m.to_owned());
                                    *self = RecvState::GetContent;
                                },
                                RecvState::GetContent => {
                                    debug!("got content:");
                                    map.insert("content".to_owned(), m.to_owned());
                                    *self = RecvState::Finish;
                                },
                                RecvState::Finish => {
                                    debug!("DONE");
                                    break 'getmessage;
                                },
                            }
                            debug!("{:?}", m);
                        },
                        None => {
                            debug!("msg.as_str() was None.");
                        }
                    }
                },
                Err(e) => {
                    error!("Err(e) was {:?}", e);
                }
            }
        }
        if *self == RecvState::Finish {
            *self = RecvState::Start;
            debug!("map is {:?}", &map);
            Ok(RawMessage::from_map(map.clone()))
        } else {
            Err(ErrorKind::MessageDecodeError("get_raw_message failed".into()).into())
        }
    }
}

impl Shell {
    pub fn new(tns: &str, addr: &str, port: u32) -> Shell {
        Shell {
            transport: tns.into(),
            addr: addr.into(),
            port: port,
        }
    }

    pub fn listen(&self, ctx: Arc<Mutex<RefCell<zmq::Context>>>) {
        let mut router = {
            let ctx = ctx.lock().unwrap();
            let mut ctx = ctx.borrow_mut();
            ctx.socket(zmq::ROUTER).unwrap()
        };
        let address = format!("{}://{}:{}", &self.transport, &self.addr, self.port);

        debug!("shell address is {}", &address);
        assert!(router.bind(&address).is_ok());
        loop {
            let mut state = RecvState::Start;
            let raw_message = state.get_raw_message(&mut router).unwrap();
            /*
            let message = match Message::from_raw(raw_message) {
                Ok(m) => m,
                Err(e) => continue,
            };
            message.reply(&mut router);
            */
        }
    }
}
