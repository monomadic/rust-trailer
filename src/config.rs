#![allow(dead_code)]
#![allow(unused_variables)]

use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub bittrex: Option<APIConfig>,
    pub binance: Option<APIConfig>,
}

#[derive(Debug, Deserialize)]
pub struct APIConfig {
    pub api_key: String,
    pub secret_key: String,
}

pub fn read() -> Config {
    let conf:Config = toml::from_str(&str_from_file(".config.toml")).expect(".config.toml to parse correctly");
    conf
}

fn str_from_file(file: &str) -> String {
    use std::io::prelude::*;
    let mut handle = ::std::fs::File::open(file).expect("file to open");
    let mut bytebuffer = Vec::new();
    handle.read_to_end(&mut bytebuffer).expect("text file to read");
    return String::from_utf8(bytebuffer).expect("file to convert from utf8")
}