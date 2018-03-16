#![allow(dead_code)]
#![allow(unused_variables)]

use kucoin;

use ::types::*;
use ::error::*;
use ::exchanges::ExchangeAPI;

pub struct KucoinAPI {
}

pub fn connect(api_key: &str, secret_key: &str) -> KucoinAPI {
    KucoinAPI {
    }
}

impl ExchangeAPI for KucoinAPI {
    fn display(&self) -> String {
        "Kucoin".into()
    }

    fn funds(&self) -> Result<Vec<CoinAsset>, TrailerError> {
        Err(TrailerError::unsupported())
    }

    fn price(&self, symbol: &str) -> Result<f64, TrailerError> {
        Err(TrailerError::unsupported())
    }

    fn prices(&self) -> Result<Prices, TrailerError> {
        Err(TrailerError::unsupported())
    }

    fn limit_buy(&self, symbol: &str, amount: f64, price: f64) -> Result<(), TrailerError> {
        Err(TrailerError::unsupported())
    }

    fn limit_sell(&self, symbol: &str, amount: f64, price: f64) -> Result<(), TrailerError> {
        Err(TrailerError::unsupported())
    }

    fn open_orders(&self) -> Result<Vec<Order>, TrailerError> {
        Err(TrailerError::unsupported())
    }

    fn past_orders(&self) -> Result<Vec<Order>, TrailerError> {
        Err(TrailerError::unsupported())
    }

    fn past_orders_for(&self, symbol: &str) -> Result<Vec<Order>, TrailerError> {
        Err(TrailerError::unsupported())
    }
}