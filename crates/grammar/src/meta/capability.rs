use std::fmt::{Debug, Display, Formatter};

/// A SPIR-V Capability
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capability(&'static str);

impl Capability {
    pub const fn new(name: &'static str) -> Self {
        Self(name)
    }

    pub const fn name(&self) -> &'static str {
        self.0
    }
}

impl Display for Capability {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Capability({})", self.0)
    }
}

impl Debug for Capability {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

/// A SPIR-V Extension
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Extension(&'static str);

impl Extension {
    pub const fn new(name: &'static str) -> Self {
        Self(name)
    }

    pub const fn name(&self) -> &'static str {
        self.0
    }
}

impl Display for Extension {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Extension({})", self.0)
    }
}

impl Debug for Extension {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
