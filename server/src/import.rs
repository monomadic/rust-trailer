extern crate trailer;
extern crate rusqlite;

use trailer::exchanges::ExchangeAPI;
use trailer::models::*;

mod cache;

use std::collections::HashMap;
pub fn hashmap_ok<T,E>(results: HashMap<String, Result<T,E>>) -> Vec<(String, T)> where E: ::std::fmt::Debug {
    results.into_iter().filter(|(_p,r)|r.is_ok()).map(|(p,r)|(p, r.unwrap())).collect()
}

// pub fn hashmap_ok<T,E>(results: Vec<Result<T,E>>) -> Vec<T> where E: ::std::fmt::Debug {
//     results.into_iter().filter(|r|r.is_ok()).map(|r|r.unwrap()).collect()
// }

pub fn main() {
    println!("import api data");

    let conn = rusqlite::Connection::open(::std::path::Path::new("./cache.sql"))
        .expect("rusqlite connection could not open on disk");

    let _ = delete_all_from(&conn, "klines");
    let _ = delete_all_from(&conn, "pairs");

    let client = ::trailer::exchanges::binance::connect("9N5duztMdrYfYg2ErhSDV837s8xfBIqF8D7mxpJTKiujvSwoIDI52UguhhkyRQBg", "OG6avXJGOeDt5Phbp150zeEgwjQZpgkXdrp8z2vwPv5bWlHuNFLrK4uAGidnpAIU");

    let prices:Prices = client.prices().expect("fetch prices failed");
    let symbols:Vec<String> = prices.clone().into_iter().filter(|(s,_p)|s.contains("BTC")).map(|(s,_p)|s).collect();

    for period in ["15m", "1h", "1d"].iter() {
        println!("getting {} data for {} symbols", period, symbols.len());

        let candles:Vec<(String,Vec<Candlestick>)> = hashmap_ok
            (::trailer::threadpool::chart_data_2(::std::sync::Arc::new(client.clone()), symbols.clone(), period));

        for (pair, candles) in candles {
            print!("\nretrieving: {}", pair);
            let closing_prices:Vec<f64> = candles.into_iter().map(|price| price.close_price).collect();
            let last_price = closing_prices.last().expect("price to exist");
            
            insert_pair(&conn, (pair.clone(), *last_price), "binance").expect("price to insert");

            let rsi = ::trailer::indicators::rsi(14, &closing_prices);
            let rsi:f64 = match rsi.last() {
                Some(rsi) => *rsi,
                None => { println!("error retrieving rsi values for {}, {}, closing_prices:  {:#?}", pair, period, closing_prices); 0.0 }
            };

            match update_rsi(&conn, &pair, &period, rsi.clone()) {
                Ok(_) => print!(" OK: {}, {}", pair, rsi),
                Err(why) => println!("error updating rsi: {:?}", why),
            };
        }
    }

    // for (pair, price) in prices.clone() {
    //     let _ = cache::delete_pair(&conn, &pair.clone());
    //     insert_pair(&conn, (pair.clone(), price.clone()), "binance").expect("price to insert");
    //     println!("inserted {:?}", price);

    //     for period in ["15m", "1h", "1d"].iter() {
    //         let closing_prices:Vec<f64> = client.chart_data(&pair, period).expect("rsi to work").into_iter().map(|price| price.close_price).collect();

    //         let rsi = ::trailer::indicators::rsi(14, &closing_prices);
    //         let rsi:f64 = match rsi.last() {
    //             Some(rsi) => *rsi,
    //             None => { println!("error retrieving rsi values for {}, {}, closing_prices:  {:#?}", pair, period, closing_prices); 0.0 }
    //         };

    //         match update_rsi(&conn, &pair, &period, rsi.clone()) {
    //             Ok(_) => println!("updated rsi for {}, {}", pair, rsi),
    //             Err(why) => println!("error updating rsi: {:?}", why),
    //         };
    //     }
    // }
}

fn delete_all_from(conn: &rusqlite::Connection, table: &str) -> Result<i32, rusqlite::Error> {
    Ok(conn.execute(&format!("DELETE FROM {}", table), &[])?)
}

fn update_rsi(conn: &rusqlite::Connection, pair: &str, period: &str, value: f64) -> Result<i32, rusqlite::Error> {
    Ok(conn.execute(&format!("UPDATE pairs
        SET rsi_{}=(?1)
        WHERE pair=(?2)", period),
      &[
        &value.to_string(),
        &pair,
      ])?
    )
}

fn insert_pair(conn: &rusqlite::Connection, price: Price, exchange: &str) -> Result<i32, rusqlite::Error> {
    let (pair, price) = price;
    let _ = cache::delete_pair(&conn, &pair.clone());
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