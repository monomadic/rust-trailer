#![allow(dead_code)]
#![allow(unused_variables)]

// NOTE: Binance rate limits are: 1200 requests per minute; 10 orders per second; 100,000 orders per 24hrs. 

use binance::websockets::MarketEventHandler;
use binance::model::TradesEvent;
use binance::model::DepthOrderBookEvent;
use binance::model::OrderBook;
use binance::websockets::WebSockets;

// use ::types::*;

pub struct BinanceWS {
    event_callback: Box<Fn()>,
}

struct BinanceWebSocketHandler;

impl BinanceWS {
    pub fn new(event_callback: impl Fn() + 'static) -> Self {
        BinanceWS {
            event_callback: Box::new(event_callback),
        }
    }

    pub fn run(self) {
        let mut socket: WebSockets = WebSockets::new();
        // let agg_trade: String = format!("{}@aggTrade", "icxbtc");

        socket.add_market_handler(self);
        socket.connect("icxbtc@aggTrade").unwrap(); // check error
        socket.event_loop();
    }
}

impl MarketEventHandler for BinanceWS {
    fn aggregated_trades_handler(&self, event: &TradesEvent) {
        println!(
            "- Symbol: {}, price: {}, qty: {}",
            event.symbol, event.price, event.qty
        );
        (self.event_callback)();
    }
    fn depth_orderbook_handler(&self, model: &DepthOrderBookEvent) {}
    fn partial_orderbook_handler(&self, model: &OrderBook) {}
}
