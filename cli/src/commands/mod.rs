#![allow(dead_code)]
#![allow(unused_variables)]

use trailer;
use trailer::exchanges::*;
use trailer::error::*;

use docopt::Docopt;

mod stop;
mod position;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const USAGE: &'static str = "
Usage:
    trade [<exchange>] funds [--sort-by-value]
    trade [<exchange>] balances
    trade [<exchange>] orders
    trade <exchange> past-orders [<symbol>]
    trade <exchange> prices
    trade <exchange> price <symbol>
    trade <exchange> (buy|sell) [<amount>] [<price>] [--slip=<num>] [--sl=<num>] <symbol>
    trade <exchange> stop (loss|gain) <symbol> [<amount>] [<price>]
    trade <exchange> b <symbol>
    trade <exchange> ev <symbol> [--group] [--limit=<num>] [--hide-losers] [--compact]
    trade <exchange> rsi <symbol>
    trade <exchange> pl <symbol>
    trade <exchange> rsis <pairs>...
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
    cmd_past_orders: bool,
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
    flag_sort_by_value: bool,
    flag_hide_losers: bool,
    flag_compact: bool,
    flag_sl: Option<f64>,
    flag_slip: Option<f64>,
}

pub fn run_docopt() -> Result<String, TrailerError> {
    let args:Args = Docopt::new(USAGE)
        .map(|d| d.version(Some(VERSION.into())))
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let conf = trailer::config::read(args.flag_verbose)?;
    let mut clients = Vec::new();

    fn get_client(exchange: Exchange, keys: trailer::config::APIConfig) -> Result<Box<ExchangeAPI>, TrailerError> {
        Ok(match exchange {
            Exchange::Bittrex => Box::new(trailer::exchanges::bittrex::connect(&keys.api_key, &keys.secret_key)),
            Exchange::Binance => Box::new(trailer::exchanges::binance::connect(&keys.api_key, &keys.secret_key)),
            Exchange::Kucoin  => Box::new(trailer::exchanges::kucoin::connect(&keys.api_key, &keys.secret_key)),
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
            let mut funds = client.funds()?;

            //if args.flag_sort_by_value {
                funds.alts.sort_by(|a, b|
                    (b.value_in_btc.unwrap_or(0.0) * b.amount)
                        .partial_cmp(&(&a.value_in_btc.unwrap_or(0.0) * &a.amount)).unwrap());
            //}

            ::display::title_bar(&format!("\n{} Balance", client.display()));
            ::display::show_funds(funds);
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

        if args.cmd_past_orders {
            if let Some(symbol) = args.arg_symbol.clone() {
                for order in client.past_trades_for(&symbol)? {
                    return Ok(::display::order::row(order));
                }
            } else {
                for order in client.past_orders()? {
                    return Ok(::display::order::row(order));
                }
            }
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
            let amount = args.arg_amount.ok_or(TrailerError::missing_argument("amount"))?;
            // let price = args.arg_price.ok_or(TrailerError::missing_argument("price"))?;

            let price = if let Some(price) = args.arg_price {
                price
            } else {
                let p = client.price_for_symbol(&symbol).unwrap_or(0.0);
                println!("price: {:?}", p);
                ::input::get_f64(p)?
            };

            return Ok(stop::stop(&symbol, amount, price, args.cmd_loss));
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

        if args.cmd_evs {
            let pairs = args.arg_pairs.clone().ok_or(TrailerError::missing_argument("pairs"))?;
            let is_compact = args.flag_compact;

            return Ok(position::positions(client, pairs, is_compact)?);
        }

        if args.cmd_positions {
            let funds = client.funds()?;
            let is_compact = args.flag_compact;

            let pairs = funds.alts.into_iter().map(|fund| format!("{}BTC", fund.symbol)).collect();

            return Ok(position::positions(client, pairs, is_compact)?);


            // let btc_price = client.btc_price()?;

            // for balance in funds.alts {
            //     let price = client.price(&format!("{}BTC", balance.symbol))?;
            //     let orders = trailer::models::average_orders(client.past_trades_for(&format!("{}BTC", balance.symbol))?);
            //     let positions = trailer::models::Position::calculate(orders, price, btc_price, Some(balance.amount));

            //     if let Some(last_position) = positions.last() {
            //         println!("{}", ::display::position::row_compact(last_position.clone()));
            //     } else {
            //         if args.flag_verbose { println!("could not find position for: {}", balance.symbol); }
            //     }

            //     let acc_positions = trailer::models::PositionAccumulated::calculate(positions);
            //     for acc_position in acc_positions {
            //         println!("{}", ::display::position_accumulated::row(acc_position));
            //     }
            // }
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

            let symbol_qty = if let Some(sq) = client.funds()?.alts.iter().find(|c|c.symbol == symbol) {
                Some(sq.amount)
            } else { None };

            let positions = trailer::models::Position::calculate(processed_orders, price, btc_price, symbol_qty);

            let acc_positions = trailer::models::PositionAccumulated::calculate(positions.clone());
            if !args.flag_compact { println!("{}", ::display::position_accumulated::row_header()) };
            for acc_position in acc_positions {
                println!("{}", ::display::position_accumulated::row(acc_position));
            }

            // if args.flag_compact {
            //     positions.into_iter().for_each(|p| println!("{}", ::display::position::row_compact(p)));
            // } else {
            //     positions.into_iter().for_each(|p| println!("{}", ::display::position::row(p)));
            // }
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

        if args.cmd_rsis {
            use colored::*;
            if args.flag_verbose { println!("fetching rsi...") };

            let pairs = args.arg_pairs.clone().ok_or(TrailerError::missing_argument("pairs"))?;

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
