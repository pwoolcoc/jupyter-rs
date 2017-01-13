use zmq;
use serde_json;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use std::sync::mpsc::{channel, Sender};

use status::Status;
use message::Message;
use Ports;

pub struct Shell {
    transport: String,
    addr: String,
    ports: Ports,
    to_iopub: Sender<Status>,
}

impl Shell {
    pub fn new(tns: &str, addr: &str, to_iopub: Sender<Status>, ports: Ports) -> Shell {
        Shell {
            transport: tns.into(),
            addr: addr.into(),
            ports: ports,
            to_iopub: to_iopub,
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
            let message = match Message::from_socket(&mut router) {
                Ok(m) => m,
                Err(e) => {
                    error!("Error in message!");
                    continue;
                },
            };
            let reply = match message.reply() {
                Ok(m) => m,
                Err(e) => {
                    error!("Error in reply!");
                    continue;
                },
            };
            let reply = reply.to_json();
            let reply = serde_json::to_string(&reply).unwrap();
            debug!("reply is {:?}", &reply);
            router.send_str(&reply, 0);
            // handle message
        }
    }
}
