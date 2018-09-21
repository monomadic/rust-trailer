extern crate trailer;
extern crate rusqlite;

use trailer::exchanges::ExchangeAPI;
use trailer::models::*;

pub fn main() {
    println!("import api data");

    let conn = rusqlite::Connection::open(::std::path::Path::new("./cache.sql"))
        .expect("rusqlite connection could not open on disk");

    let _ = delete_all_from(&conn, "klines");

    let client = ::trailer::exchanges::binance::connect("9N5duztMdrYfYg2ErhSDV837s8xfBIqF8D7mxpJTKiujvSwoIDI52UguhhkyRQBg", "OG6avXJGOeDt5Phbp150zeEgwjQZpgkXdrp8z2vwPv5bWlHuNFLrK4uAGidnpAIU");

    let prices = client.prices().expect("fetch prices failed");

    for price in prices {
        insert_pair(&conn, price.clone(), "binance").expect("price to insert");
        println!("inserted {:?}", price);
    }

    let pairs = vec!["XEMBTC".to_string(), "ADABTC".to_string()];
    let data = ::trailer::threadpool::chart_data(::std::sync::Arc::new(client.clone()), pairs.clone(), "15m");

    for (pair, candlestick_result) in data {
        if let Ok(candles) = candlestick_result {
            for candle in candles {
                // println!("{:#?}", candle);
                match insert_candle(&conn, candle, &pair, "15m", "binance") {
                    Ok(_) => print!("."),
                    Err(why) => println!("error: {:?}", why),
                }
            }
            println!("inserted {}", pair);
        } else { println!("error inserting: {}", pair); }
    }
}

fn delete_all_from(conn: &rusqlite::Connection, table: &str) -> Result<i32, rusqlite::Error> {
    Ok(conn.execute(&format!("DELETE FROM {}", table), &[])?)
}

fn insert_pair(conn: &rusqlite::Connection, price: Price, exchange: &str) -> Result<i32, rusqlite::Error> {
    let (pair, price) = price;
    Ok(conn.execute("INSERT INTO pairs (pair, price, exchange)
      VALUES (?1, ?2, ?3)",
      &[
        &pair,
        &price.to_string(),
        &exchange.to_string(),
      ])?
    )
}

// returns row id
fn insert_candle(conn: &rusqlite::Connection, candle: Candlestick, pair: &str, period: &str, exchange: &str) -> Result<i32, rusqlite::Error> {
    Ok(conn.execute("INSERT INTO klines (pair, time_period, exchange, volume, close_price, number_of_trades)
      VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
      &[
        &pair,
        &period.to_string(),
        &exchange.to_string(),
        &candle.volume.to_string(),
        &candle.close_price.to_string(),
        &candle.number_of_trades.to_string(),
      ])?
    )
}