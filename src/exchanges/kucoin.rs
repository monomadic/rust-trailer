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

    fn funds(&self) -> Result<Funds, TrailerError> {
        let balances = self.balances()?;
        let prices = self.prices()?;

        Ok(Funds {
            btc:    balances.clone().into_iter().find(|c| c.symbol == "BTC"),
            fiat:   balances.clone().into_iter().filter(|c| c.symbol == "USDT").collect(),
            alts:   balances.into_iter().filter(|c| c.symbol != "USDT" && c.symbol != "BTC").collect(),
            total_value_in_usd: 33.0,
            total_value_in_btc: 44.0,
        })
    }

    fn balances(&self) -> Result<Vec<CoinAsset>, TrailerError> {
        // println!("BALANCE in FUNDS(): {:?}", self.client.balance());
        Ok(self.client.balance()?.into_iter().map(|balance| {
            CoinAsset {
                symbol:         balance.symbol,
                amount:         balance.balance,
                locked:         balance.locked,
                exchange:       Exchange::Kucoin,
                value_in_btc:   None,
                value_in_usd:   None,
            }
        }).collect())
    }

    fn price(&self, symbol: &str) -> Result<f64, TrailerError> {
        Err(TrailerError::unsupported())
    }

    fn prices(&self) -> Result<Prices, TrailerError> {
        let response = self.client.symbols()?;
        let mut p = HashMap::new();

        // println!("\n\nresponse: {:?}", response);

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