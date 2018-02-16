// https://cobinhood.github.io/api-public/

#![allow(dead_code)]
#![allow(unused_variables)]

// Binance rate limits are: 1200 requests per minute; 10 orders per second; 100,000 orders per 24hrs. 

use cobinhood::client::Client;

pub struct CobinhoodAPI {
    pub client: Client,
}

impl CobinhoodAPI {
    pub fn connect(api_key: &str) -> Self {
        Self {
            client: Client::new(api_key),
        }
    }
}
