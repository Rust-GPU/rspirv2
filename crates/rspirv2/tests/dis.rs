use rspirv2::core::inst::{OpLoad, OpTypeFloat};
use rspirv2::core::operands::{CooperativeMatrixReduce, Dim};
use rspirv2_types::binary::{IdResultAlloc, IdResultAllocator};
use rspirv2_types::dis::{DisContext, DisOptions};
use rspirv2_types::inst::InstEncoding;
use rspirv2_types::operand::{IdRef, IdResultType, LiteralInteger, OperandEncoding};

#[test]
pub fn test_dis_optional_operand() -> anyhow::Result<()> {
    let ctx = DisContext::new(DisOptions::simple());
    let mut alloc = IdResultAllocator::default();
    let float = OpTypeFloat {
        id_result: Some(alloc.alloc_id()?),
        width: LiteralInteger::new(32),
        floating_point_encoding: None,
    };
    assert_eq!("%0 = OpTypeFloat 32", float.dis(&ctx).to_string());
    let float = OpLoad {
        id_result_type: IdResultType(float.id_result.unwrap()),
        id_result: Some(alloc.alloc_id()?),
        pointer: IdRef(alloc.alloc_id()?),
        memory_access: None,
    };
    assert_eq!("%1 = OpLoad %0 %2", float.dis(&ctx).to_string());
    Ok(())
}

/// Symbols that have been renamed in `Enumerant::variant_ident` to be valid rust idents
#[test]
pub fn test_dis_renamed_symbols() -> anyhow::Result<()> {
    let ctx = DisContext::new(DisOptions::simple());
    assert_eq!(" 1D", Dim::Dim1D.dis(&ctx).to_string());
    assert_eq!(" 2D", Dim::Dim2D.dis(&ctx).to_string());
    assert_eq!(" 3D", Dim::Dim3D.dis(&ctx).to_string());
    assert_eq!(
        " 2x2",
        CooperativeMatrixReduce::TwoByTwo.dis(&ctx).to_string()
    );
    Ok(())
}
