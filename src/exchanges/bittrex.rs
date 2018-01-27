#![allow(dead_code)]
#![allow(unused_variables)]

use bittrex_api::BittrexClient;
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
    pub fn funds(&self) -> Vec<(String, f64, f64)> {
        let mut funds = Vec::new();

        match self.client.get_balances() {
            Ok(balances) => {
                for balance in balances {
                    funds.push((
                        balance.currency,
                        balance.balance as f64,
                        (balance.balance - balance.available) as f64,
                    ))
                }
            },
            Err(e) => println!("Error: {}", e),
        };

        funds
    }

    pub fn prices(&self) -> HashMap<String, f64> {
        let mut p = HashMap::new();
        match self.client.get_market_summaries() {
            Ok(markets) => {
                for market in markets {
                    let split: Vec<&str> = market.market_name.split("-").collect();
                    // print!("{:?} ", split);
                    let pair_name = format!("{}{}", *split.last().unwrap(), *split.first().unwrap()); // dangerous, fix
                    // print!("{} ", pair_name);

                    p.insert(
                        pair_name,
                        market.last
                    );
                }

                // match answer {
                //     ::binance::model::Prices::AllPrices(prices) => {
                //         for price in prices {
                //             // println!("{}\t{}", price.symbol.yellow(), price.price);
                //             p.insert(
                //                 price.market_currency,
                //                 price.price.parse::<f64>().unwrap());
                //         }
                //     }
                // }
            },
            Err(e) => println!("Error: {}", e),
        };
        p
    }
}