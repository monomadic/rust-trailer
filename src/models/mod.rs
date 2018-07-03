#![allow(dead_code)]

use std::collections::HashMap;

mod trades; pub use self::trades::*;
mod funds; pub use self::funds::*;
mod candlestick; pub use self::candlestick::*;
mod order; pub use self::order::*;
mod trade_type; pub use self::trade_type::*;
mod position; pub use self::position::*;
mod position_sum; pub use self::position_sum::*;

pub type Price = (String, f64);
pub type Prices = HashMap<String, f64>;
pub type Balance = (String, f64, f64);
