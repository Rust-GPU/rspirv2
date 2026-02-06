mod id;
mod literal_const;
mod literal_float;
mod literal_integer;
mod literal_string;

use crate::binary::{DecodeError, InstructionReader, InstructionWriter};
use crate::meta::OperandKind;
pub use id::*;
pub use literal_const::*;
pub use literal_float::*;
pub use literal_integer::*;
pub use literal_string::*;

/// A 32bit SPIR-V Word
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Word(pub u32);

impl Word {
    pub fn to_u32(self) -> u32 {
        self.0
    }

    pub fn to_u8_array(self) -> [u8; 4] {
        u32::to_ne_bytes(self.0)
    }
}

/// A SPIR-V operand
pub trait Operand: Sized {
    const KIND: OperandKind;

    /// Encode this `Operand` to a sequence of [`Word`]s
    fn encode(&self, writer: &mut impl InstructionWriter);

    /// Parse the `Operand` from the supplied [`Iterator`] of [`Word`]s, advancing it in the process.
    fn decode(reader: &mut InstructionReader<'_>) -> Result<Self, DecodeError>;

    /// Like [`Self::decode`], but the `Operand` is guaranteed to be the last one of this Instruction. This is used by
    /// [`LiteralInteger`] to consume all the remaining [`Word`]s and not have to calculate the exact size of a type.
    ///
    /// See [`LiteralInteger`] for details.
    fn decode_last(reader: &mut InstructionReader<'_>) -> Result<Self, DecodeError> {
        Self::decode(reader)
    }
}
