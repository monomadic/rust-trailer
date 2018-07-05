#![allow(dead_code)]
#![allow(unused_variables)]

use trailer;
use trailer::exchanges::*;
use trailer::error::*;

use docopt::Docopt;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const USAGE: &'static str = "
Usage:
    trade [<exchange>] funds [--sort-by-value]
    trade [<exchange>] balances
    trade [<exchange>] orders
    trade <exchange> past-orders [<symbol>]
    trade <exchange> prices
    trade <exchange> price <symbol>
    trade <exchange> (buy|sell) <symbol> <amount> <price>
    trade <exchange> stop (loss|gain) <symbol> <amount> <price>
    trade <exchange> b <symbol>
    trade <exchange> ev <symbol> [--group] [--limit=<num>] [--hide-losers]
    trade <exchange> rsi <symbol>
    trade <exchange> pl <symbol>
    trade <exchange> pos <pairs>...
     <symbol>

Options:
    --verbose   show detailed output

Exchange:
    binance
    bittrex
    kucoin
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_exchange: Option<Exchange>,

    cmd_funds: bool,
    cmd_balances: bool,
    cmd_price: bool,
    cmd_prices: bool,
    cmd_buy: bool,
    cmd_sell: bool,
    cmd_stop: bool,
    cmd_loss: bool,
    cmd_gain: bool,
    cmd_orders: bool,
    cmd_past_orders: bool,
    cmd_b: bool,
    cmd_ev: bool,
    cmd_rsi: bool,
    cmd_pl: bool,
    cmd_pos: bool,

    arg_symbol: Option<String>,
    arg_amount: Option<f64>,
    arg_price: Option<f64>,
    arg_pairs: Option<Vec<String>>,

    flag_group: bool,
    flag_limit: usize,
    flag_verbose: bool,
    flag_sort_by_value: bool,
    flag_hide_losers: bool,
}

