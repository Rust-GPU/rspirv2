use crate::Word;
use crate::binary::{
    DisallowedIdResultAllocator, EncodeError, IdResultAlloc, IdResultAllocator, WordWriter,
};
use crate::inst::{Inst, MaybeIdResult};
use crate::operand::IdResult;

/// On an [`InstWriter`] you can directly [`push`] SPIR-V [`Inst`]ructions to encode them and allocate required
/// [`IdResult`]s.
///
/// [`push`]: `Self::push`
#[derive(Clone, Debug, Default)]
pub struct InstWriter<W: WordWriter, A: IdResultAlloc> {
    pub words: W,
    pub alloc: A,
}

impl<W: WordWriter, A: IdResultAlloc> InstWriter<W, A> {
    pub fn new(words: W, alloc: A) -> Self {
        Self { words, alloc }
    }

    #[inline]
    pub fn push<I: for<'a> Inst<'a>>(
        &mut self,
        mut inst: I,
    ) -> Result<<I::MaybeIdResult as MaybeIdResult>::IdResult, EncodeError> {
        self.push_mut(&mut inst)
    }

    #[inline]
    pub fn push_mut<I: for<'a> Inst<'a>>(
        &mut self,
        inst: &mut I,
    ) -> Result<<I::MaybeIdResult as MaybeIdResult>::IdResult, EncodeError> {
        let id_result = inst.id_result().alloc(self)?;
        inst.encode(self)?;
        Ok(id_result)
    }
}

impl<W: WordWriter> InstWriter<W, DisallowedIdResultAllocator> {
    pub fn new_no_alloc(words: W) -> Self {
        Self {
            words,
            alloc: DisallowedIdResultAllocator,
        }
    }
}

impl<W: WordWriter, A: IdResultAlloc> WordWriter for InstWriter<W, A> {
    #[inline]
    fn write(&mut self, word: Word) {
        self.words.write(word);
    }

    #[inline]
    fn write_iter(&mut self, iter: impl IntoIterator<Item = Word>) {
        self.words.write_iter(iter);
    }

    #[inline]
    fn inst_reserve(&mut self, len: usize) {
        self.words.inst_reserve(len);
    }
}

impl<W: WordWriter, A: IdResultAlloc> IdResultAlloc for InstWriter<W, A> {
    #[inline]
    fn alloc_id(&mut self) -> Result<IdResult, EncodeError> {
        self.alloc.alloc_id()
    }
}

/// A simple [`InstWriter`] writing into a `Vec<Word>` and using the simple [`IdResultAllocator`].
pub type VecInstWriter = InstWriter<Vec<Word>, IdResultAllocator>;
