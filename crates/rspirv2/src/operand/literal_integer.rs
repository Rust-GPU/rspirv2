use crate::binary::{DecodeError, EncodeError, InstructionWriter, OperandReader};
use crate::meta::OperandKind;
use crate::operand::{Operand, OperandEncoding, Word};

macro_rules! def_literal_integer {
    ($name:ident; $kind:expr; $docs:literal) => {
        #[doc = $docs]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub struct $name(pub Word);

        impl $name {
            pub fn new(value: u32) -> Self {
                Self(Word(value))
            }

            pub fn from_word(value: Word) -> Self {
                Self(value)
            }

            pub fn to_word(&self) -> Word {
                self.0
            }

            pub fn to_u32(&self) -> u32 {
                self.0.0
            }
        }

        unsafe impl Operand for $name {
            const KIND: &OperandKind = &$kind;
        }

        unsafe impl OperandEncoding for $name {
            const FIXED_LEN: Option<usize> = Some(1);

            fn encode(&self, writer: &mut impl InstructionWriter) -> Result<(), EncodeError> {
                writer.write(self.0);
                Ok(())
            }

            fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
                Ok(Self(reader.pull()?))
            }
        }
    };
}

def_literal_integer!(
    LiteralInteger;
    crate::core::operand_kinds::OPERAND_KIND_LITERAL_INTEGER;
    r#"An integer literal as defined by SPIR-V spec: a 32bit integer.

Technically, the spec doesn't actually say that it's a 32bit integer. But every use of `LiteralInteger` defines the
integer as an "unsigned 32bit integer", so most tooling has resorted to defining it as that. Also see
[this Khronos issue](https://github.com/KhronosGroup/SPIRV-Headers/issues/38).
"#
);

def_literal_integer!(
    LiteralExtInstInteger;
    crate::core::operand_kinds::OPERAND_KIND_LITERAL_EXT_INST_INTEGER;
    "The Instruction ID from an extended instruction set, backed by a 32bit integer."
);

def_literal_integer!(
    LiteralSpecConstantOpInteger;
    crate::core::operand_kinds::OPERAND_KIND_LITERAL_SPEC_CONSTANT_OP_INTEGER;
    "The Instruction ID for an `OpSpecConstantOp`, backed by a 32bit integer."
);
