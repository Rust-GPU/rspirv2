use crate::meta::{InstClass, InstMeta, OperandKind};
use std::ops::Deref;

/// A SPIR-V Grammar of any kind. There are only minor differences between the core SPIR-V specification and an
/// extended instruction set, such as versioning.
#[derive(Copy, Clone, Debug)]
pub struct Grammar {
    // pub copyright: &'static [&'static str],
    /// all [`Instructions`] defined by the grammar
    ///
    /// [`Instructions`]: [`InstructionMeta`]
    pub insts: &'static [&'static InstMeta],
    /// all [`OperandKind`]s defined by the grammar
    pub operand_kinds: &'static [&'static OperandKind],
    pub inst_class: &'static [&'static InstClass],
}

#[derive(Copy, Clone, Debug)]
pub struct CoreGrammar {
    pub grammar: Grammar,
    /// The SPIR-V magic number
    pub magic_number: u32,
    /// The major version, only used in the core spec
    pub major_version: u8,
    /// The major version, only used in the core spec
    pub minor_version: u8,
    /// The revision, used in both spec kinds
    pub revision: u32,
}

impl Deref for CoreGrammar {
    type Target = Grammar;

    fn deref(&self) -> &Self::Target {
        &self.grammar
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ExtInstSetGrammar {
    pub grammar: Grammar,
    /// The version, only used in extended instruction sets
    pub version: Option<u32>,
    /// The revision, used in both spec kinds
    pub revision: Option<u32>,
}

impl Deref for ExtInstSetGrammar {
    type Target = Grammar;

    fn deref(&self) -> &Self::Target {
        &self.grammar
    }
}
