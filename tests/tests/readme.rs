use rspirv2::core::inst::{OpConstant, OpIAdd, OpIMul, OpISub, OpTypeInt};
use rspirv2::core::inst_set::CoreInstSet;
use rspirv2_types::binary::IdResultAlloc;
use rspirv2_types::dis::DisOptions;
use rspirv2_types::operand::{IdRef, IdResultType, LiteralConst, LiteralInteger};
use rspirv2_types::vec::InstVec;

/// test for readme's contents, keep in sync!
#[test]
pub fn test_readme() {
    // an allocator for `IdResult`s (SSA value IDs)
    let mut alloc = IdResultAlloc::new();
    // a `Vec` for instructions that stores them in SPIR-V encoded form
    let mut vec = InstVec::<CoreInstSet>::new();

    // add some SPIR-V instructions
    // declare u32 type
    let u32 = vec.push_inst(OpTypeInt {
        id_result: alloc.alloc_id(),
        width: LiteralInteger::new(32),
        signedness: LiteralInteger::new(0),
    });
    // let a: u32 = 42;
    let a = vec.push_inst(OpConstant {
        id_result_type: IdResultType(u32),
        id_result: alloc.alloc_id(),
        value: LiteralConst::from(42u32),
    });
    // let b: u32 = a + a;
    let b = vec.push_inst(OpIAdd {
        id_result_type: IdResultType(u32),
        id_result: alloc.alloc_id(),
        operand_1: IdRef(a),
        operand_2: IdRef(a),
    });

    // replace `b = a + a` with `b = a * a`
    let mut modified = vec
        .iter()
        .map(|inst| match inst {
            CoreInstSet::IAdd(OpIAdd {
                id_result,
                operand_1,
                operand_2,
                ..
            }) => OpIMul {
                id_result_type: IdResultType(u32),
                id_result,
                operand_1,
                operand_2,
            }
            .into(),
            inst => inst,
        })
        .collect::<InstVec<_>>();

    // append InstVec to each other
    let mut vec2 = InstVec::new();
    vec2.push_inst(OpISub {
        id_result_type: IdResultType(u32),
        id_result: alloc.alloc_id(),
        operand_1: IdRef(a),
        operand_2: IdRef(b),
    });
    modified.append(&mut vec2);

    // disassembly with various settings:
    // `default()` for colorful terminal output
    // `simple()` to remove color and padding, for tests
    // `like_spirv_tools()` and `like_rspirv()` to mimic output of other disassemblers
    let disassembly = format!("{}", modified.dis(DisOptions::simple()));
    expect_test::expect![[r#"
        %u32 = OpTypeInt 32 0
        %u32_42 = OpConstant %u32 42
        %2 = OpIMul %u32 %u32_42 %u32_42
        %3 = OpISub %u32 %u32_42 %2
    "#]]
    .assert_eq(&disassembly);
}
