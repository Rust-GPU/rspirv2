use std::ops::{Deref, DerefMut};

/// A SPIR-V String literal. Defined as a sequence of UTF-8, so we can just use an ordinary [`String`].
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct LiteralString(pub String);

impl LiteralString {
    pub fn new(str: String) -> Self {
        Self(str)
    }

    pub fn into_string(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Deref for LiteralString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LiteralString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
