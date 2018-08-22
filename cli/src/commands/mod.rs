#![allow(dead_code)]
#![allow(unused_variables)]

use trailer;
use trailer::exchanges::*;
use trailer::error::*;
use trailer::presenters::*;
use trailer::indicators;

use docopt::Docopt;

mod stop;
mod position;
mod buy_sell;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const USAGE: &'static str = "
Usage:
    trade [<exchange>] funds [--sort-by-name]
    trade [<exchange>] balances
    trade [<exchange>] orders
    trade <exchange> trades [<symbol>] [--group]
    trade <exchange> prices
    trade <exchange> price <symbol>
    trade <exchange> (buy|sell) [<amount>] [<price>] [--slip=<num>] [--sl=<num>] <symbol>
    trade <exchange> stop (loss|gain) <symbol> [<amount>] [<price>]
    trade <exchange> b <symbol>
    trade <exchange> ev <symbol> [--group] [--limit=<num>] [--hide-losers] [--compact] [--historic]
    trade <exchange> rsi <symbol>
    trade <exchange> rsis <pairs>...
    trade <exchange> pl <symbol>
    trade <exchange> evs [--compact] <pairs>...
    trade <exchange> positions

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
    cmd_trades: bool,
    cmd_b: bool,
    cmd_ev: bool,
    cmd_evs: bool,
    cmd_rsi: bool,
    cmd_pl: bool,
    cmd_rsis: bool,
    cmd_positions: bool,

    arg_symbol: Option<String>,
    arg_amount: Option<f64>,
    arg_price: Option<f64>,
    arg_pairs: Option<Vec<String>>,

    flag_group: bool,
    flag_limit: usize,
    flag_verbose: bool,
    flag_sort_by_name: bool,
    flag_hide_losers: bool,
    flag_compact: bool,
    flag_sl: Option<f64>,
    flag_slip: Option<f64>,
    flag_historic: bool,
}

use std::sync::Arc;

