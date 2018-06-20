#![allow(dead_code)]
#![allow(unused_variables)]

extern crate trailer;
extern crate binance;

mod csv;
mod error;

#[derive(Clone, Copy, Debug)]
struct TrailingStopLoss {
    trigger_price: f64,
    stop_percent: f64,
    triggered: bool,
}

fn main() {
    let tsl = TrailingStopLoss {
        trigger_price: 10.,
        stop_percent: 10.,
        triggered: false,
    };

    let bot = trailer::socket::BinanceWS::new(move |event| {
        match event {
            price_change => {
                let _ = trailing_stop_loss(tsl);
            },
            // _ => println!("{:?}", event),
        };
    });

    match bot.run() {
        Ok(_) => println!("done."),
        Err(e) => println!("error: {:?}", e),
    };

    // let bot = Bot::new_with_config("./data/bots/new.toml");
    // let csv = csv::load_backtest_data("./data/backtests/new.csv").expect("csv failed to load");

}

fn trailing_stop_loss(tsl: TrailingStopLoss) -> Result<(), String> {
    println!("tsl: {:?}", tsl);
    Ok(())
}