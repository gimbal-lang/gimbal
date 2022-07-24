pub mod schema;
pub mod drivers;

use std::fmt;
use std::error;

#[derive(Debug)]
pub struct DatabaseError {
    source_message: String
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("I got an error from the database: {}", self.source_message))
        /*
        match self {
            Self::ConnectionError{source_message} => f.write_str(&format!("I received a connection error from the database: {}", source_message)),
            _ => f.write_str("Some kind of database error")
        }
        */
    }
}

#[derive(Debug)]
pub struct Table {
    name: String,
    columns: Vec<Column>
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Column {
    name: String,
    column_type: ColumnType
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum ColumnType {
    String,
    Integer
}

impl fmt::Display for ColumnType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
