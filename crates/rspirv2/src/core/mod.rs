#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(deprecated)]
#![allow(clippy::identity_op)]
pub mod extensions;
pub mod grammar;
pub mod inst;
pub mod inst_class;
pub mod inst_meta;
pub mod operand_kinds;
pub mod operands;
impl preamble::AnyCapability for preamble::Capability {}
pub mod preamble {
    pub use super::extensions::*;
    pub use super::grammar::*;
    pub use super::inst::*;
    pub use super::inst_class::*;
    pub use super::inst_meta::*;
    pub use super::operand_kinds::*;
    pub use super::operands::*;
    pub use crate::binary::*;
    pub use crate::inst::*;
    pub use crate::meta::*;
    pub use crate::operand::*;
    pub use bitflags::bitflags;
    pub use smallvec::SmallVec;
}
