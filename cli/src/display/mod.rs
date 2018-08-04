#![allow(dead_code)]
#![allow(unused_variables)]

use colored::*;
use trailer::models::*;
use trailer::error::*;

pub mod asset;
pub mod funds;
pub mod order;
pub mod position;
pub mod position_accumulated;

pub fn error(error: TrailerError) {
    println!("{}", format!("Error: {}", error.message).red());
}

pub fn title_bar(msg: &str) {
    println!("\n{}", msg);
    println!("====================================================================");
}

pub fn colored_number(num: f64, formatted_string: String) -> ColoredString {
    match num > 0.0 {
        true => formatted_string.green(),
        false => formatted_string.red(),
    }
}

pub fn colored_rsi(num: f64, formatted_string: String) -> ColoredString {
    if num < 30. { return formatted_string.red() }
    if num > 70. { return formatted_string.green() }
    formatted_string.white()
}

pub fn colored_balance(num: f64) -> String {
    if num > 0.0 {
        format!("{:16.8}{ }", num.to_string().green(), "".white())
    } else {
        format!("{:16.8}{ }", num.to_string().red(), "".white())
    }
}

pub fn show_balances(balances: Vec<Asset>) {
    title_bar("Balances");
    println!("{:<20}{:<20}{:<20}", "Symbol", "Amount", "Locked");
    for coin in balances {
        println!("{:<20}{:<20.2}{:<20.2}", coin.symbol.yellow(), coin.amount, coin.locked);
    }
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

pub fn display_price(price: Price) -> String {
    format!("{}\t{}", price.0, price.1)
}

pub fn show_funds(funds: Funds) {
    if let Some(btc) = funds.clone().btc {
        let value_in_usd = btc.value_in_usd.unwrap_or(0.0); // (value_in_usd * 1.0 / btc.amount)
        println!("{:<8}\t{:<8.2} \t{:<8.2}\t{:<16.8}", "BTC".blue(), btc.amount, (value_in_usd * btc.amount), value_in_usd);
    }

    for fiat in funds.clone().fiat {
        println!("{:<8}\t{:<8.2} \t{:<8.2}\t{:<16.8}", fiat.symbol.green(), fiat.amount, fiat.amount, "-");
    }

    for altcoin in funds.clone().alts {
        let value_in_btc = altcoin.value_in_btc.unwrap_or(0.0);
        println!("{:<8}\t{:<8.2} \t{:<8.3}\t{:<16.8}", altcoin.symbol.yellow(), altcoin.amount, (value_in_btc * altcoin.amount), value_in_btc);
    }

    println!("\nTotal value in BTC: {:.3}", funds.total_value_in_btc);
    println!("Total value in USD: {:.3}\n", funds.total_value_in_usd);
}

pub fn show_positions(positions: Vec<Position>, hide_losers: bool) {
    // title_bar(&format!("{}", symbol.yellow()));

    println!("{:12}{:<12}{:<16}{:<16}{:<16}{:<16}{:<16}{:<16}{:<8}",
        "trade_type", "cost_btc", "qty", "sale_price", "cur_price_btc", "cur_price_usd", "p_profit_btc", "p_profit_usd", "% change");

    for position in positions {
        if hide_losers && position.potential_profit_btc <= 0.0 { continue; }
        show_position(position);
    }
}

pub fn show_positions_compact(positions: Vec<Position>, hide_losers: bool) {
    for position in positions {
        println!("{trade_type:<12}{symbol:12}{cost_btc:<12}{percent_change:<8}",
            trade_type                  = position.trade_type.colored_string(),
            symbol                      = position.symbol,
            cost_btc                    = format!("{:.2}",  position.cost_btc),
            percent_change              = colored_number(position.potential_profit_percent,     format!("{:.2}% (${:.2})", position.potential_profit_percent, position.potential_profit_usd)));
    }
}

pub fn show_position(position: Position) {
    println!("{trade_type:<12}{cost_btc:<12}{order_amount:<16}{sale_price:<16}{price:<16}{cost_usd:<16}{potential_profit_btc:<16}{potential_profit_usd:<16}{percent_change:<8}",
        trade_type                  = position.trade_type.colored_string(),
        cost_btc                    = format!("{:.2}",  position.cost_btc),
        order_amount                = format!("{:.2}",  position.qty),
        sale_price                  = format!("{:.8}",  position.sale_price),
        price                       = format!("{:.8}",  position.current_price),
        cost_usd                    = format!("${:.2}", position.cost_btc),
        potential_profit_btc        = colored_number(position.potential_profit_btc,         format!("{:>11.8}", position.potential_profit_btc)),
        potential_profit_usd        = colored_number(position.potential_profit_usd,         format!("${:.2}", position.potential_profit_usd)),
        percent_change              = colored_number(position.potential_profit_percent,     format!("{:.2}%", position.potential_profit_percent)));
}

// pub fn show_position(symbol: String, orders: Vec<trailer::models::Order>, price: f64, btc_price: f64, hide_losers: bool) -> Result<(), TrailerError> {
//     use colored::*;
//     use ::display::colored_number;

//     let positions = trailer::models::Position::calculate(orders, &symbol, price, btc_price);

//     ::display::title_bar(&format!("{}", symbol.yellow()));

//     println!("{:12}{:<12}{:<16}{:<16}{:<16}{:<16}{:<16}{:<16}{:<8}",
//         "trade_type", "cost_btc", "qty", "sale_price", "cur_price_btc", "cur_price_usd", "p_profit_btc", "p_profit_usd", "% change");

//     trailer::models::PositionSum::calculate(positions.clone());
    
//     for position in positions {
//         let potential_profit_usd = position.potential_profit_btc * btc_price;

//         if hide_losers && position.potential_profit_btc <= 0.0 { continue; }

//         println!("{trade_type:<12}{cost_btc:<12}{order_amount:<16}{sale_price:<16}{price:<16}{cost_usd:<16}{potential_profit_btc:<16}{potential_profit_usd:<16}{percent_change:<8}",
//             trade_type                  = position.trade_type.colored_string(),
//             cost_btc                    = format!("{:.2}",  position.cost_btc),
//             order_amount                = format!("{:.2}",  position.qty),
//             sale_price                  = format!("{:.8}",  position.sale_price),
//             price                       = format!("{:.8}",  price),
//             cost_usd                    = format!("${:.2}", position.cost_btc * btc_price),
//             potential_profit_btc        = colored_number(position.potential_profit_btc, format!("{:>11.8}", position.potential_profit_btc)),
//             potential_profit_usd        = colored_number(potential_profit_usd,          format!("${:.2}", potential_profit_usd)),
//             percent_change              = colored_number(position.potential_profit_percent,      format!("{:.2}%", position.potential_profit_percent)));
//     }

//     Ok(())
// }
