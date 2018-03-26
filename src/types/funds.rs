use exchanges::*;

#[derive(Debug, Clone)]
pub struct CoinAsset {
    pub symbol: String,
    pub amount: f64,
    pub locked: f64,
    pub exchange: Exchange,
    pub value_in_btc: Option<f64>,
    pub value_in_usd: Option<f64>,
}

impl Default for CoinAsset {
    fn default() -> Self {
        CoinAsset {
            symbol: "<None>".to_string(),
            amount: 0.0,
            locked: 0.0,
            exchange: Exchange::Binance,
            value_in_btc: None,
            value_in_usd: None,
        }
    }
}

pub fn price_in_btc(symbol: String, prices: ::types::Prices) -> Option<f64> {
    match prices.get(&format!("{}BTC", symbol)) {
        Some(price) => Some(*price),
        None => None
    }
}

#[derive(Debug, Clone)]
pub struct Funds {
    pub btc: Option<CoinAsset>,
    pub fiat: Vec<CoinAsset>,
    pub alts: Vec<CoinAsset>,
    pub total_value_in_usd: f64,
    pub total_value_in_btc: f64,
}

impl Funds {
    pub fn calculate_totals(&mut self) -> Self {
        let btc_value = if let Some(ref mut b) = self.btc {
            b.amount
        } else {
            0.0
        };

        let total_usd_price:f64 = self.alts.iter().map(|a| a.value_in_usd.unwrap_or(0.0) * a.amount).sum();
        let total_alt_price_in_btc:f64 = self.alts.iter().map(|a| a.value_in_btc.unwrap_or(0.0) * a.amount).sum();

        self.total_value_in_usd = total_usd_price;
        self.total_value_in_btc = total_alt_price_in_btc + btc_value;

        self.clone()
    }
}

impl Funds {
    pub fn total_btc(&self, btc_price: f64, prices: ::types::Prices) -> f64 {
        let alts:f64 = self.alts.iter().map(|a| { a.amount * price_in_btc(a.clone().symbol, prices.clone()).unwrap_or(0.0) }).sum();
        let fiat:f64 = self.fiat.iter().map(|a| a.amount / btc_price).sum();
        self.btc.clone().unwrap_or_default().amount + alts + fiat
    }
}

pub fn sort_funds(funds: Vec<CoinAsset>) -> Funds {
    let filter:Vec<CoinAsset> = funds.clone().into_iter().filter(|c| c.amount > 0.9).collect();

    Funds {
        btc:    funds.clone().into_iter().find(|c| c.symbol == "BTC"),
        fiat:   filter.clone().into_iter().filter(|c| c.symbol == "USDT" || c.symbol == "TUSD" || c.symbol == "USD").collect(),
        alts:   filter.into_iter().filter(|c| c.symbol != "USDT" && c.symbol != "USD" && c.symbol != "BTC").collect(),
        total_value_in_usd: 0.0,
        total_value_in_btc: 0.0,
    }
}