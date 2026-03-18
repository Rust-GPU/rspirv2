use crate::Word;
use crate::binary::{DecodeError, InstOffset, InstReader};
use crate::dis::{DisModule, DisOptions};
use crate::inst::InstEncoding;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::ops::Deref;

/// An `InstVec` works like a `Vec<ISA>`, but stores instructions in the variable-sized SPIR-V binary form to safe
/// on memory size.
///
/// Instruction sets are a giant enum of all instructions, their size is determined by largest instruction plus a
/// discriminant. The largest instructions in the core ISA being made from 10-15 operands of 4 bytes each, giving us an
/// estimated size of around 64 bytes *per instruction*. With the most common instructions only need 2-4 operands or
/// 12-20 bytes, storing that enum in a regular `Vec` is wasting a lot of memory.
///
/// This `InstVec` stores instructions in their binary SPIR-V form, which is variable sized, allowing us to save a lot
/// of memory and improving cache locality. To make it as convenient to use as a `Vec<ISA>`, instructions are
/// automatically encoded and decoded on the fly. To remain performant, decoding of instructions must be cheap,
/// so operands should not allocate any memory and instead borrow slices from the underlying `Vec<Word>` whenever
/// possible.
///
/// However, storing instructions in a variable-sized way brings the same disadvantages as UTF-8 characters in
/// [`String`]. You can't arbitrarily index into an `InstVec` like you can with a [`Vec`], as only indices pointing to
/// the beginning of an instruction are valid. You also can't easily replace instructions in the middle of the stream,
/// as changing the size of an instruction requires you to move all following instructions. Although, you can remove
/// instructions by filling them with `OpNop`.
///
/// # Safety
/// The referenced words must be an encoded instruction that is valid within the `ISA` instruction set. Encountering an
/// invalid instruction will panic.
pub struct InstVec<ISA: InstEncoding> {
    words: Vec<Word>,
    _phantom: PhantomData<ISA>,
}

impl<ISA: InstEncoding> InstVec<ISA> {
    /// Create a new empty [`InstVec`]
    #[inline]
    pub const fn new() -> Self {
        Self {
            words: Vec::new(),
            _phantom: PhantomData,
        }
    }

    /// Create a new [`InstVec`] from a [`Vec`] safely, by verifying the instructions within to be valid for the `ISA`
    pub fn from_words(words: Vec<Word>) -> Result<Self, DecodeError> {
        FailableInstOffsetRefIter::<ISA>::from_words(&words).try_for_each(|v| v.map(|_| ()))?;
        Ok(Self::from_words_unchecked(words))
    }

    /// Create a new [`InstVec`] from a [`Vec`] of words without checking whether the words contain valid instructions.
    ///
    /// See [`InstVec`] # Safety
    #[inline]
    pub fn from_words_unchecked(words: Vec<Word>) -> Self {
        Self {
            words,
            _phantom: PhantomData,
        }
    }

    /// Appends an instruction to the back of the [`InstVec`].
    #[inline]
    pub fn push(&mut self, inst: impl Into<ISA>) -> InstOffset {
        let offset = InstOffset(self.words.len());
        inst.into()
            .encode(&mut self.words)
            .expect("error while encoding");
        offset
    }

    /// Moves all the instruction of `other` into `self`, leaving `other` empty.
    #[inline]
    pub fn append(&mut self, other: &mut Self) {
        self.words.append(&mut other.words);
    }

    /// Iterate over [`InstRef`]
    #[inline]
    pub fn iter_ref(&self) -> InstRefIter<'_, ISA> {
        InstRefIter::from_words_unchecked(&self.words)
    }

    /// Iterate over instructions
    #[inline]
    pub fn iter(&self) -> InstIter<'_, ISA> {
        InstIter::from_words_unchecked(&self.words)
    }

    /// Turn into the inner Vec of words
    pub fn into_vec(self) -> Vec<Word> {
        self.words
    }

    /// View this [`InstVec`] as a slice of words
    pub const fn as_slice(&self) -> &[Word] {
        self.words.as_slice()
    }

    pub fn dis(&self, opt: DisOptions) -> Result<DisModule<'_, ISA>, DecodeError> {
        DisModule::new(self.as_slice(), opt)
    }
}

impl<ISA: InstEncoding> Extend<ISA> for InstVec<ISA> {
    fn extend<T: IntoIterator<Item = ISA>>(&mut self, iter: T) {
        for inst in iter {
            self.push(inst);
        }
    }
}

impl<ISA: InstEncoding> FromIterator<ISA> for InstVec<ISA> {
    fn from_iter<T: IntoIterator<Item = ISA>>(iter: T) -> Self {
        let mut s = Self::new();
        for inst in iter {
            s.push(inst);
        }
        s
    }
}

impl<ISA: InstEncoding> Clone for InstVec<ISA> {
    fn clone(&self) -> Self {
        Self {
            words: self.words.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<ISA: InstEncoding> Debug for InstVec<ISA> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InstVec")
            .field("words", &self.words)
            .finish()
    }
}

impl<ISA: InstEncoding> Default for InstVec<ISA> {
    fn default() -> Self {
        Self::new()
    }
}

/// A reference to a valid instruction encoded in a slice of [`Word`]s.
///
/// Call [`Self::get`] to get the underlying instruction, as it unfortunately can't implement [`Deref`].
///
/// # Safety
/// The referenced words must be an encoded instruction that is valid within the `ISA` instruction set. Encountering an
/// invalid instruction will panic.
pub struct InstRef<'a, ISA: InstEncoding> {
    reader: InstReader<'a>,
    _phantom: PhantomData<ISA>,
}

