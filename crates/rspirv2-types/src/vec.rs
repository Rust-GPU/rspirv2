use crate::Word;
use crate::binary::{DecodeError, InstOffset};
use crate::dis::DisOptions;
use crate::inst::InstEncoding;
use crate::slice::{DisInstSlice, InstIter, InstRefIter, InstSlice, RawInstSlice};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

/// An `InstVec` works like a `Vec<ISA>`, but stores instructions in the variable-sized SPIR-V binary form to safe
/// on memory size.
///
/// See [`InstSlice`] for details.
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
    #[inline]
    pub fn from_words(words: Vec<Word>) -> Result<Self, DecodeError> {
        RawInstSlice::from_words(words.as_slice()).verify_valid_in_isa::<ISA>()?;
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

    /// Appends an instruction to the back of the [`InstVec`]. See [`Vec::push`].
    #[inline]
    pub fn push(&mut self, inst: impl Into<ISA>) -> InstOffset {
        let offset = InstOffset(self.words.len());
        inst.into()
            .encode(&mut self.words)
            .expect("error while encoding");
        offset
    }

    /// Moves all the instruction of `other` into `self`, leaving `other` empty. See [`Vec::append`].
    #[inline]
    pub fn append(&mut self, other: &mut Self) {
        self.words.append(&mut other.words);
    }

    /// Turn into the inner Vec of words
    #[inline]
    pub fn into_vec(self) -> Vec<Word> {
        self.words
    }

    /// View this [`InstVec`] as a [`InstSlice`]
    pub const fn as_slice(&self) -> InstSlice<'_, ISA> {
        InstSlice::from_words_unchecked(self.words.as_slice())
    }

    /// View this [`InstVec`] as a [`RawInstSlice`]
    pub const fn as_raw_slice(&self) -> RawInstSlice<'_> {
        RawInstSlice::from_words(self.words.as_slice())
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
