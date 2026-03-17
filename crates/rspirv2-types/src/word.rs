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
