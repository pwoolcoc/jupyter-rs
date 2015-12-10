use zmq;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use std::collections::HashMap;

use super::{Result, Error};
use raw_message::RawMessage;
// use message;

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

                            match self {
                                &mut RecvState::Start => {
                                    debug!("got identity:");
                                    map.insert("identity".to_owned(), m.to_owned());
                                    *self = RecvState::GetDelim;
                                },
                                &mut RecvState::GetDelim => {
                                    debug!("got delimiter:");
                                    map.insert("delimiter".to_owned(), m.to_owned());
                                    *self = RecvState::GetHMAC;
                                },
                                &mut RecvState::GetHMAC => {
                                    debug!("got hmac:");
                                    map.insert("hmac".to_owned(), m.to_owned());
                                    *self = RecvState::GetHeader;
                                },
                                &mut RecvState::GetHeader => {
                                    debug!("got header:");
                                    map.insert("header".to_owned(), m.to_owned());
                                    *self = RecvState::GetParentHeader;
                                },
                                &mut RecvState::GetParentHeader => {
                                    debug!("got parent header:");
                                    map.insert("parent_header".to_owned(), m.to_owned());
                                    *self = RecvState::GetMetadata;
                                },
                                &mut RecvState::GetMetadata => {
                                    debug!("got metadata:");
                                    map.insert("metadata".to_owned(), m.to_owned());
                                    *self = RecvState::GetContent;
                                },
                                &mut RecvState::GetContent => {
                                    debug!("got content:");
                                    map.insert("content".to_owned(), m.to_owned());
                                    *self = RecvState::Finish;
                                },
                                &mut RecvState::Finish => {
                                    debug!("DONE");
                                    break 'getmessage;
                                },
                            }
                            debug!("msg.as_str() was {:?}", m);
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
            Ok(RawMessage::from_map(map.clone()))
        } else {
            Err(Error::MessageDecodeError("get_raw_message failed".into()))
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
            debug!("msg_type: {}", raw_message.msg_type().unwrap());
        }
    }
}
