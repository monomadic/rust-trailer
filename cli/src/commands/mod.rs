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
mod rsi;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const USAGE: &'static str = "
Usage:
    trade [<exchange>] funds [--sort-by-name] [--log]
    trade [<exchange>] balances
    trade [<exchange>] orders
    trade <exchange> trades [<symbol>] [--group]
    trade <exchange> prices
    trade <exchange> price <symbol>
    trade <exchange> (buy|sell) [<amount>] [<price>] [--slip=<num>] [--sl=<num>] <symbol>
    trade <exchange> stop (loss|gain) <symbol> [<amount>] [<price>]
    trade <exchange> b <symbol>
    trade <exchange> dump <symbol>
    trade <exchange> rsi [--watch] [--all] [--triple] [<pairs>...]
    trade <exchange> mrsi <symbol> [<periods>...]
    trade <exchange> positions [--watch] [--all] [--orders] [<pairs>...]

Options:
    --debug   show debug object output

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
    cmd_rsi: bool,
    cmd_mrsi: bool,
    cmd_positions: bool,

    arg_symbol:     Option<String>,
    arg_amount:     Option<f64>,
    arg_price:      Option<f64>,
    arg_pairs:      Option<Vec<String>>,
    arg_periods:    Option<String>,

    flag_debug: bool,
    flag_group: bool,
    flag_log: bool,
    flag_sort_by_name: bool,
    flag_sl: Option<f64>,
    flag_slip: Option<f64>,
    flag_watch: bool,
    flag_all: bool,
    flag_orders: bool,
}

use std::sync::Arc;

