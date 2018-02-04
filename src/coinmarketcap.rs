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
    #[serde(deserialize_with = "deserialize_some")]
    pub price_usd: Option<f64>,
    #[serde(deserialize_with = "deserialize_some")]
    pub price_btc: Option<f64>,
    #[serde(rename = "24h_volume_usd")]
    #[serde(deserialize_with = "deserialize_some")]
    pub volume_usd_24h: Option<f64>,
    #[serde(deserialize_with = "deserialize_some")]
    pub market_cap_usd: Option<f64>,
    #[serde(deserialize_with = "deserialize_some")]
    pub available_supply: Option<f64>,
    #[serde(deserialize_with = "deserialize_some")]
    pub total_supply: Option<f64>,
    pub max_supply: Option<String>,
    #[serde(deserialize_with = "deserialize_some")]
    pub percent_change_1h: Option<f64>,
    #[serde(deserialize_with = "deserialize_some")]
    pub percent_change_24h: Option<f64>,
    #[serde(deserialize_with = "deserialize_some")]
    pub percent_change_7d: Option<f64>,
    pub last_updated: Option<String>,
}

impl CoinCap {
    pub fn cap_vs_vol_24h(&self) -> f64 {
        if let Some(cap) = self.market_cap_usd {
            if let Some(vol) = self.volume_usd_24h {
                100.0 / cap * vol
            } else { 0.0 }
        } else { 0.0 }
        // 100.0 / self.market_cap_usd * self.volume_usd_24h
    }
    // pub fn sort_by_cap_vs_vol(&self) {

    // }
}

// Any value that is present is considered Some value, including null.
fn deserialize_some<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where T: Deserialize<'de>,
          T: FromStr,
          T::Err: Display,
          D: Deserializer<'de>
{
    let s = Deserialize::deserialize(deserializer).map(Some).expect("deserialise from string");
    Ok(Some(T::from_str(s.unwrap()).map_err(de::Error::custom)?))
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
    let mut response = reqwest::get("https://api.coinmarketcap.com/v1/ticker?limit=0").expect("/v1/ticker to respond correctly");
    assert!(response.status().is_success());
    let body = response.text().expect("json response to have text");

    serde_json::from_str(&body).expect("json to deserialise")
}
