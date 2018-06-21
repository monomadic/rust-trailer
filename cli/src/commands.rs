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
    trade [<exchange>] balances
    trade [<exchange>] orders
    trade <exchange> past-orders [<symbol>]
    trade <exchange> price <symbol>
    trade <exchange> (buy|sell) <symbol> <amount> <price>
    trade <exchange> stop (loss|gain) <symbol> <amount> <price>
    trade <exchange> b <symbol>
    trade <exchange> ev <symbol> [--group]
    trade <exchange> rsi <symbol>

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
    cmd_buy: bool,
    cmd_sell: bool,
    cmd_stop: bool,
    cmd_loss: bool,
    cmd_gain: bool,
    cmd_orders: bool,
    cmd_past_orders: bool,
    cmd_b: bool,
    cmd_ev: bool,
    cmd_rsi: bool,

    arg_symbol: Option<String>,
    arg_amount: Option<f64>,
    arg_price: Option<f64>,

    flag_group: bool,
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

        if args.cmd_funds {
            println!("getting funds...");
            let funds = client.funds()?;

            ::display::title_bar(&format!("\n{} Balance", client.display()));
            ::display::show_funds(funds);
        }

        if args.cmd_balances {
            println!("getting balances...");
            ::display::show_balances(client.balances()?);
        }

        if args.cmd_orders {
            println!("getting open orders...");
            ::display::show_orders(client.open_orders()?);
        }

        if args.cmd_past_orders {
            println!("getting past orders...");
            if let Some(symbol) = args.arg_symbol.clone() {
                ::display::show_orders(client.past_trades_for(&symbol)?);
            } else {
                ::display::show_orders(client.past_orders()?);
            }
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

        if args.cmd_ev {
            let symbol = args.arg_symbol.clone().ok_or(TrailerError::missing_argument("symbol"))?;
            let orders = client.past_trades_for(&symbol)?;
            let price = client.price(&symbol)?;
            let btc_price = client.btc_price()?;

            let processed_orders = match args.flag_group {
                true => trailer::models::average_orders(orders),
                false => trailer::models::compact_orders(orders),
            };

            evaluate_trades(symbol, processed_orders, price, btc_price)?;
        }

        if args.cmd_rsi {
            println!("fetching rsi...");
            let symbol = args.arg_symbol.clone().ok_or(TrailerError::missing_argument("symbol"))?;
            client.chart_data(&symbol)?;
        }

    };

    Ok(())
}

pub fn evaluate_trades(symbol: String, orders: Vec<trailer::models::Order>, price: f64, btc_price: f64) -> Result<(), TrailerError> {
    use colored::*;
    use trailer::models::{ TradeType };

    println!("evaluating trades...");

    ::display::title_bar(&format!("{}", symbol.yellow()));

    println!("{:8}{:<8}{:<16}{:<16}{:<16}{:<16}{:<16}{:<16}{:<8}",
        "type", "btc", "qty", "price", "current_price", "cost_usd", "uprofit", "uprofit usd", "% change");

    for order in orders {
        let cost_btc = order.qty * order.price;
        let cost_usd = (price * order.qty) * btc_price;
        let percent_change = 100. - 100. / order.price * price;

        let (profit, buy_type) = match order.order_type {
            TradeType::Buy => {(
                ((order.qty * price) - cost_btc),
                ("BUY".green())
            )},
            TradeType::Sell => {(
                (cost_btc - (order.qty * price)),
                ("SELL".red())
            )},
        };

        let profit_usd = profit * btc_price;

        use ::display::colored_number;
        println!("{buy_type:<8}{cost_btc:<8}{order_amount:<16}{order_price:<16}{price:<16}{cost_usd:<16}{profit:<16}{profit_usd:<16}{percent_change:<8}",
            buy_type        = buy_type,
            cost_btc        = format!("{:.2}", cost_btc),
            order_amount    = format!("{:.2}", order.qty),
            order_price     = format!("{:.8}", order.price),
            price           = format!("{:.8}", price),
            cost_usd        = format!("${:.2}", cost_usd),
            profit          = colored_number(profit, format!("{:>11.8}", profit)),
            profit_usd      = colored_number(profit_usd, format!("${:.2}", profit_usd)),
            percent_change  = format!("{:.2}%", percent_change));
    }

    Ok(())
}
