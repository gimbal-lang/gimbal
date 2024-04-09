
use std::{fmt, path::PathBuf};

use super::parser::Rule;

#[derive(Debug)]
pub enum Error {
    IoError{msg: String, path: PathBuf},
    ParseError{error: pest::error::Error<Rule>},
}

pub trait GimbalResult<T, E> {
    fn togr(self, path: &PathBuf) -> Result<T, Error>;
}

impl<T, E: GimbalParseError> GimbalResult<T, E> for Result<T, E> {
    fn togr(self, path: &PathBuf) -> Result<T, Error> {
        self.map_err(|e| e.toge(path))
    }
}

pub trait GimbalParseError {
    fn toge(self, path: &PathBuf) -> Error; 
}

impl GimbalParseError for std::io::Error {
    fn toge(self, path: &PathBuf) -> Error {
        Error::IoError { msg: self.to_string(), path: path.clone() }
    }
}

impl GimbalParseError for pest::error::Error<Rule> {
    fn toge(self, path: &PathBuf) -> Error {
        Error::ParseError { error: self.with_path(&format!("{:?}", path)) }
    }
}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IoError { msg, path } => write!(f, "{} for `{}`", msg, path.display()),
            Error::ParseError { error } => {
                write!(f, "Parsing error {}", error)},
        }
    }
}

