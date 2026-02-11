use super::preamble::*;
pub const DEBUG_PRINTF: InstMeta = InstMeta {
    opname: "DebugPrintf",
    class: None,
    opcode: 1u16,
    operands: &[
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: Some("Format"),
            quantifier: Quantifier::One,
        },
        OperandSpecMeta {
            kind: &OPERAND_KIND_ID_REF,
            name: None,
            quantifier: Quantifier::ZeroOrMore,
        },
    ],
    capabilities: &[],
    extensions: &[],
    version: None,
    last_version: None,
    aliases: &[],
    provisional: false,
};
