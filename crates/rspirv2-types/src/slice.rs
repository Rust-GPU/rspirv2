use crate::Word;
use crate::binary::{DecodeError, InstOffset, InstReader};
use crate::dis::{DisContext, DisOptions};
use crate::inst::{InstEncoding, InstRef};
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use std::ops::Deref;

pub fn decode_failed(e: DecodeError) -> ! {
    panic!("Decode failed: {e}")
}

/// An `InstSlice` works like a `&[ISA]`, but stores instructions in the variable-sized SPIR-V binary form to safe
/// on memory.
///
/// Instruction sets are a giant enum of all instructions, their size is determined by largest instruction plus a
/// discriminant. The largest instructions in the core ISA are made from 10-15 operands, the enum of all instructions is
/// currently 80 bytes large. If you'd put that directly into a `Vec`, you'd be paying 80 bytes *per instruction*, even
/// though the most common instructions only need 2-4 operands or 12-20 bytes, wasting a lot of memory.
///
/// This `InstVec` stores instructions in their binary SPIR-V form, which is variable sized, allowing us to save a lot
/// of memory and improving cache locality. To make it as convenient to use as a `Vec<ISA>`, instructions are
/// automatically encoded and decoded on the fly. But to remain performant, decoding of instructions must be cheap,
/// so operands should not allocate any memory and instead should borrow slices from the underlying `Vec<Word>` whenever
/// possible.
///
/// However, storing instructions in a variable-sized way brings the same disadvantages as UTF-8 characters in rust
/// strings. You can't arbitrarily index into an `InstSlice` like you can with a regular slice, as only indices pointing
/// to the beginning of an instruction are valid. You also can't easily replace instructions in the middle of the
/// stream, as changing the size of an instruction requires you to move all following instructions. Although, you can
/// remove instructions by filling them with `OpNop`.
///
/// # Safety
/// The inner slice of words is assumed to contain valid instructions of the generic `ISA` Instruction Set. May panic if
/// instructions fail to decode, but will not lead to UB, allowing [`Self::from_words_unchecked`] to be safe.
pub struct InstSlice<'a, ISA: InstEncoding> {
    raw: RawInstSlice<'a>,
    _phantom: PhantomData<ISA>,
}

impl<'a, ISA: InstEncoding> InstSlice<'a, ISA> {
    /// Create a new [`InstSlice`] from a `&[Word]` safely, by verifying the instructions to be valid in the `ISA`.
    #[inline]
    pub fn from_words(words: &'a [Word]) -> Result<Self, DecodeError> {
        Self::from_raw(RawInstSlice::from_words(words))
    }

    /// Create a new [`InstSlice`] from a `&[Word]` without checking whether the words contain valid instructions.
    ///
    /// See [`crate::vec::InstVec`] # Safety
    #[inline]
    pub const fn from_words_unchecked(words: &'a [Word]) -> Self {
        Self::from_raw_unchecked(RawInstSlice::from_words(words))
    }

    /// Create a new [`InstSlice`] from a [`RawInstSlice`] safely, by verifying the instructions to be valid in the
    /// `ISA`.
    #[inline]
    pub fn from_raw(raw: RawInstSlice<'a>) -> Result<Self, DecodeError> {
        raw.verify_valid_in_isa::<ISA>()?;
        Ok(Self::from_raw_unchecked(raw))
    }

    /// Create a new [`InstSlice`] from a [`RawInstSlice`] without checking whether the words contain valid
    /// instructions.
    ///
    /// # Safety
    /// See [`InstSlice`] #Safety
    #[inline]
    pub const fn from_raw_unchecked(raw: RawInstSlice<'a>) -> Self {
        Self {
            raw,
            _phantom: PhantomData,
        }
    }

    /// View self as a [`RawInstSlice`]
    #[inline]
    pub const fn as_raw(&self) -> RawInstSlice<'a> {
        self.raw
    }

    /// Iterate over instruction references ([`InstRef`]) without decoding the instruction itself
    #[inline]
    pub const fn iter_ref(&self) -> InstRefIter<'a, ISA> {
        InstRefIter::new(*self)
    }

    /// Iterate over all instructions
    #[inline]
    pub const fn iter(&self) -> InstIter<'a, ISA> {
        InstIter::new(*self)
    }

    /// disassemble
    #[inline]
    pub fn dis(&self, opt: DisOptions) -> Result<DisInstSlice<'a, ISA>, DecodeError> {
        DisInstSlice::new(*self, opt)
    }
}

impl<'a, ISA: InstEncoding> Deref for InstSlice<'a, ISA> {
    type Target = RawInstSlice<'a>;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl<'a, ISA: InstEncoding> Copy for InstSlice<'a, ISA> {}

impl<'a, ISA: InstEncoding> Clone for InstSlice<'a, ISA> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, ISA: InstEncoding> Debug for InstSlice<'a, ISA> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InstSlice")
            .field("ISA", &ISA::name())
            .field("raw", &self.raw)
            .finish()
    }
}

