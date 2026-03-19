use crate::binary::{DecodeError, EncodeError, OperandReader, WordWriter};
use crate::operand::{OperandDisContext, OperandEncoding};
use std::fmt::Formatter;

macro_rules! impl_tuple {
    ($($A:ident $I:tt),*) => {
        unsafe impl<$($A: OperandEncoding),*> OperandEncoding for ($($A),*) {
            const FIXED_LEN: Option<usize> = FixedLenComposer::new()
                $(.append($A::FIXED_LEN))*
                .finish();

            fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
                $(<$A as OperandEncoding>::encode(&self.$I, &mut *writer)?;)*
                Ok(())
            }

            fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
                Ok(($(<$A as OperandEncoding>::decode(&mut *reader)?),*))
            }

            #[inline]
            fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &OperandDisContext<'_>) -> std::fmt::Result {
                $(<$A as OperandEncoding>::dis_fmt(&self.$I, f, ctx)?;)*
                Ok(())
            }
        }
    };
}

impl_tuple!(A 0, B 1);
impl_tuple!(A 0, B 1, C 2);
impl_tuple!(A 0, B 1, C 2, D 3);
impl_tuple!(A 0, B 1, C 2, D 3, E 4);
impl_tuple!(A 0, B 1, C 2, D 3, E 4, F 5);
impl_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6);
impl_tuple!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7);

/// Compose the [`OperandEncoding::FIXED_LEN`] from multiple maybe fixed len Operands
pub struct FixedLenComposer(Option<usize>);

impl FixedLenComposer {
    #[inline]
    pub const fn new() -> Self {
        Self(Some(0))
    }

    #[inline]
    pub const fn append(self, len: Option<usize>) -> Self {
        match (self.0, len) {
            (Some(a), Some(b)) => Self(Some(a + b)),
            (_, _) => Self(None),
        }
    }

    #[inline]
    pub const fn finish(self) -> Option<usize> {
        self.0
    }
}

impl Default for FixedLenComposer {
    fn default() -> Self {
        Self::new()
    }
}
