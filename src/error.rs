use nom::{Err, error::ErrorKind};

pub enum ListInfoError {
    ParseError(String),
    UnknownError,
}

impl From<Err<(&str, ErrorKind)>> for ListInfoError {
    fn from(err: Err<(&str, ErrorKind)>) -> Self {
        match err {
            Err::Incomplete(_) => ListInfoError::UnknownError,
            Err::Error((_, e)) => ListInfoError::ParseError(e.description().to_owned()),
            Err::Failure((_, e)) => ListInfoError::ParseError(e.description().to_owned())
        }
    }
}