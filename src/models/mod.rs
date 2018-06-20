#![allow(dead_code)]

use std::collections::HashMap;

mod trades;
pub use self::trades::*;

mod funds;
pub use self::funds::*;

mod candlestick;
pub use self::candlestick::*;

mod order;
pub use self::order::*;

pub type Price = (String, f64);
pub type Prices = HashMap<String, f64>;
pub type Balance = (String, f64, f64);

#[derive(Debug, Clone, Copy)]
pub enum TradeType {
    Buy,
    Sell,
}

impl TradeType {
    pub fn is_buy(s: bool) -> TradeType {
        match s {
            true => TradeType::Buy,
            false => TradeType::Sell,
        }
    }

    pub fn buy(&self) -> bool {
        match self {
            TradeType::Buy  => true,
            TradeType::Sell => false,
        }
    }
}

impl PartialEq for TradeType {
    fn eq(&self, other: &TradeType) -> bool { self == other }
}
impl Eq for TradeType {}

impl ::std::fmt::Display for TradeType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            TradeType::Buy => write!(f, "BUY"),
            TradeType::Sell => write!(f, "SELL"),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_show_trades() {
//         assert_eq!(
//             show_trades(),

//         );
//     }
// }

// pub struct TradeHistory {
//     pub balance: f64,
//     pub average_buy_price: f64,
//     pub average_sell_price: f64,
//     pub profit_locked: f64,
// }
