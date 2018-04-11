extern crate colored;
extern crate docopt;
extern crate toml;

#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate trailer;

mod commands;
mod display;

fn main() {
    match ::commands::run_docopt() {
        Ok(_) => println!("done."),
        Err(e) => display::error(e),
    }
}
