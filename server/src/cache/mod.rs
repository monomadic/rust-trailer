// use std::collections::HashMap;

// struct RSICache {
    
// }

// pub fn 

use rusqlite;

use trailer::models::*;

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

fn get_candles_for(conn: &rusqlite::Connection, pair: &str) -> Result<Vec<Candlestick>, rusqlite::Error> {
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