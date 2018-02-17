#![allow(dead_code)]
#![allow(unused_variables)]

use exchanges::*;
use error::*;

use docopt::Docopt;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const USAGE: &'static str = "
Usage:
    trade (binance|bittrex|cob) funds
    trade (binance|bittrex|cob) price <symbol>

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
    trade cob time
    trade cob funds
    trade bot run
    trade bot backtest <csv>
    trade caps

Options:
  -h --help     Show this screen.
  --version     Show version.
";

// #[derive(Deserialize, Debug)]
// enum Exchange { Binance, Bittrex }

#[derive(Debug, Deserialize)]
struct Args {
    cmd_binance: bool,
    cmd_bittrex: bool,
    cmd_cob: bool,
    cmd_funds: bool,
    cmd_price: bool,
    arg_symbol: String,
}

enum Exchange {
    Bittrex,
    Binance,
    Cobinhood,
}

pub fn run_docopt() -> Result<(), TrailerError> {
    // let args = Docopt::new(USAGE)
    //     .and_then(|dopt| dopt
    //     .version(Some(VERSION.to_string()))
    //     .parse())
    //     .unwrap_or_else(|e| e.exit());
    let args:Args = Docopt::new(USAGE)
        .map(|d| d.version(Some(VERSION.into())))
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    println!("{:?}", args);

    if args.cmd_binance || args.cmd_bittrex {
        // read config
        let conf = ::config::read()?;
        let client = if args.cmd_binance { get_client(Exchange::Binance, conf.binance)? }
            else { get_client(Exchange::Bittrex, conf.bittrex)? };

        if args.cmd_funds {
            println!("getting funds...");
            let funds = client.funds()?;

            println!("getting prices...");
            let prices = client.prices()?;

            ::display::show_funds(::types::sort_funds(funds), prices);
        }

        if args.cmd_price {
            println!("getting price...");
            let price = client.price(&args.arg_symbol)?;
            ::display::show_price((args.arg_symbol, price));
        }
    };

    Ok(())
}

fn get_client(exchange: Exchange, config: Option<::config::APIConfig>) -> Result<Box<ExchangeAPI>, TrailerError> {
    if let Some(config) = config {
        match exchange {
            Exchange::Binance   => Ok(Box::new(::exchanges::binance::connect(&config.api_key, &config.secret_key))),
            Exchange::Bittrex   => Ok(Box::new(::exchanges::bittrex::connect(&config.api_key, &config.secret_key))),
            Exchange::Cobinhood => Ok(Box::new(::exchanges::cobinhood::connect(&config.api_key))),
        }
    } else {
        return Err(TrailerError {
            error_type: TrailerErrorType::ConfigError,
            message: "could not find keys for that exchange in .config.toml".into(),
        });
    }
}
