#![allow(dead_code)]
#![allow(unused_variables)]

use std::io;
// use std::env;
// use std::thread;

use docopt::Docopt;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const USAGE: &'static str = "
Usage:
  trailer binance funds
  trailer binance ls <coin>
  trailer binance all
  trailer binance trades
  trailer binance orders
  trailer bittrex funds
  trailer bittrex prices
  trailer bot backtest <csv>

Options:
  -h --help     Show this screen.
  --version     Show version.
";

pub fn run_docopt() -> io::Result<()> {
    let args = Docopt::new(USAGE)
        .and_then(|dopt| dopt
            .version(Some(VERSION.to_string()))
            .parse())
        .unwrap_or_else(|e| e.exit());

    let conf = ::config::read();

    if args.get_bool("bot") {
        if args.get_bool("backtest") {
            let bot = ::bot::Bot::load_config("bots/LTC.toml".to_string());
            bot.backtest(vec![]);
        }
    }

    if args.get_bool("bittrex") {

        match conf.bittrex {
            Some(config) => {
                let bittrex = ::exchanges::bittrex::connect(&config.api_key, &config.secret_key);

                if args.get_bool("prices") {
                    println!("getting prices...");
                    let prices = bittrex.prices();

                }

                if args.get_bool("funds") {
                    println!("getting funds...");
                    let funds = bittrex.funds();

                    println!("getting prices...");
                    let prices = bittrex.prices();
                    // println!("{:?}", prices);

                    ::display::show_funds(funds, prices);
                }
            },
            None => {
                println!("No bittrex keys inside .config.toml!");
            }
        }
    }

    if args.get_bool("binance") {

        match conf.binance {
            Some(config) => {
                let binance = ::exchanges::binance::connect(&config.api_key, &config.secret_key);

                if args.get_bool("all") {
                    println!("getting prices...");
                    let prices = binance.prices();

                    ::display::show_prices(prices);
                }

                if args.get_bool("funds") {
                    println!("getting funds...");
                    let funds = binance.funds();

                    println!("getting prices...");
                    let prices = binance.prices();

                    ::display::show_funds(funds, prices);
                }

                if args.get_bool("orders") {
                    println!("getting orders...");
                    let orders = binance.orders(vec!["NEO".to_string()]);

                    // ::display::show_orders(orders);
                }

                // if args.get_bool("trades") {
                //     println!("getting trades...");
                //     let trades = binance.trades();

                //     ::display::show_trades(trades);

                //     // binance.show_all_trades();
                // }

                if args.get_bool("ls") {
                    let coins = args.get_vec("<coin>");

                    for coin in coins {
                        // binance.show_trades(coin);
                        println!("getting trades for {}...", coin);
                        let trades = binance.trades(coin);
                        ::display::show_trades(trades);
                    }
                }
            },
            None => {
                println!("No binance keys inside .config.toml!");
            }
        }
    }

    Ok(())
}
