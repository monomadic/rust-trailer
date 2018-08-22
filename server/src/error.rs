use trailer::error::TrailerError;
use std::error::Error;
use std::fmt::{self, Debug};

#[derive(Debug)]
pub enum ServerError {
    RequestError(String),
    TrailerError(TrailerError),
    TemplateError(::horrorshow::Error)
}
impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl Error for ServerError {
    fn description(&self) -> &str { "error" }
}

impl From<TrailerError> for ServerError {
    fn from(error: TrailerError) -> Self {
        ServerError::TrailerError(error)
    }
}

impl From<::horrorshow::Error> for ServerError {
    fn from(error: ::horrorshow::Error) -> Self {
        ServerError::TemplateError(error)
    }
}
