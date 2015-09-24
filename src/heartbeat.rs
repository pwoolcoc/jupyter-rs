use zmq;
use std::sync::{Arc, Mutex};
use std::cell::{RefCell};

pub struct Heartbeat {
    transport: String,
    addr: String,
    port: u32,
}

impl Heartbeat {
    pub fn new(transport: &str, addr: &str, port: u32) -> Heartbeat {
        Heartbeat {
            transport: transport.into(),
            addr: addr.into(),
            port: port,
        }
    }

    pub fn listen(&self, ctx: Arc<Mutex<RefCell<zmq::Context>>>) { // Result?
        let mut responder = {
            let ctx = ctx.lock().unwrap();
            let mut ctx = ctx.borrow_mut();
            ctx.socket(zmq::REP).unwrap()
        };
        let address = format!("{}://{}:{}", &self.transport, &self.addr, self.port);

        debug!("heartbeat address is {}", address);
        assert!(responder.bind(&address).is_ok());
        let mut msg = zmq::Message::new().unwrap();
        loop {
            responder.recv(&mut msg, 0).unwrap();
            let recvd = msg.as_str().unwrap();
            debug!("Heartbeat Received '{}'", recvd);
            responder.send_str(recvd, 0).unwrap();
        }
    }
}
