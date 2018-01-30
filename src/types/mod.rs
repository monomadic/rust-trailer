#![allow(dead_code)]

mod trades;
pub use self::trades::*;

#[derive(Debug, Clone, Copy)]
pub enum TradeType {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy)]
pub struct Order {
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
