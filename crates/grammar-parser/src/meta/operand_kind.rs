#[cfg(feature = "serde")]
use crate::meta::serde_helper::num_or_hex;
use crate::meta::{Capability, Extension, OperandMeta};
use std::borrow::Cow;

/// Specifies possible [`Operand`] values, see [`Category`] variants.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct OperandKind<'a> {
    /// The name and primary key of the `OperandKind` (called `kind` in the JSON)
    #[cfg_attr(feature = "serde", serde(borrow, rename = "kind"))]
    pub name: Cow<'a, str>,
    /// The category of this `OperandKind`
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub category: Category<'a>,
    /// optional docs
    #[cfg_attr(feature = "serde", serde(borrow, default))]
    pub doc: Cow<'a, str>,
}

/// The category of an [`OperandKind`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "category"))]
pub enum Category<'a> {
    /// A bitmask of various values
    BitEnum {
        /// The possible values of the enum
        #[cfg_attr(feature = "serde", serde(borrow))]
        enumerants: Cow<'a, [Enumerant<'a>]>,
    },
    /// A composite out of 2 or more [`Operand`]s
    Composite {
        /// describes the [`OperandKind`]s this [`OperandKind`] is made out of
        #[cfg_attr(feature = "serde", serde(borrow))]
        bases: Cow<'a, [OperandKind<'a>]>,
    },
    /// The result id of another instruction
    Id,
    /// An integer, float or string literal
    Literal,
    /// A C-like enum
    ValueEnum {
        /// The possible values of the enum
        #[cfg_attr(feature = "serde", serde(borrow))]
        enumerants: Cow<'a, [Enumerant<'a>]>,
    },
}

/// A description of possible values for [`Category::BitEnum`] and [`Category::ValueEnum`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct Enumerant<'a> {
    /// the name / symbol
    #[cfg_attr(feature = "serde", serde(rename = "enumerant", borrow))]
    pub symbol: Cow<'a, str>,
    /// the value of the enumerant
    #[cfg_attr(feature = "serde", serde(deserialize_with = "num_or_hex"))]
    pub value: u32,
    /// Parameters work like tagged enums in Rust and are used for e.g. `ExecutionMode` and `OpDecorate`.
    /// Usually 0-sized, often 1 and sometimes a 3D vector.
    #[cfg_attr(feature = "serde", serde(borrow, default))]
    pub parameters: Cow<'a, [OperandMeta<'a>]>,
    /// required capabilities.
    #[cfg_attr(feature = "serde", serde(borrow, default))]
    pub capabilities: Cow<'a, [Capability<'a>]>,
    /// required extensions.
    #[cfg_attr(feature = "serde", serde(borrow, default))]
    pub extensions: Cow<'a, [Extension<'a>]>,
    /// The SPIR-V version this enumerant was introduced in
    #[cfg_attr(feature = "serde", serde(borrow, default))]
    pub version: Option<Cow<'a, str>>,
    /// The last SPIR-V version this enumerant is valid in
    #[cfg_attr(feature = "serde", serde(borrow, default, rename = "lastVersion"))]
    pub last_version: Option<Cow<'a, str>>,
    /// Aliases for this enumerant
    #[cfg_attr(feature = "serde", serde(borrow, default))]
    pub aliases: Cow<'a, [Cow<'a, str>]>,
    /// Whether this enumerant is provisional
    #[cfg_attr(feature = "serde", serde(default))]
    pub provisional: bool,
}
