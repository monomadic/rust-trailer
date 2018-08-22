#![allow(dead_code)]

mod asset; pub use self::asset::*;
mod funds; pub use self::funds::*;
mod candlestick; pub use self::candlestick::*;
mod order; pub use self::order::*;
mod trades; pub use self::trades::*;
mod trade_bucket; pub use self::trade_bucket::*;
mod trade_type; pub use self::trade_type::*;
mod position; pub use self::position::*;
mod position_sum; pub use self::position_sum::*;
// mod position_accumulated; pub use self::position_accumulated::*;
mod price; pub use self::price::*;

pub type Balance = (String, f64, f64);
