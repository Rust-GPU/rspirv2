use crate::binary::EncodeError;
use crate::operand::{IdResult, Word};
use std::sync::Arc;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;

/// A reasonable upper bounds that ensures subsequent allocations by potentially multiple threads all fail
pub const ID_RESULT_MAX: u32 = 0x7FFFFFFF;

/// An allocator for SPIR-V [`IdResult`]s
pub trait IdResultAlloc {
    /// Alloc a new [`IdResult`], may error with [`EncodeError::OutOfIdResults`].
    fn alloc_id(&mut self) -> Result<IdResult, EncodeError>;
}

/// A single threaded [`IdResultAlloc`]
#[derive(Debug, Default)]
pub struct IdResultAllocator(u32);

impl IdResultAllocator {
    pub fn start_at(start: u32) -> Self {
        Self(start)
    }
}

impl IdResultAlloc for IdResultAllocator {
    fn alloc_id(&mut self) -> Result<IdResult, EncodeError> {
        let id = self.0;
        self.0 += 1;
        if id <= ID_RESULT_MAX {
            Ok(IdResult(Word(id)))
        } else {
            Err(EncodeError::OutOfIdResults)
        }
    }
}

/// A sharable atomic [`IdResultAlloc`], just [`Clone`] to share it
#[derive(Clone, Debug, Default)]
pub struct AtomicIdResultAllocator(Arc<AtomicU32>);

impl AtomicIdResultAllocator {
    pub fn start_at(start: u32) -> Self {
        Self(Arc::new(AtomicU32::new(start)))
    }
}

impl IdResultAlloc for AtomicIdResultAllocator {
    fn alloc_id(&mut self) -> Result<IdResult, EncodeError> {
        let id = self.0.fetch_add(1, Relaxed);
        if id <= ID_RESULT_MAX {
            Ok(IdResult(Word(id)))
        } else {
            Err(EncodeError::OutOfIdResults)
        }
    }
}

/// An [`IdResultAlloc`] that always fails to allocate
#[derive(Clone, Debug, Default)]
pub struct DisallowedIdResultAllocator;

impl IdResultAlloc for DisallowedIdResultAllocator {
    fn alloc_id(&mut self) -> Result<IdResult, EncodeError> {
        Err(EncodeError::MissingIdResult)
    }
}
