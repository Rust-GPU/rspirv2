use crate::binary::{DecodeError, EncodeError, InstructionReader, InstructionWriter};
use crate::core::inst::OpConstant;
use crate::meta::InstMeta;
use crate::operand::{OperandEncoding, Word};
use std::fmt::Debug;

pub trait Inst: Sized + Debug + Eq {
    const META: &InstMeta;

    fn encode(&self, writer: &mut impl InstructionWriter) -> Result<(), EncodeError>;

    fn decode(reader: &mut InstructionReader) -> Result<Self, DecodeError>;
}

impl Inst for OpConstant {
    const META: &InstMeta = &crate::core::inst_meta::OP_CONSTANT;

    fn encode(&self, writer: &mut impl InstructionWriter) -> Result<(), EncodeError> {
        let len =
            self.id_result_type.word_len() + self.id_result.word_len() + self.value.word_len();
        writer.push(Word::new_op(Self::META.opcode, len)?)?;
        OperandEncoding::encode(&self.id_result_type, &mut *writer)?;
        OperandEncoding::encode(&self.id_result, &mut *writer)?;
        OperandEncoding::encode(&self.value, &mut *writer)?;
        Ok(())
    }

    fn decode(reader: &mut InstructionReader) -> Result<Self, DecodeError> {
        reader.check_opcode(Self::META)?;
        Ok(Self {
            id_result_type: OperandEncoding::decode(&mut *reader)?,
            id_result: OperandEncoding::decode(&mut *reader)?,
            value: OperandEncoding::decode_last(&mut *reader)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary::ModuleReader;
    use crate::operand::{IdResult, IdResultType, LiteralConst};
    use anyhow::Context;

    fn roundtrip<T: Inst>(inst: T) {
        let mut spirv = Vec::<Word>::new();
        inst.encode(&mut spirv).unwrap();
        let mut reader = ModuleReader::new(spirv.as_slice()).next().unwrap().unwrap();
        let decoded = T::decode(&mut reader).unwrap();
        assert_eq!(inst, decoded);
    }

    #[test]
    fn test_op_constant() {
        roundtrip(OpConstant {
            id_result_type: IdResultType(IdResult(Word(42))),
            id_result: IdResult(Word(69)),
            value: LiteralConst::from(123u32),
        });
        roundtrip(OpConstant {
            id_result_type: IdResultType(IdResult(Word(42))),
            id_result: IdResult(Word(69)),
            value: LiteralConst::from(123u64),
        });
    }

    #[test]
    fn test_op_constant_sequence() -> anyhow::Result<()> {
        let c1 = OpConstant {
            id_result_type: IdResultType(IdResult(Word(42))),
            id_result: IdResult(Word(69)),
            value: LiteralConst::from(123u32),
        };
        let c2 = OpConstant {
            id_result_type: IdResultType(IdResult(Word(42))),
            id_result: IdResult(Word(69)),
            value: LiteralConst::from(123u64),
        };
        let c3 = OpConstant {
            id_result_type: IdResultType(IdResult(Word(42))),
            id_result: IdResult(Word(69)),
            value: LiteralConst::from(123u32),
        };

        let mut spirv = Vec::<Word>::new();
        c1.encode(&mut spirv)?;
        c2.encode(&mut spirv)?;
        c3.encode(&mut spirv)?;

        let mut mod_reader = ModuleReader::new(spirv.as_slice());
        let d1 = OpConstant::decode(&mut mod_reader.next()?.context("No further ops")?)?;
        let d2 = OpConstant::decode(&mut mod_reader.next()?.context("No further ops")?)?;
        let d3 = OpConstant::decode(&mut mod_reader.next()?.context("No further ops")?)?;
        assert_eq!(c1, d1);
        assert_eq!(c2, d2);
        assert_eq!(c3, d3);

        Ok(())
    }
}
