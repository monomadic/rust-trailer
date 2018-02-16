#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug)]
pub struct TrailerError {
    pub error_type: TrailerErrorType,
    pub message: String,
}

#[derive(Debug)]
pub enum TrailerErrorType {
    ImportError,
    APIError,
    CommandError,
    ConfigError,
    Unsupported,
}

impl TrailerError {
    pub fn unsupported() -> Self {
        Self {
            error_type: TrailerErrorType::Unsupported,
            message: "This feature is not supported by the selected exchange.".into(),
        }
    }
}

use bittrex::error::BittrexError as BittrexError;
impl From<BittrexError> for TrailerError {
    fn from(error: BittrexError) -> Self {
        TrailerError {
            error_type: TrailerErrorType::APIError,
            message: error.message,
        }
    }
}

impl From<::std::num::ParseFloatError> for TrailerError {
    fn from(error: ::std::num::ParseFloatError) -> Self {
        TrailerError {
            error_type: TrailerErrorType::CommandError,
            message: "One or more provided parameters could not be converted to valid floats.".into(),
        }
    }
}

impl From<::std::num::ParseIntError> for TrailerError {
    fn from(_error: ::std::num::ParseIntError) -> Self {
        TrailerError {
            error_type: TrailerErrorType::CommandError,
            message: "One or more provided parameters could not be converted to valid integers.".into(),
        }
    }
}