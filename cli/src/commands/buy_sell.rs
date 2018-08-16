use trailer::error::TrailerError;

pub fn buy_sell(client: ::std::sync::Arc<::trailer::exchanges::ExchangeAPI>, symbol: &str, price: f64) -> Result<String, TrailerError> {
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

    Ok("done".to_string())
}