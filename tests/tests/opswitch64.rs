use expect_test::expect;
use rspirv2::core::inst::{OpConstant, OpTypeInt};
use rspirv2::core::preamble::CoreInstSet;
use rspirv2::custom_inst::{OpSwitch, OpSwitchResolvedTarget, OpSwitchTarget, OpSwitchTargetLen};
use rspirv2_types::binary::IdResultAlloc;
use rspirv2_types::dis::DisOptions;
use rspirv2_types::operand::{IdRef, IdResultType, LiteralConst, LiteralInteger};
use rspirv2_types::vec::InstVec;

#[test]
pub fn test_switch_32() -> anyhow::Result<()> {
    let vec = test_switch(OpSwitchTargetLen::One)?;
    expect![[r#"
        %u32 = OpTypeInt 32 0
        %u32_42 = OpConstant %u32 42
        OpSwitch %u32_42 %2 69 %3 42 %4 123 %5
    "#]]
    .assert_eq(&vec.dis(DisOptions::simple()).to_string());
    Ok(())
}

/// last entry missing compared to 32bit is expected, see comments below
#[test]
pub fn test_switch_64() -> anyhow::Result<()> {
    let vec = test_switch(OpSwitchTargetLen::Two)?;
    expect![[r#"
        %u64 = OpTypeInt 64 0
        %u64_42 = OpConstant %u64 42
        OpSwitch %u64_42 %2 69 %3 42 %4
    "#]]
    .assert_eq(&vec.dis(DisOptions::simple()).to_string());
    Ok(())
}

fn test_switch(len: OpSwitchTargetLen) -> anyhow::Result<InstVec<CoreInstSet>> {
    let mut alloc = IdResultAlloc::default();
    let mut vec = InstVec::new();
    let uint = vec.push_inst(OpTypeInt {
        id_result: alloc.alloc_id(),
        width: LiteralInteger::new(len.bits()),
        signedness: LiteralInteger::new(0),
    });
    let selector = vec.push_inst(OpConstant {
        id_result_type: IdResultType(uint),
        id_result: alloc.alloc_id(),
        value: match len {
            OpSwitchTargetLen::One => LiteralConst::from(42u32),
            OpSwitchTargetLen::Two => LiteralConst::from(42u64),
        },
    });
    vec.push_inst(OpSwitch {
        selector: IdRef(selector),
        default: IdRef(alloc.alloc_id()),
        target: OpSwitchTarget::Resolved(OpSwitchResolvedTarget::from_iter(
            len,
            match len {
                OpSwitchTargetLen::One => vec![
                    (LiteralConst::from(69u32), IdRef(alloc.alloc_id())),
                    (LiteralConst::from(42u32), IdRef(alloc.alloc_id())),
                    (LiteralConst::from(123u32), IdRef(alloc.alloc_id())),
                ],
                OpSwitchTargetLen::Two => vec![
                    (LiteralConst::from(69u64), IdRef(alloc.alloc_id())),
                    (LiteralConst::from(42u64), IdRef(alloc.alloc_id())),
                ],
            },
        )?),
    });

    // always 2 words base + 6 words table long:
    // 32bit: 3 entries of 2 words each
    // 64bit: 2 entries of 3 words each
    // no way to tell which one it is without context!
    let switch = vec.iter_ref().nth(2).unwrap();
    assert_eq!(switch.len(), 9);
    Ok(vec)
}
