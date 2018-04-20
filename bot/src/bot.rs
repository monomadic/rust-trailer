// use trailer;
use error::*;

use binance::websockets::*;
use binance::model::{ TradesEvent, DepthOrderBookEvent, OrderBook };

#[derive(Clone)]
pub struct Bot {
    pub symbol: String,
    // entries: Vec<(f64)>, // positions at which to enter the market
    previous_price: f64,
}

impl MarketEventHandler for Bot {
    fn aggregated_trades_handler(&self, event: &TradesEvent) {
        // println!(
        //     "- Symbol: {}, price: {}, qty: {}",
        //     event.symbol, event.price, event.qty
        // );
        println!("- Event: {:?}", event);
    }
    fn depth_orderbook_handler(&self, model: &DepthOrderBookEvent) {
        println!("- Depth Order Book: {:?}", model);
    }
    fn partial_orderbook_handler(&self, model: &OrderBook) {
        println!("- Partial Order Book: {:?}", model);
    }
}

impl Bot {
    pub fn run(&self) -> Result<(), BotError> {
        println!("bot started.");
        let current_position = 0.0_f64;

        println!("attempting to connect to binance with symbol: {}", &self.symbol);
        let agg_trade: String = format!("{}@aggTrade", self.symbol);
        let mut web_socket: WebSockets = WebSockets::new();

        web_socket.add_market_handler(self.clone());
        web_socket.connect(&agg_trade).unwrap(); // check error
        web_socket.event_loop();

        // Ok(trailer::exchanges::binance::ws(self.symbol.clone()))
        Ok(())
    }

    pub fn backtest(&self, prices: Vec<f64>) -> Result<(), BotError> {
        let price = 10.0;
        let buy_price = prices.first().unwrap();

        for price in prices.clone() {
            let stop = price - (price * 0.1);
            println!("price: {}, stop: {}", price, stop);
            self.evaluate_state(price);
        }

        Ok(())
    }

    pub fn evaluate_state(&self, price: f64) {
    }

    // pub fn backtest(&self, prices: Vec<f64>) {
    //     println!("backtesting bot on coin: {}", self.symbol);

    //     let mut ratelimit = ratelimit::Builder::new()
    //         .capacity(1) //number of tokens the bucket will hold
    //         .quantum(1) //add one token per interval
    //         .interval(Duration::new(1, 0)) //add quantum tokens every 1 second
    //         .build();

    //     let handle = ratelimit.make_handle();
    //     thread::spawn(move || { ratelimit.run() });

    //     // launch threads
    //     let mut threads = Vec::new();
    //     for _ in 0..10 {
    //         let mut handle = handle.clone();
    //         threads.push(thread::spawn(move || {
    //             handle.wait();
    //             // println!("current time: {:?}", Instant::now());
    //             println!(" - heartbeat");
    //         }));
    //     }
    //     for thread in threads {
    //         thread.join().unwrap();
    //     }
    //     println!("time's up!");
    // }

    pub fn new_with_config(_config_file: &str) -> Self {
        Self {
            symbol: "icxbtc".to_string(),
            // entries
            previous_price: 0.0,
        }
    }
}
