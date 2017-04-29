use std::sync::{Arc, Mutex};
use std::cell::RefCell;

use zmq;
use tokio_core::reactor::Handle;
use zmq_tokio::{Context, Socket};
use futures::{Future, Sink, Stream};
use futures::future::{BoxFuture};

pub struct Iopub {
    transport: String,
    addr: String,
    port: u32,
}

impl Iopub {
    pub fn new(transport: &str, addr: &str, port: u32) -> Iopub {
        Iopub {
            transport: transport.into(),
            addr: addr.into(),
            port: port,
        }
    }

    fn echo(&self, rep: Socket) -> BoxFuture<(), ()> {
        trace!("entering server");
        let (responses, requests) = rep.framed().split();
        requests.fold(responses, |responses, mut request| {
                let mut part0 = None;
                for part in request.drain(0..1) {
                    part0 = Some(part);
                    break;
                }
                let p = part0.unwrap();
                trace!("got message '{}'", String::from_utf8_lossy(&p));
                responses.send(p)
        }).map(|_| {}).then(|_| Ok(())).boxed()
    }

    pub fn listen(&self, handle: &Handle, ctx: Arc<Mutex<RefCell<Context>>>) -> BoxFuture<(), ()> { 
        let mut responder = {
            let ctx = ctx.lock().expect("Could not get a lock on the zmq Context");
            let ctx = ctx.borrow();
            ctx.socket(zmq::PUB, &handle).expect("Could not create iopub socket")
        };
        let address = format!("{}://{}:{}", &self.transport, &self.addr, self.port);

        debug!("iopub address is {}", address);
        assert!(responder.bind(&address).is_ok());
        self.echo(responder)
    }
}

