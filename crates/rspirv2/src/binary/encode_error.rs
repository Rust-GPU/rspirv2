use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, PartialEq)]
pub enum EncodeError {
    /// A customizable error
    CustomError(String),
    OpTooLong,
    OutOfIdResults,
}

impl Display for EncodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EncodeError::CustomError(err) => write!(f, "{}", err),
            EncodeError::OpTooLong => write!(f, "Op too long, u16 overflow"),
            EncodeError::OutOfIdResults => write!(f, "Out of `IdResult`"),
        }
    }
}

impl Debug for EncodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Error for EncodeError {}
