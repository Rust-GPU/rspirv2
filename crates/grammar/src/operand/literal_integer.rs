use crate::operand::Word;
use smallvec::SmallVec;

/// A literal integer as defined by SPIR-V spec. May represent any number of [`Word`]s, depending on the type of the
/// constant.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct LiteralInteger(SmallVec<[Word; 2]>);

impl From<u64> for LiteralInteger {
    #[inline]
    fn from(value: u64) -> Self {
        LiteralInteger(SmallVec::from_slice(&[
            Word(value as u32),
            Word((value >> 32) as u32),
        ]))
    }
}

impl From<u32> for LiteralInteger {
    #[inline]
    fn from(value: u32) -> Self {
        LiteralInteger(SmallVec::from_slice(&[Word(value)]))
    }
}

macro_rules! impl_cast {
    ($ty:ty => $to:ty) => {
        impl From<$ty> for LiteralInteger {
            #[inline]
            fn from(value: $ty) -> Self {
                Self::from(value as $to)
            }
        }
    };
}

impl_cast!(u8 => u32);
impl_cast!(u16 => u32);
impl_cast!(i8 => i32);
impl_cast!(i16 => i32);
impl_cast!(i32 => u32);
impl_cast!(i64 => u64);

macro_rules! impl_float {
    ($ty:ty) => {
        impl From<$ty> for LiteralInteger {
            #[inline]
            fn from(value: $ty) -> Self {
                Self::from(value.to_bits())
            }
        }
    };
}

impl_float!(f32);
impl_float!(f64);
