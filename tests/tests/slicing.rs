use rspirv2::core::inst::{
    OpAccessChain, OpConstant, OpConvertUToF, OpFAdd, OpStore, OpTypeFloat, OpTypeInt,
    OpTypePointer, OpVariable,
};
use rspirv2::core::inst_set::CoreInstSet;
use rspirv2::core::operands::StorageClass;
use rspirv2::core::preamble::MemoryAccess;
use rspirv2_types::Word;
use rspirv2_types::binary::{IdResultAlloc, InstOffset};
use rspirv2_types::operand::{IdRef, IdResultType, LiteralConst, LiteralInteger};
use rspirv2_types::vec::InstVec;
use smallvec::SmallVec;
use std::assert_matches;
use std::fmt::Debug;
use std::ops::{Range, RangeBounds};

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
        memory_access: Some({
            let mut access = MemoryAccess::new();
            access.set_aligned(Some(LiteralInteger::new(4)));
            access
        }),
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

#[test]
#[expect(clippy::reversed_empty_ranges)]
fn test_slicing() {
    fn test<R: RangeBounds<usize> + Debug>(range: R, expected: Option<Range<usize>>) {
        println!("{range:?} - {expected:?}");
        let vec = demo_inst();
        let slice = vec.slice((
            range.start_bound().map(|a| InstOffset(*a)),
            range.end_bound().map(|a| InstOffset(*a)),
        ));
        let actual = slice.map(|slice| {
            let start = (slice.as_words().as_ptr() as usize - vec.as_words().as_ptr() as usize)
                / size_of::<Word>();
            let len = slice.as_words().len();
            start..(start + len)
        });
        assert_eq!(actual, expected);
    }

    assert_eq!(demo_inst().as_words().len(), 46);
    test(.., Some(0..46));

    // RangeFrom
    test(0.., Some(0..46));
    test(1.., None);
    test(2.., None);
    test(3.., None);
    test(4.., Some(4..46));
    test(5.., None);
    test(6.., None);
    test(7.., Some(7..46));
    test(11.., Some(11..46));

    // RangeTo
    test(..46, Some(0..46));
    test(..45, None);
    // Note: 0..44 is *almost* a valid instruction stream. The last 2 words of the last instruction are optional,
    // but the op word encodes an instruction of length 5 instead of 3, so it is rejected when sliced manually.
    // We must reject that sort of slicing here too.
    test(..44, None);
    test(..43, None);
    test(..42, None);
    test(..41, Some(0..41));

    // Range
    test(0..46, Some(0..46));
    test(0..1, None);
    test(0..2, None);
    test(0..3, None);
    test(0..4, Some(0..4));
    test(1..4, None);
    test(2..4, None);
    test(3..4, None);
    test(0..7, Some(0..7));
    test(4..7, Some(4..7));
    test(41..46, Some(41..46));

    // RangeInclusive & RangeToInclusive
    test(..=0, Some(0..4));
    test(0..=0, Some(0..4));
    test(1..=0, None);
    test(0..=1, None);
    test(..=4, Some(0..7));
    test(0..=4, Some(0..7));
    test(4..=4, Some(4..7));
    test(..=7, Some(0..11));
    test(0..=7, Some(0..11));
    test(4..=7, Some(4..11));
    test(7..=7, Some(7..11));
    test(..=41, Some(0..46));
    test(41..=41, Some(41..46));
    test(..=46, None);
    test(41..=46, None);
    test(46..=46, None);

    // zero length but valid
    test(0..0, Some(0..0));
    test(1..1, None);
    test(4..4, Some(4..4));
    test(7..7, Some(7..7));
    test(41..41, Some(41..41));
    assert_matches!(demo_inst().as_words().get(46..46), Some(_));
    test(46..46, Some(46..46));

    // oob
    assert_matches!(demo_inst().as_words().get(47..47), None);
    test(47..47, None);
}
