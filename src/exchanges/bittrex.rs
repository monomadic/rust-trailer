#![allow(dead_code)]
#![allow(unused_variables)]

use bittrex::BittrexClient;

use std::collections::HashMap;

use ::types::*;
use ::error::*;
use ::exchanges::ExchangeAPI;

pub struct BittrexAPI {
    client: BittrexClient,
}

pub fn connect(api_key: &str, secret_key: &str) -> BittrexAPI {
    BittrexAPI {
        client: BittrexClient::new(
            api_key.to_string(),
            secret_key.to_string()
        ),
    }
}

impl ExchangeAPI for BittrexAPI {
    fn funds(&self) -> Result<Vec<CoinAsset>, TrailerError> {
        let balances = self.client.get_balances()?;

        Ok(balances.into_iter().map(|balance| {
            CoinAsset {
                symbol: balance.currency,
                amount: balance.balance as f64,
                locked: (balance.balance - balance.available) as f64,
                exchange: "Bittrex".to_string(),
            }
        }).collect())
    }

    fn price(&self, symbol: &str) -> Result<f64, TrailerError> {
        Ok(self.client.get_ticker(symbol)?.last as f64)
    }

    fn prices(&self) -> Result<Prices, TrailerError> {
        let response = self.client.get_market_summaries()?;
        let mut p = HashMap::new();

        for market in response {
            let split: Vec<&str> = market.market_name.split("-").collect();
            let pair_name = format!("{}{}", *split.last().unwrap(), *split.first().unwrap()); // dangerous, fix

            p.insert(
                pair_name,
                market.last
            );
        }
        
        Ok(p)
    }

    fn limit_buy(&self, symbol: &str, amount: f64, price: f64) -> Result<(), TrailerError> {
        let result = self.client.buy_limit(symbol, amount, price)?;
        println!("{}", result);
        Ok(())
    }

    fn limit_sell(&self, symbol: &str, amount: f64, price: f64) -> Result<(), TrailerError> {
        let result = self.client.sell_limit(symbol, amount, price)?;
        println!("{}", result);
        Ok(())
    }

    fn open_orders(&self) -> Result<Vec<Order>, TrailerError> {
        Ok(self.client.get_open_orders()?.into_iter().map(|order| {
            Order{
                id:             order.order_uuid,
                symbol:         order.exchange,
                order_type:     order.order_type,
                amount:         order.quantity as f64,
                price:          order.limit as f64,
            }
        }).collect())
    }

    fn past_orders(&self) -> Result<Vec<Order>, TrailerError> {
        Ok(self.client.get_order_history()?.into_iter().map(|order| {
            {
                Order{
                    id:             order.order_uuid,
                    symbol:         order.exchange,
                    order_type:     order.order_type,
                    amount:         order.quantity as f64,
                    price:          order.limit as f64,
                }
            }
        }).collect())
    }

    fn past_orders_for(&self, symbol: &str) -> Result<Vec<Order>, TrailerError> {
        Err(TrailerError::unsupported())
    }
}
