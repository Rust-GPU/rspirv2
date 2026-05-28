use crate::Word;
use crate::binary::{DecodeError, DecodeErrorKind, EncodeError, OperandReader, WordWriter};
use crate::operand::{
    IdRef, IdResult, LiteralConst, LiteralInteger, OperandDisContext, OperandEncoding,
};
use smallvec::SmallVec;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::marker::PhantomData;

/// A typed literal width for an [`OpSwitch`](https://registry.khronos.org/SPIR-V/specs/unified1/SPIRV.html#OpSwitch)
/// case value.
///
/// SPIR-V encodes each `OpSwitch` case literal with the bit width of the selector's
/// integer type. That selector type is not locally available while decoding a
/// single instruction, so [`SwitchTargets`] stores the encoded case table
/// losslessly and exposes typed views through this trait.
pub trait SwitchLiteral: private::Sealed + Copy + Debug + Eq + PartialEq + Hash {
    const WORD_LEN: usize;

    fn from_words(words: &[Word]) -> Self;

    fn append_words(self, words: &mut SmallVec<[Word; 6]>);

    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &OperandDisContext<'_>) -> std::fmt::Result;
}

/// A one-word `OpSwitch` case literal.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SwitchLiteral32(Word);

impl SwitchLiteral32 {
    #[inline]
    pub const fn new(value: u32) -> Self {
        Self(Word(value))
    }

    #[inline]
    pub const fn from_word(value: Word) -> Self {
        Self(value)
    }

    #[inline]
    pub const fn to_word(self) -> Word {
        self.0
    }

    #[inline]
    pub const fn to_u32(self) -> u32 {
        self.0.0
    }
}

impl From<u32> for SwitchLiteral32 {
    #[inline]
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl private::Sealed for SwitchLiteral32 {}

impl SwitchLiteral for SwitchLiteral32 {
    const WORD_LEN: usize = 1;

    #[inline]
    fn from_words(words: &[Word]) -> Self {
        debug_assert_eq!(words.len(), Self::WORD_LEN);
        Self(words[0])
    }

    #[inline]
    fn append_words(self, words: &mut SmallVec<[Word; 6]>) {
        words.push(self.0);
    }

    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &OperandDisContext<'_>) -> std::fmt::Result {
        LiteralInteger::from_word(self.0).dis_fmt(f, ctx)
    }
}

/// A two-word `OpSwitch` case literal.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SwitchLiteral64([Word; 2]);

impl SwitchLiteral64 {
    #[inline]
    pub const fn new(value: u64) -> Self {
        Self([Word(value as u32), Word((value >> 32) as u32)])
    }

    #[inline]
    pub const fn from_words_array(words: [Word; 2]) -> Self {
        Self(words)
    }

    #[inline]
    pub const fn to_words(self) -> [Word; 2] {
        self.0
    }

    #[inline]
    pub const fn to_u64(self) -> u64 {
        self.0[0].0 as u64 | ((self.0[1].0 as u64) << 32)
    }
}

impl From<u64> for SwitchLiteral64 {
    #[inline]
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl private::Sealed for SwitchLiteral64 {}

impl SwitchLiteral for SwitchLiteral64 {
    const WORD_LEN: usize = 2;

    #[inline]
    fn from_words(words: &[Word]) -> Self {
        debug_assert_eq!(words.len(), Self::WORD_LEN);
        Self([words[0], words[1]])
    }

    #[inline]
    fn append_words(self, words: &mut SmallVec<[Word; 6]>) {
        words.extend(self.0);
    }

    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &OperandDisContext<'_>) -> std::fmt::Result {
        LiteralConst::from(self.to_u64()).dis_fmt(f, ctx)
    }
}

/// A typed `OpSwitch` case target.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct SwitchCase<L: SwitchLiteral> {
    literal: L,
    target: IdRef,
}

impl<L: SwitchLiteral> SwitchCase<L> {
    #[inline]
    pub const fn new(literal: L, target: IdRef) -> Self {
        Self { literal, target }
    }

    #[inline]
    pub const fn literal(self) -> L {
        self.literal
    }

    #[inline]
    pub const fn target(self) -> IdRef {
        self.target
    }

    #[inline]
    fn append_words(self, words: &mut SmallVec<[Word; 6]>) {
        self.literal.append_words(words);
        words.push(self.target.0.0);
    }

    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &OperandDisContext<'_>) -> std::fmt::Result {
        self.literal.dis_fmt(f, ctx)?;
        self.target.dis_fmt(f, ctx)
    }
}

/// A lossless `OpSwitch` case table.
///
/// A decoded `OpSwitch` cannot know whether the trailing words are 32-bit or
/// 64-bit cases without looking up the selector's type elsewhere in the module.
/// This type therefore preserves the raw words while providing typed
/// constructors and typed views when that selector width is known.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct SwitchTargets {
    words: SmallVec<[Word; 6]>,
}

