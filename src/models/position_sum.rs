use models::*;

#[derive(Debug)]
pub struct PositionSum {
    base_currency_qty:      f64,
    quote_currency_qty:     f64,
}

impl PositionSum {
    pub fn calculate(positions: Vec<Position>) {
        println!("hi");

    }
}
