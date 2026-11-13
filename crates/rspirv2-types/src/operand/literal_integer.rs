use crate::Word;
use crate::binary::{DecodeError, OperandReader, WordWriter};
use crate::meta::{Category, OperandKind};
use crate::operand::{Operand, OperandDisContext, OperandEncoding, OperandVisitor};
use anstyle::AnsiColor;
use std::fmt::Formatter;

pub const OPERAND_KIND_LITERAL_INTEGER: OperandKind = OperandKind {
    name: "LiteralInteger",
    category: Category::Literal,
    doc: "An integer consuming one or more words",
};
pub const OPERAND_KIND_LITERAL_EXT_INST_INTEGER: OperandKind = OperandKind {
    name: "LiteralExtInstInteger",
    category: Category::Literal,
    doc: "A 32-bit unsigned integer indicating which instruction to use and determining the layout of following operands (for OpExtInst)",
};
pub const OPERAND_KIND_LITERAL_SPEC_CONSTANT_OP_INTEGER: OperandKind = OperandKind {
    name: "LiteralSpecConstantOpInteger",
    category: Category::Literal,
    doc: "An opcode indicating the operation to be performed and determining the layout of following operands (for OpSpecConstantOp)",
};

/// The Instruction ID from an extended instruction set, backed by a 32bit integer.
///
/// In rspirv2, this is an alias to [`LiteralInteger`]. This means `LiteralExtInstInteger::KIND` returns the kind for
/// `LiteralInteger`, not for `LiteralExtInstInteger`, which differs in the grammar name and docs but is otherwise the
/// same.
pub type LiteralExtInstInteger = LiteralInteger;

/// The Instruction ID for an `OpSpecConstantOp`, backed by a 32bit integer.
///
/// In rspirv2, this is an alias to [`LiteralInteger`]. This means `LiteralExtInstInteger::KIND` returns the kind for
/// `LiteralInteger`, not for `OpSpecConstantOp`, which differs in the grammar name and docs but is otherwise the
/// same.
pub type LiteralSpecConstantOpInteger = LiteralInteger;

/// An integer literal as defined by SPIR-V spec: a 32bit integer.
///
/// Technically, the spec doesn't actually say that it's a 32bit integer. But every use of `LiteralInteger` defines the
/// integer as an "unsigned 32bit integer", so most tooling has resorted to defining it as that. Also see
/// [this Khronos issue](https://github.com/KhronosGroup/SPIRV-Headers/issues/38).
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LiteralInteger(pub Word);

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for LiteralInteger {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for LiteralInteger {}

impl LiteralInteger {
    #[inline]
    pub fn from_word(value: Word) -> Self {
        Self(value)
    }

    #[inline]
    pub fn new(value: u32) -> Self {
        Self(Word(value))
    }

    #[inline]
    pub fn from_bool(value: bool) -> Self {
        Self(Word(value as u32))
    }

    #[inline]
    pub fn to_word(&self) -> Word {
        self.0
    }

    #[inline]
    pub fn to_u32(&self) -> u32 {
        self.0.0
    }

    #[inline]
    pub fn to_bool(&self) -> bool {
        self.0.0 != 0
    }
}

unsafe impl Operand for LiteralInteger {
    const KIND: &OperandKind = &OPERAND_KIND_LITERAL_INTEGER;
}

unsafe impl OperandEncoding for LiteralInteger {
    const FIXED_LEN: Option<usize> = Some(1);

    #[inline]
    fn encode(&self, writer: &mut impl WordWriter) {
        writer.write(self.0);
    }

    #[inline]
    fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        Ok(Self(reader.pull()?))
    }

    fn visit(&self, visitor: &mut impl OperandVisitor) {
        visitor.visit_literal_integer(*self)
    }

    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: OperandDisContext<'_>) -> std::fmt::Result {
        let color = ctx.color(AnsiColor::Red.on_default());
        write!(f, " {color}{}{color:#}", self.0.0)
    }
}
