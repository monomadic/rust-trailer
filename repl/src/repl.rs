#![allow(dead_code)]
#![allow(unused_variables)]

// use trailer;
// use trailer::exchanges::*;
use trailer::error::*;
use commands::*;

pub fn run() -> Result<(), TrailerError> {
    loop {
        print!("CMD> ");
        match get_command() {
            Ok(_) => println!("ok"),
            Err(e) => println!("err in command: {:?}", e),
        }
    }
}

pub fn exec_command(command: Command) -> Result<(), TrailerError> {
    match command {
        Command::Test => println!("test command!"),
        _ => println!("default command"),
    }

    Ok(())
}

pub fn parse_command(command: &str) -> Result<Command, TrailerError> {
    let mut commands = command.split_whitespace();
    // println!("command: {}", commands.collect::<Vec<String>>()[0]);
    
    Ok(Command::Test)
}

pub fn get_command() -> Result<(), TrailerError> {
    use std::io::{stdin,stdout,Write};

    let mut s=String::new();
    let _=stdout().flush();

    let input_char_length = stdin().read_line(&mut s)?;

    if input_char_length == 1 {
        return Err(TrailerError::generic("invalid command"));
    }

    // trim
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }

    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }

    Ok(exec_command(parse_command(&s)?)?)
}