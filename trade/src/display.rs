#![allow(dead_code)]
#![allow(unused_variables)]

use colored::*;
use trailer::types::*;
use trailer::error::*;

pub fn show_error(error: TrailerError) {
    println!("{}", format!("Error: {}", error.message).red());
}

pub fn colored_balance(num: f64) -> String {
    if num > 0.0 {
        format!("{}{}", num.to_string().green(), "".white())
    } else {
        format!("{}{}", num.to_string().red(), "".white())
    }
}

pub fn show_orders(orders: Vec<Order>) {
    println!("{}", "\nOpen Orders".to_string().yellow());
    for order in orders {
        println!("{:20}\t{:20}\t{:20.8}\t{:20.2}",
            order.symbol, order.order_type, order.price, order.amount);
    }
}

pub fn show_buckets(buckets: Vec<TradeBucket>) {
    println!("{}", "\nTrade Buckets".to_string().yellow());
    println!("{:<20} {:<20} {:<20} {:<20}", "Trades", "Locked Profit", "Buy Avg", "Sell Avg");
    for bucket in buckets {
        println!("{:<20} {:<20.8} {:<20.8} {:<20.8}",
            format!("{}", bucket.trades.len()).white(),
            colored_balance(bucket.profit),
            colored_balance(bucket.average_buy),
            colored_balance(bucket.average_sell)
        );
    }
}

pub fn show_history(history: Vec<Order>) {
    println!("{:?}", history);
}

pub fn show_trades(trades: Vec<Trade>) {
    println!("{}", "\nTrade History".to_string().yellow());

    // let mut balance = 0.0_f64;
    let mut total_cost = 0.0_f64;
    // let mut average_buy_price = 0.0_f64;
    // let mut profit_locked = 0.0_f64;
    // let mut profit_potential = 0.0_f64;


    println!("{:<8} {:<8} {:<20} {:<20} {:<20}", "", "Qty", "Price", "Cost", "Acc Cost");
    // calculate all profits so far
    for trade in group_trades(trades.clone()) {
        let mut buy_display;

        if trade.buy {
            // println!("pre buy: total_cost {} balance {}", total_cost, balance);
            // let transaction_cost = trade.cost * trade.qty;
            total_cost += trade.cost * trade.qty;
            // balance += trade.qty;

            // average_buy_price = total_cost / balance;

            // println!("{}", &format!("\tbalance: {}\t transaction_cost: {}\t total cost: {}\t average_buy_price: {}", balance, transaction_cost, total_cost, average_buy_price).green());

            buy_display = "BUY".to_string().green();
            // println!("{:<8.2} {:<8} {:<20.8} {:<20.8} {:<20.8}", "BUY".to_string().green(), trade.qty, trade.cost, (trade.qty * trade.cost), total_cost);

        } else {
            // let transaction_cost = trade.cost * trade.qty;
            // let profit_in_btc = total_cost - transaction_cost;
            // let profit_ratio = trade.qty / balance;

            // // println!("PROFIT MADE: {:.4} btc, sold {}%", profit_in_btc, (100.0 * profit_ratio) as i32);

            // total_cost = total_cost * (1. - profit_ratio); // reduction of total cost by % of purchase
            // balance -= trade.qty;
            // if balance == 0. {
            //     profit_locked += profit_in_btc;
            // }

            total_cost -= trade.cost * trade.qty;

            // println!("{}", &format!("\tbalance: {}\t profit_ratio: {}\t transaction_cost: -{}\t total cost: {}\t average_buy_price: {}", balance, profit_ratio, transaction_cost, total_cost, average_buy_price).red());

            // println!("{:<8.2} {:<8} {:<20.8} {:<20.8} {:<20.8}", "SELL".to_string().red(), trade.qty, trade.cost, (trade.qty * trade.cost), total_cost);
            buy_display = "SELL".to_string().red();
        }
        // balance = balance + (trade.cost * trade.qty);
        println!("{:<8} {:<8.2} {:<20.8} {:<20.8} {:<20.8}", buy_display, trade.qty, trade.cost, (trade.qty * trade.cost), total_cost);
    }


    let buys:Vec<Trade> = trades.clone().into_iter().filter(|t| t.buy ).collect();
    let sells:Vec<Trade> = trades.clone().into_iter().filter(|t| !t.buy ).collect();
    let profit = sum_qty(sells.clone()) - sum_qty(buys.clone());

    println!("\nAvg Buy Price:   {:<20.8}", average_cost(buys));
    println!("Avg Sell Price:  {:<20.8}", average_cost(sells));
    println!("Total Amount:    {}", sum_qty(trades.clone()));
    println!("Total Cost:      {:.8}", sum_cost(trades.clone()));
}

