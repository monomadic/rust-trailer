extern crate binance;
extern crate colored;
extern crate docopt;
extern crate bittrex_api as bittrex;
extern crate reqwest;

extern crate toml;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate ratelimit;

mod exchanges;
mod command;
mod bot;
mod display;
mod types;
mod config;
mod coinmarketcap;
mod error;

fn main() {
    match ::command::run_docopt() {
        Ok(_) => println!("done."),
        Err(e) => ::display::show_error(e),
    }
}
