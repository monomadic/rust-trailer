#![allow(dead_code)]
#![allow(unused_variables)]

use colored::*;
use trailer::models::*;
use trailer::error::*;

pub fn error(error: TrailerError) {
    println!("{}", format!("Error: {}", error.message).red());
}
