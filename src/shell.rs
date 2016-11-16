use zmq;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;

use message::Message;
use Ports;

pub struct Shell {
    transport: String,
    addr: String,
    ports: Ports,
}

impl Shell {
    pub fn new(tns: &str, addr: &str, ports: Ports) -> Shell {
        Shell {
            transport: tns.into(),
            addr: addr.into(),
            ports: ports,
        }
    }

    pub fn listen(&self, ctx: Arc<Mutex<RefCell<zmq::Context>>>) {
        let mut router = {
            let ctx = ctx.lock().unwrap();
            let mut ctx = ctx.borrow_mut();
            ctx.socket(zmq::ROUTER).unwrap()
        };
        let address = format!("{}://{}:{}", &self.transport, &self.addr, self.ports.shell_port);

        debug!("shell address is {}", &address);
        assert!(router.bind(&address).is_ok());
        loop {
            let message = Message::from_socket(&mut router).expect("Could not get message");
            // handle message
        }
    }
}
