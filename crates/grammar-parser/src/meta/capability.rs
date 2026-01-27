use std::borrow::{Borrow, Cow};
use std::fmt::{Debug, Display, Formatter};

/// A SPIR-V Capability
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct Capability<'a>(#[cfg_attr(feature = "serde", serde(borrow))] Cow<'a, str>);

impl<'a> Capability<'a> {
    pub fn new(name: impl Into<Cow<'a, str>>) -> Self {
        Self(name.into())
    }

    pub fn name(&self) -> &str {
        self.0.borrow()
    }
}

impl Display for Capability<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Capability({})", self.0)
    }
}

impl Debug for Capability<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

/// A SPIR-V Extension
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct Extension<'a>(#[cfg_attr(feature = "serde", serde(borrow))] Cow<'a, str>);

impl<'a> Extension<'a> {
    pub fn new(name: impl Into<Cow<'a, str>>) -> Self {
        Self(name.into())
    }

    pub fn name(&self) -> &str {
        self.0.borrow()
    }
}

impl Display for Extension<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Extension({})", self.0)
    }
}

impl Debug for Extension<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
