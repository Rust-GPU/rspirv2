use crate::parse::serde_helper::num_or_hex;
use crate::parse::{InstructionMeta, InstructionPrintingClass, OperandKind};
use std::ops::Deref;

/// See [`spirv_grammar::meta::Grammar`]
#[derive(Clone, Debug, serde::Deserialize)]
pub struct Grammar<'a> {
    // ignore the copyright
    // #[serde(borrow, default)]
    // pub copyright: Vec<&'a str>,
    #[serde(borrow, default)]
    pub instructions: Vec<InstructionMeta<'a>>,
    #[serde(borrow, default)]
    pub operand_kinds: Vec<OperandKind<'a>>,
    #[serde(borrow, default)]
    pub instruction_printing_class: Vec<InstructionPrintingClass<'a>>,
}

/// See [`spirv_grammar::meta::CoreGrammar`]
#[derive(Clone, Debug, serde::Deserialize)]
pub struct CoreGrammar<'a> {
    #[serde(borrow, flatten)]
    pub grammar: Grammar<'a>,
    #[serde(deserialize_with = "num_or_hex")]
    pub magic_number: u32,
    pub major_version: u8,
    pub minor_version: u8,
    pub revision: u32,
}

impl<'a> Deref for CoreGrammar<'a> {
    type Target = Grammar<'a>;

    fn deref(&self) -> &Self::Target {
        &self.grammar
    }
}

/// See [`spirv_grammar::meta::ExtInstSetGrammar`]
#[derive(Clone, Debug, serde::Deserialize)]
pub struct ExtInstSetGrammar<'a> {
    #[serde(borrow, flatten)]
    pub grammar: Grammar<'a>,
    pub version: Option<u32>,
    pub revision: Option<u32>,
}

impl<'a> Deref for ExtInstSetGrammar<'a> {
    type Target = Grammar<'a>;

    fn deref(&self) -> &Self::Target {
        &self.grammar
    }
}
