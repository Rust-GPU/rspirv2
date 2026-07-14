use rspirv2::core::inst::{
    OpAccessChain, OpConstant, OpConvertUToF, OpFAdd, OpStore, OpTypeFloat, OpTypeInt,
    OpTypePointer, OpVariable,
};
use rspirv2::core::inst_set::CoreInstSet;
use rspirv2::core::operands::StorageClass;
use rspirv2_types::binary::IdResultAlloc;
use rspirv2_types::operand::{IdRef, IdResultType, LiteralConst, LiteralInteger};
use rspirv2_types::vec::InstVec;
use smallvec::SmallVec;

fn demo_inst() -> InstVec<CoreInstSet> {
    let mut alloc = IdResultAlloc::new();
    let mut vec = InstVec::new();

    // types
    let u32 = vec.push_inst(OpTypeInt {
        id_result: alloc.alloc_id(),
        width: LiteralInteger::new(32),
        signedness: LiteralInteger::new(0),
    });
    let f32 = vec.push_inst(OpTypeFloat {
        id_result: alloc.alloc_id(),
        width: LiteralInteger::new(32),
        floating_point_encoding: None,
    });
    let f32_ptr_output = vec.push_inst(OpTypePointer {
        id_result: alloc.alloc_id(),
        storage_class: StorageClass::Output,
        ty: IdRef(f32),
    });

    // const
    let u32_0 = vec.push_inst(OpConstant {
        id_result_type: IdResultType(u32),
        id_result: alloc.alloc_id(),
        value: LiteralConst::from(0u32),
    });
    let u32_42 = vec.push_inst(OpConstant {
        id_result_type: IdResultType(u32),
        id_result: alloc.alloc_id(),
        value: LiteralConst::from(42u32),
    });
    let f32_123_45 = vec.push_inst(OpConstant {
        id_result_type: IdResultType(u32),
        id_result: alloc.alloc_id(),
        value: LiteralConst::from(123.45f32),
    });

    // computation
    // a = 42u32 as f32;
    // b = 123.45f32 + a;
    let a = vec.push_inst(OpConvertUToF {
        id_result_type: IdResultType(f32),
        id_result: alloc.alloc_id(),
        unsigned_value: IdRef(u32_42),
    });
    let b = vec.push_inst(OpFAdd {
        id_result_type: IdResultType(f32),
        id_result: alloc.alloc_id(),
        operand_1: IdRef(f32_123_45),
        operand_2: IdRef(a),
    });

    // output
    let var_out = vec.push_inst(OpVariable {
        id_result_type: IdResultType(f32),
        id_result: alloc.alloc_id(),
        storage_class: StorageClass::Output,
        initializer: None,
    });
    let ptr_var_out = vec.push_inst(OpAccessChain {
        id_result_type: IdResultType(f32_ptr_output),
        id_result: alloc.alloc_id(),
        base: IdRef(var_out),
        indexes: SmallVec::from_iter([IdRef(u32_0)]),
    });
    vec.push_inst(OpStore {
        pointer: IdRef(ptr_var_out),
        object: IdRef(b),
        memory_access: None,
    });
    vec
}

#[test]
fn test_inst_offset_indexing() {
    let vec = demo_inst();
    for (off, inst) in vec.iter_ref().with_offsets() {
        let inst2 = vec.index_ref(off);
        assert_eq!(inst.get(), inst2.get());
    }
}
