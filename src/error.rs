#[derive(Debug)]
pub struct TrailerError {
    pub error_type: TrailerErrorType,
    pub message: String,
}

#[derive(Debug)]
pub enum TrailerErrorType {
    ImportError,
    APIError,
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
