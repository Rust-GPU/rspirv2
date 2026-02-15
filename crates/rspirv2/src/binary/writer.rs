use crate::binary::EncodeError;
use crate::operand::Word;
use smallvec::SmallVec;
use std::ops::{Deref, DerefMut};

/// An `InstructionWriter` is some sort of `Vec` you can [`Self::push`] [`Word`]s into.
pub trait InstructionWriter {
    fn write(&mut self, word: Word) -> Result<(), EncodeError>;
    fn write_iter(&mut self, iter: impl IntoIterator<Item = Word>) -> Result<(), EncodeError> {
        for word in iter {
            self.write(word)?;
        }
        Ok(())
    }
}

impl InstructionWriter for Vec<Word> {
    fn write(&mut self, word: Word) -> Result<(), EncodeError> {
        self.push(word);
        Ok(())
    }

    fn write_iter(&mut self, iter: impl IntoIterator<Item = Word>) -> Result<(), EncodeError> {
        Extend::extend(self, iter);
        Ok(())
    }
}

impl InstructionWriter for Vec<u32> {
    fn write(&mut self, word: Word) -> Result<(), EncodeError> {
        self.push(word.0);
        Ok(())
    }

    fn write_iter(&mut self, iter: impl IntoIterator<Item = Word>) -> Result<(), EncodeError> {
        Extend::extend(self, iter.into_iter().map(|i| i.0));
        Ok(())
    }
}

impl InstructionWriter for Vec<u8> {
    fn write(&mut self, word: Word) -> Result<(), EncodeError> {
        Extend::extend(self, word.to_u8_array());
        Ok(())
    }

    fn write_iter(&mut self, iter: impl IntoIterator<Item = Word>) -> Result<(), EncodeError> {
        Extend::extend(
            self,
            iter.into_iter().flat_map(|i| i.to_u8_array().into_iter()),
        );
        Ok(())
    }
}

impl<const N: usize> InstructionWriter for SmallVec<[Word; N]> {
    fn write(&mut self, word: Word) -> Result<(), EncodeError> {
        self.push(word);
        Ok(())
    }

    fn write_iter(&mut self, iter: impl IntoIterator<Item = Word>) -> Result<(), EncodeError> {
        Extend::extend(self, iter);
        Ok(())
    }
}

impl<const N: usize> InstructionWriter for SmallVec<[u32; N]> {
    fn write(&mut self, word: Word) -> Result<(), EncodeError> {
        self.push(word.0);
        Ok(())
    }

    fn write_iter(&mut self, iter: impl IntoIterator<Item = Word>) -> Result<(), EncodeError> {
        Extend::extend(self, iter.into_iter().map(|i| i.0));
        Ok(())
    }
}

impl<const N: usize> InstructionWriter for SmallVec<[u8; N]> {
    fn write(&mut self, word: Word) -> Result<(), EncodeError> {
        Extend::extend(self, word.to_u8_array());
        Ok(())
    }

    fn write_iter(&mut self, iter: impl IntoIterator<Item = Word>) -> Result<(), EncodeError> {
        Extend::extend(
            self,
            iter.into_iter().flat_map(|i| i.to_u8_array().into_iter()),
        );
        Ok(())
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
    fn write(&mut self, _: Word) -> Result<(), EncodeError> {
        self.0 += 1;
        Ok(())
    }

    fn write_iter(&mut self, iter: impl IntoIterator<Item = Word>) -> Result<(), EncodeError> {
        self.0 += iter.into_iter().count();
        Ok(())
    }
}
