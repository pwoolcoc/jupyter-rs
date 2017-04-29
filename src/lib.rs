#![feature(proc_macro)]
#![recursion_limit = "1024"]

//! Jupyter Kernel for Rust
//!
//! This crate implements a kernel backend for the jupyter
//! notebook system (http:/jupyter.org).

#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate error_chain;
extern crate tokio_core;
extern crate zmq;
extern crate env_logger;
extern crate serde;
extern crate serde_json;
extern crate zmq_tokio;
extern crate futures;

use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use std::io::Read;
use tokio_core::reactor::Core;
use zmq_tokio::Context;

use errors::*;
use heartbeat::Heartbeat;
use shell::Shell;
use control::Control;
use stdin::Stdin;
use iopub::Iopub;

pub mod errors;
mod heartbeat;
mod shell;
mod control;
mod stdin;
mod iopub;

/// Represents the JSON structure that the jupyter
/// application passes us when it initializes the kernel
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

impl KernelConfig {
    pub fn from_reader<R: Read>(r: R) -> Result<KernelConfig> {
        let config: KernelConfig = try!(serde_json::from_reader(r));
        Ok(config)
    }
}

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

        let mut l = Core::new().unwrap();
        let handle = l.handle();

        let ctx = Arc::new(Mutex::new(RefCell::new(Context::new())));

        let hb = Heartbeat::new(&self.config.transport, &self.config.ip, self.config.hb_port);
        let shell = Shell::new(&self.config.transport, &self.config.ip, self.config.shell_port);
        let control = Control::new(&self.config.transport, &self.config.ip,
                self.config.control_port);
        let stdin = Stdin::new(&self.config.transport, &self.config.ip,
                self.config.stdin_port);
        let iopub = Iopub::new(&self.config.transport, &self.config.ip,
                self.config.iopub_port);

        handle.spawn(shell.listen(&handle, ctx.clone()));
        handle.spawn(hb.listen(&handle, ctx.clone()));
        handle.spawn(control.listen(&handle, ctx.clone()));
        handle.spawn(stdin.listen(&handle, ctx.clone()));
        handle.spawn(iopub.listen(&handle, ctx.clone()));

        loop {
            l.turn(None);
        }
        Ok(())
    }
}
