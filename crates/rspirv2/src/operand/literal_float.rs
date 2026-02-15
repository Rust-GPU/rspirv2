use crate::binary::{DecodeError, EncodeError, InstructionReader, InstructionWriter};
use crate::meta::OperandKind;
use crate::operand::{Operand, OperandEncoding, Word};

/// A float literal as defined by SPIR-V spec: simply an `f32`. It's represented as a `u32` to ensure the bit pattern
/// isn't changing.
///
/// This is only used for some obscure Intel extensions, all other float types use [`LiteralInteger`] for backing
/// literal storage.
///
/// [`LiteralInteger`]: crate::operand::LiteralInteger
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LiteralFloat(pub Word);

impl LiteralFloat {
    pub fn new(value: f32) -> Self {
        Self(Word(value.to_bits()))
    }

    pub fn from_word(value: Word) -> Self {
        Self(value)
    }

    pub fn to_word(&self) -> Word {
        self.0
    }

    pub fn to_f32(&self) -> f32 {
        f32::from_bits(self.0.0)
    }
}

unsafe impl Operand for LiteralFloat {
    const KIND: &OperandKind = &crate::core::operand_kinds::OPERAND_KIND_LITERAL_FLOAT;
}

unsafe impl OperandEncoding for LiteralFloat {
    const FIXED_LEN: Option<usize> = Some(1);

    fn encode(&self, writer: &mut impl InstructionWriter) -> Result<(), EncodeError> {
        writer.write(self.0);
        Ok(())
    }

    fn decode(reader: &mut InstructionReader<'_>) -> Result<Self, DecodeError> {
        Ok(Self(reader.pull()?))
    }
}
