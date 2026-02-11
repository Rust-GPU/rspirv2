pub mod grammar;
pub mod inst;
pub mod inst_dyn;
pub mod inst_meta;
pub mod preamble {
    pub use super::grammar::*;
    pub use super::inst::*;
    pub use super::inst_dyn::*;
    pub use super::inst_meta::*;
    pub use crate::core::preamble::*;
}
