use crate::meta::{Capability, Extension, OperandKind};

#[derive(Copy, Clone, Debug)]
pub struct InstMeta {
    /// The name of the instruction
    pub opname: &'static str,
    /// The [`Class`] of this instruction, more informational than anything
    pub class: Option<&'static InstClass>,
    /// The u16 opcode for this instruction
    pub opcode: u16,
    /// The operands of this instruction
    pub operands: &'static [OperandMeta],
    /// required capabilities
    pub capabilities: &'static [Capability],
    /// required extensions
    pub extensions: &'static [Extension],
    /// The SPIR-V version this instruction was introduced in
    pub version: Option<&'static str>,
    /// The last SPIR-V version this instruction is valid in
    pub last_version: Option<&'static str>,
    /// Aliases for this instruction
    pub aliases: &'static [&'static str],
    /// Whether this instruction is provisional
    pub provisional: bool,
}

/// An operand of an instruction
#[derive(Copy, Clone, Debug)]
pub struct OperandMeta {
    /// The kind of operand, referencing the [`OperandKind`]s defined in [`Grammar`]
    pub kind: &'static OperandKind,
    /// Operand name
    pub name: Option<&'static str>,
    /// The repetition [`Quantifier`]
    pub quantifier: Quantifier,
}

/// How many times to repeat something?
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Ord, PartialOrd)]
pub enum Quantifier {
    #[default]
    One,
    ZeroOrOne,
    ZeroOrMore,
}

/// Informational metadata about the class of instruction
#[derive(Copy, Clone, Debug)]
pub struct InstClass {
    /// the tag, or primary key
    pub tag: &'static str,
    /// a human name for the class
    pub heading: Option<&'static str>,
}
