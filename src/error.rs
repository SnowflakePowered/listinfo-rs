use nom::{error::ErrorKind, Err};

use alloc::borrow::ToOwned;
use alloc::string::String;
use core::fmt::{self, Display, Formatter};

/// Error type for listinfo
#[derive(Debug)]
pub enum Error {
    /// Error returned by nom when parsing fails.
    ParseError(String),
    /// Unknown or unexpected error occurred.
    UnknownError,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::ParseError(msg) => f.write_str(msg),
            Error::UnknownError => f.write_str("Unknown Error"),
        }
    }
}

#[cfg(feature = "std")]
impl alloc::error::Error for Error {}

impl From<Err<(&str, ErrorKind)>> for Error {
    fn from(err: Err<(&str, ErrorKind)>) -> Self {
        match err {
            Err::Incomplete(_) => Error::UnknownError,
            Err::Error((_, e)) => Error::ParseError(e.description().to_owned()),
            Err::Failure((_, e)) => Error::ParseError(e.description().to_owned()),
        }
    }
}
