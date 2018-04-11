use std::fmt::{self, Debug};

#[derive(Debug)]
pub enum BotError {
    SetupError,
}

impl fmt::Display for BotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self, f)
    }
}