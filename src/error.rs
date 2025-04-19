use std::fmt;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Error {
    UnknownUnit(String),
    InvalidValue(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::UnknownUnit(unit) => write!(f, "Unknown unit: {unit}"),
            Error::InvalidValue(value) => write!(f, "Invalid value: {value}"),
        }
    }
}

impl std::error::Error for Error {}
