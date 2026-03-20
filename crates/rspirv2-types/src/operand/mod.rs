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
use crate::meta::{OperandKind, Quantifier};
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
/// Requires [`OperandEncoding`], see that for encoding and decoding SPIR-V.
///
/// # Safety
/// * [`Self::KIND`] must match this implementation
pub unsafe trait Operand: OperandEncoding {
    const KIND: &OperandKind;
}

/// A `OperandSpec` is an [`Operand`] with a [`Quantifier`] to describe the repetition of the [`Operand`].
///
/// Any [`Operand`] implicitly implements this with [`Quantifier::One`], wrapping an Operand in [`Option`] will get a
/// [`Quantifier::ZeroOrOne`] and wrapping it in a [`Vec`] or [`SmallVec`] will have a [`Quantifier::ZeroOrMore`].
///
/// # Safety
/// * should not be implemented outside of this file
pub unsafe trait OperandSpec: OperandEncoding {
    /// The [`Operand`]
    type Operand: Operand;
    /// The [`Quantifier`] or repetition factor of the [`Self::Operand`]
    const QUANTIFIER: Quantifier;
}

unsafe impl<T: Operand> OperandSpec for T {
    type Operand = Self;
    const QUANTIFIER: Quantifier = Quantifier::One;
}

/// Something that can be decoded from or encoded to SPIR-V, not necessarily a full [`Operand`].
///
/// Both [`Option`] and [`Vec`] implement `OperandEncoding` but not [`Operand`]. This allows for an easier
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
/// [`OperandSpecMeta`]: `crate::meta::OperandSpecMeta`
pub unsafe trait OperandEncoding: Sized + Debug {
    /// The fixed length of the Operand, or `None` if it's variable length. Specifying this is an optimization for
    /// operand length calculation. See the safety contract in [`OperandEncoding`].
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

    /// Encode this `Operand` to a sequence of words.
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError>;

    /// Validate this `Option<Operand>` before encoding.
    ///
    /// May implement special behavior for failing, like [`IdResult`] does to validate it has been initialized.
    #[inline]
    fn validate_optional(opt: &Option<Self>) -> Result<(), EncodeError> {
        let _ = opt;
        Ok(())
    }

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

pub struct OperandDis<'a, T: OperandEncoding>(&'a T, &'a OperandDisContext<'a>);

impl<'a, T: OperandEncoding> Display for OperandDis<'a, T> {
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

unsafe impl<T: OperandEncoding> OperandEncoding for Option<T> {
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
        T::validate_optional(self)?;
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

    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &OperandDisContext<'_>) -> std::fmt::Result {
        match self {
            None => Ok(()),
            Some(e) => e.dis_fmt(f, ctx),
        }
    }
}

unsafe impl<T: Operand> OperandSpec for Option<T> {
    type Operand = T;
    const QUANTIFIER: Quantifier = Quantifier::ZeroOrOne;
}

pub type ZeroOrMore<T> = SmallVec<[T; 4]>;

unsafe impl<T: OperandEncoding> OperandEncoding for Vec<T> {
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
    #[inline]
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut vec = if let Some(fixed_len) = T::FIXED_LEN {
            let remaining = reader.remaining();
            if !remaining.is_multiple_of(fixed_len) {
                return Err(DecodeErrorKind::InstructionWithMismatchedVariableOperants {
                    op_len: reader.remaining(),
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

    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &OperandDisContext<'_>) -> std::fmt::Result {
        profiling::function_scope!();
        for v in self {
            T::dis_fmt(v, &mut *f, ctx)?;
        }
        Ok(())
    }
}

unsafe impl<T: Operand> OperandSpec for Vec<T> {
    type Operand = T;
    const QUANTIFIER: Quantifier = Quantifier::ZeroOrMore;
}

/// copy of Vec impl above
unsafe impl<T: OperandEncoding, const N: usize> OperandEncoding for SmallVec<[T; N]> {
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
                    op_len: reader.remaining(),
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

    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &OperandDisContext<'_>) -> std::fmt::Result {
        profiling::function_scope!();
        for v in self {
            T::dis_fmt(v, &mut *f, ctx)?;
        }
        Ok(())
    }
}

unsafe impl<T: Operand, const N: usize> OperandSpec for SmallVec<[T; N]> {
    type Operand = T;
    const QUANTIFIER: Quantifier = Quantifier::ZeroOrMore;
}
