#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
pub struct Trade {
    pub cost: f64,
    pub qty: f64,
    pub buy: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum TradeType {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy)]
pub struct Order {
}
