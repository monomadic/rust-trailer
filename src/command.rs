#![allow(dead_code)]
#![allow(unused_variables)]

use exchanges::*;
use error::*;

use docopt::Docopt;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const USAGE: &'static str = "
Usage:
    trade <exchange> funds
    trade <exchange> price <symbol>
    trade <exchange> (buy|sell) <symbol> <amount> <price>

Exchange:
    binance
    bittrex
";


//     trade (binance|bittrex|cob) funds
//     trade (binance|bittrex|cob) price <symbol>
//     trade (binance|bittrex|cob) (buy|sell) <symbol> <amount> <price>

//     trade binance funds
//     trade binance ls <coin>
//     trade binance buckets <coin>
//     trade binance all
//     trade binance trades
//     trade binance price <symbol>
//     trade binance buy <pair> <amount> <price>
//     trade binance sell <pair> <amount> <price>
//     trade binance orders cancel
//     trade binance orders ls <pairs>
//     trade bittrex funds
//     trade bittrex prices
//     trade bittrex orders ls
//     trade bittrex history
//     trade cob time
//     trade cob funds
//     trade bot run
//     trade bot backtest <csv>
//     trade caps

// Options:
//   -h --help     Show this screen.
//   --version     Show version.
// ";

// #[derive(Deserialize, Debug)]
// enum Exchange { Binance, Bittrex }

// #[derive(Debug, Deserialize)]
// struct Args {
//     cmd_binance: bool,
//     cmd_bittrex: bool,
//     cmd_cob: bool,
//     cmd_funds: bool,
//     cmd_price: bool,
//     arg_symbol: String,
//     arg_amount: Option<f64>,
//     arg_price: Option<f64>,
//     cmd_buy: bool,
//     cmd_sell: bool,
// }

#[derive(Debug, Deserialize)]
struct Args {
    arg_exchange: Exchange,
    cmd_funds: bool,
    cmd_price: bool,
    cmd_buy: bool,
    cmd_sell: bool,
    arg_symbol: Option<String>,
    arg_amount: Option<f64>,
    arg_price: Option<f64>,
}

#[derive(Debug, Deserialize)]
enum Exchange {
    Bittrex,
    Binance,
}

pub fn run_docopt() -> Result<(), TrailerError> {
    let args:Args = Docopt::new(USAGE)
        .map(|d| d.version(Some(VERSION.into())))
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    // println!("{:?}", args);

    let conf = ::config::read()?;

    let keys = match args.arg_exchange {
        Exchange::Bittrex => conf.bittrex.ok_or(TrailerError::missing_config_keys("bittrex"))?,
        Exchange::Binance => conf.binance.ok_or(TrailerError::missing_config_keys("binance"))?,
    };

    let client:Box<ExchangeAPI> = match args.arg_exchange {
        Exchange::Bittrex => Box::new(::exchanges::bittrex::connect(&keys.api_key, &keys.secret_key)),
        Exchange::Binance => Box::new(::exchanges::binance::connect(&keys.api_key, &keys.secret_key)),
    };

    if args.cmd_funds {
        println!("getting funds...");
        let funds = client.funds()?;

        println!("getting prices...");
        let prices = client.prices()?;

        ::display::show_funds(::types::sort_funds(funds), prices);
    }

    if args.cmd_price {
        println!("getting price...");
        let symbol = args.arg_symbol.clone().ok_or(TrailerError::missing_argument("symbol"))?;
        let price = client.price(&symbol)?;

        ::display::show_price((symbol, price));
    }

    if args.cmd_buy || args.cmd_sell {
        let symbol = args.arg_symbol.ok_or(TrailerError::missing_argument("symbol"))?;
        let amount = args.arg_amount.ok_or(TrailerError::missing_argument("amount"))?;
        let price = args.arg_price.ok_or(TrailerError::missing_argument("price"))?;

        if args.cmd_buy {
            let limit_sell = client.limit_buy(&symbol, amount as u32, price)?;
        } else if args.cmd_sell {
            let limit_sell = client.limit_sell(&symbol, amount as u32, price)?;
        }
    }


        // let price = args.arg_price.ok_or(TrailerError::missing_argument("price"))?;
    // if args.cmd_binance || args.cmd_bittrex {
    //     // read config
    //     let conf = ::config::read()?;
    //     let client = if args.cmd_binance { get_client(Exchange::Binance, conf.binance)? }
    //         else { get_client(Exchange::Bittrex, conf.bittrex)? };

    //     if args.cmd_funds {
    //         println!("getting funds...");
    //         let funds = client.funds()?;

    //         println!("getting prices...");
    //         let prices = client.prices()?;

    //         ::display::show_funds(::types::sort_funds(funds), prices);
    //     }

    //     if args.cmd_price {
    //         println!("getting price...");
    //         let price = client.price(&args.arg_symbol)?;
    //         ::display::show_price((args.arg_symbol.clone(), price));
    //     }

    //     if args.cmd_buy {

    //         let amount = args.arg_amount.unwrap_or(|| { return TrailerError::missing_argument("amount"); });

    //         if args.arg_amount.is_empty() {
    //             return 
    //         }

    //         let amount = args.arg_amount? as u32;


    //         // .unwrap_or_else(|| return Err(TrailerError::missing_argument("amount"))) as u32;
    //         let limit_buy = client.limit_buy(&args.arg_symbol, args.arg_amount as u32, args.arg_price)?;
    //     }

    //     if args.cmd_sell {
    //         let limit_sell = client.limit_sell(&args.arg_symbol, args.arg_amount as u32, args.arg_price)?;
    //     }

    // };

    Ok(())
}

fn get_client(exchange: Exchange, config: Option<::config::APIConfig>) -> Result<Box<ExchangeAPI>, TrailerError> {
    if let Some(config) = config {
        match exchange {
            Exchange::Binance   => Ok(Box::new(::exchanges::binance::connect(&config.api_key, &config.secret_key))),
            Exchange::Bittrex   => Ok(Box::new(::exchanges::bittrex::connect(&config.api_key, &config.secret_key))),
            // Exchange::Cobinhood => Ok(Box::new(::exchanges::cobinhood::connect(&config.api_key))),
        }
    } else {
        return Err(TrailerError {
            error_type: TrailerErrorType::ConfigError,
            message: "could not find keys for that exchange in .config.toml".into(),
        });
    }
}