pub fn run_docopt() -> Result<String, TrailerError> {
    let args:Args = Docopt::new(USAGE)
        .map(|d| d.version(Some(VERSION.into())))
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let conf = trailer::config::read(args.flag_verbose)?;
    let mut clients = Vec::new();

    fn get_client(exchange: Exchange, keys: trailer::config::APIConfig) -> Box<ExchangeAPI> {
        match exchange {
            Exchange::Bittrex => Box::new(trailer::exchanges::bittrex::connect(&keys.api_key, &keys.secret_key)),
            Exchange::Binance => Box::new(trailer::exchanges::binance::connect(&keys.api_key, &keys.secret_key)),
            Exchange::Kucoin  => Box::new(trailer::exchanges::kucoin::connect(&keys.api_key, &keys.secret_key)),
        }
    };

    if let Some(arg_exchange) = args.arg_exchange {
        // user supplied a specific exchange.
        let exchange_keys = &conf.exchange[&arg_exchange.to_string()];
        clients.push(get_client(arg_exchange, exchange_keys.clone()));
    } else {
        // try to use all exchanges in the config.
        for (exchange, config) in conf.exchange {
            match exchange.parse::<Exchange>() {
                Ok(e) => {
                    clients.push(get_client(e, config));
                },
                Err(e) => { return Err(TrailerError::missing_exchange_adaptor(&exchange)) },
            };
        };
    }

    for client in clients {

        if args.cmd_funds {
            if args.flag_verbose { println!("getting funds...") };
            let mut funds = client.funds()?;

            if args.flag_sort_by_value {
                funds.alts.sort_by(|a, b|
                    (b.value_in_btc.unwrap_or(0.0) * b.amount)
                        .partial_cmp(&(&a.value_in_btc.unwrap_or(0.0) * &a.amount)).unwrap())
            }

            ::display::title_bar(&format!("\n{} Balance", client.display()));
            ::display::show_funds(funds);
        }

        if args.cmd_balances {
            if args.flag_verbose { println!("getting balances...") };
            ::display::show_balances(client.balances()?);
        }

        if args.cmd_orders {
            println!("getting open orders...");
            ::display::show_orders(client.open_orders()?);
        }

        if args.cmd_past_orders {
            if args.flag_verbose { println!("getting past orders...") };
            if let Some(symbol) = args.arg_symbol.clone() {
                ::display::show_orders(client.past_trades_for(&symbol)?);
            } else {
                ::display::show_orders(client.past_orders()?);
            }
        }

        if args.cmd_prices {
            if args.flag_verbose { println!("getting prices...") };
            let prices = client.prices()?;

            println!("{:?}", prices);

            for price in prices { ::display::show_price(price); }
        }

        if args.cmd_price {
            if args.flag_verbose { println!("getting prices...") };
            let symbol = args.arg_symbol.clone().ok_or(TrailerError::missing_argument("symbol"))?;
            let price = client.price(&symbol)?;

            ::display::show_price((symbol, price));
        }

        if args.cmd_stop {
            let symbol = args.arg_symbol.clone().ok_or(TrailerError::missing_argument("symbol"))?;
            let amount = args.arg_amount.ok_or(TrailerError::missing_argument("amount"))?;
            let price = args.arg_price.ok_or(TrailerError::missing_argument("price"))?;

            if args.cmd_buy || args.cmd_sell {
                println!("stop loss/gain");
            }
        }

        if args.cmd_b {
            let symbol = args.arg_symbol.clone().ok_or(TrailerError::missing_argument("symbol"))?;
            let price = client.price(&symbol)?;

            println!("{}:", symbol);
            println!("current price {}\n", price);

            print!("price ({}): ", price);
            let buy_price = ::input::get_f64(price)?;
            if buy_price > price {
                println!("WARNING: your buy price is higher than the current price!");
                print!("\ncontinue with purchase? (y/N) ");
                match ::input::get_confirmation()? {
                    false => { return Err(TrailerError::generic("transaction aborted.")) },
                    _ => (),
                }
            }

            print!("amount in btc (0.1): ");
            let btc_amount = ::input::get_f64(10.)?;
            let amount = (btc_amount / buy_price).round();

            println!("\ncreating limit order of {} {} at {}. total price: {:.8}.", amount, symbol, buy_price, price * amount);
            print!("\ncontinue with purchase? (y/N) ");
            match ::input::get_confirmation()? {
                true => {
                    println!("\npurchasing...");
                    let _ = client.limit_buy(&symbol, amount, buy_price);
                },
                false => println!("\nno purchase made."),
            }

        }

        if args.cmd_ev {
            if args.flag_verbose { println!("evaluating trades...") };

            let symbol = args.arg_symbol.clone().ok_or(TrailerError::missing_argument("symbol"))?;
            let orders = client.past_trades_for(&symbol)?;
            let price = client.price(&symbol)?;
            let btc_price = client.btc_price()?;

            // --group
            let mut processed_orders = match args.flag_group {
                true => trailer::models::average_orders(orders.clone()),
                false => trailer::models::compact_orders(orders.clone()),
            };

            // --limit=<num>
            if args.flag_limit > 0 {
                use trailer::models::Order;
                processed_orders = processed_orders.into_iter().rev().take(args.flag_limit).collect::<Vec<Order>>().into_iter().rev().collect();
            };

            let positions = trailer::models::Position::calculate(processed_orders, &symbol, price, btc_price);
            ::display::show_positions(positions, args.flag_hide_losers);

            // evaluate_trades(symbol, processed_orders, price, btc_price, args.flag_hide_losers)?;
        }

        if args.cmd_rsi {
            use colored::*;

            if args.flag_verbose { println!("fetching rsi...") };
            let symbol = args.arg_symbol.clone().ok_or(TrailerError::missing_argument("symbol"))?;

            let rsi_15m  = rsi(client.chart_data(&symbol, "15m")?);
            let rsi_1h   = rsi(client.chart_data(&symbol, "1h")?);
            let rsi_1d   = rsi(client.chart_data(&symbol, "1d")?);

            println!("{symbol:12}15m: {rsi_15m:<8}1h: {rsi_1h:<8}1d: {rsi_1d:<8}",
                symbol      = symbol.yellow(),
                rsi_15m     = ::display::colored_rsi(*rsi_15m.last().unwrap(), format!("{:.0}", rsi_15m.last().unwrap())),
                rsi_1h      = ::display::colored_rsi(*rsi_1h.last().unwrap(), format!("{:.0}", rsi_1h.last().unwrap())),
                rsi_1d      = ::display::colored_rsi(*rsi_1d.last().unwrap(), format!("{:.0}", rsi_1d.last().unwrap())));
        }

        if args.cmd_pl {
            let symbol = args.arg_symbol.clone().ok_or(TrailerError::missing_argument("symbol"))?;
            let orders = trailer::models::average_orders(client.past_trades_for(&format!("{}BTC", symbol))?);
            let price = client.price(&format!("{}BTC", symbol))?;
            let btc_price = client.btc_price()?;
            let symbol_qty = client.funds()?.alts.iter().find(|c|c.symbol == symbol).ok_or(TrailerError::generic(&format!("symbol not in funds: {:?}", client.funds())))?.amount;

            trade_position(symbol, symbol_qty, orders, price, btc_price)?;
        }

        if args.cmd_pos {
            let pairs = args.arg_pairs.clone().ok_or(TrailerError::missing_argument("pairs"))?;

            use colored::*;

            if args.flag_verbose { println!("fetching rsi...") };

            for pair in pairs {
                let rsi_15m  = rsi(client.chart_data(&pair, "15m")?);
                let rsi_1h   = rsi(client.chart_data(&pair, "1h")?);
                let rsi_1d   = rsi(client.chart_data(&pair, "1d")?);

                println!("{pair:12}15m: {rsi_15m:<8}1h: {rsi_1h:<8}1d: {rsi_1d:<8}",
                    pair        = pair.yellow(),
                    rsi_15m     = ::display::colored_rsi(*rsi_15m.last().unwrap(), format!("{:.0}", rsi_15m.last().unwrap())),
                    rsi_1h      = ::display::colored_rsi(*rsi_1h.last().unwrap(), format!("{:.0}", rsi_1h.last().unwrap())),
                    rsi_1d      = ::display::colored_rsi(*rsi_1d.last().unwrap(), format!("{:.0}", rsi_1d.last().unwrap())));
            }
        }
    };

    Ok(if args.flag_verbose { "done.".to_string() } else { "".to_string() })
}

