//! Closely follows the struct definitions in `rspirv2_types::meta`, but represents the SPIR-V JSON much more closely:
//! * no `'static` lifetimes
//!   * strings use a `'a` lifetime to borrow string from the JSON
//!   * slices use [`Vec`] and [`SmallVec`]
//! * structs are not `Copy`, only `Clone`
//! * no self-references within the struct, e.g. [`InstMeta`]`.class` is a string, like in the JSON, instead of
//!   referencing a [`InstClass`] directly
//!
//! [`SmallVec`]: smallvec::SmallVec

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

/// Source of the object, decides whether to generate the definition of the object or not.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum Source {
    /// The object was created in this build and should generate a declaration.
    #[default]
    Declare,
    /// Only codegen metadata declarations, do NOT codegen any implementation.
    MetaOnly,
    /// The object was imported from an external declaration, usually from another crate.
    ///
    /// Instead of duplicating the declaration, the imported declaration should be used. It is up to the crate using the
    /// generated source code to reexport / `pub use` the symbol within the `preamble` module.
    Import,
}

impl Source {
    /// Whether to codegen metadata declarations
    pub fn codegen_meta(&self) -> bool {
        match self {
            Source::Declare | Source::MetaOnly => true,
            Source::Import => false,
        }
    }

    /// Whether to codegen any implementations
    pub fn codegen_impl(&self) -> bool {
        match self {
            Source::Declare => true,
            Source::MetaOnly | Source::Import => false,
        }
    }
}

/// A type declared at compile-time that can be imported into some runtime object
pub trait Import {
    type Imported: Sized;

    fn import(&self) -> Self::Imported;
}
