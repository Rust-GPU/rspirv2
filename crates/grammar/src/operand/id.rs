//! All Operands with `Category::Id`

use crate::operand::Word;

/// A SPIR-V "Result ID".
///
/// An Operation may have a result, which is stored in the `ResultId` and may be used by other instructions to reference
/// the result of said instruction. See [`IdRef`].
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IdResult(pub Word);

macro_rules! id_ref {
    ($name:ident$(; $docs:literal)?) => {
        #[doc = concat!("A `", stringify!($name), "` is a reference to a [`ResultId`] of another operation.")]
        $(#[doc = $docs])?
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
    };
}

id_ref!(IdResultType);
id_ref!(IdMemorySemantics);
id_ref!(IdScope);
id_ref!(IdRef);
