use crate::Word;
use crate::binary::EncodeError;
use crate::operand::{IdResult, IdResultType, OperandEncoding};
use smallvec::SmallVec;
use std::ops::{Deref, DerefMut};

/// A `WordWriter` allows you to [`Self::write`] [`Word`]s into it and is usually backed by a [`Vec`].
///
/// Implementations include `Vec<Word>` and [`WordCounter`].
pub trait WordWriter: Sized {
    /// Write a single [`Word`]
    fn write(&mut self, word: Word);

    /// Write an [`Iterator`] of [`Word`]s
    fn write_iter(&mut self, iter: impl IntoIterator<Item = Word>);

    /// Write the opcode, optional [`ResultId`], optional [`ResultIdType`] and the len of additional operands of this
    /// Instruction.
    ///
    /// The len is the length of "additional" operands written **after** this function call, e.g. for `OpIAdd` the len
    /// would be 2 for the two [`IdRef`]s referencing the SSA values to be added.
    #[inline]
    fn write_op(
        &mut self,
        op: u16,
        id_result: Option<IdResult>,
        id_result_type: Option<IdResultType>,
        len: usize,
    ) -> Result<(), EncodeError> {
        let len = len + 1 + id_result.is_some() as usize + id_result_type.is_some() as usize;
        self.inst_reserve(len);
        self.write(Word::new_op(op, len)?);
        if let Some(id_result) = id_result {
            id_result.encode(&mut *self)?;
        }
        if let Some(id_result_type) = id_result_type {
            id_result_type.encode(&mut *self)?;
        }
        Ok(())
    }

    /// Expect this many words to be written by the next instruction.
    ///
    /// Usually called right before an instruction is emitted, with the length of the instruction, so expect this to be
    /// called quite often. If you're writing into a [`Vec`], prefer [`Vec::reserve`] over [`Vec::reserve_exact`].
    #[inline]
    fn inst_reserve(&mut self, len: usize) {
        let _ = len;
    }
}

impl WordWriter for Vec<Word> {
    #[inline]
    fn write(&mut self, word: Word) {
        self.push(word);
    }

    #[inline]
    fn write_iter(&mut self, iter: impl IntoIterator<Item = Word>) {
        self.extend(iter);
    }

    #[inline]
    fn inst_reserve(&mut self, len: usize) {
        self.reserve(len);
    }
}

impl<const N: usize> WordWriter for SmallVec<[Word; N]> {
    #[inline]
    fn write(&mut self, word: Word) {
        self.push(word);
    }

    #[inline]
    fn write_iter(&mut self, iter: impl IntoIterator<Item = Word>) {
        self.extend(iter);
    }

    #[inline]
    fn inst_reserve(&mut self, len: usize) {
        self.reserve(len);
    }
}

impl<T: WordWriter> WordWriter for &mut T {
    #[inline]
    fn write(&mut self, word: Word) {
        T::write(*self, word);
    }

    #[inline]
    fn write_iter(&mut self, iter: impl IntoIterator<Item = Word>) {
        T::write_iter(*self, iter);
    }

    #[inline]
    fn inst_reserve(&mut self, len: usize) {
        T::inst_reserve(*self, len);
    }
}

/// Counts the amount of [`Word`]s that were written, discarding the words itself
#[derive(Clone, Debug, Default)]
pub struct WordCounter(pub usize);

impl Deref for WordCounter {
    type Target = usize;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for WordCounter {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl WordWriter for WordCounter {
    #[inline]
    fn write(&mut self, _: Word) {
        self.0 += 1;
    }

    #[inline]
    fn write_iter(&mut self, iter: impl IntoIterator<Item = Word>) {
        self.0 += iter.into_iter().count();
    }
}

pub struct WordSliceWriter<'a> {
    slice: &'a mut [Word],
    offset: usize,
}

impl<'a> WordSliceWriter<'a> {
    pub fn new(slice: &'a mut [Word]) -> Self {
        Self { slice, offset: 0 }
    }

    pub fn finalize(self) {
        assert_eq!(
            self.offset,
            self.slice.len(),
            "Wrote {} Words but expected {} Words to be written",
            self.offset,
            self.slice.len()
        );
    }
}

impl WordWriter for WordSliceWriter<'_> {
    fn write(&mut self, word: Word) {
        self.slice[self.offset] = word;
        self.offset += 1;
    }

    fn write_iter(&mut self, iter: impl IntoIterator<Item = Word>) {
        for word in iter {
            self.write(word);
        }
    }
}

pub struct FnWriter<F: FnMut(Word)>(pub F);

impl<F: FnMut(Word)> WordWriter for FnWriter<F> {
    fn write(&mut self, word: Word) {
        self.0(word);
    }

    fn write_iter(&mut self, iter: impl IntoIterator<Item = Word>) {
        for word in iter {
            self.write(word);
        }
    }
}
