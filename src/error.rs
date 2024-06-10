use std::num::ParseIntError;
use std::error::Error as StdError;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Error<'s> {
    NotAscii,
    InvalidUnit(&'s str),
    ParseIntError(&'s str, Option<ParseIntError>),
}

impl Display for Error<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::NotAscii => write!(f, "input must be ascii"),
            Error::InvalidUnit(input) => write!(f, r#"invalid unit "{input}""#),
            Error::ParseIntError(input, _) => write!(f, r#"invalid number "{input}""#),
        }
    }
}

impl StdError for Error<'_> {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::NotAscii => None,
            Error::ParseIntError(_, err) => err.as_ref().map(|err| err as &(dyn StdError + 'static)),
            Error::InvalidUnit(_) => None,
        }
    }
}