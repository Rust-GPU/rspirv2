mod id;
mod literal_const;
mod literal_float;
mod literal_integer;
mod literal_string;

use crate::binary::{DecodeError, EncodeError, InstructionReader, InstructionWriter, WordCounter};
use crate::meta::{OperandKind, Quantifier};
pub use id::*;
pub use literal_const::*;
pub use literal_float::*;
pub use literal_integer::*;
pub use literal_string::*;
use smallvec::SmallVec;

/// A 32bit SPIR-V Word
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Word(pub u32);

impl Word {
    pub fn new_op(op: u16, len: usize) -> Result<Self, EncodeError> {
        let len = u16::try_from(len).map_err(|_e| EncodeError::OpTooLong)?;
        Ok(Self(op as u32 | ((len as u32) << 16)))
    }

    pub fn to_op(&self) -> (u16, usize) {
        (self.0 as u16, (self.0 >> 16) as u16 as usize)
    }

    pub fn to_u32(self) -> u32 {
        self.0
    }

    pub fn to_u8_array(self) -> [u8; 4] {
        u32::to_ne_bytes(self.0)
    }
}

/// A SPIR-V operand. The associated const [`Self::KIND`] links to it's [`OperandKind`].
///
/// Requires [`OperandEncoding`], see that for encoding and decoding SPIR-V.
pub trait Operand: OperandEncoding {
    const KIND: &OperandKind;
}

/// A `OperandSpec` is an [`Operand`] with a [`Quantifier`] to describe the repetition of the [`Operand`].
///
/// Any [`Operand`] implicitly implements this with [`Quantifier::One`], wrapping an Operand in [`Option`] will get a
/// [`Quantifier::ZeroOrOne`] and wrapping it in a [`Vec`] or [`SmallVec`] will have a [`Quantifier::ZeroOrMore`].
pub trait OperandSpec: OperandEncoding {
    /// The [`Operand`]
    type Operand: Operand;
    /// The [`Quantifier`] or repetition factor of the [`Self::Operand`]
    const QUANTIFIER: Quantifier;
}

impl<T: Operand> OperandSpec for T {
    type Operand = Self;
    const QUANTIFIER: Quantifier = Quantifier::One;
}

/// Something that can be decoded from or encoded to SPIR-V, not necessarily a full [`Operand`].
///
/// Both [`Option`] and [`Vec`] implement `OperandEncoding` but not [`Operand`]. This allows for an easier
/// representation of [`OperandSpecMeta`]s with [`Quantifier`] of [`Quantifier::ZeroOrOne`] (`Option`) and
/// [`Quantifier::ZeroOrMore`] (`Vec`).
///
/// [`OperandSpecMeta`]: `crate::meta::OperandSpecMeta`
pub trait OperandEncoding: Sized {
    /// The fixed length of the Operand, or `None` if it's variable length.
    ///
    /// If `Some`:
    /// * [`Self::encode`] must [`InstructionWriter::write`] (or [`InstructionWriter::extend`]) exactly this many
    ///   [`Word`]s
    /// * [`Self::decode`] must [`InstructionReader::pull`] (or [`Iterator::next`]) exactly this many [`Word`]s
    const FIXED_LEN: Option<usize>;

    /// The length of the operand in [`Word`]s.
    ///
    /// By default, returns [`Self::FIXED_LEN`] if it is `Some`, or computes it by [`Self::encode`]ing with
    /// [`WordCounter`]. If encoding your type could be expensive, we recommend overwriting this implementation,
    /// although LLVM should be able to optimize most encoders into a simple expression.
    ///
    /// Several debug assertions to check whether the length is correct are masked behind `cfg!(debug_assertions)`.
    fn word_len(&self) -> usize {
        fn computed_word_len(op: &impl OperandEncoding) -> usize {
            let mut counter = WordCounter::default();
            // `WordCounter` never fails
            let _ = op.encode(&mut counter);
            counter.0
        }

        if let Some(fixed) = Self::FIXED_LEN {
            if cfg!(debug_assertions) {
                let computed = computed_word_len(self);
                assert_eq!(
                    fixed, computed,
                    "Operand claims a fixed size of {} mismatches computed size {}",
                    fixed, computed
                );
            }
            fixed
        } else {
            computed_word_len(self)
        }
    }

    /// Encode this `Operand` to a sequence of [`Word`]s.
    fn encode(&self, writer: &mut impl InstructionWriter) -> Result<(), EncodeError>;

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

impl<T: OperandEncoding> OperandEncoding for Option<T> {
    const FIXED_LEN: Option<usize> = None;

    fn word_len(&self) -> usize {
        match self {
            None => 0,
            Some(e) => e.word_len(),
        }
    }

    fn encode(&self, writer: &mut impl InstructionWriter) -> Result<(), EncodeError> {
        match self {
            None => Ok(()),
            Some(e) => e.encode(writer),
        }
    }

