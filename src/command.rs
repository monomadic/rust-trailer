#![allow(dead_code)]
#![allow(unused_variables)]

// use std::io;
// use std::env;
// use std::thread;
use exchanges::*;
use error::*;

use docopt::Docopt;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const USAGE: &'static str = "
Usage:
  trade binance funds
  trade binance ls <coin>
  trade binance buckets <coin>
  trade binance all
  trade binance trades
  trade binance price <symbol>
  trade binance buy <pair> <amount> <price>
  trade binance sell <pair> <amount> <price>
  trade binance orders cancel
  trade binance orders ls <pairs>
  trade bittrex funds
  trade bittrex prices
  trade bittrex orders ls
  trade bittrex history
  trade bot run
  trade bot backtest <csv>

  trade caps

Options:
  -h --help     Show this screen.
  --version     Show version.
";

pub fn run_docopt() -> Result<(), TrailerError> {
    let args = Docopt::new(USAGE)
        .and_then(|dopt| dopt
            .version(Some(VERSION.to_string()))
            .parse())
        .unwrap_or_else(|e| e.exit());

    let conf = ::config::read()?;

    if args.get_bool("caps") {
        println!("cap.symbol, cap.volume_usd_24h, cap.market_cap_usd, cap.cap_vs_vol_24h()");
        for cap in ::coinmarketcap::all() {
            println!("{},{:?},{:?},{}", cap.symbol, cap.volume_usd_24h, cap.market_cap_usd, cap.cap_vs_vol_24h());
        }
    }

    if args.get_bool("bot") {
        use ::bot::*;

        if args.get_bool("run") {
            let bot = ::bot::Bot::load_config("bots/LTC.toml".to_string());
            bot.run();
        }

        if args.get_bool("backtest") {
            // let bot = ::bot::Bot::load_config("bots/LTC.toml".to_string());
            let bot = Bot {
                symbol: "FUDCOIN".to_string(),
            };

            println!("loading csv...");
            let data = ::bot::csv::load_backtest_data(args.get_str("<csv>"))?;

            println!("starting bot...");
            bot.backtest(data);
        }

    }

    if args.get_bool("bittrex") {

        match conf.bittrex {
            Some(config) => {
                let bittrex = ::exchanges::bittrex::connect(&config.api_key, &config.secret_key);

                if args.get_bool("prices") {
                    println!("getting prices...");

                    match bittrex.prices() {
                        Ok(prices) => ::display::show_prices(prices),
                        Err(error) => ::display::show_error(error),
                    };
                }

                if args.get_bool("funds") {
                    println!("getting funds...");
                    let funds = bittrex.funds(); // FIX

                    println!("getting prices...");
                    let prices = bittrex.prices()?;

                    ::display::show_prices(prices.clone());
                    ::display::show_funds(::types::sort_funds(funds), prices);                 
                }

                if args.get_bool("orders") {

                    if args.get_bool("ls") {
                        println!("getting orders...");
                        let orders = bittrex.orders();
                        // let orders = bittrex.price();
                        ::display::show_orders(orders);
                    }

                }

                if args.get_bool("history") {
                    println!("getting history...");
                    let history = bittrex.history();

                    ::display::show_orders(history);
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
                    let prices = binance.prices()?;
                    ::display::show_prices(prices);
                }

                if args.get_bool("price") {
                    println!("getting price...");
                    let symbol = args.get_str("<symbol>");

                    let price = binance.price(symbol)?;
                    ::display::show_price((symbol.to_string(), price));
                }

                if args.get_bool("funds") {
                    println!("getting funds...");
                    let funds = binance.funds()?;

                    println!("getting prices...");
                    let prices = binance.prices()?;

                    ::display::show_prices(prices.clone());
                    ::display::show_funds(::types::sort_funds(funds), prices);                 
                }

                if args.get_bool("orders") {
                    if args.get_bool("ls") {
                        println!("getting orders...");
                        let pairs = args.get_vec("pairs");
                        let orders = binance.orders(pairs);
                    }

                    if args.get_bool("cancel") {
                        println!("attempting to cancel orders...");
                        binance.cancel_orders();
                    }
                }

                if args.get_bool("buy") {
                    let symbol = args.get_str("<pair>");
                    let amount = args.get_str("<amount>").parse::<u32>()?;
                    let price = args.get_str("<price>").parse::<f64>()?;

                    let limit_buy = binance.limit_buy(symbol, amount, price)?;
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
                        let price = binance.price(coin);

                        let trades = binance.trades(coin);
                        ::display::show_trades(trades);

                        let orders = binance.orders(vec![coin]);
                        ::display::show_orders(orders);
                    }
                }

                if args.get_bool("buckets") {
                    let coins = args.get_vec("<coin>");

                    for coin in coins {
                        let trades = binance.trades(coin);
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
