use core::fmt;
use std::{
    error::Error as ErrorTrait,
    io::{stdin, BufRead, BufReader},
};

use scanf::sscanf;

#[derive(Debug)]
enum Error {
    InvalidCommand(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCommand(command) => write!(f, "invalid command: {}", command),
        }
    }
}

impl ErrorTrait for Error {}

fn main() -> Result<(), Box<dyn ErrorTrait>> {
    let stdin = stdin();
    let stdin = BufReader::new(stdin);
    let mut depth: isize = 0;
    let mut horizontal_position: isize = 0;
    let mut aim: isize = 0;
    for line in stdin.lines() {
        let line = line?;
        let mut command = String::new();
        let mut difference: isize = 0;
        sscanf!(line.as_str(), "{} {}", command, difference)?;
        match command.as_str() {
            "forward" => {
                horizontal_position += difference;
                depth += aim * difference;
            }
            "down" => aim += difference,
            "up" => aim -= difference,
            _ => return Err(Box::new(Error::InvalidCommand(command))),
        };
    }
    println!("{}", horizontal_position * depth);
    Ok(())
}
