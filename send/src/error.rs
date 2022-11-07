use std::error::Error;
use std::fmt;
use std::io;
use std::num;
#[derive(Debug)]
pub struct SendError {
    kind: String,
    message: String,
}

impl SendError {
    pub fn new(kind: &str, message: &str) -> Self {
        Self {
            kind: kind.to_string(),
            message: message.to_string(),
        }
    }
}
/// fix :doesn't satisfy `SendError: std::error::Error`
impl Error for SendError {}
impl fmt::Display for SendError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.kind, self.message)
    }
}

// Implement std::convert::From for AppError; from io::Error
impl From<io::Error> for SendError {
    fn from(error: io::Error) -> Self {
        SendError {
            kind: String::from("io"),
            message: error.to_string(),
        }
    }
}

// Implement std::convert::From for AppError; from mslnk::MSLinkError
impl From<mslnk::MSLinkError> for SendError {
    fn from(error: mslnk::MSLinkError) -> Self {
        SendError {
            kind: String::from("mslnk"),
            message: error.to_string(),
        }
    }
}

// Implement std::convert::From for AppError; from num::ParseIntError
impl From<num::ParseIntError> for SendError {
    fn from(error: num::ParseIntError) -> Self {
        SendError {
            kind: String::from("parse"),
            message: error.to_string(),
        }
    }
}
