extern crate jupyter_kernel;

use jupyter_kernel::{Kernel};
use jupyter_kernel::errors::Result;

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
    let fh = File::open(fname)?;

    let kernel = Kernel::from_reader(fh)?;

    kernel.run()
}

fn main() {
    if let Err(e) = run() {
        err(&format!("error was: {}", e.description()));
        process::exit(255);
    }
}
