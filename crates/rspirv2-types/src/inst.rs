use crate::binary::{DecodeError, EncodeError, IdResultAlloc, InstReader, WordWriter};
use crate::dis::DisContext;
use crate::meta::InstMeta;
use crate::operand::{IdResult, OptionIdResult};
use std::fmt::{Debug, Display, Formatter};

pub trait Inst: InstEncoding {
    const META: &InstMeta;

    /// `MaybeIdResult` is either an [`IdResult`] or `()`, depending on whether this Instruction has an [`IdResult`].
    type MaybeIdResult: MaybeIdResult;

    /// Query the potential [`IdResult`] of this Instruction, or `()` if it has none.
    fn id_result(&mut self) -> &mut Self::MaybeIdResult;
}

pub trait InstEncoding: Sized + Debug + Eq {
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
            Err(DecodeError::WrongOpCode { .. } | DecodeError::UnknownOpCode { .. }) => Ok(None),
            Err(e) => Err(e),
        }
    }

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

impl InstEncoding for () {
    fn name() -> &'static str {
        "()"
    }

    fn encode(&self, _: &mut impl WordWriter) -> Result<(), EncodeError> {
        Ok(())
    }

    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        Err(DecodeError::UnknownOpCode {
            opcode: reader.opcode(),
        })
    }

    fn dis_fmt(&self, _: &mut Formatter<'_>, _: &DisContext) -> std::fmt::Result {
        Ok(())
    }
}

pub struct InstDis<'a, T: InstEncoding>(&'a T, &'a DisContext);

impl<'a, T: InstEncoding> Display for InstDis<'a, T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.dis_fmt(f, self.1)
    }
}

/// A type that may be an [`IdResult`] or `()`.
pub trait MaybeIdResult: Copy {
    type IdResult;
    fn alloc(&mut self, alloc: &mut impl IdResultAlloc) -> Result<Self::IdResult, EncodeError>;
}

impl MaybeIdResult for () {
    type IdResult = ();

    #[inline]
    fn alloc(&mut self, _: &mut impl IdResultAlloc) -> Result<Self::IdResult, EncodeError> {
        Ok(())
    }
}

impl MaybeIdResult for OptionIdResult {
    type IdResult = IdResult;

    #[inline]
    fn alloc(&mut self, alloc: &mut impl IdResultAlloc) -> Result<Self::IdResult, EncodeError> {
        Ok(match self {
            None => {
                let id = alloc.alloc_id()?;
                *self = Some(id);
                id
            }
            Some(id) => *id,
        })
    }
}

/// as `()` is a ZST, this should optimize away into `ptr::dangling()`
#[inline]
pub fn make_mut_ref_unit() -> &'static mut () {
    Box::leak(Box::new(()))
}
