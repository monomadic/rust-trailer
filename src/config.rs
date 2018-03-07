#![allow(dead_code)]
#![allow(unused_variables)]

use toml;
use error::*;

use std::collections::BTreeMap;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub exchange: BTreeMap<String, APIConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct APIConfig {
    pub api_key: String,
    pub secret_key: String,
}

impl From<::std::io::Error> for TrailerError {
    fn from(error: ::std::io::Error) -> Self {
        use std::error::Error;

        TrailerError {
            error_type: TrailerErrorType::ConfigError,
            message: format!("Error reading .config.toml: {}", error.description()),
        }
    }
}

impl From<::std::string::FromUtf8Error> for TrailerError {
    fn from(error: ::std::string::FromUtf8Error) -> Self {
        use std::error::Error;

        TrailerError {
            error_type: TrailerErrorType::ConfigError,
            message: format!("Error converting .config.toml to UTF8: {}", error.description()),
        }
    }
}

impl From<::toml::de::Error> for TrailerError {
    fn from(error: ::toml::de::Error) -> Self {
        use std::error::Error;

        TrailerError {
            error_type: TrailerErrorType::ConfigError,
            message: format!("Error reading .config.toml: {}", error.description()),
        }
    }
}

pub fn read() -> Result<Config, TrailerError> {
    Ok(toml::from_str(&str_from_file(".config.toml")?)?)
}

fn str_from_file(file: &str) -> Result<String, TrailerError> {
    use std::io::prelude::*;
    let mut handle = ::std::fs::File::open(file)?;
    let mut bytebuffer = Vec::new();
    handle.read_to_end(&mut bytebuffer)?;
    return Ok(String::from_utf8(bytebuffer)?)
}