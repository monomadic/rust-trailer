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
extern crate cobinhood;

pub mod exchanges;
mod command;
mod bot;
mod display;
pub mod types;
pub mod config;
pub mod coinmarketcap;
pub mod error;

// fn main() {
//     match ::command::run_docopt() {
//         Ok(_) => println!("done."),
//         Err(e) => ::display::show_error(e),
//     }
// }
