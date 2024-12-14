use std::{
    env::{self},
    fmt::Display,
};
#[derive(Debug)]
pub enum ParsingError {
    InvalidFilename,
    NoFilename,
}

pub fn cmd_args() -> Result<String, ParsingError> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        return Err(ParsingError::NoFilename);
    }
    let filename = &args[1];
    if !filename.contains(".png") {
        return Err(ParsingError::InvalidFilename);
    }
    println!("Filename: {}", filename);
    Ok(filename.to_string())
}

impl std::error::Error for ParsingError {}

impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingError::InvalidFilename => {
                write!(f, "Couldn't parse the filename. Please choose anoter one")
            }
            ParsingError::NoFilename => {
                write!(f, "No filename was provided")
            }
        }
    }
}