pub fn rsi(prices: Vec<trailer::models::Candlestick>) -> Vec<f64> {
    use ta::indicators::RelativeStrengthIndex;
    use ta::Next;
    let mut rsi = RelativeStrengthIndex::new(14).unwrap();
    prices.iter().map(|price| rsi.next(price.close_price)).collect()
}

pub fn trade_position(symbol: String, symbol_qty: f64, orders: Vec<trailer::models::Order>, price: f64, btc_price: f64) -> Result<(), TrailerError> {
    use colored::*;
    let mut symbol_balance = symbol_qty;
    let mut btc_balance = 0.0;

    // enum TradePosition {
    //     Entry(EntryPosition),
    //     Exit(ExitPosition),
    // };

    // struct EntryPosition {
    //     cost: f64,
    // };

    // struct ExitPosition {
    //     profit: f64,
    // }

    // // we work backwards from the current balance as there could be missing figures.
    // for order in orders.iter().rev() {
    //     println!("position: bal: {}", symbol_balance);
    //     match order.order_type {
    //         trailer::models::TradeType::Buy => {
    //             // println!("{} +{} {} for {:.2} btc. balance: ( btc: {}, sym: {} )", "BUY ".green(), order.qty, order.symbol, order.qty * order.price, btc_balance, symbol_balance);
    //             println!("{} {}, cost: {}", "BUY ".green(), order.qty, order.qty * order.price);
    //             btc_balance -= order.qty * order.price;
    //             symbol_balance -= order.qty;
    //         },
    //         trailer::models::TradeType::Sell => {
    //             // println!("{} -{} {} for {:.2} btc. balance: ( btc: {}, sym: {} )", "SELL".red(), order.qty, order.symbol, order.qty * order.price, btc_balance, symbol_balance);
    //             btc_balance += order.qty * order.price;
    //             symbol_balance += order.qty;
    //             println!("{} {}, profit: {}", "SELL".red(), order.qty, order.qty * order.price);
    //         },
    //     }
    // }

    let mut last_price = orders.first().unwrap().price;

    println!("{:8}{:<12}{:<16}{:<8}{:<16}{:<16}", "type", "qty", "price", "btc_price", "profit", "profit_usd");

    for order in orders {
        let cost_btc = order.qty * order.price;

        match order.order_type {
            trailer::models::TradeType::Buy => {
                let profit = (last_price - order.price) * order.qty;
                println!("{:8}{:<12.2}{:<16.8}{:<8.2}{:<16.4}{:<16.2}",
                    "BUY".green(), order.qty, order.price, order.qty * order.price, profit, profit * btc_price);
            },
            trailer::models::TradeType::Sell => {
                let profit = (order.price - last_price) * order.qty;
                println!("{:8}{:<12.2}{:<16.8}{:<8.2}{:<16.4}{:<16.2}",
                    "SELL".red(), order.qty, order.price, order.qty * order.price, profit, profit * btc_price);
            }
        }
        last_price = order.price;
    }

    println!("assumed initial bal: {}", symbol_balance);

    // println!("\nbalance:\n\tCOIN {}\n\tBTC  {}", symbol_balance, btc_balance);

    Ok(())
}

