extern crate jupyter_kernel;

use jupyter_kernel::{Kernel, KernelConfig, Result};

use std::env;
use std::process;
use std::fs::File;
use std::io::{self, Write};
use std::error::Error as StdError;


fn err(msg: &str) {
    let _ = io::stderr().write(msg.as_bytes());
}

fn run() -> Result<()> {
    let mut args = env::args();
    if args.len() < 1 {
        err(&format!("Not enough arguments"));
        panic!("Must pass the name of a file that contains config info");
    }
    let fname = args.nth(2).unwrap();
    let fh = try!(File::open(fname));

    let config = KernelConfig::from_reader(fh).unwrap();

    let kernel = Kernel::from_config(config);

    kernel.run()
}

fn main() {
    match run() {
        Ok(_) => {
            process::exit(0);
        }
        Err(e) => {
            err(&format!("error was: {}", e.description()));
            process::exit(255);
        }
    }
}
