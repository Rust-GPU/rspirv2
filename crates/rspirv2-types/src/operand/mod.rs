mod id;
mod literal_const;
mod literal_float;
mod literal_integer;
mod literal_string;
mod parameterized_bitmask;
mod tuple;

use crate::binary::{
    DecodeError, DecodeErrorKind, EncodeError, OperandReader, WordCounter, WordWriter,
};
use crate::dis::DisContext;
use crate::meta::OperandKind;
pub use id::*;
pub use literal_const::*;
pub use literal_float::*;
pub use literal_integer::*;
pub use literal_string::*;
pub use parameterized_bitmask::*;
use smallvec::SmallVec;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
pub use tuple::*;

/// A SPIR-V operand. The associated const [`Self::KIND`] links to it's [`OperandKind`].
///
/// Requires [`SpvOperandEncoding`], see that for encoding and decoding SPIR-V.
///
/// # Safety
/// * [`Self::KIND`] must match this implementation
pub unsafe trait SpvOperandMeta: SpvOperandEncoding {
    const KIND: &OperandKind;
}

/// Something that can be decoded from or encoded to SPIR-V, not necessarily a full [`SpvOperandMeta`].
///
/// Both [`Option`] and [`Vec`] implement `OperandEncoding` but not [`SpvOperandMeta`]. This allows for an easier
/// representation of [`OperandSpecMeta`]s with [`Quantifier`] of [`Quantifier::ZeroOrOne`] (`Option`) and
/// [`Quantifier::ZeroOrMore`] (`Vec`).
///
/// # Safety
/// * [`Self::encode`] must [`WordWriter::write`] exactly [`Self::word_len`] many words.
/// * [`Self::decode`] must [`OperandReader::pull`] (or [`Iterator::next`]) exactly [`Self::word_len`] many
///   words.
/// * If [`Self::FIXED_LEN`] is `Some`, it must equal the computed [`Self::word_len`].
///
/// These constraints should only be validated with `cfg!(debug_assertions)`, which are enabled by default in debug
/// builds.
///
/// [`OperandSpecMeta`]: crate::meta::OperandSpecMeta
/// [`Quantifier`]: crate::meta::Quantifier
/// [`Quantifier::ZeroOrOne`]: crate::meta::Quantifier::ZeroOrOne
/// [`Quantifier::ZeroOrMore`]: crate::meta::Quantifier::ZeroOrMore
pub unsafe trait SpvOperandEncoding: Sized {
    /// The fixed length of the Operand, or `None` if it's variable length. Specifying this is an optimization for
    /// operand length calculation. See the safety contract in [`SpvOperandEncoding`].
    const FIXED_LEN: Option<usize>;

    /// The length of the operand in words.
    ///
    /// By default, returns [`Self::FIXED_LEN`] if it is `Some`, or computes it by [`Self::encode`]ing with
    /// [`WordCounter`]. If encoding your type could be expensive, we recommend overwriting this implementation,
    /// although LLVM should be able to optimize most encoders into a simple expression.
    ///
    /// Several debug assertions to check whether the length is correct are masked behind `cfg!(debug_assertions)`.
    fn word_len(&self) -> usize {
        profiling::function_scope!();
        fn computed_word_len(op: &impl SpvOperandEncoding) -> usize {
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

    /// Encode this `Operand` to a sequence of words.
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError>;

    /// Parse the `Operand` from the supplied [`Iterator`] of words, advancing it in the process.
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError>;

    /// Like [`Self::decode`], but the `Operand` is guaranteed to be the last one of this Instruction. This is used by
    /// [`LiteralInteger`] to consume all the remaining words and not have to calculate the exact size of a type.
    ///
    /// See [`LiteralInteger`] for details.
    #[inline]
    fn decode_last(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        let result = Self::decode(reader)?;
        reader.finalize()?;
        Ok(result)
    }
}

/// Operands that can be disassembled into a SPIR-V like disassembly.
pub trait SpvOperandDis: Sized {
    /// Disassemble this operand to the supplied [`Formatter`]. Prefer [`Self::dis`] over calling this.
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &OperandDisContext<'_>) -> std::fmt::Result;

    /// Disassemble this operand
    ///
    /// Returns a type that impl [`Display`], so it's usable with `format!`:
    /// ```no_run
    /// # use rspirv2_types::operand::IdResult;
    /// let my_op = IdResult(Word(42));
    /// let dis = format!("OpMyInst {}", my_op.dis());
    /// assert_eq!(dis, "OpMyInst %42");
    /// ```
    #[inline]
    fn dis<'a>(&'a self, ctx: &'a OperandDisContext<'_>) -> OperandDis<'a, Self> {
        OperandDis(self, ctx)
    }
}

pub struct OperandDis<'a, T: SpvOperandDis>(&'a T, &'a OperandDisContext<'a>);

