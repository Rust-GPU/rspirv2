use crate::Word;
use crate::operand::IdResult;
use std::sync::Arc;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;

/// A reasonable upper bounds that ensures subsequent allocations by potentially multiple threads all fail
pub const ID_RESULT_MAX: u32 = 0x7FFFFFFF;

/// A sharable atomic [`IdResultAlloc`], just [`Clone`] to share it
#[derive(Clone, Debug, Default)]
pub struct IdResultAlloc(Arc<AtomicU32>);

impl IdResultAlloc {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn start_at(start: u32) -> Self {
        Self(Arc::new(AtomicU32::new(start)))
    }

    #[inline]
    pub fn alloc_id(&mut self) -> IdResult {
        let id = self.0.fetch_add(1, Relaxed);
        if id <= ID_RESULT_MAX {
            IdResult(Word(id))
        } else {
            panic!("Out of IdResults")
        }
    }
}