impl<'a, ISA: InstEncoding> InstRef<'a, ISA> {
    #[inline]
    pub fn from_words(words: &'a [Word]) -> Result<Self, DecodeError> {
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

impl<'a, ISA: InstEncoding> Deref for InstRef<'a, ISA> {
    type Target = InstReader<'a>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.reader
    }
}

impl<ISA: InstEncoding> Copy for InstRef<'_, ISA> {}

impl<ISA: InstEncoding> Clone for InstRef<'_, ISA> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<ISA: InstEncoding> Debug for InstRef<'_, ISA> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InstRef")
            .field("reader", &self.reader)
            .finish()
    }
}

/// An [`Iterator`] of [`Result`] yielding either `(`[`InstOffset`]`, `[`InstRef`]`)` or [`DecodeError`]
pub struct FailableInstOffsetRefIter<'a, ISA: InstEncoding> {
    offset: InstOffset,
    words: &'a [Word],
    _phantom: PhantomData<ISA>,
}

impl<'a, ISA: InstEncoding> FailableInstOffsetRefIter<'a, ISA> {
    /// Create a new iterator
    #[inline]
    pub fn from_words(words: &'a [Word]) -> Self {
        Self {
            offset: InstOffset::default(),
            words,
            _phantom: PhantomData,
        }
    }

    fn debug_fmt(&self, f: &mut Formatter<'_>, name: &str) -> std::fmt::Result {
        f.debug_struct(name)
            .field("offset", &self.offset)
            .field("words", &self.words)
            .finish()
    }
}

impl<'a, ISA: InstEncoding> Iterator for FailableInstOffsetRefIter<'a, ISA> {
    type Item = Result<(InstOffset, InstRef<'a, ISA>), DecodeError>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match InstRef::from_words(&self.words[*self.offset..]) {
            Ok(inst_reader) => {
                let ret = Some(Ok((self.offset, inst_reader)));
                *self.offset += inst_reader.len();
                ret
            }
            Err(DecodeError::OutOfInstructions) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

impl<ISA: InstEncoding> Clone for FailableInstOffsetRefIter<'_, ISA> {
    fn clone(&self) -> Self {
        Self {
            offset: self.offset,
            words: self.words,
            _phantom: PhantomData,
        }
    }
}

impl<ISA: InstEncoding> Debug for FailableInstOffsetRefIter<'_, ISA> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.debug_fmt(f, "FailableInstOffsetRefIter")
    }
}

/// An [`Iterator`] of `(`[`InstOffset`]`, `[`InstRef`]`)` that panics when instructions fail to decode
pub struct InstRefOffsetIter<'a, ISA: InstEncoding>(FailableInstOffsetRefIter<'a, ISA>);

impl<'a, ISA: InstEncoding> InstRefOffsetIter<'a, ISA> {
    /// Create a new iterator, trusting that the supplied Words are valid in the `ISA`
    #[inline]
    pub fn from_words_unchecked(words: &'a [Word]) -> Self {
        Self(FailableInstOffsetRefIter::from_words(words))
    }
}

impl<'a, ISA: InstEncoding> Iterator for InstRefOffsetIter<'a, ISA> {
    type Item = (InstOffset, InstRef<'a, ISA>);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(
            self.0
                .next()?
                .expect("Instruction decoding failed unexpectingly"),
        )
    }
}

impl<ISA: InstEncoding> Clone for InstRefOffsetIter<'_, ISA> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<ISA: InstEncoding> Debug for InstRefOffsetIter<'_, ISA> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.debug_fmt(f, "InstRefOffsetIter")
    }
}

/// An [`Iterator`] of [`InstRef`] that panics when instructions fail to decode
pub struct InstRefIter<'a, ISA: InstEncoding>(InstRefOffsetIter<'a, ISA>);

impl<'a, ISA: InstEncoding> InstRefIter<'a, ISA> {
    /// Create a new iterator, trusting that the supplied Words are valid in the `ISA`
    #[inline]
    pub fn from_words_unchecked(words: &'a [Word]) -> Self {
        Self(InstRefOffsetIter::from_words_unchecked(words))
    }

    pub fn with_offsets(self) -> InstRefOffsetIter<'a, ISA> {
        self.0
    }
}

impl<'a, ISA: InstEncoding> Iterator for InstRefIter<'a, ISA> {
    type Item = InstRef<'a, ISA>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.0.next()?.1)
    }
}

impl<ISA: InstEncoding> Clone for InstRefIter<'_, ISA> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<ISA: InstEncoding> Debug for InstRefIter<'_, ISA> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.0.debug_fmt(f, "InstRefIter")
    }
}

/// An [`Iterator`] of instructions that panics when instructions fail to decode
pub struct InstIter<'a, ISA: InstEncoding>(InstRefOffsetIter<'a, ISA>);

impl<'a, ISA: InstEncoding> InstIter<'a, ISA> {
    /// Create a new iterator, trusting that the supplied Words are valid in the `ISA`
    #[inline]
    pub fn from_words_unchecked(words: &'a [Word]) -> Self {
        Self(InstRefOffsetIter::from_words_unchecked(words))
    }
}

impl<'a, ISA: InstEncoding> Iterator for InstIter<'a, ISA> {
    type Item = ISA;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.0.next()?.1.get())
    }
}

impl<ISA: InstEncoding> Clone for InstIter<'_, ISA> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<ISA: InstEncoding> Debug for InstIter<'_, ISA> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.0.debug_fmt(f, "InstIter")
    }
}
