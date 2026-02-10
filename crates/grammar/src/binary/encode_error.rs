use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, PartialEq)]
pub enum EncodeError {
    /// A customizable error for [`InstructionWriter`]s, currently unused
    ///
    /// [`InstructionWriter`]: `crate::binary::InstructionWriter`
    WriterError(String),
}

impl Display for EncodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EncodeError::WriterError(err) => write!(f, "{}", err),
        }
    }
}

impl Debug for EncodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Error for EncodeError {}
