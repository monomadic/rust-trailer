use exchanges::*;

#[derive(Debug, Clone)]
pub struct Asset {
    pub symbol: String,
    pub amount: f64,
    pub locked: f64,
    pub exchange: Exchange,
    // pub value_in_btc: Option<f64>,
    // pub value_in_usd: Option<f64>,
}

impl Default for Asset {
    fn default() -> Self {
        Asset {
            symbol: "<None>".to_string(),
            amount: 0.0,
            locked: 0.0,
            exchange: Exchange::Binance,
            // value_in_btc: None,
            // value_in_usd: None,
        }
    }
}
