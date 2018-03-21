#![allow(dead_code)]
#![allow(unused_variables)]

use kucoin;

use ::types::*;
use ::error::*;
use ::exchanges::{ ExchangeAPI, Exchange };

use std::collections::HashMap;

pub struct KucoinAPI {
    client: kucoin::Client,
}

pub fn connect(api_key: &str, secret_key: &str) -> KucoinAPI {
    KucoinAPI {
        client: kucoin::Client::new(api_key, secret_key)
    }
}

use kucoin::error::KucoinError;
impl From<KucoinError> for TrailerError {
    fn from(error: KucoinError) -> Self {
        TrailerError {
            error_type: TrailerErrorType::APIError,
            message: error.message,
        }
    }
}

impl ExchangeAPI for KucoinAPI {
    fn display(&self) -> String {
        "Kucoin".into()
    }

    fn funds(&self) -> Result<Vec<CoinAsset>, TrailerError> {
        Ok(self.client.balances()?.into_iter().map(|balance| {
            CoinAsset {
                symbol:     balance.symbol,
                amount:     balance.total,
                locked:     balance.locked,
                exchange:   "Kucoin".to_string(),
            }
        }).collect())
    }

    fn price(&self, symbol: &str) -> Result<f64, TrailerError> {
        Err(TrailerError::unsupported())
    }

    fn prices(&self) -> Result<Prices, TrailerError> {
        let response = self.client.prices()?;
        let mut p = HashMap::new();

        for coin in response {
            p.insert(
                coin.symbol,
                coin.last_price
            );
        }
        
        Ok(p)
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