// pub fn evaluate_trades(symbol: String, orders: Vec<trailer::models::Order>, price: f64, btc_price: f64, hide_losers: bool) -> Result<(), TrailerError> {
//     use colored::*;
//     use ::display::colored_number;

//     let positions = trailer::models::Position::calculate(orders, &symbol, price, btc_price);

//     ::display::title_bar(&format!("{}", symbol.yellow()));

//     ::display::show_positions(positions);

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

// pub fn _evaluate_trades(symbol: String, orders: Vec<trailer::models::Order>, price: f64, btc_price: f64) -> Result<(), TrailerError> {
//     use colored::*;
//     use trailer::models::{ TradeType };

//     ::display::title_bar(&format!("{}", symbol.yellow()));

//     println!("{:8}{:<8}{:<16}{:<16}{:<16}{:<16}{:<16}{:<16}{:<8}",
//         "type", "btc", "qty", "sale_price", "cur_price_btc", "cur_price_usd", "uprofit", "uprofit usd", "% change");

//     for order in orders {
//         let cost_btc = order.qty * order.price;
//         let cost_usd = (price * order.qty) * btc_price;
//         let percent_change = 100. - 100. / order.price * price;

//         let (profit, buy_type) = match order.order_type {
//             TradeType::Buy => {(
//                 ((order.qty * price) - cost_btc),
//                 ("BUY".green())
//             )},
//             TradeType::Sell => {(
//                 (cost_btc - (order.qty * price)),
//                 ("SELL".red())
//             )},
//         };

//         let profit_usd = profit * btc_price;

//         use ::display::colored_number;
//         println!("{buy_type:<8}{cost_btc:<8}{order_amount:<16}{order_price:<16}{price:<16}{cost_usd:<16}{profit:<16}{profit_usd:<16}{percent_change:<8}",
//             buy_type        = buy_type,
//             cost_btc        = format!("{:.2}", cost_btc),
//             order_amount    = format!("{:.2}", order.qty),
//             order_price     = format!("{:.8}", order.price),
//             price           = format!("{:.8}", price),
//             cost_usd        = format!("${:.2}", cost_usd),
//             profit          = colored_number(profit,            format!("{:>11.8}", profit)),
//             profit_usd      = colored_number(profit_usd,        format!("${:.2}", profit_usd)),
//             percent_change  = colored_number(percent_change,    format!("{:.2}%", percent_change)));
//     }

//     Ok(())
// }
