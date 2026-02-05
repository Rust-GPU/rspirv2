use super::preamble::*;
pub const GRAMMAR_EXTINST: ExtInstSetGrammar = ExtInstSetGrammar {
    grammar: Grammar {
        insts: ALL_INSTS,
        operand_kinds: ALL_OPERAND_KINDS,
        inst_class: ALL_INST_CLASSES,
    },
    version: None,
    revision: Some(1u32),
};
const ALL_INSTS: &'static [&'static InstMeta] = &[&DEBUG_PRINTF];
const ALL_OPERAND_KINDS: &'static [&'static OperandKind] = &[];
const ALL_INST_CLASSES: &'static [&'static InstClass] = &[];
