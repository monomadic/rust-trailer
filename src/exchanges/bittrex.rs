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
        Err(TrailerError::unsupported())
    }

    fn limit_sell(&self, symbol: &str, amount: f64, price: f64) -> Result<(), TrailerError> {
        Err(TrailerError::unsupported())
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
}

impl BittrexAPI {
    pub fn funds(&self) -> Vec<CoinAsset> {
        let mut funds = Vec::new();

        match self.client.get_balances() {
            Ok(balances) => {
                for balance in balances {
                    funds.push(CoinAsset {
                        symbol: balance.currency,
                        amount: balance.balance as f64,
                        locked: (balance.balance - balance.available) as f64,
                        exchange: "Bittrex".to_string(),
                    })
                }
            },
            Err(e) => println!("Error: {}", e),
        };

        funds
    }

    // pub fn orders(&self) -> Result<Vec<Order>, TrailerError> {
    //     Ok(self.client.get_open_orders()?.into_iter().map(|order| {
    //         Order{
    //             id:             order.order_uuid,
    //             symbol:         order.exchange,
    //             order_type:     order.order_type,
    //             amount:         order.quantity as f64,
    //             price:          order.limit as f64,
    //         }
    //     }).collect())
    // }

    pub fn history(&self) -> Vec<Order> {
        let mut orders = Vec::new();

        match self.client.get_order_history() {
            Ok(result) => {
                for order in result {
                    // println!("{}", order);
                    orders.push(Order{
                        id: order.order_uuid,
                        symbol: order.exchange,
                        order_type: order.order_type,
                        amount: order.quantity as f64,
                        price: order.price as f64,
                    });
                }
            },
            Err(e) => println!("Error: {}", e),
        };

        orders
    }
}
