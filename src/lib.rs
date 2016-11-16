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
use std::sync::mpsc::channel;
use std::cell::RefCell;

use errors::*;
use heartbeat::Heartbeat;
use shell::Shell;
use iopub::Iopub;

pub mod errors;
mod msg_type;
mod message;
mod heartbeat;
mod shell;
mod status;
mod iopub;
// mod stdin;
mod messages;

#[derive(Serialize, Deserialize, Debug)]
pub struct KernelConfig {
    pub control_port: u32,
    pub shell_port: u32,
    transport: String,
    signature_scheme: String,
    pub stdin_port: u32,
    pub hb_port: u32,
    ip: String,
    pub iopub_port: u32,
    key: String,
}

pub struct Ports {
    pub control_port: u32,
    pub shell_port: u32,
    pub stdin_port: u32,
    pub hb_port: u32,
    pub iopub_port: u32,
}

impl<'a> From<&'a KernelConfig> for Ports {
    fn from(k: &KernelConfig) -> Ports {
        Ports {
            control_port: k.control_port,
            shell_port: k.shell_port,
            stdin_port: k.stdin_port,
            hb_port: k.hb_port,
            iopub_port: k.iopub_port,
        }
    }
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

        let (tx, rx) = channel();
        let ctx = Arc::new(Mutex::new(RefCell::new(zmq::Context::new())));
        let mut threads = vec![];

        let hb = Heartbeat::new(&transport, &ip, self.config.hb_port);
        let hb_ctx = ctx.clone();
        threads.push(thread::spawn(move || hb.listen(hb_ctx)));

        // Control is the same as shell
        let ctrl = Shell::new(&transport, &ip, tx.clone(), Ports::from(&self.config));
        let ctrl_ctx = ctx.clone();
        threads.push(thread::spawn(move || ctrl.listen(ctrl_ctx)));

        let shell = Shell::new(&transport, &ip, tx.clone(), Ports::from(&self.config));
        let shell_ctx = ctx.clone();
        threads.push(thread::spawn(move || shell.listen(shell_ctx)));

        let iopub = Iopub::new(&transport, &ip, rx, self.config.iopub_port);
        let iopub_ctx = ctx.clone();
        threads.push(thread::spawn(move || iopub.listen(iopub_ctx)));

        for thread in threads {
            let _ = thread.join();
        }
        Ok(())
    }
}