pub fn show_prices(prices: Prices) {
    println!("Pair");
    for price in prices.clone() {
        println!("{}\t{}", price.0, price.1);
    }
    println!("Total Pairs: {}", prices.len());
}

pub fn show_price(price: Price) {
    println!("{}\t{}", price.0, price.1);
}

// pub fn show_total_profits(funds: Vec<(String, f64, f64)>) {
//     let mut profit = 0.0_f64;

//     for (symbol, total, _locked) in funds {
//     }

//     println!("\nTotal Profit: {}", profit);
// }

pub fn show_funds(funds: Funds, prices: Prices) {
    println!("\nBalances");
    println!("========");

    let btc_price:f64 = match prices.get("BTCUSDT") {
        Some(p) => *p,
        None => 0.0,
    };

    println!("{}", format!("{:8}\t{:8} \t{:8}\t{:16}", "Coin", "Total", "Value BTC", "Current Price"));

    if let Some(btc) = funds.clone().btc {
        println!("{:<8}\t{:<8.2} \t{:<8.2}\t{:<16.8}", "BTC".blue(), btc.amount, (btc_price / btc.amount), btc_price);
    }

    for fiat in funds.clone().fiat {
        println!("{:<8}\t{:<8.2} \t{:<8.2}\t{:<16.8}", fiat.symbol.green(), fiat.amount, fiat.amount, "-");
    }

    for altcoin in funds.clone().alts {
        println!("{:<8}\t{:<8.2} \t{:<8.2}\t{:<16.8}", altcoin.symbol.yellow(), altcoin.amount, altcoin.amount * price_in_btc(altcoin.symbol.clone(), prices.clone()), price_in_btc(altcoin.symbol, prices.clone()));
    }

    println!("\nTotal BTC: {}", funds.total_btc(btc_price, prices.clone()));
    println!("Total USD: {}\n", funds.total_btc(btc_price, prices) * btc_price);
}

// pub fn show_funds(funds: Vec<CoinAsset>, current_prices: Prices) {
//     println!("\nBalances");
//     println!("========");
//     let mut total_btc = 0.0_f64;

//     // println!("{:?}", funds);

//     // let btc_value = current_prices.get("BTC").expect(&format!("BTCUSDT to be present in current prices: {:?}", current_prices));
//     println!("{}", format!("{:8}\t{:16} \t{}\t{}", "Coin", "Total", "Value BTC", "Current Price").bold());

//     for asset in funds {

//         if asset.symbol == "BTC" {
//             *current_prices
//                 .get(&format!("{}BTC", asset.symbol))
//                 .expect(&format!("{}BTC to be present in current prices: {:?}", asset.symbol, current_prices))

//         } else if (asset.symbol == "USDT" || asset.symbol == "USD") {

//         } else {
//             if asset.amount >= 1.0 {

//             }
//         }

//         // let locked_str = if locked > 0.0 {
//         //     format!("({} in orders)", locked)
//         // } else { "".to_string() };

//         if asset.amount >= 1.0 || asset.symbol == "BTC" {

//             let btc_value:f64 = if (asset.symbol != "BTC" && asset.symbol != "USDT") {
//                 *current_prices
//                     .get(&format!("{}BTC", asset.symbol))
//                     .expect(&format!("{}BTC to be present in current prices: {:?}", asset.symbol, current_prices))
//             } else {
//                 *current_prices
//                     .get("BTCUSDT")
//                     .expect(&format!("BTCUSDT to be present in current prices: {:?}", current_prices))
//             };

//             let coin_value_in_btc = if asset.symbol != "BTC" {
//                 total_btc += asset.amount * btc_value;
//                 format!("{:.3} btc", asset.amount * btc_value)
//             } else {
//                 total_btc += asset.amount;
//                 format!("${:.2}", btc_value)
//             };

//             println!("{:8}\t{:16.2} \t{:.3}\t{:.8}", asset.symbol.yellow(), asset.amount, coin_value_in_btc, btc_value);
//         }
//     }
//     println!("\nTotal BTC: {}", total_btc);
// }
