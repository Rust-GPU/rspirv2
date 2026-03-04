use crate::meta::{AnyCapability, Extension, OperandSpecMeta};

/// Specifies possible Operand values, see [`Category`] variants.
#[derive(Copy, Clone, Debug)]
pub struct OperandKind {
    /// The name and primary key of the `OperandKind` (called `kind` in the JSON)
    pub name: &'static str,
    /// The category of this `OperandKind`
    pub category: Category,
    /// optional docs
    pub doc: &'static str,
}

/// The category of an [`OperandKind`]
#[derive(Copy, Clone, Debug)]
pub enum Category {
    /// A bitmask of various values
    BitEnum {
        /// The possible values of the enum
        enumerants: &'static [Enumerant],
    },
    /// A composite out of 2 or more Operands
    Composite {
        /// describes the [`OperandKind`]s this [`OperandKind`] is made out of
        bases: &'static [&'static OperandKind],
    },
    /// The result id of another instruction
    Id,
    /// An integer, float or string literal
    Literal,
    /// A C-like enum
    ValueEnum {
        /// The possible values of the enum
        enumerants: &'static [Enumerant],
    },
}

/// A description of possible values for [`Category::BitEnum`] and [`Category::ValueEnum`]
#[derive(Copy, Clone, Debug)]
pub struct Enumerant {
    /// the name / symbol
    pub symbol: &'static str,
    /// the value of the enumerant
    pub value: u32,
    /// Parameters work like tagged enums in Rust and are used for e.g. `ExecutionMode` and `OpDecorate`.
    /// Usually 0-sized, often 1 and sometimes a 3D vector.
    pub parameters: &'static [OperandSpecMeta],
    /// required capabilities.
    pub capabilities: &'static [&'static dyn AnyCapability],
    /// required extensions.
    pub extensions: &'static [Extension],
    /// The SPIR-V version this enumerant was introduced in
    pub version: Option<&'static str>,
    /// The last SPIR-V version this enumerant is valid in
    pub last_version: Option<&'static str>,
    /// Aliases for this enumerant
    pub aliases: &'static [&'static str],
    /// Whether this enumerant is provisional
    pub provisional: bool,
}
