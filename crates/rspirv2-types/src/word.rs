/// A 32bit SPIR-V Word
pub type Word = u32;

pub trait WordExt: Copy {
    fn new_op(op: u16, len: usize) -> Self;

    fn to_op(self) -> (u16, usize);
}

impl WordExt for Word {
    #[inline]
    fn new_op(op: u16, len: usize) -> Self {
        let len = u16::try_from(len).expect("Op too long, u16 overflow");
        op as u32 | ((len as u32) << 16)
    }

    #[inline]
    fn to_op(self) -> (u16, usize) {
        (self as u16, (self >> 16) as u16 as usize)
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
            assert_eq!(
                bytemuck::cast_slice::<u32, u8>(case),
                cast_words_to_ne_bytes(case)
            );
        }
    }
}
