use nom::Err;

use alloc::borrow::ToOwned;
use alloc::string::String;

use core::fmt::{self, Display, Formatter};

/// Error type for `listinfo` crate.
#[derive(Debug)]
pub enum Error {
    /// Error returned by the parser when parsing fails.
    ParseError(String),
    /// Error returned by serde.
    SerdeError(String),
    /// Unknown or unexpected error occurred.
    UnknownError,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::ParseError(msg) => f.write_str(msg),
            Error::SerdeError(msg) => f.write_str(msg),
            Error::UnknownError => f.write_str("Unknown Error"),
        }
    }
}

#[cfg(feature = "std")]
impl alloc::error::Error for Error {}

#[cfg(feature = "deserialize")]
impl serde::de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::SerdeError(msg.to_string())
    }
}

impl From<Err<nom::error::Error<&str>>> for Error {
    fn from(err: Err<nom::error::Error<&str>>) -> Self {
        match err {
            Err::Incomplete(_) => Error::UnknownError,
            Err::Error(e) => Error::ParseError(e.code.description().to_owned()),
            Err::Failure(e) => Error::ParseError(e.code.description().to_owned()),
        }
    }
}
