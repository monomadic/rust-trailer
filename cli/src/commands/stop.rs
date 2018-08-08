// use trailer;
use trailer::error::TrailerError;
use colored::Colorize;

pub fn stop(client: Box<::trailer::exchanges::ExchangeAPI>, symbol: &str, arg_amount: Option<f64>, arg_price: Option<f64>, is_stop_loss: bool) -> Result<String, TrailerError> {
    println!("set stop for: {}", symbol.yellow());

    let amount = if let Some(amount) = arg_amount {
        amount
    } else {
        let amt = client.amount_for_symbol(&symbol).unwrap_or(0.0);
        print!("amount (holding {}): ", amt);
        ::input::get_f64(amt)?
    };

    if amount <= 0.0 { return Err(TrailerError::generic("amount can not be zero.")); }

    let current_price = client.price(&symbol)?;
    println!("\ncurrent price: {}", current_price);

    let stop = if let Some(price) = arg_price {
        price
    } else {
        let stop_default = current_price * 0.9;
        print!("\nstop (10% / {}): ", stop_default);
        ::input::get_f64(stop_default)?
    };

    let limit_default = current_price * 0.88;
    print!("limit (12% / {}): ", limit_default);
    let limit = ::input::get_f64(limit_default)?;

    println!("{:?}:", (symbol, amount, stop, limit));

    Ok(format!("stop hit"))
}
