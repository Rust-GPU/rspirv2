use bitflags::Flags;
use std::error::Error;
use std::ffi::FromBytesUntilNulError;
use std::fmt::{Debug, Display, Formatter};
use std::str::Utf8Error;
use std::string::FromUtf8Error;

#[derive(Clone)]
pub enum DecodeError {
    UnknownOpCode {
        opcode: u16,
    },
    WrongOpCode {
        name: &'static str,
        expected: u16,
        actual: u16,
    },
    LiteralConstNotLastOperand,
    LiteralConstOfWrongWordSize {
        expected_size: usize,
        actual_size: usize,
    },
    LiteralConstTooLarge {
        value: u32,
        bits: usize,
    },
    /// This error must be cheap to crate, it will be discarded when iterating an `InstructionReader`.
    InstructionDecodePulledTooManyWords {
        op_len: usize,
    },
    InstructionWithAdditionalOperants {
        op_len: usize,
        remaining: usize,
    },
    InstructionWithMismatchedVariableOperants {
        op_len: usize,
        expected_multiple: usize,
    },
    InstructionTooLong {
        op_len: usize,
        module_remaining: usize,
    },
    InstructionZeroSized,
    StringNotNulTerminated,
    Utf8Error(Utf8Error),
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
            Self::UnknownOpCode { opcode } => write!(
                f,
                "Instruction Set couldn't decode instruction with unknown opcode {opcode}"
            ),
            Self::WrongOpCode {
                name,
                expected,
                actual,
            } => write!(
                f,
                "Op {name} with opcode {expected} got InstructionReader with differing opcode {actual}."
            ),
            Self::LiteralConstNotLastOperand => write!(
                f,
                "Implementation Limitation: The `LiteralConst` must be the last operand of an Instruction for parsing \
                 to function properly. See documentation of `LiteralConst` for details."
            ),
            Self::LiteralConstOfWrongWordSize {
                expected_size,
                actual_size,
            } => write!(
                f,
                "Tried to read value from LiteralConst that expects {expected_size} Word(s) but \
                LiteralConst has {actual_size} Word(s)"
            ),
            Self::LiteralConstTooLarge { value, bits } => write!(
                f,
                "LiteralConst's value `{value:x}` is too large for {} bits",
                bits
            ),
            Self::InstructionDecodePulledTooManyWords { op_len } => write!(
                f,
                "Instruction with {op_len} param words tried to decode more Operants than were available."
            ),
            Self::InstructionWithAdditionalOperants { op_len, remaining } => write!(
                f,
                "The fixed-size Instruction with {op_len} param words has {remaining} Words left over after decoding."
            ),
            Self::InstructionWithMismatchedVariableOperants {
                op_len,
                expected_multiple,
            } => write!(
                f,
                "The variable-sized Instruction has an unexpected operand length {op_len} \
                which was expected to be a multiple of {expected_multiple}."
            ),
            Self::InstructionTooLong {
                op_len,
                module_remaining,
            } => write!(
                f,
                "Instruction has a supposed length of {op_len} but the module only has {module_remaining} words \
                 remaining."
            ),
            Self::InstructionZeroSized => write!(
                f,
                "Instruction has an invalid length of 0, must be least 1 word as it includes the opcode itself."
            ),
            Self::StringNotNulTerminated => write!(f, "String is not null-terminated."),
            Self::Utf8Error(error) => write!(f, "Invalid UTF-8 string: {error}"),
            Self::InvalidBitflags {
                name,
                unknown,
                bits,
            } => write!(
                f,
                "Bitflag {name} encountered unknown bits `{unknown:x}` in pattern `{bits:x}`."
            ),
            Self::UnknownEnumVariant { name, variant } => {
                write!(f, "Enum {name} encountered unknown variant `{variant}`.")
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

impl DecodeError {
    #[inline]
    pub fn invalid_bitflags<T: Flags<Bits = u32>>(name: &'static str, bits: u32) -> Self {
        Self::InvalidBitflags {
            name,
            unknown: bits & T::all().bits(),
            bits,
        }
    }
}

impl From<FromBytesUntilNulError> for DecodeError {
    fn from(_: FromBytesUntilNulError) -> Self {
        Self::StringNotNulTerminated
    }
}

impl From<Utf8Error> for DecodeError {
    fn from(value: Utf8Error) -> Self {
        Self::Utf8Error(value)
    }
}

impl From<FromUtf8Error> for DecodeError {
    fn from(value: FromUtf8Error) -> Self {
        Self::Utf8Error(value.utf8_error())
    }
}
