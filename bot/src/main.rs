#![allow(dead_code)]
#![allow(unused_variables)]

extern crate trailer;
extern crate binance;

mod csv;
mod error;
mod bot;

fn main() {
    let conf = ::trailer::config::read(true).expect("config failed to read");
    let keys = &conf.exchange["binance"];
    let binance_client = ::trailer::exchanges::binance::connect(&keys.api_key, &keys.secret_key);

    use ::trailer::exchanges::*;
    let price = binance_client.price("ICXBTC").expect("ICXBTC to return price");

    println!("price retrieved for ICXBTC: {}", price);

    use std::cell::RefCell;
    let bot = bot::TrailerBot::new("icxbtc", price, 10.0);

    // bot::TrailerBot{symbol: "icxbtc".to_string(), sell_threshold: RefCell::new(price), stop_distance_percent: 10.0};

    match bot.run() {
        Ok(m) => print!("{}", m),
        Err(e) => println!("error: {:?}", e),
    };

    // let bot = Bot::new_with_config("./data/bots/new.toml");
    // let csv = csv::load_backtest_data("./data/backtests/new.csv").expect("csv failed to load");

}
