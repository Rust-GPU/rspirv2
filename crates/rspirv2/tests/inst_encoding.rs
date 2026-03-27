use rspirv2::core::inst::{
    OpConstant, OpConvertUToF, OpDecorate, OpIAdd, OpNop, OpStore, OpTypeFloat, OpTypeInt,
    OpTypePointer, OpVariable,
};
use rspirv2::core::inst_set::CoreInstSet;
use rspirv2::core::operands::{Decoration, StorageClass};
use rspirv2::core::preamble::OpAccessChain;
use rspirv2::operand::{IdRef, IdResult, IdResultType, LiteralConst, LiteralInteger};
use rspirv2_types::Word;
use rspirv2_types::binary::IdResultAlloc;
use rspirv2_types::inst::{Inst, InstEncoding};
use rspirv2_types::slice::InstSlice;
use rspirv2_types::vec::InstVec;

fn roundtrip<T: Inst>(inst: T)
where
    CoreInstSet: From<T>,
{
    let mut spirv = Vec::<Word>::new();
    inst.encode(&mut spirv).unwrap();
    let mut iter = InstSlice::from_words_unchecked(&spirv).iter();
    let decoded = iter.next().unwrap();
    assert!(iter.next().is_none());
    assert_eq!(CoreInstSet::from(inst), decoded);
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

    let mut iter = InstSlice::from_words_unchecked(&spirv).iter();
    assert_eq!(c1, iter.next().unwrap());
    assert_eq!(c2, iter.next().unwrap());
    assert_eq!(c3, iter.next().unwrap());

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
    let mut alloc = IdResultAlloc::new();
    let mut spirv = InstVec::default();
    let u32_op = OpTypeInt {
        id_result: alloc.alloc_id(),
        width: LiteralInteger::new(32),
        signedness: LiteralInteger::new(0),
    };
    let u32 = spirv.push(u32_op.clone());
    let u32_1_op = OpConstant {
        id_result_type: IdResultType(u32),
        id_result: alloc.alloc_id(),
        value: LiteralConst::from(123u32),
    };
    let u32_1 = spirv.push(u32_1_op.clone());
    let add_op = OpIAdd {
        id_result_type: IdResultType(u32),
        id_result: alloc.alloc_id(),
        operand_1: IdRef(u32_1),
        operand_2: IdRef(u32_1),
    };
    let add = spirv.push(add_op.clone());
    let f32_op = OpTypeFloat {
        id_result: alloc.alloc_id(),
        width: LiteralInteger::new(32),
        floating_point_encoding: None,
    };
    let f32 = spirv.push(f32_op.clone());
    let u_to_f_op = OpConvertUToF {
        id_result_type: IdResultType(f32),
        id_result: alloc.alloc_id(),
        unsigned_value: IdRef(add),
    };
    let u_to_f = spirv.push(u_to_f_op.clone());
    let f32_ptr_op = OpTypePointer {
        id_result: alloc.alloc_id(),
        storage_class: StorageClass::Output,
        ty: IdRef(f32),
    };
    let f32_ptr = spirv.push(f32_ptr_op.clone());
    let var_out_op = OpVariable {
        id_result_type: IdResultType(f32_ptr),
        id_result: alloc.alloc_id(),
        storage_class: StorageClass::Output,
        initializer: None,
    };
    let var_out = spirv.push(var_out_op.clone());
    let var_out_location_op = OpDecorate {
        target: IdRef(var_out),
        decoration: Decoration::Location(LiteralInteger::new(69)),
    };
    spirv.push(var_out_location_op.clone());
    let store_op = OpStore {
        pointer: IdRef(var_out),
        object: IdRef(u_to_f),
        memory_access: None,
    };
    spirv.push(store_op.clone());

    let mut iter = spirv.iter();
    assert_eq!(Some(CoreInstSet::from(u32_op)), iter.next());
    assert_eq!(Some(CoreInstSet::from(u32_1_op)), iter.next());
    assert_eq!(Some(CoreInstSet::from(add_op)), iter.next());
    assert_eq!(Some(CoreInstSet::from(f32_op)), iter.next());
    assert_eq!(Some(CoreInstSet::from(u_to_f_op)), iter.next());
    assert_eq!(Some(CoreInstSet::from(f32_ptr_op)), iter.next());
    assert_eq!(Some(CoreInstSet::from(var_out_op)), iter.next());
    assert_eq!(Some(CoreInstSet::from(var_out_location_op)), iter.next());
    assert_eq!(Some(CoreInstSet::from(store_op)), iter.next());

    Ok(())
}

#[test]
fn test_result_type_ret() -> anyhow::Result<()> {
    let mut spirv = InstVec::<CoreInstSet>::default();
    // push an Instruction, get the `IdResult` out
    let f32: IdResult = spirv.push(OpTypeFloat {
        // will alloc an `IdResult` using an atomic counter in `InstWriter`
        id_result: IdResult(Word(42)),
        width: LiteralInteger::new(32),
        floating_point_encoding: None,
    });
    // `OpDecorate` doesn't have an `IdResult`, so this returns `()`
    let _: () = spirv.push(OpDecorate {
        // reuse the `IdResult` in the next Instruction
        target: IdRef(f32),
        decoration: Decoration::RelaxedPrecision,
    });
    Ok(())
}
