#![allow(dead_code)]
#![allow(unused_variables)]

use std::io;
// use std::env;
// use std::thread;

use docopt::Docopt;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const USAGE: &'static str = "
Usage:
  trade binance funds
  trade binance ls <coin>
  trade binance buckets <coin>
  trade binance all
  trade binance trades
  trade binance orders
  trade bittrex funds
  trade bittrex prices
  trade bittrex orders
  trade bot run
  trade bot backtest <csv>
  trade caps

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

    if args.get_bool("caps") {
        println!("cap.symbol, cap.volume_usd_24h, cap.market_cap_usd, cap.cap_vs_vol_24h()");
        for cap in ::coinmarketcap::all() {
            println!("{},{:?},{:?},{}", cap.symbol, cap.volume_usd_24h, cap.market_cap_usd, cap.cap_vs_vol_24h());
        }
    }

    if args.get_bool("bot") {

        if args.get_bool("run") {
            let bot = ::bot::Bot::load_config("bots/LTC.toml".to_string());
            bot.run();
        }

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

                if args.get_bool("orders") {
                    println!("getting orders...");
                    let orders = bittrex.orders();

                    ::display::show_orders(orders);
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
                        // println!("getting trades for {}...", coin);
                        let trades = binance.trades(coin);
                        ::display::show_trades(trades);

                        let orders = binance.orders(vec![coin.to_string()]);
                        ::display::show_orders(orders);
                    }
                }

                if args.get_bool("buckets") {
                    let coins = args.get_vec("<coin>");

                    for coin in coins {
                        // binance.show_trades(coin);
                        // println!("getting trades for {}...", coin);
                        let trades = binance.trades(coin);
                        // ::display::show_trades(trades);

                        // let orders = binance.orders(vec![coin.to_string()]);
                        // ::display::show_orders(orders);

                        let buckets = ::types::trade_buckets(trades);
                        ::display::show_buckets(buckets);
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
