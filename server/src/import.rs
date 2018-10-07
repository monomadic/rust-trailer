extern crate trailer;
extern crate rusqlite;

use trailer::exchanges::ExchangeAPI;
use trailer::models::*;

mod cache;

pub fn main() {
    println!("import api data");

    let conn = rusqlite::Connection::open(::std::path::Path::new("./cache.sql"))
        .expect("rusqlite connection could not open on disk");

    let _ = delete_all_from(&conn, "klines");
    let _ = delete_all_from(&conn, "pairs");

    let client = ::trailer::exchanges::binance::connect("9N5duztMdrYfYg2ErhSDV837s8xfBIqF8D7mxpJTKiujvSwoIDI52UguhhkyRQBg", "OG6avXJGOeDt5Phbp150zeEgwjQZpgkXdrp8z2vwPv5bWlHuNFLrK4uAGidnpAIU");

    let prices = client.prices().expect("fetch prices failed");

    for (pair, price) in prices {
        let _ = cache::delete_pair(&conn, &pair.clone());
        insert_pair(&conn, (pair.clone(), price.clone()), "binance").expect("price to insert");
        println!("inserted {:?}", price);

        let closing_prices:Vec<f64> = client.chart_data(&pair, "15m").expect("rsi to work").into_iter().map(|price| price.close_price).collect();
        let rsi = ::trailer::indicators::rsi(14, &closing_prices);
        let rsi = rsi.last().expect("an rsi value");

        match update_rsi(&conn, &pair, rsi.clone()) {
            Ok(_) => println!("updated rsi for {}, {}", pair, rsi),
            Err(why) => println!("error updating rsi: {:?}", why),
        };
    }

    // let pairs = vec!["XEMBTC".to_string(), "ADABTC".to_string()];
    // let data = ::trailer::threadpool::chart_data(::std::sync::Arc::new(client.clone()), pairs.clone(), "15m");

    // for (pair, candlestick_result) in data {
    //     if let Ok(candles) = candlestick_result {

    //         for candle in candles.clone() {
    //             match insert_candle(&conn, candle, &pair, "15m", "binance") {
    //                 Ok(_) => print!("."),
    //                 Err(why) => println!("error: {:?}", why),
    //             }
    //         }

    //         println!("inserted {}", pair);

    //         let candles = candles.clone()
    //             .into_iter()
    //             .map(|c| c.close_price)
    //             .collect();

    //         let rsi = ::trailer::indicators::rsi(14, &candles);
    //         let rsi = rsi.last().expect("an rsi value");

    //         match update_rsi(&conn, &pair, rsi.clone()) {
    //             Ok(_) => println!("updated rsi for {}, {}", pair, rsi),
    //             Err(why) => println!("error updating rsi: {:?}", why),
    //         };



    //     } else { println!("error inserting: {}", pair); }
    // }
}

fn delete_all_from(conn: &rusqlite::Connection, table: &str) -> Result<i32, rusqlite::Error> {
    Ok(conn.execute(&format!("DELETE FROM {}", table), &[])?)
}

fn update_rsi(conn: &rusqlite::Connection, pair: &str, rsi_15m: f64) -> Result<i32, rusqlite::Error> {
    Ok(conn.execute("UPDATE pairs
        SET rsi_15m=(?1)
        WHERE pair=(?2)",
      &[
        &rsi_15m.to_string(),
        &pair,
      ])?
    )
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