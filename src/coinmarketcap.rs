#![allow(dead_code)]
#![allow(unused_variables)]

use reqwest;
use serde_json;
use serde::de::{self, Deserialize, Deserializer};

use std::str::FromStr;
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinCap {
    pub id: String,
    pub name: String,
    pub symbol: String,
    #[serde(deserialize_with = "from_str")]
    pub rank: u32,
    #[serde(deserialize_with = "from_str")]
    pub price_usd: f64,
    #[serde(deserialize_with = "from_str")]
    pub price_btc: f64,
    #[serde(rename = "24h_volume_usd")]
    #[serde(deserialize_with = "from_str")]
    pub volume_usd_24h: f64,
    #[serde(deserialize_with = "from_str")]
    pub market_cap_usd: f64,
    #[serde(deserialize_with = "from_str")]
    pub available_supply: f64,
    #[serde(deserialize_with = "from_str")]
    pub total_supply: f64,
    pub max_supply: Option<String>,
    #[serde(deserialize_with = "from_str")]
    pub percent_change_1h: f64,
    #[serde(deserialize_with = "from_str")]
    pub percent_change_24h: f64,
    #[serde(deserialize_with = "from_str")]
    pub percent_change_7d: f64,
    pub last_updated: String,
}

impl CoinCap {
    pub fn cap_vs_vol_24h(&self) -> f64 {
        100.0 / self.market_cap_usd * self.volume_usd_24h
    }
    // pub fn sort_by_cap_vs_vol(&self) {

    // }
}

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where T: FromStr,
          T::Err: Display,
          D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}

pub fn all() -> Vec<CoinCap> {
    let client = reqwest::Client::new();
    let mut response = reqwest::get("https://api.coinmarketcap.com/v1/ticker").expect("/v1/ticker to respond correctly");
    let body = response.text().expect("json response to have text");

    let caps: Vec<CoinCap> = serde_json::from_str(&body).expect("json to deserialise");

    caps
}
