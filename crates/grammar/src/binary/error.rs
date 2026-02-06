use std::fmt::{Debug, Display, Formatter};
use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Clone, Error, PartialEq)]
pub enum DecodeError {
    #[error(
        "Implementation Limitation: The `LiteralConst` must be the last operand of an Instruction for parsing to \
        function properly. See documentation of `LiteralConst` for details."
    )]
    LiteralIntegerNotLastOperand,
    /// This error must be cheap to crate, it will be discarded when iterating an `InstructionReader`.
    #[error(
        "The Instruction at offset {inst_offset} with {op_len} param words tried to decode more Operants than \
    were available"
    )]
    InstructionDecodePulledTooManyWords { inst_offset: usize, op_len: usize },
    #[error("The fixed-size Instruction at offset {inst_offset} has extra Operants")]
    InstructionWithAdditionalOperants { inst_offset: usize },
    #[error(
        "The variable-sized Instruction at offset {inst_offset} has an unexpected operand length {op_len} which \
    was expected to be a multiple of {expected_multiple}"
    )]
    InstructionWithMismatchedVariableOperants {
        inst_offset: usize,
        op_len: usize,
        expected_multiple: usize,
    },
    #[error(
        "The Instruction at offset {inst_offset} has a supposed length of {op_len} but the module only has \
        {module_remaining} words remaining"
    )]
    InstructionTooLong {
        inst_offset: usize,
        op_len: usize,
        module_remaining: usize,
    },
    #[error("UTF-8 error: {0}")]
    Utf8Error(#[from] FromUtf8Error),
}

impl Debug for DecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