/// A sequence of words that has been pre-processed and may be [`Display`]ed.
///
/// The `ISA: `[`InstEncoding`] generic determines for which instruction set these Words are disassembled.
pub struct DisInstSlice<'a, ISA: InstEncoding> {
    slice: InstSlice<'a, ISA>,
    dis: DisContext,
}

impl<'a, ISA: InstEncoding> DisInstSlice<'a, ISA> {
    #[inline]
    pub fn new(slice: InstSlice<'a, ISA>, opt: DisOptions) -> Result<Self, DecodeError> {
        Ok(Self {
            slice,
            dis: DisContext::new(opt),
        })
    }
}

impl<'a, ISA: InstEncoding> Display for DisInstSlice<'a, ISA> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for inst in self.slice.iter() {
            writeln!(f, "{}", inst.dis(&self.dis))?;
        }
        Ok(())
    }
}

/// An [`Iterator`] yielding a tuple of [`InstOffset`] and [`InstRef`]
pub struct InstOffsetRefIter<'a, ISA: InstEncoding> {
    inner: RawInstOffsetRefIter<'a>,
    _phantom: PhantomData<ISA>,
}

impl<'a, ISA: InstEncoding> InstOffsetRefIter<'a, ISA> {
    #[inline]
    pub const fn new(slice: InstSlice<'a, ISA>) -> Self {
        Self {
            inner: RawInstOffsetRefIter::new(slice.as_raw()),
            _phantom: PhantomData,
        }
    }
}

impl<'a, ISA: InstEncoding> Iterator for InstOffsetRefIter<'a, ISA> {
    type Item = (InstOffset, InstRef<'a, ISA>);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next()? {
            Ok((offset, reader)) => Some((
                offset,
                match InstRef::from_words_unchecked(reader.to_words()) {
                    Ok(e) => e,
                    Err(e) => decode_failed(e),
                },
            )),
            Err(e) => decode_failed(e),
        }
    }
}

impl<'a, ISA: InstEncoding> Clone for InstOffsetRefIter<'a, ISA> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<'a, ISA: InstEncoding> Debug for InstOffsetRefIter<'a, ISA> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InstOffsetRefIter")
            .field("ISA", &ISA::name())
            .field("inner", &self.inner)
            .finish()
    }
}

/// An [`Iterator`] yielding [`InstRef`]
pub struct InstRefIter<'a, ISA: InstEncoding>(InstOffsetRefIter<'a, ISA>);

impl<'a, ISA: InstEncoding> InstRefIter<'a, ISA> {
    #[inline]
    pub const fn new(slice: InstSlice<'a, ISA>) -> Self {
        Self(InstOffsetRefIter::new(slice))
    }
}

impl<'a, ISA: InstEncoding> Iterator for InstRefIter<'a, ISA> {
    type Item = InstRef<'a, ISA>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.0.next()?.1)
    }
}

impl<'a, ISA: InstEncoding> Clone for InstRefIter<'a, ISA> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<'a, ISA: InstEncoding> Debug for InstRefIter<'a, ISA> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("InstRefIter").field(&self.0).finish()
    }
}

/// An [`Iterator`] yielding `ISA`
pub struct InstOffsetIter<'a, ISA: InstEncoding>(InstOffsetRefIter<'a, ISA>);

impl<'a, ISA: InstEncoding> InstOffsetIter<'a, ISA> {
    #[inline]
    pub const fn new(slice: InstSlice<'a, ISA>) -> Self {
        Self(InstOffsetRefIter::new(slice))
    }

    /// Add [`InstOffset`]s to this iterator, akin to `enumerate`
    #[inline]
    pub const fn with_offsets(self) -> InstOffsetRefIter<'a, ISA> {
        self.0
    }
}

impl<'a, ISA: InstEncoding> Iterator for InstOffsetIter<'a, ISA> {
    type Item = (InstOffset, ISA);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (offset, inst_ref) = self.0.next()?;
        Some((offset, inst_ref.get()))
    }
}

impl<'a, ISA: InstEncoding> Clone for InstOffsetIter<'a, ISA> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<'a, ISA: InstEncoding> Debug for InstOffsetIter<'a, ISA> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("InstOffsetIter").field(&self.0).finish()
    }
}

/// An [`Iterator`] yielding `ISA`
pub struct InstIter<'a, ISA: InstEncoding>(InstOffsetIter<'a, ISA>);

impl<'a, ISA: InstEncoding> InstIter<'a, ISA> {
    #[inline]
    pub const fn new(slice: InstSlice<'a, ISA>) -> Self {
        Self(InstOffsetIter::new(slice))
    }

    /// Add [`InstOffset`]s to this iterator, akin to `enumerate`
    #[inline]
    pub const fn with_offsets(self) -> InstOffsetIter<'a, ISA> {
        self.0
    }
}

impl<'a, ISA: InstEncoding> Iterator for InstIter<'a, ISA> {
    type Item = ISA;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.0.next()?.1)
    }
}

