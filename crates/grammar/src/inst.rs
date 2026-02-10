use crate::binary::{DecodeError, EncodeError, InstructionReader, InstructionWriter};
use crate::meta::InstMeta;
use std::fmt::Debug;

pub trait Inst: Sized + Debug + Eq {
    const META: &InstMeta;

    fn encode(&self, writer: &mut impl InstructionWriter) -> Result<(), EncodeError>;

    fn decode(reader: &mut InstructionReader) -> Result<Self, DecodeError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary::ModuleReader;
    use crate::core::inst::OpConstant;
    use crate::operand::{IdResult, IdResultType, LiteralConst, Word};
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
