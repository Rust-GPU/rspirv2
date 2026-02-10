use crate::binary::{DecodeError, EncodeError, InstructionReader, InstructionWriter};
use crate::meta::OperandKind;
use crate::operand::{Operand, OperandEncoding, Word};
use smallvec::SmallVec;

/// A `LiteralContextDependentNumber`, or `LiteralConst` for short since it's only used by `OpConstant` (and
/// `OpSpecConstant`) instructions.
///
/// A number of content dependent length, as defined by SPIR-V spec. May represent any number of [`Word`]s, depending on
/// the type of the constant.
///
/// # Parsing Assumption
/// > We assume that `LiteralInteger` is always the last [`Operand`] in an [`Instruction`] and never has a quantity of
/// > [`Quantifier::ZeroOrMore`].
///
/// The current spec satisfies this requirement. `OpConstant` / `OpSpecConstant` are the only instructions to consume
/// a `LiteralInteger` as the last operant and expect exactly one operant.
///
/// This greatly simplifies parsing, as we can simply assume the remaining words of this instruction all contribute to
/// the constant operand. The "correct" way to handle this would be to parse the "result type id", resolve its type
/// from previously parsed instructions and compute its size from the type layout specification. This would make the
/// implementation vastly more complex and likely have a negative impact on decoding performance, which is why we
/// decided to make this assumption about SPIR-V grammars.
///
/// A `LiteralInteger` can only be decoded with [`Operand::decode_last`]. Decoding an `LiteralInteger` not as the last
/// operand, aka. calling [`Operand::decode`], will always return an Error.
///
/// [`Instruction`]: `crate::instruction::Instruction`
/// [`Quantifier::ZeroOrMore`]: `crate::meta::Quantifier`
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct LiteralConst(SmallVec<[Word; 2]>);

pub type LiteralContextDependentNumber = LiteralConst;

impl From<u64> for LiteralConst {
    #[inline]
    fn from(value: u64) -> Self {
        LiteralConst(SmallVec::from_slice(&[
            Word(value as u32),
            Word((value >> 32) as u32),
        ]))
    }
}

impl From<u32> for LiteralConst {
    #[inline]
    fn from(value: u32) -> Self {
        LiteralConst(SmallVec::from_slice(&[Word(value)]))
    }
}

macro_rules! impl_cast {
    ($ty:ty => $to:ty) => {
        impl From<$ty> for LiteralConst {
            #[inline]
            fn from(value: $ty) -> Self {
                Self::from(value as $to)
            }
        }
    };
}

impl_cast!(u8 => u32);
impl_cast!(u16 => u32);
impl_cast!(i8 => i32);
impl_cast!(i16 => i32);
impl_cast!(i32 => u32);
impl_cast!(i64 => u64);

macro_rules! impl_float {
    ($ty:ty) => {
        impl From<$ty> for LiteralConst {
            #[inline]
            fn from(value: $ty) -> Self {
                Self::from(value.to_bits())
            }
        }
    };
}

impl_float!(f32);
impl_float!(f64);

impl Operand for LiteralConst {
    const KIND: OperandKind =
        crate::core::operand_kinds::OPERAND_KIND_LITERAL_CONTEXT_DEPENDENT_NUMBER;
}

impl OperandEncoding for LiteralConst {
    const FIXED_LEN: Option<usize> = None;

    fn word_len(&self) -> usize {
        self.0.len()
    }

    fn encode(&self, writer: &mut impl InstructionWriter) -> Result<(), EncodeError> {
        writer.extend(self.0.iter().copied())
    }

    fn decode(_: &mut InstructionReader<'_>) -> Result<Self, DecodeError> {
        Err(DecodeError::LiteralIntegerNotLastOperand)
    }

    fn decode_last(reader: &mut InstructionReader<'_>) -> Result<Self, DecodeError> {
        Ok(Self(reader.collect()))
    }
}
