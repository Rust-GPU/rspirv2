use expect_test::expect;
use rspirv2::core::inst::{OpConstant, OpTypeInt};
use rspirv2::core::preamble::CoreInstSet;
use rspirv2::custom_inst::{OpSwitch, OpSwitchResolvedTarget, OpSwitchTarget, OpSwitchTargetLen};
use rspirv2_types::Word;
use rspirv2_types::binary::IdResultAlloc;
use rspirv2_types::dis::{DisContext, DisOptions};
use rspirv2_types::inst::SpvInstDis;
use rspirv2_types::operand::{IdRef, IdResultType, LiteralConst, LiteralInteger};
use rspirv2_types::vec::InstVec;

#[test]
pub fn test_switch_32() -> anyhow::Result<()> {
    let vec = test_switch(SwitchTestCase::Words6_32)?;
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
    let vec = test_switch(SwitchTestCase::Words6_64)?;
    expect![[r#"
        %u64 = OpTypeInt 64 0
        %u64_42 = OpConstant %u64 42
        OpSwitch %u64_42 %2 69 %3 42 %4
    "#]]
    .assert_eq(&vec.dis(DisOptions::simple()).to_string());
    Ok(())
}

/// decoding an `OpSwitch` without context is impossible.
/// yet, our disassembler should turn it into something sensible and not error out
///
/// 2 words only make sense in 32bit
#[test]
pub fn test_switch_no_context_2_words_32() -> anyhow::Result<()> {
    expect![[r#"
        # ERROR: Missing context to resolve whether OpSwitch has 32 or 64bit constants! With 2 words must be 32bit
        OpSwitch %1 %2 69 %3"#]]
        .assert_eq(&test_switch_no_context(SwitchTestCase::Words2_32)?);
    Ok(())
}

/// 3 words don't make sense with 32bit, that's 1.5 entries, this needs to auto-detect 64bit
#[test]
pub fn test_switch_no_context_3_words_64() -> anyhow::Result<()> {
    expect![[r#"
        # ERROR: Missing context to resolve whether OpSwitch has 32 or 64bit constants! With 3 words must be 64bit
        OpSwitch %1 %2 69 %3"#]]
        .assert_eq(&test_switch_no_context(SwitchTestCase::Words3_64)?);
    Ok(())
}

/// could be 32 or 64bit, guess 32bit by default
#[test]
pub fn test_switch_no_context_6_words_32() -> anyhow::Result<()> {
    expect![[r#"
        # ERROR: Missing context to resolve whether OpSwitch has 32 or 64bit constants! With 6 words can be 32 or 64bit, *guessing 32bit*
        OpSwitch %1 %2 69 %3 42 %4 123 %5"#]]
        .assert_eq(&test_switch_no_context(SwitchTestCase::Words6_32)?);
    Ok(())
}

/// this disassembly is expected to be wrong, due to assuming 32bit labels when it was encoded with 64bit labels
#[test]
pub fn test_switch_no_context_6_words_64() -> anyhow::Result<()> {
    expect![[r#"
        # ERROR: Missing context to resolve whether OpSwitch has 32 or 64bit constants! With 6 words can be 32 or 64bit, *guessing 32bit*
        OpSwitch %1 %2 69 %0 3 %42 0 %4"#]]
        .assert_eq(&test_switch_no_context(SwitchTestCase::Words6_64)?);
    Ok(())
}

/// 5 words makes no sense, this must fail
#[test]
pub fn test_switch_no_context_5_words_degenerate() -> anyhow::Result<()> {
    let mut alloc = IdResultAlloc::default();
    let switch = OpSwitch {
        selector: IdRef(alloc.alloc_id()),
        default: IdRef(alloc.alloc_id()),
        target: OpSwitchTarget::Unresolved([1, 2, 3, 4, 5].into_iter().map(Word).collect()),
    };
    expect![[r#"
        # ERROR: Missing context to resolve whether OpSwitch has 32 or 64bit constants! With 5 words neither 32bit or 64bit make sense, refusing to decode entry table!
        OpSwitch %0 %1 ???"#]]
        .assert_eq(&switch.dis(&DisContext::no_context(DisOptions::simple())).to_string());
    Ok(())
}

pub fn test_switch_no_context(len: SwitchTestCase) -> anyhow::Result<String> {
    let vec = test_switch(len)?;
    let switch = vec.iter().nth(2).unwrap();
    let context = DisContext::no_context(DisOptions::simple());
    Ok(switch.dis(&context).to_string())
}

pub enum SwitchTestCase {
    Words2_32,
    Words3_64,
    Words6_32,
    Words6_64,
}

impl SwitchTestCase {
    #[allow(clippy::match_same_arms)]
    pub fn len(&self) -> OpSwitchTargetLen {
        match self {
            SwitchTestCase::Words2_32 => OpSwitchTargetLen::One,
            SwitchTestCase::Words3_64 => OpSwitchTargetLen::Two,
            SwitchTestCase::Words6_32 => OpSwitchTargetLen::One,
            SwitchTestCase::Words6_64 => OpSwitchTargetLen::Two,
        }
    }
}

fn test_switch(case: SwitchTestCase) -> anyhow::Result<InstVec<CoreInstSet>> {
    let mut alloc = IdResultAlloc::default();
    let mut vec = InstVec::new();
    let uint = vec.push_inst(OpTypeInt {
        id_result: alloc.alloc_id(),
        width: LiteralInteger::new(case.len().bits()),
        signedness: LiteralInteger::new(0),
    });
    let selector = vec.push_inst(OpConstant {
        id_result_type: IdResultType(uint),
        id_result: alloc.alloc_id(),
        value: match case.len() {
            OpSwitchTargetLen::One => LiteralConst::from(42u32),
            OpSwitchTargetLen::Two => LiteralConst::from(42u64),
        },
    });
    vec.push_inst(OpSwitch {
        selector: IdRef(selector),
        default: IdRef(alloc.alloc_id()),
        target: OpSwitchTarget::Resolved(OpSwitchResolvedTarget::from_iter(
            case.len(),
            match case {
                SwitchTestCase::Words2_32 => {
                    vec![(LiteralConst::from(69u32), IdRef(alloc.alloc_id()))]
                }
                SwitchTestCase::Words3_64 => {
                    vec![(LiteralConst::from(69u64), IdRef(alloc.alloc_id()))]
                }
                SwitchTestCase::Words6_32 => vec![
                    (LiteralConst::from(69u32), IdRef(alloc.alloc_id())),
                    (LiteralConst::from(42u32), IdRef(alloc.alloc_id())),
                    (LiteralConst::from(123u32), IdRef(alloc.alloc_id())),
                ],
                SwitchTestCase::Words6_64 => vec![
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
    let expected_len = match case {
        SwitchTestCase::Words6_32 | SwitchTestCase::Words6_64 => 6,
        SwitchTestCase::Words3_64 => 3,
        SwitchTestCase::Words2_32 => 2,
    };
    assert_eq!(switch.len(), 3 + expected_len);
    Ok(vec)
}