pub fn run_docopt() -> Result<String, TrailerError> {
    let args:Args = Docopt::new(USAGE)
        .map(|d| d.version(Some(VERSION.into())))
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let conf = trailer::config::read(args.flag_verbose)?;
    let mut clients = Vec::new();



    fn get_client(exchange: Exchange, keys: trailer::config::APIConfig) -> Result<Arc<ExchangeAPI+Send+Sync>, TrailerError> {
        Ok(match exchange {
            Exchange::Bittrex => Arc::new(trailer::exchanges::bittrex::connect(&keys.api_key, &keys.secret_key)),
            Exchange::Binance => Arc::new(trailer::exchanges::binance::connect(&keys.api_key, &keys.secret_key)),
            Exchange::Kucoin  => Arc::new(trailer::exchanges::kucoin::connect(&keys.api_key, &keys.secret_key)),
            _ => { return Err(TrailerError::missing_exchange_adaptor(&exchange.to_string())); },
        })
    };

    if let Some(arg_exchange) = args.arg_exchange {
        // user supplied a specific exchange.
        let exchange_keys = &conf.exchange[&arg_exchange.to_string()];
        clients.push(get_client(arg_exchange, exchange_keys.clone())?);
    } else {
        // try to use all exchanges in the config.
        for (exchange, config) in conf.exchange {
            match exchange.parse::<Exchange>() {
                Ok(e) => {
                    clients.push(get_client(e, config)?);
                },
                Err(e) => { return Err(TrailerError::missing_exchange_adaptor(&exchange)) },
            };
        };
    }

    for client in clients {

        if args.cmd_funds {
            let mut prices = client.prices()?;
            // let btc_price = *prices.get(&client.btc_symbol()).unwrap_or(&0.0);
            let btc_price = *prices.get("BTCUSDT").expect("btc price not found."); // fix this with exchange agnostic value

            let mut funds = FundsPresenter::new(client.funds()?, prices, btc_price);

            if args.flag_sort_by_name {
                funds.alts.sort_by(|a, b| b.value_in_btc.partial_cmp(&a.value_in_btc).expect("order failed"));
            } else {
                funds.alts.sort_by(|a, b| b.value_in_btc.partial_cmp(&a.value_in_btc).expect("order failed"));
            }

            ::display::title_bar(&format!("{} Balance", client.display()));
            ::display::funds::show_funds(funds);
        }

        if args.cmd_balances {
            if args.flag_verbose { println!("getting balances...") };
            return Ok(client.balances()?.into_iter().map(|asset|
                ::display::asset::row(asset)).collect::<Vec<String>>().join("\n"));
        }

        if args.cmd_buy || args.cmd_sell {
            let symbol = args.arg_symbol.clone().ok_or(TrailerError::missing_argument("symbol"))?;
            let amount = args.arg_amount.ok_or(TrailerError::missing_argument("amount"))?;
            let price = args.arg_price.ok_or(TrailerError::missing_argument("price"))?;

            if let Some(stoploss) = args.flag_sl {
                println!("added stoploss: {}", price * stoploss);
            }

            if let Some(slip) = args.flag_slip {
                println!("added slip: {}", slip);
            }

            return Ok(format!("buy: {:?}", (symbol, amount, price)));
        }

        if args.cmd_orders {
            return Ok(client.open_orders()?.into_iter().map({ |order|
                ::display::order::row(order)
            }).collect::<Vec<String>>().join(""))
        }

        if args.cmd_trades {
            let mut orders:Vec<::trailer::models::Order> = if let Some(symbol) = args.arg_symbol.clone() {
                client.past_trades_for(&symbol)?
            } else {
                client.past_orders()?
            };

            if args.flag_group {
                println!("grouping...");
                orders = trailer::models::average_orders(orders.clone());
            }

            return Ok(orders.into_iter().map(|order| ::display::order::row(order)).collect::<Vec<String>>().join(""))
        }

        if args.cmd_prices {
            if args.flag_verbose { println!("getting prices...") };
            let mut prices = client.prices()?;
            
            return Ok(
                prices.iter_mut().map({|price|
                    ::display::display_price((price.0.to_string(), *price.1))}).collect::<Vec<String>>().join(""));
        }

        if args.cmd_price {
            if args.flag_verbose { println!("getting prices...") };
            let symbol = args.arg_symbol.clone().ok_or(TrailerError::missing_argument("symbol"))?;
            let price = client.price(&symbol)?;

            return Ok(::display::display_price((symbol, price)));
        }

        if args.cmd_stop {
            let symbol = args.arg_symbol.clone().ok_or(TrailerError::missing_argument("symbol"))?;
            return stop::stop(client, &symbol, args.arg_amount, args.arg_price, args.cmd_loss);
        }

        if args.cmd_b {
            let symbol = args.arg_symbol.clone().ok_or(TrailerError::missing_argument("symbol"))?;
            let price = client.price(&symbol)?;

            return Ok(buy_sell::buy_sell(client, &symbol, price)?);
        }

        if args.cmd_evs {
            let pairs = args.arg_pairs.clone().ok_or(TrailerError::missing_argument("pairs"))?;
            let is_compact = args.flag_compact;

            return Ok(position::positions(client, pairs, is_compact)?);
        }

        if args.cmd_positions {
            let funds = client.funds()?;
            let is_compact = args.flag_compact;
            let pairs = funds.alts.into_iter().map(|fund| format!("{}BTC", fund.symbol)).collect();

            let mut output_buffer = String::new();

            // if !args.flag_compact { println!("{}", &::display::position_accumulated::row_header()); }
            output_buffer.push_str(&position::positions(client, pairs, is_compact)?);

            return Ok(output_buffer);
        }

        if args.cmd_ev {
            let symbol = args.arg_symbol.clone().ok_or(TrailerError::missing_argument("symbol"))?;
            let is_compact = args.flag_compact;
            let mut symbols = Vec::new();
            symbols.push(symbol.clone());

            if args.flag_historic {
                return Ok(position::position_historic(client, &symbol)?);
            } else {
                return Ok(position::positions(client, symbols, is_compact)?);
            }



            // if args.flag_verbose { println!("evaluating trades...") };

            // let symbol = args.arg_symbol.clone().ok_or(TrailerError::missing_argument("symbol"))?;
            // let orders = client.past_trades_for(&symbol)?;
            // let price = client.price(&symbol)?;
            // let btc_price = client.btc_price()?;

            // // --group
            // let mut processed_orders = match args.flag_group {
            //     true => trailer::models::average_orders(orders.clone()),
            //     false => trailer::models::compact_orders(orders.clone()),
            // };

            // // --limit=<num>
            // if args.flag_limit > 0 {
            //     use trailer::models::Order;
            //     processed_orders = processed_orders.into_iter().rev().take(args.flag_limit).collect::<Vec<Order>>().into_iter().rev().collect();
            // };

            // let symbol_qty = if let Some(sq) = client.funds()?.alts.iter().find(|c|c.symbol == symbol) {
            //     Some(sq.amount)
            // } else { None };

            // let positions = trailer::models::Position::calculate(processed_orders, price, btc_price, symbol_qty);

            // let acc_positions = trailer::models::PositionAccumulated::calculate(positions.clone());
            // if !args.flag_compact { println!("{}", ::display::position_accumulated::row_header()) };
            // for acc_position in acc_positions {
            //     println!("{}", ::display::position_accumulated::row(acc_position));
            // }

            // if args.flag_compact {
            //     positions.into_iter().for_each(|p| println!("{}", ::display::position::row_compact(p)));
            // } else {
            //     positions.into_iter().for_each(|p| println!("{}", ::display::position::row(p)));
            // }
        }

        if args.cmd_rsi {
            use colored::*;
            let symbol = args.arg_symbol.clone().ok_or(TrailerError::missing_argument("symbol"))?;

            let rsi_15m:Vec<f64> = client.chart_data(&symbol, "15m")?.into_iter().map(|price| price.close_price).collect();
            let rsi_1h:Vec<f64>  = client.chart_data(&symbol, "1h")?.into_iter().map(|price| price.close_price).collect();
            let rsi_1d:Vec<f64> = client.chart_data(&symbol, "1d")?.into_iter().map(|price| price.close_price).collect();

            let rsi_15m = indicators::rsi(14, &rsi_15m);
            let rsi_1h   = indicators::rsi(14, &rsi_1h);
            let rsi_1d   = indicators::rsi(14, &rsi_1d);

            println!("{symbol:12} {rsi_15m:<8} | {rsi_1h:<8} | {rsi_1d:<8}",
                symbol      = symbol.yellow(),
                rsi_15m     = ::display::colored_rsi(*rsi_15m.last().unwrap(), format!("{:.0}", rsi_15m.last().unwrap())),
                rsi_1h      = ::display::colored_rsi(*rsi_1h.last().unwrap(), format!("{:.0}", rsi_1h.last().unwrap())),
                rsi_1d      = ::display::colored_rsi(*rsi_1d.last().unwrap(), format!("{:.0}", rsi_1d.last().unwrap())));


            ::graphs::rsi::draw(rsi_15m);
        }

        if args.cmd_pl {
            let symbol = args.arg_symbol.clone().ok_or(TrailerError::missing_argument("symbol"))?;
            let orders = trailer::models::average_orders(client.past_trades_for(&format!("{}BTC", symbol))?);
            let price = client.price(&format!("{}BTC", symbol))?;
            let btc_price = client.btc_price()?;
            let symbol_qty = client.funds()?.alts.iter().find(|c|c.symbol == symbol).ok_or(TrailerError::generic(&format!("symbol not in funds: {:?}", client.funds())))?.amount;

            trade_position(symbol, symbol_qty, orders, price, btc_price)?;
        }

        if args.cmd_rsis {
            use colored::*;
            use std::thread;

            let pairs = args.arg_pairs.clone().ok_or(TrailerError::missing_argument("pairs"))?;
            // let mut output:Arc<Vec<String>> = Arc::new(Vec::new());

            let mut threads = Vec::new();

            for pair in pairs {
                let client = Arc::clone(&client);
                // let mut output = Arc::clone(&output);

                threads.push(thread::spawn(move || {

                    use colored::*;
                    // let symbol = args.arg_symbol.clone().ok_or(TrailerError::missing_argument("symbol"))?;

                    let rsi_15m:Vec<f64> = client.chart_data(&pair, "15m").expect("rsi to work").into_iter().map(|price| price.close_price).collect();
                    let rsi_1h:Vec<f64>  = client.chart_data(&pair, "1h").expect("rsi to work").into_iter().map(|price| price.close_price).collect();
                    let rsi_1d:Vec<f64> = client.chart_data(&pair, "1d").expect("rsi to work").into_iter().map(|price| price.close_price).collect();

                    let rsi_15m = indicators::rsi(14, &rsi_15m);
                    let rsi_1h   = indicators::rsi(14, &rsi_1h);
                    let rsi_1d   = indicators::rsi(14, &rsi_1d);

                    println!("{symbol:12} {rsi_15m:<2} | {rsi_1h:<2} | {rsi_1d:<2}",
                        symbol      = pair.yellow(),
                        rsi_15m     = ::display::colored_rsi(*rsi_15m.last().unwrap(), format!("{:.0}", rsi_15m.last().unwrap())),
                        rsi_1h      = ::display::colored_rsi(*rsi_1h.last().unwrap(), format!("{:.0}", rsi_1h.last().unwrap())),
                        rsi_1d      = ::display::colored_rsi(*rsi_1d.last().unwrap(), format!("{:.0}", rsi_1d.last().unwrap())));


                    // let rsi_15m = rsi(client.chart_data(&pair, "15m").expect("rsi to work"));
                    // let rsi_1h   = rsi(client.chart_data(&pair, "1h").expect("rsi to work"));
                    // let rsi_1d   = rsi(client.chart_data(&pair, "1d").expect("rsi to work"));

                    // println!("{pair:12}15m: {rsi_15m:<8}1h: {rsi_1h:<8}1d: {rsi_1d:<8}",
                    //         pair        = pair.yellow(),
                    //         rsi_15m     = ::display::colored_rsi(*rsi_15m.last().unwrap(), format!("{:.0}", rsi_15m.last().unwrap())),
                    //         rsi_1h      = ::display::colored_rsi(*rsi_1h.last().unwrap(), format!("{:.0}", rsi_1h.last().unwrap())),
                    //         rsi_1d      = ::display::colored_rsi(*rsi_1d.last().unwrap(), format!("{:.0}", rsi_1d.last().unwrap()))
                    // )

                    // output.push(
                    //     format!("{pair:12}15m: {rsi_15m:<8}1h: {rsi_1h:<8}1d: {rsi_1d:<8}",
                    //             pair        = pair.yellow(),
                    //             rsi_15m     = ::display::colored_rsi(*rsi_15m.last().unwrap(), format!("{:.0}", rsi_15m.last().unwrap())),
                    //             rsi_1h      = ::display::colored_rsi(*rsi_1h.last().unwrap(), format!("{:.0}", rsi_1h.last().unwrap())),
                    //             rsi_1d      = ::display::colored_rsi(*rsi_1d.last().unwrap(), format!("{:.0}", rsi_1d.last().unwrap()))
                    //     )
                    // );
                }));
            }

            for thread in threads { thread.join().expect("threading failed"); }
        }
    };

    Ok(if args.flag_verbose { "done.".to_string() } else { "".to_string() })
}

// pub fn rsi(prices: Vec<trailer::models::Candlestick>) -> Vec<f64> {
//     // use ta::indicators::RelativeStrengthIndex;
//     // use ta::Next;

//     // let mut rsi = RelativeStrengthIndex::new(14).unwrap();
//     // prices.iter().map(|price| rsi.next(price.close_price)).collect()

//     use ta_lib_wrapper::{TA_Integer, TA_Real, TA_RSI,  TA_RetCode};

//     let close_prices = 
// }

pub fn trade_position(symbol: String, symbol_qty: f64, orders: Vec<trailer::models::Order>, price: f64, btc_price: f64) -> Result<(), TrailerError> {
    use colored::*;
    let symbol_balance = symbol_qty;
    let btc_balance = 0.0;
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
    Ok(())
}
