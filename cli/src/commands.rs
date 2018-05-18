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
    trade <exchange> stop (loss|gain) <symbol> <amount> <price>
    trade <exchange> b <symbol>

Exchange:
    binance
    bittrex
    kucoin
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_exchange: Option<Exchange>,

    cmd_funds: bool,
    cmd_price: bool,
    cmd_buy: bool,
    cmd_sell: bool,
    cmd_stop: bool,
    cmd_loss: bool,
    cmd_gain: bool,
    cmd_orders: bool,
    cmd_past_orders: bool,
    cmd_b: bool,

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

    if let Some(arg_exchange) = args.arg_exchange {
        // user supplied a specific exchange.
        let exchange_keys = &conf.exchange[&arg_exchange.to_string()];
        clients.push(get_client(arg_exchange, exchange_keys.clone()));
    } else {
        // try to use all exchanges in the config.
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

            ::display::title_bar(&format!("\n{} Balance", client.display()));
            ::display::show_funds(funds);
        }

        if args.cmd_price {
            println!("getting price...");
            let symbol = args.arg_symbol.clone().ok_or(TrailerError::missing_argument("symbol"))?;
            let price = client.price(&symbol)?;

            ::display::show_price((symbol, price));
        }

        if args.cmd_stop {
            let symbol = args.arg_symbol.clone().ok_or(TrailerError::missing_argument("symbol"))?;
            let amount = args.arg_amount.ok_or(TrailerError::missing_argument("amount"))?;
            let price = args.arg_price.ok_or(TrailerError::missing_argument("price"))?;

            // let price = client.price(&symbol)?;
            // println!("current price for ")
            // ::display::show_price((symbol, price));

            // if args.cmd_buy {
            //     let limit_sell = client.limit_buy(&symbol, amount, price)?;
            // } else if args.cmd_sell {
            //     let limit_sell = client.limit_sell(&symbol, amount, price)?;
            // }

            if args.cmd_buy || args.cmd_sell {
                println!("stop loss/gain");
            }
        }

        if args.cmd_b {
            let symbol = args.arg_symbol.clone().ok_or(TrailerError::missing_argument("symbol"))?;
            let price = client.price(&symbol)?;

            println!("{}:", symbol);
            println!("current price {}\n", price);

            print!("price ({}): ", price);
            let buy_price = ::input::get_f64(price)?;
            if buy_price > price { println!("WARNING: your buy price is higher than the current price!"); }

            print!("amount (10): ");
            let amount = ::input::get_f64(10.)?;

            println!("\nbuying {} {} at {}. total price: {:.8}.", amount, symbol, buy_price, price * amount);
            print!("\ncontinue with purchase? (y/N) ");
            match ::input::get_confirmation()? {
                true => println!("\npurchasing..."),
                false => println!("\nno purchase made."),
            }

        }
    };

    Ok(())
}
