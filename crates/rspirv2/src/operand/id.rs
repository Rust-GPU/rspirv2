//! All Operands with `Category::Id`

use crate::binary::{DecodeError, EncodeError, OperandReader, WordWriter};
use crate::meta::OperandKind;
use crate::operand::{Operand, OperandEncoding, Word};

/// A SPIR-V "Result ID".
///
/// An Operation may have a result, which is stored in the `ResultId` and may be used by other instructions to reference
/// the result of said instruction. See [`IdRef`].
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IdResult(pub Word);

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for IdResult {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for IdResult {}

unsafe impl Operand for IdResult {
    const KIND: &OperandKind = &crate::core::operand_kinds::OPERAND_KIND_ID_RESULT;
}

unsafe impl OperandEncoding for IdResult {
    const FIXED_LEN: Option<usize> = Some(1);

    #[inline]
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(self.0);
        Ok(())
    }

    #[inline]
    fn validate_optional(opt: &Option<Self>) -> Result<(), EncodeError> {
        match opt {
            None => Err(EncodeError::MissingIdResult),
            Some(_) => Ok(()),
        }
    }

    #[inline]
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        Ok(Self(reader.pull()?))
    }
}

pub type OptionIdResult = Option<IdResult>;

macro_rules! id_ref {
    ($name:ident; $kind:expr; $docs:literal) => {
        #[doc = concat!("A `", stringify!($name), "` is a reference to a [`ResultId`] of another operation.")]
        #[doc = $docs]
        #[repr(transparent)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub struct $name(pub IdResult);

        #[cfg(feature = "bytemuck")]
        unsafe impl bytemuck::Zeroable for $name {}
        #[cfg(feature = "bytemuck")]
        unsafe impl bytemuck::Pod for $name {}

        impl From<IdResult> for $name {
            #[inline]
            fn from(id: IdResult) -> Self {
                Self(id)
            }
        }

        impl From<$name> for IdResult {
            #[inline]
            fn from(id: $name) -> Self {
                id.0
            }
        }

        unsafe impl Operand for $name {
            const KIND: &OperandKind = &$kind;
        }

        unsafe impl OperandEncoding for $name {
            const FIXED_LEN: Option<usize> = Some(1);

            #[inline]
            fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
                writer.write(self.0.0);
                Ok(())
            }

            #[inline]
            fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
                Ok(Self(IdResult(reader.pull()?)))
            }
        }
    };
}

id_ref!(IdResultType; crate::core::operand_kinds::OPERAND_KIND_ID_RESULT_TYPE; "");
id_ref!(IdMemorySemantics; crate::core::operand_kinds::OPERAND_KIND_ID_MEMORY_SEMANTICS; "");
id_ref!(IdScope; crate::core::operand_kinds::OPERAND_KIND_ID_SCOPE; "");
id_ref!(IdRef; crate::core::operand_kinds::OPERAND_KIND_ID_REF; "");