pub fn run_docopt() -> Result<String, TrailerError> {
    let args:Args = Docopt::new(USAGE)
        .map(|d| d.version(Some(VERSION.into())))
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let conf = trailer::config::read(args.flag_debug)?;
    let mut clients = Vec::new();

    fn get_client(exchange: Exchange, keys: trailer::config::APIConfig) -> Result<Arc<ExchangeAPI+Send+Sync>, TrailerError> {
        Ok(match exchange {
            // Exchange::Bittrex => Arc::new(trailer::exchanges::bittrex::connect(&keys.api_key, &keys.secret_key)),
            Exchange::Binance => Arc::new(trailer::exchanges::binance::connect(&keys.api_key, &keys.secret_key)),
            // Exchange::Kucoin  => Arc::new(trailer::exchanges::kucoin::connect(&keys.api_key, &keys.secret_key)),
            _ => { return Err(TrailerError::missing_exchange_adaptor(&exchange.to_string())); },
        })
    };

    if let Some(arg_exchange) = args.arg_exchange {
        // user supplied a specific exchange.
        let config = &conf.exchange[&arg_exchange.to_string()];
        clients.push(
            (get_client(arg_exchange, config.clone())?, config.clone())
        );
    } else {
        // try to use all exchanges in the config.
        for (exchange, config) in conf.exchange {
            match exchange.parse::<Exchange>() {
                Ok(e) => {
                    clients.push((get_client(e, config.clone())?, config.clone()));
                },
                Err(e) => { return Err(TrailerError::missing_exchange_adaptor(&exchange)) },
            };
        };
    }

    for (client, config) in clients {

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
            ::display::funds::show_funds(funds.clone());

            if args.flag_log { ::log::log_funds(funds)? };
        }

        if args.cmd_balances {
            let balances = client.balances()?;
            if args.flag_debug { println!("{:#?}", balances); }

            return Ok(balances.into_iter().map(|asset|
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
                client.trades_for(&symbol)?
            } else {
                client.past_orders()?
            };

            if args.flag_group {
                orders = trailer::models::average_orders(orders.clone());
            }

            return Ok(orders.into_iter().map(|order| ::display::order::row(order)).collect::<Vec<String>>().join(""))
        }

        if args.cmd_prices {
            let mut prices = client.prices()?;
            
            return Ok(
                prices.iter_mut().map({|price|
                    ::display::display_price((price.0.to_string(), *price.1))}).collect::<Vec<String>>().join(""));
        }

        if args.cmd_price {
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

        if args.cmd_positions {
            let pairs:Vec<String> = args.arg_pairs.unwrap_or({ // grab positions from args, or...
                if args.flag_watch { // if --watch
                    config.positions.unwrap_or({ // config, or...
                        let funds = client.funds()?; // wallet.
                        funds.alts.into_iter().map(|fund| format!("{}BTC", fund.symbol)).collect()
                    })
                } else {
                    let funds = client.funds()?; // wallet.
                    funds.alts.into_iter().map(|fund| format!("{}BTC", fund.symbol)).collect()
                }
            });

            let mut positions = position::positions(client, pairs.clone(), args.flag_all)?;
            // let ok_positions = positions.into_iter().filter_map(|e| e.ok()).collect();

            if pairs.len() != 1 {
                positions.sort_by(|a, b| {
                    match a {
                        Ok(a) => match b {
                            Ok(b) => b.percent_change().partial_cmp(&a.percent_change()).expect("sort failed"),
                            _ => ::std::cmp::Ordering::Less,
                        },
                        _ => ::std::cmp::Ordering::Less,
                    }
                });
            }

            let mut output_buffer = String::new();
            for position in positions.clone() {
                match position {
                    Ok(position) => {
                        output_buffer.push_str(&::display::position::row_compact(position.clone()));
                        if args.flag_orders {
                            for order in position.position.orders.clone() {
                                output_buffer.push_str(&format!("  > {}", ::display::order::row(order.clone())));
                            }
                        }
                    },
                    Err(err) => output_buffer.push_str(&err),
                }
            }

            let position_presenters = positions.into_iter().filter(|r|r.is_ok()).map(|r|r.unwrap()).collect();
            output_buffer.push_str(&format!("\nTotal BTC Staked: {:.3}", ::trailer::presenters::total_btc_staked(position_presenters)));

            // output_buffer.push_str(&::display::position::total(positions.clone()));

            return Ok(output_buffer);
        }

        if args.cmd_rsi {
            let pairs:Vec<String> = if args.flag_all {
                let pairs = client.prices()?;
                pairs.into_iter().map(|(k,v)| k).filter(|s| s.contains("BTC")).collect()
            } else {
                args.arg_pairs.clone().unwrap_or({ // grab positions from args, or...
                    if args.flag_watch { // if --watch
                        config.watch.unwrap_or({ // config, or...
                            let funds = client.funds()?; // wallet.
                            funds.alts.into_iter().map(|fund| format!("{}BTC", fund.symbol)).collect()
                        })
                    } else {
                        let funds = client.funds()?; // wallet.
                        funds.alts.into_iter().map(|fund| format!("{}BTC", fund.symbol)).collect()
                    }
                })
            };

            let rsi_values:Vec<(String, Vec<f64>)> = indicators::rsi_from_chart_data(14, ::trailer::threadpool::chart_data(client.clone(), pairs.clone(), "15m"));
            let rsi_values = indicators::sort_by_last_value(rsi_values);
            ::display::rsi::rsi(rsi_values);
        }

        if args.cmd_mrsi {
            let periods = args.arg_periods.clone();
            let pair = args.arg_symbol.clone().unwrap();

            let rsi_values:Vec<f64> = periods.into_iter().map(|period:String| {
                *indicators::rsi(14,
                    &client.chart_data(&pair.clone(), &period).unwrap()
                        .into_iter()
                        .map(|c| c.close_price)
                        .collect()
                ).last().unwrap()
            }).collect();

            // print!("{:16}", "");
            // for period in periods {
            //     print!("{:6}", period);
            // }
            // print!("\n");

            print!("{:16}", pair);

            for rsi_value in rsi_values {
                print!("{}:{} ", "xx", rsi_value);
            }
        }
    };

    Ok(if args.flag_debug { "done.".to_string() } else { "".to_string() })
}