impl<'a, ISA: InstEncoding> Clone for InstIter<'a, ISA> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<'a, ISA: InstEncoding> Debug for InstIter<'a, ISA> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("InstIter").field(&self.0).finish()
    }
}

/// A slice of SPIR-V instructions of an arbitrary instruction set.
///
/// Compared to [`InstSlice`], does not specify the instruction set nor require the inner words to be valid instructions
/// in said instruction set.
#[derive(Copy, Clone, Debug)]
pub struct RawInstSlice<'a>(pub &'a [Word]);

impl<'a> RawInstSlice<'a> {
    pub const fn from_words(words: &'a [Word]) -> Self {
        Self(words)
    }

    /// Returns the underlying slice of words
    pub const fn as_words(&self) -> &'a [Word] {
        self.0
    }

    /// Iterate over instruction references ([`InstRef`]) without decoding the instruction itself
    pub const fn iter(&self) -> RawInstRefIter<'a> {
        RawInstRefIter::new(*self)
    }

    /// Verify whether the instructions are valid within the generic `ISA` instruction set
    pub fn verify_valid_in_isa<ISA: InstEncoding>(&self) -> Result<(), DecodeError> {
        self.iter().try_for_each(|result| {
            ISA::decode(result?)?;
            Ok(())
        })
    }
}

impl<'a, ISA: InstEncoding> From<InstSlice<'a, ISA>> for RawInstSlice<'a> {
    fn from(value: InstSlice<'a, ISA>) -> Self {
        value.raw
    }
}

/// An [`Iterator`] of [`Result`]s yielding either a tuple of [`InstOffset`] and [`InstReader`], or a [`DecodeError`]
#[derive(Clone, Debug)]
pub struct RawInstOffsetRefIter<'a> {
    raw: RawInstSlice<'a>,
    offset: InstOffset,
}

impl<'a> RawInstOffsetRefIter<'a> {
    /// Create a new iterator
    #[inline]
    pub const fn new(raw: RawInstSlice<'a>) -> Self {
        Self {
            raw,
            offset: InstOffset(0),
        }
    }
}

impl<'a> Iterator for RawInstOffsetRefIter<'a> {
    type Item = Result<(InstOffset, InstReader<'a>), DecodeError>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match InstReader::from_words(&self.raw.0[*self.offset..]) {
            Ok(inst_reader) => {
                let old_offset = self.offset;
                *self.offset += inst_reader.len();
                Some(Ok((old_offset, inst_reader)))
            }
            Err(DecodeError::OutOfInstructions) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

/// An [`Iterator`] of [`Result`] yielding either an [`InstReader`] or a [`DecodeError`].
///
/// Use [`Self::with_offsets`] to also get [`InstOffset`] of the instruction.
#[derive(Clone, Debug)]
pub struct RawInstRefIter<'a>(RawInstOffsetRefIter<'a>);

impl<'a> RawInstRefIter<'a> {
    /// Create a new iterator
    #[inline]
    pub const fn new(raw: RawInstSlice<'a>) -> Self {
        Self(RawInstOffsetRefIter::new(raw))
    }

    /// Add [`InstOffset`]s to this iterator, akin to `enumerate`
    pub const fn with_offsets(self) -> RawInstOffsetRefIter<'a> {
        self.0
    }
}

impl<'a> Iterator for RawInstRefIter<'a> {
    type Item = Result<InstReader<'a>, DecodeError>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            Some(Ok((_, inst))) => Some(Ok(inst)),
            Some(Err(e)) => Some(Err(e)),
            None => None,
        }
    }
}

pub trait SkipDecodeErrorIteratorExt<T> {
    /// Skip over any decode errors
    fn skip_errors(self) -> impl Iterator<Item = T>;
}

impl<T, I: Iterator<Item = Result<T, DecodeError>>> SkipDecodeErrorIteratorExt<T> for I {
    fn skip_errors(self) -> impl Iterator<Item = T> {
        self.filter_map(|v| v.ok())
    }
}

pub trait DecodeIteratorExt {
    /// Decode the [`InstReader`]s in this stream into `ISA` Instructions
    fn decode<ISA: InstEncoding>(self) -> impl Iterator<Item = Result<ISA, DecodeError>>;
}

impl<'a, I: Iterator<Item = InstReader<'a>>> DecodeIteratorExt for I {
    fn decode<ISA: InstEncoding>(self) -> impl Iterator<Item = Result<ISA, DecodeError>> {
        self.map(|reader| ISA::decode(reader))
    }
}

pub trait TryDecodeIteratorExt {
    /// Decode the [`InstReader`]s in this stream into `ISA` Instructions, and forward any [`DecodeError`]s
    fn try_decode<ISA: InstEncoding>(self) -> impl Iterator<Item = Result<ISA, DecodeError>>;
}

impl<'a, I: Iterator<Item = Result<InstReader<'a>, DecodeError>>> TryDecodeIteratorExt for I {
    fn try_decode<ISA: InstEncoding>(self) -> impl Iterator<Item = Result<ISA, DecodeError>> {
        self.map(|reader| ISA::decode(reader?))
    }
}
