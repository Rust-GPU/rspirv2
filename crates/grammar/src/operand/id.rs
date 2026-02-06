//! All Operands with `Category::Id`

use crate::binary::{DecodeError, InstructionReader, InstructionWriter};
use crate::meta::OperandKind;
use crate::operand::{Operand, Word};

/// A SPIR-V "Result ID".
///
/// An Operation may have a result, which is stored in the `ResultId` and may be used by other instructions to reference
/// the result of said instruction. See [`IdRef`].
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IdResult(pub Word);

macro_rules! id_ref {
    ($name:ident; $kind:expr; $docs:literal) => {
        #[doc = concat!("A `", stringify!($name), "` is a reference to a [`ResultId`] of another operation.")]
        #[doc = $docs]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub struct $name(pub IdResult);

        impl From<IdResult> for $name {
            fn from(id: IdResult) -> Self {
                Self(id)
            }
        }

        impl From<$name> for IdResult {
            fn from(id: $name) -> Self {
                id.0
            }
        }

        impl Operand for $name {
            const KIND: OperandKind = $kind;

            fn encode(&self, writer: &mut impl InstructionWriter) {
                writer.push(self.0.0)
            }

            fn decode(reader: &mut InstructionReader<'_>) -> Result<Self, DecodeError> {
                Ok(Self(IdResult(reader.pull()?)))
            }
        }
    };
}

id_ref!(IdResultType; crate::core::operand_kinds::OPERAND_KIND_ID_RESULT_TYPE; "");
id_ref!(IdMemorySemantics; crate::core::operand_kinds::OPERAND_KIND_ID_MEMORY_SEMANTICS; "");
id_ref!(IdScope; crate::core::operand_kinds::OPERAND_KIND_ID_SCOPE; "");
id_ref!(IdRef; crate::core::operand_kinds::OPERAND_KIND_ID_REF; "");
