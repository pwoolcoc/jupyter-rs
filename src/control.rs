use zmq;
use std::sync::{Arc, Mutex};
use std::cell::{RefCell};

pub struct Control {
    transport: String,
    addr: String,
    port: u32,
}

impl Control {
    pub fn new(tns: &str, addr: &str, port: u32) -> Control {
        Control {
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

        debug!("ctrl address is {}", &address);
        assert!(router.bind(&address).is_ok());
        let mut msg = zmq::Message::new().unwrap();
        loop {
            router.recv(&mut msg, 0).unwrap();
            debug!("Ctrl received {}", msg.as_str().unwrap());
        }
    }
}

