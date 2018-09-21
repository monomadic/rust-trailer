extern crate iron;
extern crate router;
extern crate staticfile;
extern crate mount;
#[macro_use] extern crate horrorshow;
extern crate rusqlite;

extern crate trailer;

use iron::prelude::*;
use std::sync::{Arc, Mutex};

mod controllers;
mod error;
mod views;
mod cache;

fn main() {
    let mut router = router::Router::new();
    let mut mount = mount::Mount::new();

    // let conn = Arc::new(
    //     rusqlite::Connection::open(::std::path::Path::new("./cache.sql"))
    //         .expect("./cache.sql failed to open")
    // );

    let conn = Arc::new(Mutex::new(
        rusqlite::Connection::open(::std::path::Path::new("./cache.sql"))
            .expect("./cache.sql failed to open")
    ));

    // let db = rusqlite::Connection::open(::std::path::Path::new("./cache.sql"))
    //     .expect("./cache.sql failed to open");



    // if let db = db.clone() {
    //     router.get("/rsi", move |r: &mut Request| {
    //         controllers::handle_request(controllers::rsi(r))
    //     }, "rsi");
    // }

    router.get("/funds", move |r: &mut Request| {
        controllers::handle_request(controllers::funds(r))
    }, "funds");

    router.get("/positions", move |r: &mut Request| {
        controllers::handle_request(controllers::positions(r))
    }, "positions");

    router.get("/chart/:symbol", move |r: &mut Request| {
        controllers::handle_request(controllers::chart(r))
    }, "chart");

    let db_root = conn.clone();
    router.get("/", move |r: &mut Request| {
        controllers::handle_request(controllers::rsi(r, &db_root.lock().unwrap()))
    }, "root");

    let db_prices = conn.clone();
    router.get("/prices.json", move |r: &mut Request| {
        controllers::handle_request(controllers::prices(r, &db_prices.lock().unwrap()))
    }, "prices");

    // let db_by_tag = db.clone();
    // router.get("/tag/:tag", move |r: &mut Request| {
    //     handle_request(controllers::filter_by_tag(r, &db_by_tag.lock().unwrap()))
    // }, "filter_by_tag");

    mount.mount("/", router);

    // // static mount
    // let static_assets = staticfile::Static::new(::std::path::Path::new("static"));
    // mount.mount("/static", static_assets);

    // launch server
    println!("launching server at http://localhost:4000/");

    match iron::Iron::new(mount).http("localhost:4000") {
        Ok(_) => println!("server running ok."),
        Err(e) => println!("error: {}", e),
    };
}