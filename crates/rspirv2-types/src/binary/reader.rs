use crate::Word;
use crate::binary::{DecodeError, DecodeErrorKind};
use crate::meta::InstMeta;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

/// An offset to some instruction, a wrapper around `usize`.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct InstOffset(pub usize);

impl Deref for InstOffset {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InstOffset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for InstOffset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

/// Reader for a single instruction
#[derive(Copy, Clone, Debug)]
pub struct InstReader<'a>(&'a [Word]);

impl<'a> InstReader<'a> {
    /// Create an [`InstReader`] from the instruction encoded in the slice of [`Word`]s at offset 0, cut off any further
    /// words not associated with this instruction. Use [`Self::len`] to figure out how many words have been consumed
    /// by this instruction.
    pub fn from_words(words: &'a [Word]) -> Result<Self, DecodeError> {
        let first = words.first().ok_or(DecodeErrorKind::OutOfInstructions)?;
        let (_, op_len) = first.to_op();
        if op_len == 0 {
            // len must at least be 1, as it includes the op Word itself
            return Err(DecodeErrorKind::InstructionZeroSized.into());
        }
        let inst_words = words
            .get(0..op_len)
            .ok_or(DecodeErrorKind::InstructionTooLong {
                op_len,
                module_remaining: words.len(),
            })?;
        Ok(Self(inst_words))
    }

    /// Length of the **entire** instruction, including the op word
    #[inline]
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Opcode of the instruction
    #[inline]
    pub fn opcode(&self) -> u16 {
        self.0.first().unwrap().to_op().0
    }

    /// Check whether the opcode matches, return an [`OperandReader`] if it is, or a [`DecodeError`] if it isn't
    #[inline]
    pub fn check_opcode(&self, meta: &InstMeta) -> Result<OperandReader<'a>, DecodeError> {
        let opcode = self.opcode();
        if opcode != meta.opcode {
            Err(DecodeErrorKind::WrongOpCode {
                name: meta.opname,
                expected: meta.opcode,
                actual: opcode,
            }
            .into())
        } else {
            Ok(self.operand_reader())
        }
    }

    /// Create an [`OperandReader`] for the operands of this Instruction
    #[inline]
    pub fn operand_reader(&self) -> OperandReader<'a> {
        // skip over opcode word
        OperandReader::new(&self.0[1..])
    }

    /// Returns the slice of Words that encodes a single instruction
    #[inline]
    pub fn to_words(&self) -> &'a [Word] {
        self.0
    }
}

/// Read operand [`Word`]s from an instruction
///
/// Not `Copy` to prevent accidental copies.
#[derive(Clone, Debug)]
pub struct OperandReader<'a> {
    /// params encoded as words
    params: &'a [Word],
    /// advancing offset pointing into params
    offset: usize,
}

impl<'a> OperandReader<'a> {
    #[inline]
    pub fn new(params: &'a [Word]) -> Self {
        Self { params, offset: 0 }
    }

    /// Peek at the next [`Word`] in the [`InstReader`] without advancing the `params_offset`. Fails if there are no
    /// further params.
    ///
    /// Calling this again will yield the same value, advance the [`Self::offset`] by [`Self::pull`]ing the
    /// [`Word`].
    #[inline]
    pub fn peek(&self) -> Result<Word, DecodeError> {
        Ok(*self
            .params
            .get(self.offset)
            .ok_or(self.err_too_many_words())?)
    }

    /// Pull a single [`Word`] from the [`InstReader`], advancing the `params_offset`. Fails if there are no further
    /// params.
    ///
    /// Calling this again will yield the next [`Word`].
    #[inline]
    pub fn pull(&mut self) -> Result<Word, DecodeError> {
        let result = self.peek();
        // always advance, even if we're out of words, so `finalize()` fails
        self.advance();
        result
    }

    /// Finalize an [`OperandReader`] to verify *exactly* all operands have been consumed.
    pub fn finalize(&self) -> Result<(), DecodeError> {
        let remaining = self.remaining();
        match 0.cmp(&remaining) {
            Ordering::Less => Err(DecodeErrorKind::InstructionWithAdditionalOperants {
                param_len: self.params.len(),
                remaining,
            }
            .into()),
            Ordering::Equal => Ok(()),
            Ordering::Greater => Err(self.err_too_many_words()),
        }
    }

    fn err_too_many_words(&self) -> DecodeError {
        DecodeErrorKind::InstructionDecodePulledTooManyWords {
            param_len: self.len(),
        }
        .into()
    }

    /// View the *remaining* Words as a slice, does not advance the `params_offset`.
    #[inline]
    pub fn as_slice(&self) -> &'a [Word] {
        &self.params[self.offset..]
    }

    /// Advance the `params_offset` by 1.
    #[inline]
    pub fn advance(&mut self) {
        self.advance_by(1);
    }

    /// Advance the `params_offset` by an arbitrary amount.
    #[inline]
    pub fn advance_by(&mut self, count: usize) {
        self.offset += count;
    }

    /// Returns the number of params.
    #[inline]
    pub fn len(&self) -> usize {
        self.params.len()
    }

    /// Returns `true` if the param slice has a length of 0.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.params.is_empty()
    }

    /// Current offset of the params, in [`Word`]s.
    #[inline]
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Amount of remaining param [`Word`]s.
    #[inline]
    pub fn remaining(&self) -> usize {
        self.params.len() - self.offset
    }
}

impl Iterator for OperandReader<'_> {
    type Item = Word;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.pull().ok()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.params.len() - self.offset;
        (size, Some(size))
    }
}

impl ExactSizeIterator for OperandReader<'_> {}
