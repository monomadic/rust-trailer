#![allow(dead_code)]
#![allow(unused_variables)]

use trailer;
use trailer::exchanges::*;
use trailer::error::*;

use docopt::Docopt;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const USAGE: &'static str = "
Usage:
    stats all
";

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

#[derive(Debug, Deserialize)]
struct Args {
    cmd_all: bool,
}

pub fn run_docopt() -> Result<(), TrailerError> {
    let args:Args = Docopt::new(USAGE)
        .map(|d| d.version(Some(VERSION.into())))
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    // let conf = trailer::config::read()?;

    let keys = match args.arg_exchange {
        Exchange::Bittrex => conf.bittrex.ok_or(TrailerError::missing_config_keys("bittrex"))?,
        Exchange::Binance => conf.binance.ok_or(TrailerError::missing_config_keys("binance"))?,
        Exchange::Kucoin  => conf.kucoin.ok_or(TrailerError::missing_config_keys("kucoin"))?,
    };

    let client:Box<ExchangeAPI> = match args.arg_exchange {
        Exchange::Bittrex => Box::new(trailer::exchanges::bittrex::connect(&keys.api_key, &keys.secret_key)),
        Exchange::Binance => Box::new(trailer::exchanges::binance::connect(&keys.api_key, &keys.secret_key)),
    };

    if args.cmd_orders {
        println!("getting open orders...");
        ::display::show_orders(client.open_orders()?);
    }

    if args.cmd_past_orders {
        println!("getting past orders...");
        ::display::show_orders(client.past_orders()?);
    }

    if args.cmd_funds {
        println!("getting funds...");
        let funds = client.funds()?;

        println!("getting prices...");
        let prices = client.prices()?;

        ::display::show_funds(trailer::types::sort_funds(funds), prices);
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
            let limit_sell = client.limit_buy(&symbol, amount, price)?;
        } else if args.cmd_sell {
            let limit_sell = client.limit_sell(&symbol, amount, price)?;
        }
    }

    Ok(())
}
