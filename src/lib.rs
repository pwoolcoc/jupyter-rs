#![feature(proc_macro)]
#![recursion_limit = "1024"]

//! Jupyter Kernel for Rust
//!
//! This crate implements a kernel backend for the jupyter
//! notebook system (http:/jupyter.org).

#[macro_use] extern crate nom;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate error_chain;
extern crate env_logger;
extern crate zmq;
extern crate serde;
extern crate serde_json;

use std::io::Read;
use std::thread;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;

use errors::*;
use heartbeat::Heartbeat;
use control::Control;
use shell::Shell;

// mod message;
pub mod errors;
mod msg_type;
mod raw_message;
mod heartbeat;
mod control;
mod shell;
// mod iopub;
// mod stdin;

#[derive(Serialize, Deserialize, Debug)]
pub struct KernelConfig {
    control_port: u32,
    shell_port: u32,
    transport: String,
    signature_scheme: String,
    stdin_port: u32,
    hb_port: u32,
    ip: String,
    iopub_port: u32,
    key: String,
}

impl KernelConfig {
    pub fn from_reader<R: Read>(r: R) -> Result<KernelConfig> {
        let config: KernelConfig = try!(serde_json::from_reader(r));
        Ok(config)
    }
}

#[derive(Debug)]
pub struct Kernel {
    config: KernelConfig,
}

impl Kernel {
    pub fn from_reader<R: Read>(r: R) -> Result<Kernel> {
        let config = try!(KernelConfig::from_reader(r));
        Ok(Kernel::from_config(config))
    }

    pub fn from_config(config: KernelConfig) -> Kernel {
        Kernel { config: config }
    }

    pub fn run(&self) -> Result<()> {
        env_logger::init().unwrap();
        debug!("Using config: {:?}", &self.config);
        let transport = self.config.transport.clone();
        let ip = self.config.ip.clone();

        let ctx = Arc::new(Mutex::new(RefCell::new(zmq::Context::new())));
        let mut threads = vec![];

        let hb = Heartbeat::new(&transport, &ip, self.config.hb_port);
        let hb_ctx = ctx.clone();
        threads.push(thread::spawn(move || hb.listen(hb_ctx)));

        let ctrl = Control::new(&transport, &ip, self.config.control_port);
        let ctrl_ctx = ctx.clone();
        threads.push(thread::spawn(move || ctrl.listen(ctrl_ctx)));

        let shell = Shell::new(&transport, &ip, self.config.shell_port);
        let shell_ctx = ctx.clone();
        threads.push(thread::spawn(move || shell.listen(shell_ctx)));

        for thread in threads {
            let _ = thread.join();
        }
        Ok(())
    }
}
