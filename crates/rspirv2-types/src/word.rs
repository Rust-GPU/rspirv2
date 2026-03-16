use crate::binary::EncodeError;

/// A 32bit SPIR-V Word
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Word(pub u32);

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for Word {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for Word {}

impl Word {
    #[inline]
    pub const fn from_le_bytes(value: [u8; 4]) -> Self {
        Self(u32::from_le_bytes(value))
    }

    #[inline]
    pub const fn to_le_bytes(&self) -> [u8; 4] {
        self.0.to_le_bytes()
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
