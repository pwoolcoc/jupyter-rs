use zmq;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;

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
            let ctx = ctx.lock().expect("Could not get a lock on the zmq Context");
            let mut ctx = ctx.borrow_mut();
            ctx.socket(zmq::REP).expect("Could not create heartbeat socket")
        };
        let address = format!("{}://{}:{}", &self.transport, &self.addr, self.port);

        debug!("heartbeat address is {}", address);
        assert!(responder.bind(&address).is_ok());
        let mut msg = zmq::Message::new().expect("Could not create new zmq Message");
        loop {
            responder.recv(&mut msg, 0).expect("got an Err on the heartbeat responder.recv");
            let recvd = msg.as_str().expect("Msg from heartbeat message was empty");
            debug!("heartbeat received '{}'", recvd);
            responder.send_str(recvd, 0).expect("Could not send ping back");
        }
    }
}
