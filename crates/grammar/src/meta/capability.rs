use std::borrow::Cow;
use std::fmt::{Debug, Display, Formatter};

/// A SPIR-V Capability
#[derive(Clone, Eq, PartialEq, serde::Deserialize)]
pub struct Capability<'a>(#[serde(borrow)] Cow<'a, str>);

impl<'a> Capability<'a> {
    pub const fn new(name: Cow<'a, str>) -> Self {
        Self(name)
    }

    pub fn name(&self) -> &str {
        &self.0
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
#[derive(Clone, Eq, PartialEq, serde::Deserialize)]
pub struct Extension<'a>(#[serde(borrow)] Cow<'a, str>);

impl<'a> Extension<'a> {
    pub const fn new(name: Cow<'a, str>) -> Self {
        Self(name)
    }

    pub fn name(&self) -> &str {
        &self.0
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
