**THIS IS CURRENTLY BEING DEVELOPED, BUT IS NOT FUNCTIONAL YET**

# Jupyter backend for Rust

# Getting Started

Right now I have installation instructions for linux, if you are on a
different platform then I will do what I can to help.

  1. Make sure `jupyter` is on your `$PATH`
  2. Compile `jupyter-rs` and make sure the `jupyter-rust` binary is on
     your `$PATH`. If you already have the `cargo-install` directory on
     your `$PATH`, then the easiest way to accomplish this is to run
     `cargo install --path .` in this directory.
  3. Run `./setup.sh`

`setup.sh` should copy the kernelspec into the correct place for
`jupyter` to be able to find it.

From here, if you start a notebook, `rust` should appear in the `New`
menu. Or, you can run `jupyter console --kernel rust`.

