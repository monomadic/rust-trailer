#![allow(dead_code)]
#![allow(unused_variables)]

// Binance rate limits are: 1200 requests per minute; 10 orders per second; 100,000 orders per 24hrs. 

use binance::api::*;
use colored::*;
use binance::account::*;
use binance::market::*;
use std::collections::HashMap;

use ::types::*;

pub struct BinanceAPI {
    account: Account,
    market: Market,
}

pub fn connect(api_key: &str, secret_key: &str) -> BinanceAPI {
    BinanceAPI {
        account: Binance::new(
            Some(api_key.to_string()),
            Some(secret_key.to_string())
        ),
        market: Market::new(None, None),
    }
}

impl BinanceAPI {
    pub fn funds(&self) -> Vec<(String, f64, f64)> {
        let mut funds = Vec::new();

        match self.account.get_account() {
            Ok(answer) => {
                for balance in answer.balances {
                    funds.push((
                        balance.asset,
                        balance.free.parse::<f64>().unwrap() + balance.locked.parse::<f64>().unwrap(),
                        balance.locked.parse::<f64>().unwrap(),
                    ))
                }
            },
            Err(e) => println!("Error: {}", e),
        };

        funds
    }

    pub fn prices(&self) -> HashMap<String, f64> {
        let mut p = HashMap::new();
        match self.market.get_all_prices() {
            Ok(answer) => {
                match answer {
                    ::binance::model::Prices::AllPrices(prices) => {
                        for price in prices {
                            // println!("{}\t{}", price.symbol.yellow(), price.price);
                            p.insert(
                                price.symbol,
                                price.price.parse::<f64>().unwrap());
                        }
                    }
                }
            },
            Err(e) => println!("Error: {}", e),
        };
        p
    }

    pub fn orders(&self, symbols: Vec<String>) -> Vec<Order> {
        let mut orders = Vec::new();

        for symbol in symbols {
            match self.account.get_open_orders(symbol) {
                Ok(o) => {
                    orders.push(Order{});
                },
                Err(e) => println!("Error: {}", e),
            };
        }
        orders
    }

    // pub fn all_trades(&self) {
    //     match self.account.trade_history(coin.into()) {
    //         Ok(answer) => {
    //             println!("\nTrade History: {}", coin);
    //             let mut total = 0.0f64;
    //             for trade in answer {
    //                 // let price = trade.price.parse::<f64>().unwrap();
    //                 let qty = trade.qty.parse::<f64>().unwrap();
    //                 // println!("{:?}", trade);
    //                 if trade.is_buyer {
    //                     total = total + qty;
    //                     println!("+ {:12} {:12} b: {:.2}", trade.qty.green(), trade.price, total);
    //                 } else {
    //                     total = total - qty;
    //                     println!("- {:12} {:12} b: {:.2}", trade.qty.red(), trade.price, total);
    //                 }
    //             }
    //         },
    //         Err(e) => println!("Error: {}", e),
    //     }
    // }

    pub fn trades(&self, coin: &str) -> Vec<Trade> {
        match self.account.trade_history(coin.into()) {
            Ok(answer) => {
                answer.iter().map(|trade| {
                    let cost = trade.price.parse::<f64>().unwrap();
                    let qty = trade.qty.parse::<f64>().unwrap();

                    Trade { cost: cost, qty: qty, buy: trade.is_buyer }
                }).collect()
                // let mut trades = Vec::new();

                // for trade in answer {
                //     let cost = trade.price.parse::<f64>().unwrap();
                //     let qty = trade.qty.parse::<f64>().unwrap();
                //     // println!("{:?}", trade);
                //     if trade.is_buyer {
                //         total_amount = total_amount + qty;
                //         total_cost = total_cost + cost;

                //         total_buy_cost = total_buy_cost + (qty * cost);
                //         total_buy_amount = total_buy_amount + qty;
                //         println!("+ {:12} {:12} b: {:.2}", trade.qty.green(), trade.price, total_amount);
                //     } else {
                //         total_amount = total_amount - qty;
                //         total_cost = total_cost - cost;

                //         total_sell_cost = total_sell_cost + (qty * cost);
                //         total_sell_amount = total_sell_amount + qty;

                //         println!("- {:12} {:12} b: {:.2}", trade.qty.red(), trade.price, total_amount);
                //     }
                // };

                // trades
            },
            Err(e) => {
                println!("Error: {}", e);
                Vec::new()
            },
        }
    }

    pub fn show_trades(&self, coin: &str) {

        match self.account.trade_history(coin.into()) {
            Ok(answer) => {
                println!("\nTrade History: {}", coin);
                let mut total_cost = 0.0_f64;
                let mut total_amount = 0.0f64;
                // let average_buy_price = 0.0_f64;

                let mut total_buy_cost = 0.0_f64;
                let mut total_buy_amount = 0.0_f64;

                let mut total_sell_cost = 0.0_f64;
                let mut total_sell_amount = 0.0_f64;

                for trade in answer {
                    let cost = trade.price.parse::<f64>().unwrap();
                    let qty = trade.qty.parse::<f64>().unwrap();
                    // println!("{:?}", trade);
                    if trade.is_buyer {
                        total_amount = total_amount + qty;
                        total_cost = total_cost + cost;

                        total_buy_cost = total_buy_cost + (qty * cost);
                        total_buy_amount = total_buy_amount + qty;
                        println!("+ {:12} {:12} b: {:.2}", trade.qty.green(), trade.price, total_amount);
                    } else {
                        total_amount = total_amount - qty;
                        total_cost = total_cost - cost;

                        total_sell_cost = total_sell_cost + (qty * cost);
                        total_sell_amount = total_sell_amount + qty;

                        println!("- {:12} {:12} b: {:.2}", trade.qty.red(), trade.price, total_amount);
                    }
                }

                println!("\n{} average buy cost:\n\tall time: {}", coin, format!("{:.8}", total_buy_cost / total_buy_amount).green());
                println!("\n{} average sell cost:\n\tall time: {}", coin, format!("{:.8}", total_sell_cost / total_sell_amount).red());
            },
            Err(e) => println!("Error: {}", e),
        }
    }

    pub fn sell(&self, pair: &str, amount: u32, price: f64) {
        match self.account.limit_sell(pair.to_string(), amount, price) {
            Ok(answer) => println!("{:?}", answer),
            Err(e) => println!("Error: {}", e),
        }
    }

}

pub fn historic() {
    use binance::market::*;
    let market: Market = Binance::new(None, None);

    match market.get_depth("BNBBTC".into()) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match market.get_all_prices() {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn calculate_overall_position(prices: Vec<f64>) -> f64 {
    0.0
}
