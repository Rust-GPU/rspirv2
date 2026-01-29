use crate::parse::{Capability, Extension};
use smallvec::SmallVec;

/// See [`spirv_grammar::meta::InstructionMeta`]
#[derive(Clone, Debug, serde::Deserialize)]
pub struct InstructionMeta<'a> {
    pub opname: &'a str,
    /// The name of the [`InstructionPrintingClass`], references `Grammar.instruction_printing_class`
    pub class: Option<&'a str>,
    pub opcode: u16,
    #[serde(default)]
    pub operands: SmallVec<[OperandMeta<'a>; 4]>,
    #[serde(default)]
    pub capabilities: SmallVec<[Capability<'a>; 2]>,
    #[serde(default)]
    pub extensions: SmallVec<[Extension<'a>; 1]>,
    #[serde(default)]
    pub version: Option<&'a str>,
    #[serde(default, rename = "lastVersion")]
    pub last_version: Option<&'a str>,
    #[serde(default)]
    pub aliases: SmallVec<[&'a str; 1]>,
    #[serde(default)]
    pub provisional: bool,
}

/// See [`spirv_grammar::meta::OperandMeta`]
#[derive(Clone, Debug, serde::Deserialize)]
pub struct OperandMeta<'a> {
    /// The name of the [`OperandKind`], references `Grammar.operand_kinds`
    pub kind: &'a str,
    #[serde(borrow, default)]
    pub name: Option<&'a str>,
    #[serde(default)]
    pub quantifier: Quantifier,
}

/// How many times to repeat something?
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Ord, PartialOrd, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Quantifier {
    #[default]
    #[serde(rename = "")]
    One,
    #[serde(rename = "?")]
    ZeroOrOne,
    #[serde(rename = "*")]
    ZeroOrMore,
}

/// See [`spirv_grammar::meta::InstructionPrintingClass`]
#[derive(Clone, Debug, serde::Deserialize)]
pub struct InstructionPrintingClass<'a> {
    #[serde(borrow)]
    pub tag: &'a str,
    #[serde(borrow)]
    pub heading: Option<&'a str>,
}
