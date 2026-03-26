use crate::Word;
use crate::binary::{DecodeError, InstOffset, WordWriter};
use crate::dis::DisOptions;
use crate::inst::InstEncoding;
use crate::slice::{DisInstSlice, InstIter, InstRefIter, InstSlice, RawInstRefIter, RawInstSlice};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

/// An `InstVec` works like a `Vec<ISA>`, but stores instructions in the variable-sized SPIR-V binary form to safe
/// on memory size.
///
/// See [`InstSlice`] for details.
pub struct InstVec<ISA: InstEncoding> {
    raw: RawInstVec,
    _phantom: PhantomData<ISA>,
}

impl<ISA: InstEncoding> InstVec<ISA> {
    /// Create a new empty [`InstVec`]
    #[inline]
    pub const fn new() -> Self {
        Self {
            raw: RawInstVec::new(),
            _phantom: PhantomData,
        }
    }

    /// Create a new [`InstVec`] from a [`Vec`] safely, by verifying the instructions within to be valid for the `ISA`
    #[inline]
    pub fn from_words(words: Vec<Word>) -> Result<Self, DecodeError> {
        Self::from_raw(RawInstVec::from_words(words))
    }

    /// Create a new [`InstVec`] from a [`RawInstVec`] safely, by verifying the instructions within to be valid for the
    /// `ISA`
    #[inline]
    pub fn from_raw(raw: RawInstVec) -> Result<Self, DecodeError> {
        raw.as_slice().verify_valid_in_isa::<ISA>()?;
        Ok(Self::from_raw_unchecked(raw))
    }

    /// Create a new [`InstVec`] from a [`Vec`] of words without checking whether the words contain valid instructions.
    ///
    /// See [`InstVec`] # Safety
    #[inline]
    pub fn from_words_unchecked(words: Vec<Word>) -> Self {
        Self::from_raw_unchecked(RawInstVec::from_words(words))
    }

    /// Create a new [`InstVec`] from a [`RawInstVec`] without checking whether the words contain valid instructions.
    ///
    /// See [`InstVec`] # Safety
    #[inline]
    pub fn from_raw_unchecked(raw: RawInstVec) -> Self {
        Self {
            raw,
            _phantom: PhantomData,
        }
    }

    /// Appends an instruction to the back of the [`InstVec`]. See [`Vec::push`].
    #[inline]
    pub fn push(&mut self, inst: impl Into<ISA>) -> InstOffset {
        let offset = InstOffset(self.raw.0.len());
        inst.into().encode(&mut self.raw).expect("error while encoding");
        offset
    }

    /// Moves all the instruction of `other` into `self`, leaving `other` empty. See [`Vec::append`].
    #[inline]
    pub fn append(&mut self, other: &mut Self) {
        self.raw.append(&mut other.raw);
    }

    /// Turn into the inner Vec of words
    #[inline]
    pub fn into_raw(self) -> RawInstVec {
        self.raw
    }

    /// Turn into the inner Vec of words
    #[inline]
    pub fn into_vec(self) -> Vec<Word> {
        self.raw.0
    }

    /// View this [`InstVec`] as a [`InstSlice`]
    pub const fn as_slice(&self) -> InstSlice<'_, ISA> {
        InstSlice::from_raw_unchecked(self.raw.as_slice())
    }

    /// View this [`InstVec`] as a [`RawInstSlice`]
    pub const fn as_raw_slice(&self) -> RawInstSlice<'_> {
        self.raw.as_slice()
    }

    // ---------------------------------
    // below are equivalent to InstSlice
    // ---------------------------------

    /// View this [`InstVec`] as a slice of words
    #[inline]
    pub const fn as_words(&self) -> &[Word] {
        self.as_slice().as_raw().as_words()
    }

    /// Iterate over instruction references ([`InstRef`]) without decoding the instruction itself
    ///
    /// [`InstRef`]: crate::inst::InstRef
    #[inline]
    pub const fn iter_ref(&self) -> InstRefIter<'_, ISA> {
        InstRefIter::new(self.as_slice())
    }

    /// Iterate over all instructions
    #[inline]
    pub const fn iter(&self) -> InstIter<'_, ISA> {
        InstIter::new(self.as_slice())
    }

    /// disassemble
    #[inline]
    pub fn dis(&self, opt: DisOptions) -> Result<DisInstSlice<'_, ISA>, DecodeError> {
        DisInstSlice::new(self.as_slice(), opt)
    }
}