impl<'a, T: SpvOperandDis> Display for OperandDis<'a, T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.dis_fmt(f, self.1)
    }
}

/// Context object for disassembly generation of operands within an instruction
#[derive(Copy, Clone, Debug)]
pub struct OperandDisContext<'a> {
    pub ctx: &'a DisContext,
    pub id_result: Option<IdResult>,
    pub id_result_type: Option<IdResultType>,
}

impl<'a> OperandDisContext<'a> {
    pub fn new(ctx: &'a DisContext) -> Self {
        Self {
            ctx,
            id_result: None,
            id_result_type: None,
        }
    }

    pub fn id_result_writer(&self) -> IdResultWriter<'_> {
        IdResultWriter(self)
    }
}

impl<'a> Deref for OperandDisContext<'a> {
    type Target = DisContext;
    fn deref(&self) -> &Self::Target {
        self.ctx
    }
}

pub type ZeroOrOne<T> = Option<T>;

unsafe impl<T: SpvOperandEncoding> SpvOperandEncoding for Option<T> {
    const FIXED_LEN: Option<usize> = None;

    #[inline]
    fn word_len(&self) -> usize {
        match self {
            None => 0,
            Some(e) => e.word_len(),
        }
    }

    #[inline]
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        match self {
            None => Ok(()),
            Some(e) => e.encode(writer),
        }
    }

    #[inline]
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        if reader.peek().is_ok() {
            Ok(Some(T::decode(reader)?))
        } else {
            Ok(None)
        }
    }

    #[inline]
    fn decode_last(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        if reader.peek().is_ok() {
            Ok(Some(T::decode_last(reader)?))
        } else {
            Ok(None)
        }
    }
}

impl<T: SpvOperandDis> SpvOperandDis for Option<T> {
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &OperandDisContext<'_>) -> std::fmt::Result {
        match self {
            None => Ok(()),
            Some(e) => e.dis_fmt(f, ctx),
        }
    }
}

pub type ZeroOrMore<T> = SmallVec<[T; 6]>;

unsafe impl<T: SpvOperandEncoding> SpvOperandEncoding for Vec<T> {
    const FIXED_LEN: Option<usize> = None;

    #[inline]
    fn word_len(&self) -> usize {
        profiling::function_scope!();
        self.iter().map(|e| e.word_len()).sum()
    }

    #[inline]
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
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
    /// [`Quantifier::ZeroOrMore`]: crate::meta::Quantifier::ZeroOrMore
    #[inline]
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut vec = if let Some(fixed_len) = T::FIXED_LEN {
            let remaining = reader.remaining();
            if !remaining.is_multiple_of(fixed_len) {
                return Err(DecodeErrorKind::InstructionWithMismatchedVariableOperants {
                    op_len: remaining,
                    expected_multiple: fixed_len,
                }
                .into());
            }
            Vec::with_capacity(remaining / fixed_len)
        } else {
            Vec::new()
        };
        while reader.peek().is_ok() {
            vec.push(T::decode(reader)?);
        }
        Ok(vec)
    }
}

impl<T: SpvOperandDis> SpvOperandDis for Vec<T> {
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &OperandDisContext<'_>) -> std::fmt::Result {
        profiling::function_scope!();
        for v in self {
            T::dis_fmt(v, &mut *f, ctx)?;
        }
        Ok(())
    }
}

/// copy of Vec impl above
unsafe impl<T: SpvOperandEncoding, const N: usize> SpvOperandEncoding for SmallVec<[T; N]> {
    const FIXED_LEN: Option<usize> = None;

    #[inline]
    fn word_len(&self) -> usize {
        profiling::function_scope!();
        self.iter().map(|e| e.word_len()).sum()
    }

    #[inline]
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        for x in self {
            x.encode(&mut *writer)?;
        }
        Ok(())
    }

    #[inline]
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut vec = if let Some(fixed_len) = T::FIXED_LEN {
            let remaining = reader.remaining();
            if !remaining.is_multiple_of(fixed_len) {
                return Err(DecodeErrorKind::InstructionWithMismatchedVariableOperants {
                    op_len: remaining,
                    expected_multiple: fixed_len,
                }
                .into());
            }
            SmallVec::with_capacity(remaining / fixed_len)
        } else {
            SmallVec::new()
        };
        while reader.peek().is_ok() {
            vec.push(T::decode(reader)?);
        }
        Ok(vec)
    }
}

impl<T: SpvOperandDis, const N: usize> SpvOperandDis for SmallVec<[T; N]> {
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &OperandDisContext<'_>) -> std::fmt::Result {
        profiling::function_scope!();
        for v in self {
            T::dis_fmt(v, &mut *f, ctx)?;
        }
        Ok(())
    }
}
