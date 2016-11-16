use zmq;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use std::sync::mpsc::Receiver;

use status::Status;

pub struct Iopub {
    transport: String,
    addr: String,
    port: u32,
    from_shell: Receiver<Status>,
}

impl Iopub {
    pub fn new(tns: &str, addr: &str, from_shell: Receiver<Status>, port: u32) -> Iopub {
        Iopub {
            transport: tns.into(),
            addr: addr.into(),
            port: port,
            from_shell: from_shell,
        }
    }

    pub fn listen(&self, ctx: Arc<Mutex<RefCell<zmq::Context>>>) {
        let mut pub_ = {
            let ctx = ctx.lock().unwrap();
            let mut ctx = ctx.borrow_mut();
            ctx.socket(zmq::PUB).unwrap()
        };
        let address = format!("{}://{}:{}", &self.transport, &self.addr, self.port);

        debug!("iopub addres is {}", &address);
        assert!(pub_.bind(&address).is_ok());
    }
}

