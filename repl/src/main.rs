extern crate colored;
extern crate trailer;

mod repl;
mod display;
mod commands;

fn main() {
    match ::repl::run() {
        Ok(_) => println!("done."),
        Err(e) => display::error(e),
    }
}
