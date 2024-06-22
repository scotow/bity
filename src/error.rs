use std::{
    error::Error as StdError,
    fmt,
    fmt::{Display, Formatter},
    num::ParseIntError,
};

/// Errors that can happen when using parsing functions.
#[derive(Debug, Clone)]
pub enum Error<'s> {
    /// The input string is not fully ASCII.
    NotAscii,
    /// The unit string is invalid.
    InvalidUnit(&'s str),
    /// The numeric part of the input could not be parsed.
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
            Error::ParseIntError(_, err) => {
                err.as_ref().map(|err| err as &(dyn StdError + 'static))
            }
            Error::InvalidUnit(_) => None,
        }
    }
}
