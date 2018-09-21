extern crate trailer;
extern crate rusqlite;

pub fn main() {
    println!("setup db");

    let conn = rusqlite::Connection::open(::std::path::Path::new("./cache.sql"))
        .expect("rusqlite connection could not open on disk");

    conn.execute_batch("
         DROP TABLE IF EXISTS pairs;
         DROP TABLE IF EXISTS klines;
    ").expect("error deleting databases");

    conn.execute_batch("
        CREATE TABLE pairs (
            id                  INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            pair                VARCHAR NOT NULL,
            exchange            VARCHAR NOT NULL,
            volume              REAL,
            price               REAL
        );

        CREATE TABLE klines (
            id                  INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            pair                VARCHAR NOT NULL,
            time_period         VARCHAR NOT NULL,
            exchange            VARCHAR NOT NULL,
            volume              REAL,
            close_price         REAL,
            number_of_trades    INTEGER
        );
    ").expect("error regenerating databases");
}
