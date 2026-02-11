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
    use crate::core::inst::{
        OpConstant, OpConvertUToF, OpDecorate, OpIAdd, OpNop, OpStore, OpTypeFloat, OpTypeInt,
        OpTypePointer, OpVariable,
    };
    use crate::core::operands::{Decoration, StorageClass};
    use crate::core::preamble::OpAccessChain;
    use crate::operand::{IdRef, IdResult, IdResultType, LiteralConst, LiteralInteger, Word};
    use anyhow::Context;
    use smallvec::SmallVec;
    use std::sync::atomic::AtomicU32;
    use std::sync::atomic::Ordering::Relaxed;

    fn roundtrip<T: Inst>(inst: T) {
        let mut spirv = Vec::<Word>::new();
        inst.encode(&mut spirv).unwrap();
        let mut mod_reader = ModuleReader::new(spirv.as_slice());
        let mut inst_reader = mod_reader.next().unwrap().unwrap();
        assert!(matches!(mod_reader.next(), Ok(None)));
        let decoded = T::decode(&mut inst_reader).unwrap();
        assert_eq!(inst, decoded);
        assert_eq!(inst_reader.next(), None);
    }

    #[test]
    fn test_nop() {
        roundtrip(OpNop {});
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
    fn test_op_with_array() {
        let test = |indexes: &[u32]| {
            roundtrip(OpAccessChain {
                id_result_type: IdResultType(IdResult(Word(42))),
                id_result: IdResult(Word(69)),
                base: IdRef(IdResult(Word(123))),
                indexes: SmallVec::from_iter(indexes.iter().map(|i| IdRef(IdResult(Word(*i))))),
            });
        };
        test(&[1, 2, 3, 4, 5]);
        test(&[1, 2, 3]);
        test(&[]);
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

    /// Conceptual rust-gpu code:
    ///
    /// ```no_run
    /// #[spirv(fragment)]
    /// pub fn main(var_out: &mut f32) {
    ///     let add = 1u32 + 1u32;
    ///     let u_to_f = add as f32;
    ///     *var_out = u_to_f;
    /// }
    /// ```
    #[test]
    fn test_non_trivial_code() -> anyhow::Result<()> {
        let id = AtomicU32::new(0);

        let u32 = OpTypeInt {
            id_result: IdResult(Word(id.fetch_add(1, Relaxed))),
            width: LiteralInteger::new(32),
            signedness: LiteralInteger::new(0),
        };
        let u32_1 = OpConstant {
            id_result_type: IdResultType(u32.id_result),
            id_result: IdResult(Word(id.fetch_add(1, Relaxed))),
            value: LiteralConst::from(123u32),
        };
        let add = OpIAdd {
            id_result_type: IdResultType(u32.id_result),
            id_result: IdResult(Word(id.fetch_add(1, Relaxed))),
            operand_1: IdRef(u32_1.id_result),
            operand_2: IdRef(u32_1.id_result),
        };
        let f32 = OpTypeFloat {
            id_result: IdResult(Word(id.fetch_add(1, Relaxed))),
            width: LiteralInteger::new(32),
            floating_point_encoding: None,
        };
        let u_to_f = OpConvertUToF {
            id_result_type: IdResultType(f32.id_result),
            id_result: IdResult(Word(id.fetch_add(1, Relaxed))),
            unsigned_value: IdRef(add.id_result),
        };
        let f32_ptr = OpTypePointer {
            id_result: IdResult(Word(id.fetch_add(1, Relaxed))),
            storage_class: StorageClass::Output,
            ty: IdRef(f32.id_result),
        };
        let var_out = OpVariable {
            id_result_type: IdResultType(f32_ptr.id_result),
            id_result: IdResult(Word(id.fetch_add(1, Relaxed))),
            storage_class: StorageClass::Output,
            initializer: None,
        };
        let var_out_location = OpDecorate {
            target: IdRef(var_out.id_result),
            decoration: Decoration::Location(LiteralInteger::new(69)),
        };
        let store = OpStore {
            pointer: IdRef(var_out.id_result),
            object: IdRef(u_to_f.id_result),
            memory_access: None,
        };

        let mut spirv = Vec::<Word>::new();
        u32.encode(&mut spirv)?;
        u32_1.encode(&mut spirv)?;
        add.encode(&mut spirv)?;
        f32.encode(&mut spirv)?;
        u_to_f.encode(&mut spirv)?;
        f32_ptr.encode(&mut spirv)?;
        var_out.encode(&mut spirv)?;
        var_out_location.encode(&mut spirv)?;
        store.encode(&mut spirv)?;

        let mut mod_reader = ModuleReader::new(spirv.as_slice());
        let mut decode = || Ok::<_, anyhow::Error>(mod_reader.next()?.context("No further ops")?);
        assert_eq!(u32, OpTypeInt::decode(&mut decode()?)?);
        assert_eq!(u32_1, OpConstant::decode(&mut decode()?)?);
        assert_eq!(add, OpIAdd::decode(&mut decode()?)?);
        assert_eq!(f32, OpTypeFloat::decode(&mut decode()?)?);
        assert_eq!(u_to_f, OpConvertUToF::decode(&mut decode()?)?);
        assert_eq!(f32_ptr, OpTypePointer::decode(&mut decode()?)?);
        assert_eq!(var_out, OpVariable::decode(&mut decode()?)?);
        assert_eq!(var_out_location, OpDecorate::decode(&mut decode()?)?);
        assert_eq!(store, OpStore::decode(&mut decode()?)?);

        Ok(())
    }
}
