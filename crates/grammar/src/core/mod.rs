pub mod capabilities;
pub mod extensions;
pub mod grammar;
pub mod inst;
pub mod inst_class;
pub mod operand_kinds;
pub mod preamble {
    pub use super::capabilities::*;
    pub use super::extensions::*;
    pub use super::grammar::*;
    pub use super::inst::*;
    pub use super::inst_class::*;
    pub use super::operand_kinds::*;
    pub use crate::meta::*;
}
