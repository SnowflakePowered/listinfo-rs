use nom::{Err, error::ErrorKind};

/// Error type for listinfo
pub enum Error {
    /// Error returned by nom when parsing fails.
    ParseError(String),
    /// Unknown or unexpected error occurred.
    UnknownError,
}

impl From<Err<(&str, ErrorKind)>> for Error {
    fn from(err: Err<(&str, ErrorKind)>) -> Self {
        match err {
            Err::Incomplete(_) => Error::UnknownError,
            Err::Error((_, e)) => Error::ParseError(e.description().to_owned()),
            Err::Failure((_, e)) => Error::ParseError(e.description().to_owned())
        }
    }
}