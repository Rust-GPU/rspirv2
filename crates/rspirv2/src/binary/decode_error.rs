use bitflags::Flags;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::string::FromUtf8Error;

#[derive(Clone, PartialEq)]
pub enum DecodeError {
    WrongOpCode {
        name: &'static str,
        expected: u16,
        actual: u16,
    },
    UnknownOpCode {
        opcode: u16,
    },
    LiteralIntegerNotLastOperand,
    /// This error must be cheap to crate, it will be discarded when iterating an `InstructionReader`.
    InstructionDecodePulledTooManyWords {
        inst_offset: usize,
        op_len: usize,
    },
    InstructionWithAdditionalOperants {
        inst_offset: usize,
    },
    InstructionWithMismatchedVariableOperants {
        inst_offset: usize,
        op_len: usize,
        expected_multiple: usize,
    },
    InstructionTooLong {
        inst_offset: usize,
        op_len: usize,
        module_remaining: usize,
    },
    InstructionZeroSized {
        inst_offset: usize,
    },
    Utf8Error(FromUtf8Error),
    InvalidBitflags {
        name: &'static str,
        unknown: u32,
        bits: u32,
    },
    UnknownEnumVariant {
        name: &'static str,
        variant: u32,
    },
}

impl Display for DecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DecodeError::WrongOpCode {
                name,
                expected,
                actual,
            } => write!(
                f,
                "Op {name} with opcode {expected} got InstructionReader with differing opcode {actual}"
            ),
            DecodeError::UnknownOpCode { opcode } => write!(f, "Unknown opcode {opcode}"),
            DecodeError::LiteralIntegerNotLastOperand => write!(
                f,
                "Implementation Limitation: The `LiteralConst` must be the last operand of an Instruction for parsing \
                 to function properly. See documentation of `LiteralConst` for details."
            ),
            DecodeError::InstructionDecodePulledTooManyWords {
                inst_offset,
                op_len,
            } => write!(
                f,
                "The Instruction at offset {inst_offset} with {op_len} param words tried to decode more Operants than \
                were available"
            ),
            DecodeError::InstructionWithAdditionalOperants { inst_offset } => write!(
                f,
                "The fixed-size Instruction at offset {inst_offset} has unexpected additional Operants"
            ),
            DecodeError::InstructionWithMismatchedVariableOperants {
                inst_offset,
                op_len,
                expected_multiple,
            } => write!(
                f,
                "The variable-sized Instruction at offset {inst_offset} has an unexpected operand length {op_len} \
                which was expected to be a multiple of {expected_multiple}"
            ),
            DecodeError::InstructionTooLong {
                inst_offset,
                op_len,
                module_remaining,
            } => write!(
                f,
                "The Instruction at offset {inst_offset} has a supposed length of {op_len} but the module only has \
                {module_remaining} words remaining"
            ),
            DecodeError::InstructionZeroSized { inst_offset } => write!(
                f,
                "The Instruction at offset {inst_offset} has an invalid length of 0, but must at least be of length 1 to include the opcode itself."
            ),
            DecodeError::Utf8Error(inner) => write!(f, "UTF-8 error: {inner}"),
            DecodeError::InvalidBitflags {
                name,
                unknown,
                bits,
            } => write!(
                f,
                "Bitflag {name} encountered unknown bits `{unknown:x}` in pattern `{bits:x}`"
            ),
            DecodeError::UnknownEnumVariant { name, variant } => {
                write!(f, "Enum {name} encountered unknown variant `{variant}`")
            }
        }
    }
}

impl Debug for DecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Error for DecodeError {}

impl From<FromUtf8Error> for DecodeError {
    fn from(value: FromUtf8Error) -> Self {
        Self::Utf8Error(value)
    }
}

impl DecodeError {
    pub fn invalid_bitflags<T: Flags<Bits = u32>>(name: &'static str, bits: u32) -> Self {
        Self::InvalidBitflags {
            name,
            unknown: bits & T::all().bits(),
            bits,
        }
    }
}
