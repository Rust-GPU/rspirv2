use crate::binary::{DecodeError, EncodeError, IdResultAlloc, InstReader, WordWriter};
use crate::meta::InstMeta;
use crate::operand::{IdResult, OptionIdResult};
use std::fmt::Debug;

pub trait Inst: InstEncoding {
    const META: &InstMeta;

    /// `MaybeIdResult` is either an [`IdResult`] or `()`, depending on whether this Instruction has an [`IdResult`].
    type MaybeIdResult: MaybeIdResult;

    /// Query the potential [`IdResult`] of this Instruction, or `()` if it has none.
    fn id_result(&mut self) -> &mut Self::MaybeIdResult;
}

pub trait InstEncoding: Sized + Debug + Eq {
    /// Encode this instruction to a [`WordWriter`]
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError>;

    /// Decode this instruction from an [`InstReader`]
    fn decode(reader: &mut InstReader) -> Result<Self, DecodeError>;
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
