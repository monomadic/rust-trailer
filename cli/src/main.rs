extern crate colored;
extern crate docopt;
extern crate ta;
extern crate plotlib;

#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate trailer;

mod commands;
mod display;
mod input;
mod graphs;

fn main() {
    match ::commands::run_docopt() {
        Ok(m) => print!("{}", m),
        Err(e) => display::error(e),
    }
}
