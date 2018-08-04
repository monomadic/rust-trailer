#[derive(Debug)]
pub enum ServerError {
    RequestError(String),
}

use std::error::Error;
use std::fmt::{self, Debug};
impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self, f)
    }
}
impl Error for ServerError {
    fn description(&self) -> &str { "error" }
}
