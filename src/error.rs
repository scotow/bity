use std::num::ParseIntError;
use std::error::Error as StdError;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Error<'s> {
    NotAscii,
    ParseIntError(&'s str, ParseIntError),
    InvalidUnit(&'s str),
}

impl Display for Error<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::NotAscii => write!(f, "input must be ascii"),
            Error::ParseIntError(input, _) => write!(f, "invalid number \"{input}\""),
            Error::InvalidUnit(input) => write!(f, "invalid unit \"{input}\""),
        }
    }
}

impl StdError for Error<'_> {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::NotAscii => None,
            Error::ParseIntError(_, err) => Some(err),
            Error::InvalidUnit(_) => None,
        }
    }
}