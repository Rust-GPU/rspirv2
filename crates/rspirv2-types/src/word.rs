use crate::binary::EncodeError;
use std::fmt::{Debug, Formatter};

/// A 32bit SPIR-V Word
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Word(pub u32);

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for Word {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for Word {}

impl Word {
    // strings are little-endian encoded, likely the correct choice in many instances
    #[inline]
    pub const fn from_le_bytes(value: [u8; 4]) -> Self {
        Self(u32::from_le_bytes(value))
    }

    #[inline]
    pub const fn to_le_bytes(&self) -> [u8; 4] {
        self.0.to_le_bytes()
    }

    // big-endian is only needed for `Module::from_bytes()`
    #[inline]
    pub const fn from_be_bytes(value: [u8; 4]) -> Self {
        Self(u32::from_be_bytes(value))
    }

    #[inline]
    pub const fn to_be_bytes(&self) -> [u8; 4] {
        self.0.to_be_bytes()
    }

    #[inline]
    pub fn new_op(op: u16, len: usize) -> Result<Self, EncodeError> {
        let len = u16::try_from(len).map_err(|_e| EncodeError::OpTooLong)?;
        Ok(Self(op as u32 | ((len as u32) << 16)))
    }

    #[inline]
    pub fn to_op(&self) -> (u16, usize) {
        (self.0 as u16, (self.0 >> 16) as u16 as usize)
    }
}

impl Debug for Word {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Word({:#010x})", self.0)
    }
}

/// Convert a slice of [`Word`]s to a slice of `u8`s in native endian.
///
/// Implementation replicates `bytemuck::cast_slice()`, see `test_cast_words_bytemuck_equivalence`.
pub fn cast_words_to_ne_bytes(words: &[Word]) -> &[u8] {
    unsafe { core::slice::from_raw_parts(words.as_ptr().cast::<u8>(), words.len() * 4) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CORPUS: &[&[u32]] = &[
        &[1, 2, 3, 4],
        &[0xDEADBEEF],
        &[0, 0xDEADBEEF, 0x12345678, 0xFFFFFFFF],
        &[0],
        &[],
    ];

    #[test]
    pub fn test_cast_words_bytemuck_equivalence() {
        for case in TEST_CORPUS {
            // convert to Words (potentially changing byte order)
            let case = case.iter().map(|v| Word(*v)).collect::<Vec<_>>();
            // "transmute" slice of words to slice of u32s, without impl bytemuck::Pod for Word, NO CONVERSIONS HERE!
            let case_u32s = case.iter().map(|w| w.0).collect::<Vec<_>>();
            assert_eq!(
                bytemuck::cast_slice::<u32, u8>(&case_u32s),
                cast_words_to_ne_bytes(&case)
            );
        }
    }
}
