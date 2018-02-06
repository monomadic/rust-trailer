#[derive(Debug, Clone)]
pub struct CoinAsset {
    pub symbol: String,
    pub amount: f64,
    pub locked: f64,
    pub exchange: String,
}

impl Default for CoinAsset {
    fn default() -> Self {
        CoinAsset {
            symbol: "<None>".to_string(),
            amount: 0.0,
            locked: 0.0,
            exchange: "<None>".to_string(),
        }
    }
}

pub fn price_in_btc(symbol: String, prices: ::types::Prices) -> f64 {
    *prices.get(&format!("{}BTC", symbol)).expect(&format!("{}BTC to exist in prices", symbol))
}

#[derive(Debug, Clone)]
pub struct Funds {
    pub btc: Option<CoinAsset>,
    pub fiat: Vec<CoinAsset>,
    pub alts: Vec<CoinAsset>,
}

impl Funds {
    pub fn total_btc(&self, btc_price: f64, prices: ::types::Prices) -> f64 {
        let alts:f64 = self.alts.iter().map(|a| { a.amount * price_in_btc(a.clone().symbol, prices.clone()) }).sum();
        let fiat:f64 = self.fiat.iter().map(|a| a.amount / btc_price).sum();
        self.btc.clone().unwrap_or_default().amount + alts + fiat
    }
}

pub fn sort_funds(funds: Vec<CoinAsset>) -> Funds {
    let filter:Vec<CoinAsset> = funds.clone().into_iter().filter(|c| c.amount > 0.9).collect();

    Funds {
        btc:    funds.clone().into_iter().find(|c| c.symbol == "BTC"),
        fiat:   filter.clone().into_iter().filter(|c| c.symbol == "USDT" || c.symbol == "USD").collect(),
        alts:   filter.into_iter().filter(|c| c.symbol != "USDT" && c.symbol != "USD" && c.symbol != "BTC").collect(),
    }
}