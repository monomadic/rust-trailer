// https://cobinhood.github.io/api-public/

#![allow(dead_code)]
#![allow(unused_variables)]

// Binance rate limits are: 1200 requests per minute; 10 orders per second; 100,000 orders per 24hrs. 

use cobinhood::client::Client;

use ::types::*;
use ::error::*;

pub struct CobinhoodAPI {
    pub client: Client,
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

impl CobinhoodAPI {
    pub fn connect(api_key: &str) -> Self {
        Self {
            client: Client::new(api_key),
        }
    }

    pub fn funds(&self) -> Result<Vec<CoinAsset>, TrailerError> {
        let balances = self.client.balances()?;
        Ok(balances.into_iter().map(|b| CoinAsset {
            symbol: b.currency,
            amount: b.total,
            locked: 0.0,
            exchange: "Cobinhood".into(),
        }).collect())
    }
}
