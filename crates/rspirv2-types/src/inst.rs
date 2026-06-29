use crate::Word;
use crate::binary::{DecodeError, DecodeErrorKind, EncodeError, InstReader, WordWriter};
use crate::dis::DisContext;
use crate::meta::InstMeta;
use crate::operand::IdResult;
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use std::ops::Deref;

pub trait SpvInst: SpvInstEncoding + SpvInstDis + SpvInstDefUse {}

impl<T> SpvInst for T where T: SpvInstEncoding + SpvInstDis + SpvInstDefUse {}

pub trait SpvInstMeta {
    const META: &InstMeta;
}

pub trait SpvInstDefUse: Sized {
    /// `IdResult` is either an [`IdResult`] or `()`, depending on whether this Instruction has an [`IdResult`].
    type IdResult: MaybeIdResult;
    /// `IdResult` is either an [`IdResult`] or `()`, depending on whether this Instruction has an
    /// [`crate::operand::IdResultType`].
    type IdResultType: MaybeIdResult;

    /// Query the potential [`IdResult`] of this Instruction, or `()` if it has none.
    fn id_result(&self) -> Self::IdResult;

    /// Query the potential [`IdResult`] of this Instruction, or `()` if it has none.
    fn id_result_type(&self) -> Self::IdResultType;
}

pub trait SpvInstEncoding: Sized {
    /// Name of the instruction set, for debug printing
    fn name() -> &'static str {
        "unknown"
    }

    /// Encode this instruction to a [`WordWriter`]
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError>;

    /// Decode this instruction from an [`InstReader`], error when opcode is unknown.
    ///
    /// See [`Self::try_decode`] for a variant that returns `None` when opcode is unknown.
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError>;

    /// Try to decode this instruction from an [`InstReader`], return `None` when the opcode is unknown.
    ///
    /// See [`Self::decode`] for a variant that errors when opcode is unknown.
    #[inline]
    fn try_decode(reader: InstReader<'_>) -> Result<Option<Self>, DecodeError> {
        match Self::decode(reader) {
            Ok(e) => Ok(Some(e)),
            Err(DecodeError {
                kind: DecodeErrorKind::WrongOpCode { .. } | DecodeErrorKind::UnknownOpCode { .. },
                ..
            }) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

pub trait SpvInstDis: Sized {
    /// Disassemble this instruction to the supplied [`Formatter`] `f`
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result;

    /// Disassemble this instruction, returns a type that impl [`Display`] you may use in `format!`.
    ///
    /// You shouldn't overwrite this function but [`Self::dis_fmt`] instead.
    #[inline]
    fn dis<'a>(&'a self, ctx: &'a DisContext) -> InstDis<'a, Self> {
        InstDis(self, ctx)
    }
}

impl SpvInstDefUse for () {
    type IdResult = ();
    type IdResultType = ();

    fn id_result(&self) -> Self::IdResult {}
    fn id_result_type(&self) -> Self::IdResultType {}
}

impl SpvInstEncoding for () {
    fn name() -> &'static str {
        "()"
    }

    fn encode(&self, _: &mut impl WordWriter) -> Result<(), EncodeError> {
        Ok(())
    }

    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        Err(DecodeErrorKind::UnknownOpCode {
            opcode: reader.opcode(),
        }
        .into())
    }
}

impl SpvInstDis for () {
    fn dis_fmt(&self, _: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        Ok(())
    }
}

pub struct InstDis<'a, T: SpvInstDis>(&'a T, &'a DisContext);

impl<'a, T: SpvInstDis> Display for InstDis<'a, T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.dis_fmt(f, self.1)
    }
}

/// A type that may be an [`IdResult`] or `()`.
pub trait MaybeIdResult: Copy {
    fn to_optional(&self) -> Option<IdResult>;
}

impl MaybeIdResult for () {
    fn to_optional(&self) -> Option<IdResult> {
        None
    }
}

impl MaybeIdResult for IdResult {
    fn to_optional(&self) -> Option<IdResult> {
        Some(*self)
    }
}

impl MaybeIdResult for Option<IdResult> {
    fn to_optional(&self) -> Option<IdResult> {
        *self
    }
}

/// A reference to a valid instruction encoded in a slice of [`Word`]s.
///
/// Call [`Self::get`] to get the underlying instruction, as it unfortunately can't implement [`Deref`].
///
/// # Safety
/// The referenced words must be an encoded instruction that is valid within the `ISA` instruction set. Encountering an
/// invalid instruction may panic.
pub struct InstRef<'a, ISA: SpvInstEncoding> {
    reader: InstReader<'a>,
    _phantom: PhantomData<ISA>,
}

impl<'a, ISA: SpvInstEncoding> InstRef<'a, ISA> {
    #[inline]
    pub fn from_words_unchecked(words: &'a [Word]) -> Result<Self, DecodeError> {
        Ok(Self {
            reader: InstReader::from_words(words)?,
            _phantom: PhantomData,
        })
    }

    #[inline]
    pub fn get(&self) -> ISA {
        ISA::decode(**self).unwrap()
    }
}

impl<'a, ISA: SpvInstEncoding> Deref for InstRef<'a, ISA> {
    type Target = InstReader<'a>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.reader
    }
}

impl<ISA: SpvInstEncoding> Copy for InstRef<'_, ISA> {}

impl<ISA: SpvInstEncoding> Clone for InstRef<'_, ISA> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<ISA: SpvInstEncoding> Debug for InstRef<'_, ISA> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InstRef")
            .field("reader", &self.reader)
            .finish()
    }
}
