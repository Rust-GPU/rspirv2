use crate::binary::EncodeError;
use crate::operand::Word;
use smallvec::SmallVec;
use std::ops::{Deref, DerefMut};

/// An `InstructionWriter` is some sort of `Vec` you can [`Self::push`] [`Word`]s into.
pub trait InstructionWriter {
    fn write(&mut self, word: Word);

    fn write_op(&mut self, op: u16, len: usize) -> Result<(), EncodeError> {
        self.write(Word::new_op(op, len)?);
        Ok(())
    }

    fn write_iter(&mut self, iter: impl IntoIterator<Item = Word>) {
        for word in iter {
            self.write(word);
        }
    }
}

impl InstructionWriter for Vec<Word> {
    fn write(&mut self, word: Word) {
        self.push(word);
    }

    fn write_op(&mut self, op: u16, len: usize) -> Result<(), EncodeError> {
        self.reserve(len);
        self.write(Word::new_op(op, len)?);
        Ok(())
    }

    fn write_iter(&mut self, iter: impl IntoIterator<Item = Word>) {
        Extend::extend(self, iter);
    }
}

impl InstructionWriter for Vec<u32> {
    fn write(&mut self, word: Word) {
        self.push(word.0);
    }

    fn write_op(&mut self, op: u16, len: usize) -> Result<(), EncodeError> {
        self.reserve(len);
        self.write(Word::new_op(op, len)?);
        Ok(())
    }

    fn write_iter(&mut self, iter: impl IntoIterator<Item = Word>) {
        Extend::extend(self, iter.into_iter().map(|i| i.0));
    }
}

impl InstructionWriter for Vec<u8> {
    fn write(&mut self, word: Word) {
        Extend::extend(self, word.to_u8_array());
    }

    fn write_op(&mut self, op: u16, len: usize) -> Result<(), EncodeError> {
        self.reserve(len);
        self.write(Word::new_op(op, len)?);
        Ok(())
    }

    fn write_iter(&mut self, iter: impl IntoIterator<Item = Word>) {
        Extend::extend(
            self,
            iter.into_iter().flat_map(|i| i.to_u8_array().into_iter()),
        );
    }
}

impl<const N: usize> InstructionWriter for SmallVec<[Word; N]> {
    fn write(&mut self, word: Word) {
        self.push(word);
    }

    fn write_op(&mut self, op: u16, len: usize) -> Result<(), EncodeError> {
        self.reserve(len);
        self.write(Word::new_op(op, len)?);
        Ok(())
    }

    fn write_iter(&mut self, iter: impl IntoIterator<Item = Word>) {
        Extend::extend(self, iter);
    }
}

impl<const N: usize> InstructionWriter for SmallVec<[u32; N]> {
    fn write(&mut self, word: Word) {
        self.push(word.0);
    }

    fn write_op(&mut self, op: u16, len: usize) -> Result<(), EncodeError> {
        self.reserve(len);
        self.write(Word::new_op(op, len)?);
        Ok(())
    }

    fn write_iter(&mut self, iter: impl IntoIterator<Item = Word>) {
        Extend::extend(self, iter.into_iter().map(|i| i.0));
    }
}

impl<const N: usize> InstructionWriter for SmallVec<[u8; N]> {
    fn write(&mut self, word: Word) {
        Extend::extend(self, word.to_u8_array());
    }

    fn write_op(&mut self, op: u16, len: usize) -> Result<(), EncodeError> {
        self.reserve(len);
        self.write(Word::new_op(op, len)?);
        Ok(())
    }

    fn write_iter(&mut self, iter: impl IntoIterator<Item = Word>) {
        Extend::extend(
            self,
            iter.into_iter().flat_map(|i| i.to_u8_array().into_iter()),
        );
    }
}

/// Counts the amount of [`Word`]s that were written, discarding the words itself
#[derive(Clone, Debug, Default)]
pub struct WordCounter(pub usize);

impl Deref for WordCounter {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for WordCounter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl InstructionWriter for WordCounter {
    fn write(&mut self, _: Word) {
        self.0 += 1;
    }

    fn write_iter(&mut self, iter: impl IntoIterator<Item = Word>) {
        self.0 += iter.into_iter().count();
    }
}