    fn decode(reader: &mut InstructionReader<'_>) -> Result<Self, DecodeError> {
        if let Ok(_) = reader.peek() {
            Ok(Some(T::decode(reader)?))
        } else {
            Ok(None)
        }
    }

    fn decode_last(reader: &mut InstructionReader<'_>) -> Result<Self, DecodeError> {
        if let Ok(_) = reader.peek() {
            Ok(Some(T::decode_last(reader)?))
        } else {
            Ok(None)
        }
    }
}

impl<T: Operand> OperandSpec for Option<T> {
    type Operand = T;
    const QUANTIFIER: Quantifier = Quantifier::ZeroOrOne;
}

impl<T: OperandEncoding> OperandEncoding for Vec<T> {
    const FIXED_LEN: Option<usize> = None;

    fn word_len(&self) -> usize {
        self.iter().map(|e| e.word_len()).sum()
    }

    fn encode(&self, writer: &mut impl InstructionWriter) -> Result<(), EncodeError> {
        for x in self {
            x.encode(&mut *writer)?;
        }
        Ok(())
    }

    /// The current SPIR-V spec only uses [`Quantifier::ZeroOrMore`] when the "element" Operand has a fixed length
    /// encoding. So we can assume that our element `T` has a [`Self::FIXED_LEN`] of `Some`, but we'd like to support
    /// custom instruction sets that may make use of many variable sized operands as best we can.
    ///
    /// Possible paths to have a variable sized element (aka. `Self::FIXED_LEN = None`):
    /// * `ZeroOrMore` `LiteralString`: works
    /// * `ZeroOrMore` `LiteralConst`: fails since `LiteralConst` must be the last operand to decode correctly,
    ///   this is a limitation of our impl, see `LiteralConst` docs. Also why we don't implement [`Self::decode_last`]
    ///   or call it on our element.
    /// * `ZeroOrMore` `ZeroOrMore` T: fails, but unrepresentable in the spec, only by using it manually.
    ///
    /// [`Quantifier`]: `crate::meta::Quantifier`
    fn decode(reader: &mut InstructionReader<'_>) -> Result<Self, DecodeError> {
        let mut vec = if let Some(fixed_len) = T::FIXED_LEN {
            let remaining = reader.remaining();
            if remaining % fixed_len != 0 {
                return Err(DecodeError::InstructionWithMismatchedVariableOperants {
                    inst_offset: reader.inst_offset(),
                    op_len: reader.remaining(),
                    expected_multiple: fixed_len,
                });
            }
            Vec::with_capacity(remaining / fixed_len)
        } else {
            Vec::new()
        };
        while let Ok(_) = reader.peek() {
            vec.push(T::decode(reader)?);
        }
        Ok(vec)
    }
}

impl<T: Operand> OperandSpec for Vec<T> {
    type Operand = T;
    const QUANTIFIER: Quantifier = Quantifier::ZeroOrMore;
}

/// copy of Vec impl above
impl<T: OperandEncoding, const N: usize> OperandEncoding for SmallVec<[T; N]> {
    const FIXED_LEN: Option<usize> = None;

    fn word_len(&self) -> usize {
        self.iter().map(|e| e.word_len()).sum()
    }

    fn encode(&self, writer: &mut impl InstructionWriter) -> Result<(), EncodeError> {
        for x in self {
            x.encode(&mut *writer)?;
        }
        Ok(())
    }

    fn decode(reader: &mut InstructionReader<'_>) -> Result<Self, DecodeError> {
        let mut vec = if let Some(fixed_len) = T::FIXED_LEN {
            let remaining = reader.remaining();
            if remaining % fixed_len != 0 {
                return Err(DecodeError::InstructionWithMismatchedVariableOperants {
                    inst_offset: reader.inst_offset(),
                    op_len: reader.remaining(),
                    expected_multiple: fixed_len,
                });
            }
            SmallVec::with_capacity(remaining / fixed_len)
        } else {
            SmallVec::new()
        };
        while let Ok(_) = reader.peek() {
            vec.push(T::decode(reader)?);
        }
        Ok(vec)
    }
}

impl<T: Operand, const N: usize> OperandSpec for SmallVec<[T; N]> {
    type Operand = T;
    const QUANTIFIER: Quantifier = Quantifier::ZeroOrMore;
}

/// Compose the [`OperandEncoding::FIXED_LEN`] from multiple maybe fixed len Operands
pub struct FixedLenComposer(Option<usize>);

impl FixedLenComposer {
    pub const fn new() -> Self {
        Self(Some(0))
    }

    pub const fn append(self, len: Option<usize>) -> Self {
        match (self.0, len) {
            (Some(a), Some(b)) => Self(Some(a + b)),
            (_, _) => Self(None),
        }
    }

    pub const fn finish(self) -> Option<usize> {
        self.0
    }
}
