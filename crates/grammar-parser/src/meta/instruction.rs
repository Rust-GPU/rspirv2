use crate::meta::{Capability, Extension, OperandKind};
use std::borrow::Cow;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct InstructionMeta<'a> {
    /// The name of the instruction
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub opname: Cow<'a, str>,
    /// The [`Class`] of this instruction, more informational than anything
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub class: Option<Cow<'a, InstructionPrintingClass<'a>>>,
    /// The u16 opcode for this instruction
    pub opcode: u16,
    /// The operands of this instruction
    #[cfg_attr(feature = "serde", serde(borrow, default))]
    pub operands: Cow<'a, [OperandMeta<'a>]>,
    /// required capabilities
    #[cfg_attr(feature = "serde", serde(borrow, default))]
    pub capabilities: Cow<'a, [Capability<'a>]>,
    /// required extensions
    #[cfg_attr(feature = "serde", serde(borrow, default))]
    pub extensions: Cow<'a, [Extension<'a>]>,
    /// The SPIR-V version this instruction was introduced in
    #[cfg_attr(feature = "serde", serde(borrow, default))]
    pub version: Option<Cow<'a, str>>,
    /// The last SPIR-V version this instruction is valid in
    #[cfg_attr(feature = "serde", serde(borrow, default, rename = "lastVersion"))]
    pub last_version: Option<Cow<'a, str>>,
    /// Aliases for this instruction
    #[cfg_attr(feature = "serde", serde(borrow, default))]
    pub aliases: Cow<'a, [Cow<'a, str>]>,
    /// Whether this instruction is provisional
    #[cfg_attr(feature = "serde", serde(default))]
    pub provisional: bool,
}

/// An operand of an instruction
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct OperandMeta<'a> {
    /// The kind of operand, referencing the [`OperandKind`]s defined in [`Grammar`]
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub kind: Cow<'a, OperandKind<'a>>,
    /// Operand name
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub name: Cow<'a, str>,
    /// The repetition [`Quantifier`]
    #[cfg_attr(feature = "serde", serde(default))]
    pub quantifier: Quantifier,
}

/// How many times to repeat something?
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub enum Quantifier {
    #[default]
    #[cfg_attr(feature = "serde", serde(rename = ""))]
    One,
    #[cfg_attr(feature = "serde", serde(rename = "?"))]
    ZeroOrOne,
    #[cfg_attr(feature = "serde", serde(rename = "*"))]
    ZeroOrMore,
}

/// Informational metadata about the class of instruction
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct InstructionPrintingClass<'a> {
    /// the tag, or primary key
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub tag: Cow<'a, str>,
    /// a human name for the class
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub heading: Option<Cow<'a, str>>,
}
