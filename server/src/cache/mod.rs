#![allow(dead_code)]
// use std::collections::HashMap;

// struct RSICache {
    
// }

// pub fn 

use rusqlite;
use trailer::models::*;

pub fn get_all_pairs(conn: &rusqlite::Connection) -> Result<Vec<(String, f64, Option<f64>)>, rusqlite::Error> {
    let mut query = conn.prepare("SELECT pair, price, rsi_15m FROM pairs;")?;
    
    let results = query.query_map(&[], |row| {
        (
            row.get::<_,String>(0),
            row.get::<_,f64>(1),
            row.get::<_,Option<f64>>(2),
        )
    })?.filter(|r|r.is_ok()).map(|r|r.expect("5566")).collect();

    Ok(results)
}

pub fn delete_pair(conn: &rusqlite::Connection, pair: &str) -> Result<i32, ::rusqlite::Error> {
    let mut query = conn.prepare("DELETE * FROM prices WHERE pair=?1;")?;
    Ok(query.execute(&[&pair])?)
}

pub fn delete_candles_for(conn: &rusqlite::Connection, pair: &str) -> Result<i32, ::rusqlite::Error> {
    let mut query = conn.prepare("DELETE * FROM klines WHERE pair=?1;")?;
    Ok(query.execute(&[&pair])?)
}

pub fn get_all_candles(conn: &rusqlite::Connection) -> Vec<(String, Vec<Candlestick>)> {
    // let mut query = conn.prepare("SELECT * FROM klines;").unwrap();
    // let results: Result<Vec<Candlestick>>, rusqlite::Error> = query.query_map(&[], |row|
    //     (
    //         row.get(0),
    //         Candlestick {
    //             open_time:          0,
    //             open_price:         0.0,
    //             close_price:        0.0,
    //             high_price:         0.0,
    //             low_price:          0.0,
    //             volume:             0.0,
    //             number_of_trades:   0,
    //         }
    //     )
    // ).unwrap().collect();

    // results.unwrap()
    vec![("ADABTC".to_string(), get_candles_for(conn, "ADABTC").expect("dfdffd"))]
}

pub fn get_candles_for(conn: &rusqlite::Connection, pair: &str) -> Result<Vec<Candlestick>, rusqlite::Error> {
    let mut query = conn.prepare("SELECT close_price, volume, number_of_trades FROM klines where pair=?1;")?;
    
    let results = query.query_map(&[&pair], |row| {
        Candlestick {
            open_time:          0,
            open_price:         0.0,
            close_price:        row.get::<_,f64>(0),
            high_price:         0.0,
            low_price:          0.0,
            volume:             row.get::<_,f64>(1),
            number_of_trades:   row.get::<_,f64>(2) as u64,
        }
    })?.filter(|r|r.is_ok()).map(|r|r.unwrap()).collect();
    Ok(results)
}