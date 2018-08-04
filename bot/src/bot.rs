// use trailer;
use error::*;

use binance::websockets::*;
use binance::model::{ TradesEvent, DepthOrderBookEvent, OrderBook };

use std::cell::RefCell;

#[derive(Clone)]
pub struct TrailerBot {
    pub symbol: String,
    pub dump_price: RefCell<f64>,
    pub stop_distance: f64,
}

impl MarketEventHandler for TrailerBot {
    fn aggregated_trades_handler(&self, event: &TradesEvent) {
        let price = event.price.parse::<f64>().unwrap();
        self.update_price(price);
    }
    fn depth_orderbook_handler(&self, model: &DepthOrderBookEvent) {
        println!("- Depth Order Book: {:?}", model);
    }
    fn partial_orderbook_handler(&self, model: &OrderBook) {
        println!("- Partial Order Book: {:?}", model);
    }
}

impl TrailerBot {
    pub fn new(symbol: &str, entry_price: f64, stop_distance_percent: f64) -> Self {
        TrailerBot {
            symbol: symbol.into(),
            dump_price: RefCell::new(entry_price * (1.0 - stop_distance_percent / 100.0)),
            stop_distance: entry_price - entry_price * (1.0 - stop_distance_percent / 100.0),
        }
    }

    pub fn run(&self) -> Result<String, BotError> {
        println!("bot started.");
        let current_position = 0.0_f64;

        println!("attempting to connect to binance with symbol: {}", &self.symbol);
        let agg_trade: String = format!("{}@aggTrade", self.symbol);
        let mut web_socket: WebSockets = WebSockets::new();

        web_socket.add_market_handler(self.clone());
        web_socket.connect(&agg_trade).unwrap(); // check error
        web_socket.event_loop();

        Ok("done.".to_string())
    }

    // pub fn backtest(&self, prices: Vec<f64>) -> Result<(), BotError> {
    //     let price = 10.0;
    //     let buy_price = prices.first().unwrap();

    //     for price in prices.clone() {
    //         let stop = price - (price * 0.1);
    //         println!("price: {}, stop: {}", price, stop);
    //         self.evaluate_state(price);
    //     }

    //     Ok(())
    // }

    // pub fn trigger_price(&self) -> f64 {
    //     *self.dump_price.borrow() * self.stop_distance_multiplier
    // }

    // pub fn buffer_distance(&self, current_price: f64) -> f64 {
    //     current_price - self.trigger_price() 
    // }

    pub fn update_price(&self, current_price: f64) {
        let dump_price = self.dump_price.borrow().clone();

        if current_price > dump_price {
            // price went up
            *self.dump_price.borrow_mut() = current_price - self.stop_distance;
            println!("price went above threshold, dump price updated");
        } else {
            // price went down, assess whether to sell
            println!("threshold hit, sell");
        }

        // if self.buffer_distance(current_price) > self.stop_distance_percent {
        //     let new_price = current_price - self.stop_distance_percent;

        //     *self.dump_price.borrow_mut() = new_price;

        //     println!("[trigger price update: {}]", new_price);
        // }
    }

    pub fn print_state(&self) {
        println!("{:20}{:20}{:20}{:20}{:20}", "PAIR", "AMOUNT", "BASE_PRICE", "P/L", "P/L%");
        println!("{:20}{:20}{:20}{:20}{:20}", self.symbol, "-", "-", "-", "-");
    }

    // pub fn new_with_config(_config_file: &str) -> Self {
    //     Self {
    //         symbol: "icxbtc".to_string(),
    //         dump_price: 0.,
    //         stop_distance: 0.,
    //     }
    // }
}

#[test]
fn test_stop() {
    // let bot = TrailerBot{ symbol: "TESTBTC".to_string(), dump_price: RefCell::new(1000.0), stop_distance_percent: 20.0 };

    let bot = TrailerBot::new("TESTBTC", 1000.0, 20.0);
    assert_eq!(bot.symbol, "TESTBTC".to_string());
    assert_eq!(*bot.dump_price.borrow(), 800.0);
    assert_eq!(bot.stop_distance, 200.0);

    bot.update_price(1000.1);
    assert_eq!(*bot.dump_price.borrow(), 800.1);

    bot.update_price(900.0);
    assert_eq!(*bot.dump_price.borrow(), 800.1);

    // assert_eq!(bot.buffer_distance(1000.0), 200.0);
    // assert_eq!(*bot.dump_price.borrow(), 200.0);

    // price goes UP, so trigger price should go up.
    // bot.update_price(2000.0);
    // assert_eq!(bot.trigger_price(), 200.0);
}