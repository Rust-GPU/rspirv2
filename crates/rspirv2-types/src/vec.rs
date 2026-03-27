use crate::Word;
use crate::binary::{DecodeError, InstOffset, WordWriter};
use crate::inst::{Inst, InstEncoding};
use crate::slice::{InstSlice, RawInstSlice};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::ops::Deref;

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
        self.raw.push(inst.into())
    }

    /// Appends an instruction to the back of the [`InstVec`]. See [`Vec::push`].
    #[inline]
    pub fn push_inst<I: Inst + Into<ISA>>(&mut self, inst: I) -> I::MaybeIdResult {
        self.raw.push_inst(inst)
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
    pub const fn as_slice(&self) -> &InstSlice<ISA> {
        InstSlice::from_raw_unchecked(self.raw.as_slice())
    }

    /// View this [`InstVec`] as a [`RawInstSlice`]
    pub const fn as_raw_slice(&self) -> &RawInstSlice {
        self.raw.as_slice()
    }
}

impl<ISA: InstEncoding> Deref for InstVec<ISA> {
    type Target = InstSlice<ISA>;

    fn deref(&self) -> &Self::Target {
        self.as_slice()
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

    /// Appends an instruction to the back of the [`RawInstVec`]. See [`Vec::push`].
    #[inline]
    pub fn push_inst<I: Inst>(&mut self, inst: I) -> I::MaybeIdResult {
        let id_result = inst.id_result();
        self.push(inst);
        id_result
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
    pub const fn as_slice(&self) -> &RawInstSlice {
        RawInstSlice::from_words(self.0.as_slice())
    }
}

impl Deref for RawInstVec {
    type Target = RawInstSlice;

    fn deref(&self) -> &Self::Target {
        self.as_slice()
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
