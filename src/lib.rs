extern crate binance;
extern crate colored;
extern crate docopt;
extern crate bittrex_api as bittrex;
extern crate reqwest;
extern crate kucoin;

extern crate toml;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate ratelimit;

pub mod exchanges;
pub mod socket;
pub mod types;
pub mod config;
pub mod error;
