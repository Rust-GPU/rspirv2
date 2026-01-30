use crate::parse::{Capability, Extension};
use smallvec::SmallVec;
use spirv_grammar::meta::Quantifier;
use std::borrow::Cow;

/// See [`spirv_grammar::meta::InstMeta`]
#[derive(Clone, Debug, serde::Deserialize)]
pub struct InstMeta<'a> {
    #[serde(borrow)]
    pub opname: Cow<'a, str>,
    /// The name of the [`InstClass`], references `Grammar.instruction_printing_class`
    #[serde(borrow)]
    pub class: Option<Cow<'a, str>>,
    pub opcode: u16,
    #[serde(borrow, default)]
    pub operands: SmallVec<[OperandMeta<'a>; 4]>,
    #[serde(borrow, default)]
    pub capabilities: SmallVec<[Capability<'a>; 2]>,
    #[serde(borrow, default)]
    pub extensions: SmallVec<[Extension<'a>; 1]>,
    #[serde(borrow, default)]
    pub version: Option<Cow<'a, str>>,
    #[serde(borrow, default, rename = "lastVersion")]
    pub last_version: Option<Cow<'a, str>>,
    #[serde(borrow, default)]
    pub aliases: SmallVec<[Cow<'a, str>; 1]>,
    #[serde(default)]
    pub provisional: bool,
}

/// See [`spirv_grammar::meta::OperandMeta`]
#[derive(Clone, Debug, serde::Deserialize)]
pub struct OperandMeta<'a> {
    /// The name of the [`OperandKind`], references `Grammar.operand_kinds`
    #[serde(borrow)]
    pub kind: Cow<'a, str>,
    #[serde(borrow, default)]
    pub name: Option<Cow<'a, str>>,
    #[serde(default)]
    pub quantifier: Quantifier,
}

/// See [`spirv_grammar::meta::InstClass`]
#[derive(Clone, Debug, serde::Deserialize)]
pub struct InstClass<'a> {
    #[serde(borrow)]
    pub tag: Cow<'a, str>,
    #[serde(borrow)]
    pub heading: Option<Cow<'a, str>>,
}
