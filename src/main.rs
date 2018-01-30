extern crate binance;
extern crate colored;
extern crate docopt;
extern crate bittrex_api;
extern crate reqwest;

extern crate toml;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

mod exchanges;
mod command;
mod bot;
mod display;
mod types;
mod config;
mod coinmarketcap;

fn main() {
    ::command::run_docopt().expect("success");

    // println!("cap.symbol, cap.volume_usd_24h, cap.market_cap_usd, cap.cap_vs_vol_24h()");
    // for cap in ::coinmarketcap::all() {
    //     println!("{},{},{},{}", cap.symbol, cap.volume_usd_24h, cap.market_cap_usd, cap.cap_vs_vol_24h());
    // }
    // println!("test");
    // let trades = vec![
    //     types::Trade{ cost: 10., qty: 10.0, buy: true },
    //     types::Trade{ cost: 10., qty: 10.0, buy: true },
    //     types::Trade{ cost: 12., qty: 20.0, buy: false },
    //     types::Trade{ cost: 200., qty: 20.0, buy: true },
    // ];
    // display::show_trades(trades);
}
