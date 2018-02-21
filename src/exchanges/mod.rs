pub mod binance;
pub mod bittrex;
pub mod cobinhood;

use ::types::*;
use ::error::*;

pub trait ExchangeAPI {
    // fn connect(api_key: &str, secret_key: &str) -> Self;
    fn funds(&self) -> Result<Vec<CoinAsset>, TrailerError>;
    fn price(&self, symbol: &str) -> Result<f64, TrailerError>;
    fn prices(&self) -> Result<Prices, TrailerError>;
    // fn history(&self) -> Result<Vec<Order>, TrailerError>;
    fn limit_buy(&self, symbol: &str, amount: f64, price: f64) -> Result<(), TrailerError>;
    fn limit_sell(&self, symbol: &str, amount: f64, price: f64) -> Result<(), TrailerError>;
    fn orders(&self) -> Result<Vec<Order>, TrailerError>;
}
