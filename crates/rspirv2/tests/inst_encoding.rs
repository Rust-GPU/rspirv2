use anyhow::Context;
use rspirv2::binary::{ModuleReader, VecInstWriter};
use rspirv2::core::inst::{
    OpConstant, OpConvertUToF, OpDecorate, OpIAdd, OpNop, OpStore, OpTypeFloat, OpTypeInt,
    OpTypePointer, OpVariable,
};
use rspirv2::core::operands::{Decoration, StorageClass};
use rspirv2::core::preamble::OpAccessChain;
use rspirv2::operand::{IdRef, IdResult, IdResultType, LiteralConst, LiteralInteger};
use rspirv2_types::Word;
use rspirv2_types::binary::EncodeError;
use rspirv2_types::inst::{Inst, InstEncoding};

fn roundtrip<T: Inst>(inst: T) {
    let mut spirv = Vec::<Word>::new();
    inst.encode(&mut spirv).unwrap();
    let mut mod_reader = ModuleReader::new(spirv.as_slice());
    let inst_reader = mod_reader.next().unwrap().unwrap();
    assert!(matches!(mod_reader.next(), Ok(None)));
    let decoded = T::decode(inst_reader).unwrap();
    assert_eq!(inst, decoded);
}

#[test]
fn test_nop() {
    roundtrip(OpNop {});
}

#[test]
fn test_op_constant() {
    roundtrip(OpConstant {
        id_result_type: IdResultType(IdResult(Word(42))),
        id_result: Some(IdResult(Word(69))),
        value: LiteralConst::from(123u32),
    });
    roundtrip(OpConstant {
        id_result_type: IdResultType(IdResult(Word(42))),
        id_result: Some(IdResult(Word(69))),
        value: LiteralConst::from(123u64),
    });
}

#[test]
fn test_op_with_array() {
    let test = |indexes: &[u32]| {
        roundtrip(OpAccessChain {
            id_result_type: IdResultType(IdResult(Word(42))),
            id_result: Some(IdResult(Word(69))),
            base: IdRef(IdResult(Word(123))),
            indexes: indexes.iter().map(|i| IdRef(IdResult(Word(*i)))).collect(),
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
        id_result: Some(IdResult(Word(69))),
        value: LiteralConst::from(123u32),
    };
    let c2 = OpConstant {
        id_result_type: IdResultType(IdResult(Word(42))),
        id_result: Some(IdResult(Word(69))),
        value: LiteralConst::from(123u64),
    };
    let c3 = OpConstant {
        id_result_type: IdResultType(IdResult(Word(42))),
        id_result: Some(IdResult(Word(69))),
        value: LiteralConst::from(123u32),
    };

    let mut spirv = Vec::<Word>::new();
    c1.encode(&mut spirv)?;
    c2.encode(&mut spirv)?;
    c3.encode(&mut spirv)?;

    let mut mod_reader = ModuleReader::new(spirv.as_slice());
    let d1 = OpConstant::decode(mod_reader.next()?.context("No further ops")?)?;
    let d2 = OpConstant::decode(mod_reader.next()?.context("No further ops")?)?;
    let d3 = OpConstant::decode(mod_reader.next()?.context("No further ops")?)?;
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
    let mut spirv = VecInstWriter::default();
    let mut u32_op = OpTypeInt {
        id_result: None,
        width: LiteralInteger::new(32),
        signedness: LiteralInteger::new(0),
    };
    let u32 = spirv.push_mut(&mut u32_op)?;
    let mut u32_1_op = OpConstant {
        id_result_type: IdResultType(u32),
        id_result: None,
        value: LiteralConst::from(123u32),
    };
    let u32_1 = spirv.push_mut(&mut u32_1_op)?;
    let mut add_op = OpIAdd {
        id_result_type: IdResultType(u32),
        id_result: None,
        operand_1: IdRef(u32_1),
        operand_2: IdRef(u32_1),
    };
    let add = spirv.push_mut(&mut add_op)?;
    let mut f32_op = OpTypeFloat {
        id_result: None,
        width: LiteralInteger::new(32),
        floating_point_encoding: None,
    };
    let f32 = spirv.push_mut(&mut f32_op)?;
    let mut u_to_f_op = OpConvertUToF {
        id_result_type: IdResultType(f32),
        id_result: None,
        unsigned_value: IdRef(add),
    };
    let u_to_f = spirv.push_mut(&mut u_to_f_op)?;
    let mut f32_ptr_op = OpTypePointer {
        id_result: None,
        storage_class: StorageClass::Output,
        ty: IdRef(f32),
    };
    let f32_ptr = spirv.push_mut(&mut f32_ptr_op)?;
    let mut var_out_op = OpVariable {
        id_result_type: IdResultType(f32_ptr),
        id_result: None,
        storage_class: StorageClass::Output,
        initializer: None,
    };
    let var_out = spirv.push_mut(&mut var_out_op)?;
    let mut var_out_location_op = OpDecorate {
        target: IdRef(var_out),
        decoration: Decoration::Location(LiteralInteger::new(69)),
    };
    spirv.push_mut(&mut var_out_location_op)?;
    let mut store_op = OpStore {
        pointer: IdRef(var_out),
        object: IdRef(u_to_f),
        memory_access: None,
    };
    spirv.push_mut(&mut store_op)?;

    let mut mod_reader = ModuleReader::new(spirv.words.as_slice());
    let mut decode = || mod_reader.next()?.context("No further ops");
    assert_eq!(u32_op, OpTypeInt::decode(decode()?)?);
    assert_eq!(u32_1_op, OpConstant::decode(decode()?)?);
    assert_eq!(add_op, OpIAdd::decode(decode()?)?);
    assert_eq!(f32_op, OpTypeFloat::decode(decode()?)?);
    assert_eq!(u_to_f_op, OpConvertUToF::decode(decode()?)?);
    assert_eq!(f32_ptr_op, OpTypePointer::decode(decode()?)?);
    assert_eq!(var_out_op, OpVariable::decode(decode()?)?);
    assert_eq!(var_out_location_op, OpDecorate::decode(decode()?)?);
    assert_eq!(store_op, OpStore::decode(decode()?)?);

    Ok(())
}

#[test]
fn test_result_type_ret() -> anyhow::Result<()> {
    let mut spirv = VecInstWriter::default();
    // push an Instruction, get the `IdResult` out
    let f32: IdResult = spirv.push(OpTypeFloat {
        // will alloc an `IdResult` using an atomic counter in `InstWriter`
        id_result: None,
        width: LiteralInteger::new(32),
        floating_point_encoding: None,
    })?;
    // `OpDecorate` doesn't have an `IdResult`, so this returns `()`
    let _: () = spirv.push(OpDecorate {
        // reuse the `IdResult` in the next Instruction
        target: IdRef(f32),
        decoration: Decoration::RelaxedPrecision,
    })?;
    Ok(())
}

#[test]
fn test_result_type_none() {
    let err = OpTypeFloat {
        id_result: None,
        width: LiteralInteger::new(32),
        floating_point_encoding: None,
    }
    .encode(&mut Vec::default());
    assert_eq!(err, Err(EncodeError::MissingIdResult));
}
