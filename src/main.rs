extern crate binance;
extern crate colored;
extern crate docopt;
extern crate bittrex_api;

extern crate toml;
#[macro_use]
extern crate serde_derive;

mod exchanges;
mod command;
mod bot;
mod display;
mod types;
mod config;

fn main() {
    ::command::run_docopt().expect("success");
    // println!("test");
    // let trades = vec![
    //     types::Trade{ cost: 10., qty: 10.0, buy: true },
    //     types::Trade{ cost: 10., qty: 10.0, buy: true },
    //     types::Trade{ cost: 12., qty: 20.0, buy: false },
    //     types::Trade{ cost: 200., qty: 20.0, buy: true },
    // ];
    // display::show_trades(trades);
}
