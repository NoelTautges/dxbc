use std::str::Utf8Error;
use std::string::FromUtf8Error;
use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    StreamExpected(usize),
    LimitReached(usize),
    DecodeStringFailed(usize, FromUtf8Error),
    DecodeStrFailed(usize, Utf8Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::DecodeStringFailed(index, ref e) => {
                write!(f, "cannot decode string at index {}: {}", index, e)
            }
            _ => write!(f, "unimplemented"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        // TODO: add descriptions for other errors
        #[allow(clippy::match_single_binding)]
        match *self {
            _ => "unknown operand value for the given kind",
        }
    }
}
