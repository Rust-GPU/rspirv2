use expect_test::expect;
use rspirv2::core::inst::{OpLoad, OpPhi, OpSwitch, OpTypeFloat};
use rspirv2::core::operands::{CooperativeMatrixReduce, Dim};
use rspirv2::custom_inst::{OpSwitchResolvedTarget, OpSwitchTarget, OpSwitchTargetLen};
use rspirv2_types::binary::IdResultAlloc;
use rspirv2_types::dis::{DisContext, DisOptions};
use rspirv2_types::inst::SpvInstDis;
use rspirv2_types::operand::{
    IdRef, IdResultType, LiteralConst, LiteralInteger, OperandDisContext, OperandEncoding,
};
use smallvec::SmallVec;

#[test]
pub fn test_dis_optional_operand() -> anyhow::Result<()> {
    let ctx = DisContext::no_context(DisOptions::simple());
    let mut alloc = IdResultAlloc::default();
    let float = OpTypeFloat {
        id_result: alloc.alloc_id(),
        width: LiteralInteger::new(32),
        floating_point_encoding: None,
    };
    expect!["%0 = OpTypeFloat 32"].assert_eq(&float.dis(&ctx).to_string());
    let load = OpLoad {
        id_result_type: IdResultType(float.id_result),
        id_result: alloc.alloc_id(),
        pointer: IdRef(alloc.alloc_id()),
        memory_access: None,
    };
    expect!["%1 = OpLoad %0 %2"].assert_eq(&load.dis(&ctx).to_string());
    Ok(())
}

#[test]
pub fn test_dis_composite_types() -> anyhow::Result<()> {
    let ctx = DisContext::no_context(DisOptions::simple());
    let mut alloc = IdResultAlloc::default();
    let switch = OpSwitch {
        selector: IdRef(alloc.alloc_id()),
        default: IdRef(alloc.alloc_id()),
        target: OpSwitchTarget::Resolved(OpSwitchResolvedTarget::from_iter(
            OpSwitchTargetLen::One,
            [
                (LiteralConst::from(42u32), IdRef(alloc.alloc_id())),
                (LiteralConst::from(69u32), IdRef(alloc.alloc_id())),
            ],
        )?),
    };
    expect!["OpSwitch %0 %1 42 %2 69 %3"].assert_eq(&switch.dis(&ctx).to_string());
    let phi = OpPhi {
        id_result_type: IdResultType(alloc.alloc_id()),
        id_result: alloc.alloc_id(),
        pair_id_ref_id_ref: SmallVec::from_iter([
            (IdRef(alloc.alloc_id()), IdRef(alloc.alloc_id())),
            (IdRef(alloc.alloc_id()), IdRef(alloc.alloc_id())),
        ]),
    };
    expect!["%5 = OpPhi %4 %6 %7 %8 %9"].assert_eq(&phi.dis(&ctx).to_string());
    Ok(())
}

/// Symbols that have been renamed in `Enumerant::variant_ident` to be valid rust idents
#[test]
pub fn test_dis_renamed_symbols() -> anyhow::Result<()> {
    let ctx = DisContext::no_context(DisOptions::simple());
    let ctx = OperandDisContext::new(&ctx);
    expect!(" 1D").assert_eq(&Dim::Dim1D.dis(&ctx).to_string());
    expect!(" 2D").assert_eq(&Dim::Dim2D.dis(&ctx).to_string());
    expect!(" 3D").assert_eq(&Dim::Dim3D.dis(&ctx).to_string());
    expect!(" 2x2").assert_eq(&CooperativeMatrixReduce::TwoByTwo.dis(&ctx).to_string());
    Ok(())
}
