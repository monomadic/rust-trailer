#![allow(dead_code)]
#![allow(unused_variables)]

use trailer;
use trailer::exchanges::*;
use trailer::error::*;

use docopt::Docopt;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const USAGE: &'static str = "
Usage:
    trade [<exchange>] funds
    trade [<exchange>] orders
    trade <exchange> past-orders
    trade <exchange> price <symbol>
    trade <exchange> (buy|sell) <symbol> <amount> <price>

Exchange:
    binance
    bittrex
    kucoin
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
    arg_exchange: Option<Exchange>,
    cmd_funds: bool,
    cmd_price: bool,
    cmd_buy: bool,
    cmd_sell: bool,
    cmd_orders: bool,
    cmd_past_orders: bool,
    arg_symbol: Option<String>,
    arg_amount: Option<f64>,
    arg_price: Option<f64>,
}

pub fn run_docopt() -> Result<(), TrailerError> {
    let args:Args = Docopt::new(USAGE)
        .map(|d| d.version(Some(VERSION.into())))
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let conf = trailer::config::read()?;
    let mut clients = Vec::new();

    fn get_client(exchange: Exchange, keys: trailer::config::APIConfig) -> Box<ExchangeAPI> {
        match exchange {
            Exchange::Bittrex => Box::new(trailer::exchanges::bittrex::connect(&keys.api_key, &keys.secret_key)),
            Exchange::Binance => Box::new(trailer::exchanges::binance::connect(&keys.api_key, &keys.secret_key)),
            Exchange::Kucoin  => Box::new(trailer::exchanges::kucoin::connect(&keys.api_key, &keys.secret_key)),
        }
    };

    if let Some(arg_exchange) = args.arg_exchange { // if the user supplied an exchange
        let exchange_keys = &conf.exchange[&arg_exchange.to_string()];
        clients.push(get_client(arg_exchange, exchange_keys.clone()));
    } else {
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

            println!("\n{} Balance", client.display());
            println!("====================================================================");

            ::display::show_funds(funds);
        }

        if args.cmd_price {
            println!("getting price...");
            let symbol = args.arg_symbol.clone().ok_or(TrailerError::missing_argument("symbol"))?;
            let price = client.price(&symbol)?;

            ::display::show_price((symbol, price));
        }

        if args.cmd_buy || args.cmd_sell {
            let symbol = args.arg_symbol.clone().ok_or(TrailerError::missing_argument("symbol"))?;
            let amount = args.arg_amount.ok_or(TrailerError::missing_argument("amount"))?;
            let price = args.arg_price.ok_or(TrailerError::missing_argument("price"))?;

            if args.cmd_buy {
                let limit_sell = client.limit_buy(&symbol, amount, price)?;
            } else if args.cmd_sell {
                let limit_sell = client.limit_sell(&symbol, amount, price)?;
            }
        }
    };

    Ok(())
}
