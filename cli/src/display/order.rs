// use colored::*;
use trailer::models::*;
// use trailer::error::*;
// use super::colored_number;

pub fn row(order: Order) -> String {
    format!("{:20}\t{:20}\t{:20.8}\t{:20.2}\n",
        order.symbol, order.order_type, order.price, order.qty)
}
