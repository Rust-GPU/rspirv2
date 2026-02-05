//! Closely follows the struct definitions in [`spirv_grammar::meta`], but represents the SPIR-V JSON much more closely:
//! * no `'static` lifetimes
//!   * strings use a `'a` lifetime to borrow string from the JSON
//!   * slices use [`Vec`] and [`SmallVec`]
//! * structs are not `Copy`, only `Clone`
//! * no self-references within the struct, e.g. [`InstMeta`]`.class` is a string, like in the JSON, instead of
//!   referencing a [`InstClass`] directly
//!
//! [`SmallVec`]: `smallvec::SmallVec`

mod capability;
mod extension;
mod files;
mod grammar;
mod instruction;
mod operand_kind;
mod serde_helper;

pub use capability::*;
pub use extension::*;
pub use files::*;
pub use grammar::*;
pub use instruction::*;
pub use operand_kind::*;
