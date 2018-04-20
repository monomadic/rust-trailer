#![allow(dead_code)]
#![allow(unused_variables)]

extern crate trailer;
extern crate binance;

//use ratelimit;
//use std::thread;
//use std::time::{Duration};

mod csv;

mod bot;
use bot::Bot;

mod error;

fn main() {
    let bot = Bot::new_with_config("./data/bots/new.toml");
    let csv = csv::load_backtest_data("./data/backtests/new.csv").expect("csv failed to load");

    match bot.backtest(csv) {
        Ok(_) => println!("done."),
        Err(e) => println!("error: {:?}", e),
    }
}
