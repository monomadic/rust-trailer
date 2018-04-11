#![allow(dead_code)]
#![allow(unused_variables)]

extern crate trailer;

//use ratelimit;
//use std::thread;
//use std::time::{Duration};

mod csv;

mod bot;
use bot::Bot;

mod error;

fn main() {
    let bot = Bot::new_with_config("./data/bots/new.toml");
    match bot.run() {
        Ok(_) => println!("done."),
        Err(e) => println!("error: {:?}", e),
    }
}
