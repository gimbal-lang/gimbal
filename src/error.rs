use std::fmt;

use crate::GimbalString;

#[derive(Debug)]
pub struct Error(String);

impl Error {
    pub fn new(msg: &str) -> Self {
        Error(msg.tos())
    }

    pub fn no_app() -> Self {
        Error("There is no app loaded".tos())
    }
    
    pub(crate) fn no_module(module: &str) -> Error {
        Error(format!("There is no module called `{}`", module))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error(value.to_string())
    }
}

impl From<crate::lang::error::Error> for Error {
    fn from(value: crate::lang::error::Error) -> Self {
        Error(value.to_string())
    }
}

impl From<pest::error::Error<crate::lang::parser::Rule>> for Error {
    fn from(value: pest::error::Error<crate::lang::parser::Rule>) -> Self {
        Error(value.to_string())
    }
}

impl From<pest::error::Error<crate::repl::Rule>> for Error {
    fn from(value: pest::error::Error<crate::repl::Rule>) -> Self {
        Error(value.to_string())
    }
}