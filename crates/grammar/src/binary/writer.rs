use crate::operand::Word;
use smallvec::SmallVec;

/// An `InstructionWriter` is some sort of `Vec` you can [`Self::push`] [`Word`]s into.
pub trait InstructionWriter {
    fn push(&mut self, word: Word);
    fn extend(&mut self, iter: impl IntoIterator<Item = Word>) {
        for word in iter {
            self.push(word);
        }
    }
}

impl InstructionWriter for Vec<Word> {
    fn push(&mut self, word: Word) {
        self.push(word);
    }

    fn extend(&mut self, iter: impl IntoIterator<Item = Word>) {
        Extend::extend(self, iter);
    }
}

impl InstructionWriter for Vec<u32> {
    fn push(&mut self, word: Word) {
        self.push(word.0);
    }

    fn extend(&mut self, iter: impl IntoIterator<Item = Word>) {
        Extend::extend(self, iter.into_iter().map(|i| i.0));
    }
}

impl InstructionWriter for Vec<u8> {
    fn push(&mut self, word: Word) {
        Extend::extend(self, word.to_u8_array());
    }

    fn extend(&mut self, iter: impl IntoIterator<Item = Word>) {
        Extend::extend(
            self,
            iter.into_iter().flat_map(|i| i.to_u8_array().into_iter()),
        );
    }
}

impl<const N: usize> InstructionWriter for SmallVec<[Word; N]> {
    fn push(&mut self, word: Word) {
        self.push(word);
    }

    fn extend(&mut self, iter: impl IntoIterator<Item = Word>) {
        Extend::extend(self, iter);
    }
}

impl<const N: usize> InstructionWriter for SmallVec<[u32; N]> {
    fn push(&mut self, word: Word) {
        self.push(word.0);
    }

    fn extend(&mut self, iter: impl IntoIterator<Item = Word>) {
        Extend::extend(self, iter.into_iter().map(|i| i.0));
    }
}

impl<const N: usize> InstructionWriter for SmallVec<[u8; N]> {
    fn push(&mut self, word: Word) {
        Extend::extend(self, word.to_u8_array());
    }

    fn extend(&mut self, iter: impl IntoIterator<Item = Word>) {
        Extend::extend(
            self,
            iter.into_iter().flat_map(|i| i.to_u8_array().into_iter()),
        );
    }
}
