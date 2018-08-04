

pub fn stop(symbol: &str, amount: f64, price: f64, is_stop_loss: bool) -> String {
    println!("{:?}:", (symbol, amount, price));

    format!("stop hit")
}
