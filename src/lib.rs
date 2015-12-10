#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

//! Jupyter Kernel for Rust
//!
//! This crate implements a kernel backend for the jupyter
//! notebook system (http:/jupyter.org).

#[macro_use]
extern crate nom;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate zmq;
extern crate serde;
extern crate serde_json;

use std::convert::From;
use std::io::{self, Read, Write};
use std::fmt;
use std::error::Error as StdError;
use std::thread;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;

use self::heartbeat::Heartbeat;
use self::control::Control;
use self::shell::Shell;

mod raw_message;
mod message;
mod heartbeat;
mod control;
mod shell;
// mod iopub;
// mod stdin;

#[derive(Debug)]
pub enum Error {
    MessageDecodeError(String),
    KernelError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::MessageDecodeError(ref msg) => {
                write!(f, "{}", msg)
            },
            &Error::KernelError(ref msg) => {
                write!(f, "{}", msg)
            }
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match self {
            &Error::MessageDecodeError(ref msg) => {
                msg
            },
            &Error::KernelError(ref msg) => {
                msg
            }
        }
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Error {
        Error::MessageDecodeError(err.description().into())
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::KernelError(err.description().into())
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;

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
