#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(deprecated)]
#![allow(missing_docs)]
#![allow(clippy::identity_op)]
#![allow(clippy::semicolon_if_nothing_returned)]
#![allow(clippy::doc_markdown)]
pub mod extensions;
pub mod grammar;
pub mod inst;
pub mod inst_class;
pub mod inst_meta;
pub mod inst_set;
pub mod operand_kinds;
pub mod operands;
impl preamble::AnyCapability for preamble::Capability {}
pub mod preamble {
    pub use super::extensions::*;
    pub use super::grammar::*;
    pub use super::inst::*;
    pub use super::inst_class::*;
    pub use super::inst_meta::*;
    pub use super::inst_set::*;
    pub use super::operand_kinds::*;
    pub use super::operands::*;
    pub use crate::binary::*;
    pub use crate::dis::*;
    pub use crate::inst::*;
    pub use crate::meta::*;
    pub use crate::operand::*;
    pub use crate::*;
    pub use bitflags::bitflags;
    pub use smallvec::SmallVec;
    pub use std::fmt::Formatter;
}
