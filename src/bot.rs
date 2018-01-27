#![allow(dead_code)]
#![allow(unused_variables)]

pub struct Bot {
    coin: String,
    entries: Vec<(f64, f64)>, // positions at which to enter the market
}

impl Bot {
    pub fn run(&self) {
        println!("running bot on coin: {}", self.coin);
    }

    pub fn backtest(&self) {
        println!("backtesting bot on coin: {}", self.coin);
    }
}
