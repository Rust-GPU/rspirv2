//! All Operands with `Category::Id`

use crate::Word;
use crate::binary::{DecodeError, EncodeError, OperandReader, WordWriter};
use crate::dis::DisContext;
use crate::meta::{Category, OperandKind};
use crate::operand::{Operand, OperandEncoding};
use anstyle::AnsiColor;
use std::fmt::Formatter;

pub const OPERAND_KIND_ID_RESULT_TYPE: OperandKind = OperandKind {
    name: "IdResultType",
    category: Category::Id,
    doc: "Reference to an <id> representing the result's type of the enclosing instruction",
};
pub const OPERAND_KIND_ID_RESULT: OperandKind = OperandKind {
    name: "IdResult",
    category: Category::Id,
    doc: "Definition of an <id> representing the result of the enclosing instruction",
};
pub const OPERAND_KIND_ID_MEMORY_SEMANTICS: OperandKind = OperandKind {
    name: "IdMemorySemantics",
    category: Category::Id,
    doc: "Reference to an <id> representing a 32-bit integer that is a mask from the MemorySemantics operand kind",
};
pub const OPERAND_KIND_ID_SCOPE: OperandKind = OperandKind {
    name: "IdScope",
    category: Category::Id,
    doc: "Reference to an <id> representing a 32-bit integer that is a mask from the Scope operand kind",
};
pub const OPERAND_KIND_ID_REF: OperandKind = OperandKind {
    name: "IdRef",
    category: Category::Id,
    doc: "Reference to an <id>",
};

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

unsafe impl Operand<'_> for IdResult {
    const KIND: &'static OperandKind = &OPERAND_KIND_ID_RESULT;
}

unsafe impl OperandEncoding<'_> for IdResult {
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

    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        self.dis_fmt_color(f, ctx, AnsiColor::Blue.on_default(), false)
    }
}

impl IdResult {
    pub fn dis_fmt_color(
        &self,
        f: &mut Formatter<'_>,
        ctx: &DisContext,
        style: anstyle::Style,
        prepend_space: bool,
    ) -> std::fmt::Result {
        let style = ctx.color(style);
        let prepend_space = if prepend_space { " " } else { "" };
        write!(f, "{prepend_space}{style}%{}{style:#}", self.0.0)
    }
}

pub type OptionIdResult = Option<IdResult>;

macro_rules! id_ref {
    ($name:ident; $kind:expr; $docs:literal) => {
        #[doc = concat!("A `", stringify!($name), "` is a reference to a [`IdResult`] of another operation.")]
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

        unsafe impl Operand<'_> for $name {
            const KIND: &'static OperandKind = &$kind;
        }

        unsafe impl OperandEncoding<'_> for $name {
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

            #[inline]
            fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
                self.0.dis_fmt_color(f, ctx, AnsiColor::Yellow.on_default(), true)
            }
        }
    };
}

id_ref!(IdResultType; OPERAND_KIND_ID_RESULT_TYPE; "");
id_ref!(IdMemorySemantics; OPERAND_KIND_ID_MEMORY_SEMANTICS; "");
id_ref!(IdScope; OPERAND_KIND_ID_SCOPE; "");
id_ref!(IdRef; OPERAND_KIND_ID_REF; "");
