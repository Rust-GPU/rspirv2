mod id;
mod literal_const;
mod literal_float;
mod literal_integer;
mod literal_string;

pub use id::*;
pub use literal_const::*;
pub use literal_float::*;
pub use literal_integer::*;
pub use literal_string::*;

/// A 32bit SPIR-V Word
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Word(pub u32);
