#![allow(dead_code)]
#![allow(unused_variables)]

extern crate trailer;
extern crate binance;

mod csv;
mod error;

#[derive(Clone, Copy, Debug)]
struct TrailingStopLoss {
    sell_price: f64,
    stop_distance: f64,
}

impl TrailingStopLoss {
    pub fn trigger_price(&self) -> f64 {
        self.sell_price - self.stop_distance
    }

    pub fn buffer_distance(&self, current_price: f64) -> f64 {
        current_price - self.trigger_price() 
    }

    pub fn update_price(&mut self, current_price: f64) {
        if self.buffer_distance(current_price) > self.stop_distance {
            let new_price = current_price - self.stop_distance;
            println!("price sell catch updated to {}", new_price);
            self.sell_price = new_price;
        }
    }
}

fn main() {
    let tsl = TrailingStopLoss {
        sell_price:     0.000209,
        stop_distance:  0.00001,
    };

    use trailer::socket::*;

    let bot = trailer::socket::BinanceWS::new(move |event| {
        println!("{:?}", event);
        match event {
            Event::price_change(symbol, price, vol) => {
                println!("data recieved: {}@{} - trigger price = {} - buffer distance = {}", symbol, price, tsl.trigger_price(), tsl.buffer_distance(price));
                // tsl.update_price(price);
            },
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