impl<ISA: InstEncoding> Extend<ISA> for InstVec<ISA> {
    fn extend<T: IntoIterator<Item = ISA>>(&mut self, iter: T) {
        self.raw.extend(iter);
    }
}

impl<ISA: InstEncoding> FromIterator<ISA> for InstVec<ISA> {
    fn from_iter<T: IntoIterator<Item = ISA>>(iter: T) -> Self {
        Self::from_raw_unchecked(RawInstVec::from_iter(iter))
    }
}

impl<ISA: InstEncoding> Clone for InstVec<ISA> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            raw: self.raw.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<ISA: InstEncoding> Debug for InstVec<ISA> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InstVec")
            .field("ISA", &ISA::name())
            .field("raw", &self.raw)
            .finish()
    }
}

impl<ISA: InstEncoding> Default for InstVec<ISA> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

/// A slice of SPIR-V instructions of an arbitrary instruction set.
///
/// Compared to [`InstVec`], does not specify the instruction set nor require the inner words to be valid instructions
/// in said instruction set.
#[derive(Clone, Debug, Default)]
pub struct RawInstVec(pub Vec<Word>);

impl RawInstVec {
    /// Create a new empty [`RawInstVec`]
    #[inline]
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    /// Create a new [`RawInstVec`] from a [`Vec`]
    #[inline]
    pub fn from_words(words: Vec<Word>) -> Self {
        Self(words)
    }

    /// Appends an instruction to the back of the [`RawInstVec`], like [`Vec::push`].
    #[inline]
    pub fn push(&mut self, inst: impl InstEncoding) -> InstOffset {
        let offset = InstOffset(self.0.len());
        inst.encode(&mut self.0).expect("error while encoding");
        offset
    }

    /// Moves all the instruction of `other` into `self`, leaving `other` empty. See [`Vec::append`].
    #[inline]
    pub fn append(&mut self, other: &mut Self) {
        self.0.append(&mut other.0);
    }

    /// Turn into the inner Vec of words
    #[inline]
    pub fn into_vec(self) -> Vec<Word> {
        self.0
    }

    /// View this [`RawInstVec`] as a [`RawInstSlice`]
    pub const fn as_slice(&self) -> RawInstSlice<'_> {
        RawInstSlice::from_words(self.0.as_slice())
    }

    // ---------------------------------
    // below are equivalent to InstSlice
    // ---------------------------------

    /// View this [`RawInstVec`] as a slice of words
    #[inline]
    pub const fn as_words(&self) -> &[Word] {
        self.as_slice().as_words()
    }

    /// Iterate over instruction references ([`InstRef`]) without decoding the instruction itself
    pub const fn iter(&self) -> RawInstRefIter<'_> {
        self.as_slice().iter()
    }
}

impl<ISA: InstEncoding> Extend<ISA> for RawInstVec {
    fn extend<T: IntoIterator<Item = ISA>>(&mut self, iter: T) {
        for inst in iter {
            self.push(inst);
        }
    }
}

impl<ISA: InstEncoding> FromIterator<ISA> for RawInstVec {
    fn from_iter<T: IntoIterator<Item = ISA>>(iter: T) -> Self {
        let mut s = Self::new();
        for inst in iter {
            s.push(inst);
        }
        s
    }
}

impl WordWriter for RawInstVec {
    #[inline]
    fn write(&mut self, word: Word) {
        self.0.write(word);
    }

    #[inline]
    fn write_iter(&mut self, iter: impl IntoIterator<Item = Word>) {
        self.0.write_iter(iter);
    }

    #[inline]
    fn inst_reserve(&mut self, len: usize) {
        self.0.inst_reserve(len);
    }
}
