pub mod binance;
pub mod bittrex;
pub mod cobinhood;

use ::types::*;
use ::error::*;

pub trait ExchangeAPI {
    fn funds(&self) -> Result<Vec<CoinAsset>, TrailerError>;
    fn price(&self, symbol: &str) -> Result<f64, TrailerError>;
    fn prices(&self) -> Result<Prices, TrailerError>;
    fn limit_buy(&self, symbol: &str, amount: f64, price: f64) -> Result<(), TrailerError>;
    fn limit_sell(&self, symbol: &str, amount: f64, price: f64) -> Result<(), TrailerError>;
    fn open_orders(&self) -> Result<Vec<Order>, TrailerError>;
    fn past_orders(&self) -> Result<Vec<Order>, TrailerError>;
}
