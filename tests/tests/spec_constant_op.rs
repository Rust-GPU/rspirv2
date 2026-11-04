use expect_test::expect;
use rspirv2::binary::IdResultAlloc;
use rspirv2::core::inst::{OpIAdd, OpSpecConstant, OpTypeInt};
use rspirv2::core::inst_set::CoreInstSet;
use rspirv2::dis::DisOptions;
use rspirv2::module::Module;
use rspirv2::op_spec_op::OpSpecConstantOp;
use rspirv2::operand::{IdRef, IdResultType, LiteralConst, LiteralInteger};
use rspirv2::vec::InstVec;
use rspirv2_tools_gen::dis::Profile;
use spv::SPEC_CONSTANT_OP;

#[test]
fn test_encode_decode() {
    let mut alloc = IdResultAlloc::new();
    let mut vec = InstVec::<CoreInstSet>::new();
    let u32 = IdResultType(vec.push_inst(OpTypeInt {
        id_result: alloc.alloc_id(),
        width: LiteralInteger::new(32),
        signedness: LiteralInteger::new(0),
    }));
    let a = vec.push_inst(OpSpecConstant {
        id_result_type: u32,
        id_result: alloc.alloc_id(),
        value: LiteralConst::from(42u32),
    });

    let mut vec2 = InstVec::<CoreInstSet>::new();
    vec2.push_inst(OpSpecConstantOp(
        OpIAdd {
            id_result_type: u32,
            id_result: alloc.alloc_id(),
            operand_1: IdRef(a),
            operand_2: IdRef(a),
        }
        .into(),
    ));

    // raw bytes
    expect![[r#"
        [
            393268,
            0,
            2,
            128,
            1,
            1,
        ]
    "#]]
    .assert_debug_eq(&vec2.as_words().iter().map(|a| a.0).collect::<Vec<_>>());

    // dis
    expect![[r#"
        %2 = OpSpecConstantOp %0 OpIAdd %1 %1
    "#]]
    .assert_eq(&vec2.dis(DisOptions::simple()).to_string());
}

#[test]
fn test_spec_constant_op_dis() -> anyhow::Result<()> {
    SPEC_CONSTANT_OP.test_disassembly("rspirv2", Profile::Default)
}

#[test]
fn test_spec_constant_op_checked() -> anyhow::Result<()> {
    Module::<CoreInstSet>::from_bytes(&std::fs::read(SPEC_CONSTANT_OP.spv())?)?;
    Ok(())
}
