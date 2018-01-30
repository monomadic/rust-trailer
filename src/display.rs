#![allow(dead_code)]
#![allow(unused_variables)]

use colored::*;
use std::collections::HashMap;
use ::types::*;

type Prices = HashMap<String, f64>;
type Balance = (String, f64, f64);

pub fn colored_balance(num: f64) -> String {
    if num > 0.0 {
        format!("{}{}", num.to_string().green(), "".clear())
    } else {
        format!("{}{}", num.to_string().red(), "".clear())
    }
}

pub fn show_trades(trades: Vec<Trade>) {
    println!("{}", "\nTrade History".to_string().yellow());

    let mut balance = 0.0_f64;
    let mut total_cost = 0.0_f64;
    let mut average_buy_price = 0.0_f64;
    let mut profit_locked = 0.0_f64;
    // let mut profit_potential = 0.0_f64;

    // calculate all profits so far
    for trade in average_trades(trades) {
        if trade.buy {
            // println!("pre buy: total_cost {} balance {}", total_cost, balance);
            let transaction_cost = trade.cost * trade.qty;
            total_cost += trade.cost * trade.qty;
            balance += trade.qty;

            average_buy_price = total_cost / balance;

            // println!("{}", &format!("\tbalance: {}\t transaction_cost: {}\t total cost: {}\t average_buy_price: {}", balance, transaction_cost, total_cost, average_buy_price).green());

            println!("{}\t {} at ₿{} a total cost of ₿{}", "BUY".to_string().green(), trade.qty, trade.cost, total_cost);

        } else {
            let transaction_cost = trade.cost * trade.qty;
            let profit_in_btc = total_cost - transaction_cost;
            let profit_ratio = trade.qty / balance;

            // println!("PROFIT MADE: {:.4} btc, sold {}%", profit_in_btc, (100.0 * profit_ratio) as i32);

            total_cost = total_cost * (1. - profit_ratio); // reduction of total cost by % of purchase
            balance -= trade.qty;
            if balance == 0. {
                profit_locked += profit_in_btc;
            }

            // println!("{}", &format!("\tbalance: {}\t profit_ratio: {}\t transaction_cost: -{}\t total cost: {}\t average_buy_price: {}", balance, profit_ratio, transaction_cost, total_cost, average_buy_price).red());

            println!("{}\t {} at ₿{} for a profit of ₿{}", "SELL".to_string().red(), trade.qty, trade.cost, profit_in_btc);
        }
        // balance = balance + (trade.cost * trade.qty);
    }

    println!("\nOverall average buy price: {}", colored_balance(average_buy_price));
    println!("Profit:\n\tlocked: {}", colored_balance(profit_locked));
}

pub fn show_prices(prices: Prices) {
    println!("Total Pairs: {}", prices.len());
}

// pub fn show_total_profits(funds: Vec<(String, f64, f64)>) {
//     let mut profit = 0.0_f64;

//     for (symbol, total, _locked) in funds {
//     }

//     println!("\nTotal Profit: {}", profit);
// }

pub fn show_funds(funds: Vec<Balance>, current_prices: Prices) {
    println!("\nBalances");
    println!("========");
    let mut total_btc = 0.0_f64;

    // println!("{:?}", funds);

    // let btc_value = current_prices.get("BTC").expect(&format!("BTCUSDT to be present in current prices: {:?}", current_prices));
    println!("{}", format!("{:8}\t{:16} \t{}\t{}", "Coin", "Total", "Value BTC", "Current Price").bold());

    for (symbol, total, _locked) in funds {

        // let locked_str = if locked > 0.0 {
        //     format!("({} in orders)", locked)
        // } else { "".to_string() };

        if total >= 1.0 || symbol == "BTC" {

            let btc_value:f64 = if symbol != "BTC" {
                *current_prices
                    .get(&format!("{}BTC", symbol))
                    .expect(&format!("{}BTC to be present in current prices: {:?}", symbol, current_prices))
            } else {
                *current_prices
                    .get("BTCUSDT")
                    .expect(&format!("BTCUSDT to be present in current prices: {:?}", current_prices))
            };

            let coin_value_in_btc = if symbol != "BTC" {
                total_btc += total * btc_value;
                format!("{:.3} btc", total * btc_value)
            } else {
                total_btc += total;
                format!("${:.2}", btc_value)
            };

            println!("{:8}\t{:16} \t{}\t{:.8}", symbol.yellow(), total, coin_value_in_btc, btc_value);
        }
    }
    println!("\nTotal BTC: {}", total_btc);
}
