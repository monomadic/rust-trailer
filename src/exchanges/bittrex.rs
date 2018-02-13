#![allow(dead_code)]
#![allow(unused_variables)]

use bittrex::BittrexClient;
use std::collections::HashMap;

use ::types::*;

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

    pub fn orders(&self) -> Vec<Order> {
        let mut orders = Vec::new();

        match self.client.get_open_orders() {
            Ok(result) => {
                for order in result {
                    // println!("{}", order);
                    orders.push(Order{
                        id: order.order_uuid,
                        symbol: order.exchange,
                        order_type: order.order_type,
                        amount: order.quantity as f64,
                        price: order.limit as f64,
                    });
                }
            },
            Err(e) => println!("Error: {}", e),
        };

        orders
    }

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

    pub fn prices(&self) -> Result<Prices, ::error::TrailerError> {
        let response = self.client.get_market_summaries()?;
        let mut p = HashMap::new();

        for market in response {
            let split: Vec<&str> = market.market_name.split("-").collect();
            // print!("{:?} ", split);
            let pair_name = format!("{}{}", *split.last().unwrap(), *split.first().unwrap()); // dangerous, fix
            // print!("{} ", pair_name);

            p.insert(
                pair_name,
                market.last
            );
        }
        
        Ok(p)
    }
}