impl SwitchTargets {
    #[inline]
    pub fn empty() -> Self {
        Self::default()
    }

    #[inline]
    pub fn from_cases<L: SwitchLiteral>(cases: impl IntoIterator<Item = SwitchCase<L>>) -> Self {
        let mut words = SmallVec::new();
        for case in cases {
            case.append_words(&mut words);
        }
        Self { words }
    }

    #[inline]
    pub fn try_from_words(
        words: impl IntoIterator<Item = Word>,
    ) -> Result<Self, InvalidSwitchTargets> {
        let words = SmallVec::from_iter(words);
        validate_switch_target_words(words.len())?;
        Ok(Self { words })
    }

    #[inline]
    pub fn as_words(&self) -> &[Word] {
        &self.words
    }

    #[inline]
    pub fn cases<L: SwitchLiteral>(&self) -> Result<SwitchCases<'_, L>, InvalidSwitchTargets> {
        let case_word_len = switch_case_word_len::<L>();
        if !self.words.len().is_multiple_of(case_word_len) {
            return Err(InvalidSwitchTargets::new(self.words.len()));
        }
        Ok(SwitchCases {
            words: &self.words,
            offset: 0,
            _literal: PhantomData,
        })
    }
}

unsafe impl OperandEncoding for SwitchTargets {
    const FIXED_LEN: Option<usize> = None;

    #[inline]
    fn word_len(&self) -> usize {
        self.words.len()
    }

    #[inline]
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write_iter(self.words.iter().copied());
        Ok(())
    }

    #[inline]
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        Self::decode_last(reader)
    }

    #[inline]
    fn decode_last(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        Self::try_from_words(reader.by_ref()).map_err(DecodeError::from)
    }

    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &OperandDisContext<'_>) -> std::fmt::Result {
        if self
            .words
            .len()
            .is_multiple_of(switch_case_word_len::<SwitchLiteral32>())
        {
            for case in self.cases::<SwitchLiteral32>().expect("validated above") {
                case.dis_fmt(f, ctx)?;
            }
        } else {
            for case in self
                .cases::<SwitchLiteral64>()
                .expect("validated on construction")
            {
                case.dis_fmt(f, ctx)?;
            }
        }
        Ok(())
    }
}

/// A typed view over [`SwitchTargets`].
#[derive(Clone, Debug)]
pub struct SwitchCases<'a, L: SwitchLiteral> {
    words: &'a [Word],
    offset: usize,
    _literal: PhantomData<L>,
}

impl<L: SwitchLiteral> Iterator for SwitchCases<'_, L> {
    type Item = SwitchCase<L>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let case_word_len = switch_case_word_len::<L>();
        if self.offset == self.words.len() {
            return None;
        }
        let literal = L::from_words(&self.words[self.offset..self.offset + L::WORD_LEN]);
        let target = IdRef(IdResult(self.words[self.offset + L::WORD_LEN]));
        self.offset += case_word_len;
        Some(SwitchCase::new(literal, target))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<L: SwitchLiteral> ExactSizeIterator for SwitchCases<'_, L> {
    #[inline]
    fn len(&self) -> usize {
        (self.words.len() - self.offset) / switch_case_word_len::<L>()
    }
}

/// Error returned when an `OpSwitch` target table cannot be represented as a
/// sequence of 32-bit or 64-bit cases.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct InvalidSwitchTargets {
    word_len: usize,
}

impl InvalidSwitchTargets {
    #[inline]
    pub const fn new(word_len: usize) -> Self {
        Self { word_len }
    }

    #[inline]
    pub const fn word_len(self) -> usize {
        self.word_len
    }
}

impl Display for InvalidSwitchTargets {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "OpSwitch target table has {} words, which is neither 32-bit nor 64-bit case encoding",
            self.word_len
        )
    }
}

impl Error for InvalidSwitchTargets {}

impl From<InvalidSwitchTargets> for DecodeError {
    #[inline]
    fn from(value: InvalidSwitchTargets) -> Self {
        DecodeErrorKind::InvalidSwitchTargets {
            word_len: value.word_len,
        }
        .into()
    }
}

#[inline]
fn validate_switch_target_words(word_len: usize) -> Result<(), InvalidSwitchTargets> {
    if word_len.is_multiple_of(switch_case_word_len::<SwitchLiteral32>())
        || word_len.is_multiple_of(switch_case_word_len::<SwitchLiteral64>())
    {
        Ok(())
    } else {
        Err(InvalidSwitchTargets::new(word_len))
    }
}

#[inline]
fn switch_case_word_len<L: SwitchLiteral>() -> usize {
    L::WORD_LEN + IdRef::FIXED_LEN.expect("IdRef has a fixed width")
}

mod private {
    pub trait Sealed {}
}
