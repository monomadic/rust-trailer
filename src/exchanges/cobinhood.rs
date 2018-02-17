// https://cobinhood.github.io/api-public/

#![allow(dead_code)]
#![allow(unused_variables)]

// Binance rate limits are: 1200 requests per minute; 10 orders per second; 100,000 orders per 24hrs. 

use cobinhood::client::Client;

use ::types::*;
use ::error::*;
use ::exchanges::ExchangeAPI;

pub struct CobinhoodAPI {
    pub client: Client,
}

pub fn connect(api_key: &str) -> CobinhoodAPI {
    CobinhoodAPI {
        client: Client::new(api_key),
    }
}

use cobinhood::error::CobinhoodError as CobError;
impl From<CobError> for TrailerError {
    fn from(error: CobError) -> Self {
        Self {
            error_type: TrailerErrorType::APIError,
            message: error.message,
        }
    }
}

impl ExchangeAPI for CobinhoodAPI {
    fn funds(&self) -> Result<Vec<CoinAsset>, TrailerError> {
        let balances = self.client.balances()?;

        Ok(balances.into_iter().map(|b| CoinAsset {
            symbol: b.currency,
            amount: b.total,
            locked: 0.0,
            exchange: "Cobinhood".into(),
        }).collect())
    }

    fn price(&self, symbol: &str) -> Result<f64, TrailerError> {
        // Ok(self.client.get_price(symbol)?)
        Err(TrailerError::unsupported())
    }

    fn prices(&self) -> Result<Prices, TrailerError> {
        Err(TrailerError::unsupported())
    }

    fn limit_buy(&self, symbol: &str, amount: u32, price: f64) -> Result<(), TrailerError> {
        Err(TrailerError::unsupported())
    }

    fn limit_sell(&self, symbol: &str, amount: u32, price: f64) -> Result<(), TrailerError> {
        Err(TrailerError::unsupported())
    }
}
