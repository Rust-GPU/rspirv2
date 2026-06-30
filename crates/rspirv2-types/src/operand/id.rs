//! All Operands with `Category::Id`

use crate::Word;
use crate::binary::{DecodeError, EncodeError, OperandReader, WordWriter};
use crate::dis::ResolvedIdName;
use crate::meta::{Category, OperandKind};
use crate::operand::{OperandDisContext, SpvOperandDis, SpvOperandEncoding, SpvOperandMeta};
use anstyle::AnsiColor;
use std::fmt::{Display, Formatter};

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

unsafe impl SpvOperandMeta for IdResult {
    const KIND: &OperandKind = &OPERAND_KIND_ID_RESULT;
}

unsafe impl SpvOperandEncoding for IdResult {
    const FIXED_LEN: Option<usize> = Some(1);

    #[inline]
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write(self.0);
        Ok(())
    }

    #[inline]
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        Ok(Self(reader.pull()?))
    }
}

impl SpvOperandDis for IdResult {
    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &OperandDisContext<'_>) -> std::fmt::Result {
        self.dis_fmt_color(f, ctx, ID_RESULT_COLOR, false)
    }
}

pub const ID_RESULT_COLOR: anstyle::Style = AnsiColor::Blue.on_default();

impl IdResult {
    pub fn dis_fmt_color(
        &self,
        f: &mut Formatter<'_>,
        ctx: &OperandDisContext<'_>,
        style: anstyle::Style,
        prepend_space: bool,
    ) -> std::fmt::Result {
        let style = ctx.color(style);
        let prepend_space = if prepend_space { " " } else { "" };
        let name = ctx.id_to_name(*self);
        write!(f, "{prepend_space}{style}%{name}{style:#}")
    }
}

pub struct IdResultWriter<'a>(pub &'a OperandDisContext<'a>);

impl Display for IdResultWriter<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ctx = self.0;
        if let Some(id_result) = self.0.id_result {
            let name = ctx.id_to_name(id_result);
            let name_len = match name {
                ResolvedIdName::Named(name) => name.len(),
                ResolvedIdName::Raw(id) => id.0.0.checked_ilog10().unwrap_or(1) as usize + 1,
            };
            let pad_len = ctx.padding.len().saturating_sub(name_len + 4);
            let style = ctx.color(ID_RESULT_COLOR);
            write!(f, "{}{style}%{}{style:#} = ", &ctx.padding[..pad_len], name)
        } else {
            write!(f, "{}", &ctx.padding)
        }
    }
}

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

        unsafe impl SpvOperandMeta for $name {
            const KIND: &OperandKind = &$kind;
        }

        unsafe impl SpvOperandEncoding for $name {
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

        impl SpvOperandDis for $name {
            #[inline]
            fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &OperandDisContext<'_>) -> std::fmt::Result {
                self.0.dis_fmt_color(f, ctx, AnsiColor::Yellow.on_default(), true)
            }
        }
    };
}

id_ref!(IdResultType; OPERAND_KIND_ID_RESULT_TYPE; "");
id_ref!(IdMemorySemantics; OPERAND_KIND_ID_MEMORY_SEMANTICS; "");
id_ref!(IdScope; OPERAND_KIND_ID_SCOPE; "");
id_ref!(IdRef; OPERAND_KIND_ID_REF; "");
