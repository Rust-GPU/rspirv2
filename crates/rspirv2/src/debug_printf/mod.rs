#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(deprecated)]
#![allow(missing_docs)]
#![allow(clippy::identity_op)]
#![allow(clippy::semicolon_if_nothing_returned)]
#![allow(clippy::doc_markdown)]
pub mod grammar;
pub mod inst;
pub mod inst_meta;
pub mod inst_set;

pub mod preamble {
    pub use super::grammar::*;
    pub use super::inst::*;
    pub use super::inst_meta::*;
    pub use super::inst_set::*;
    pub use crate::core::preamble::*;
}
