use zmq;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;

use message;

pub struct Shell {
    transport: String,
    addr: String,
    port: u32,
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
        let mut msg = zmq::Message::new().unwrap();
        loop {
            match router.recv(&mut msg, 0) {
                Ok(m) => {
                    match msg.as_str() {
                        Some(m) => {
                            debug!("msg.as_str was {:?}", m);
                        }
                        None => {
                            debug!("msg.as_str was none?");
                            // debug!("when msg was {:?}", msg.msg.unnamed_field1);
                        }
                    };
                    debug!("m is {:?}", m);
                }
                Err(e) => {
                    error!("Err(e) was {:?}", e);
                }
            }
        }
    }
}
