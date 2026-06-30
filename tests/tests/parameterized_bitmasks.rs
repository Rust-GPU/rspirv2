use expect_test::expect;
use rspirv2::core::operands::ImageOperands;
use rspirv2_types::Word;
use rspirv2_types::dis::{DisContext, DisOptions};
use rspirv2_types::operand::{IdRef, IdResult, OperandDisContext, SpvOperandDis};

#[test]
pub fn test_param_bitmask() -> anyhow::Result<()> {
    let ctx = DisContext::no_context(DisOptions::simple());
    let ctx = OperandDisContext {
        ctx: &ctx,
        id_result: None,
        id_result_type: None,
    };

    let mut operand = ImageOperands::new();

    // enable 5 bits in random order
    expect![" None"].assert_eq(&operand.dis(&ctx).to_string());
    operand.set_bias(Some(IdRef(IdResult(Word(1)))));
    expect![" Bias %1"].assert_eq(&operand.dis(&ctx).to_string());
    operand.set_offsets(Some(IdRef(IdResult(Word(5)))));
    expect![" Bias|Offsets %1 %5"].assert_eq(&operand.dis(&ctx).to_string());
    operand.set_lod(Some(IdRef(IdResult(Word(2)))));
    expect![" Bias|Lod|Offsets %1 %2 %5"].assert_eq(&operand.dis(&ctx).to_string());
    operand.set_nontemporal(true);
    expect![" Bias|Lod|Nontemporal|Offsets %1 %2 %5"].assert_eq(&operand.dis(&ctx).to_string());
    operand.set_grad(Some((IdRef(IdResult(Word(3))), IdRef(IdResult(Word(4))))));
    expect![" Bias|Lod|Grad|Nontemporal|Offsets %1 %2 %3 %4 %5"]
        .assert_eq(&operand.dis(&ctx).to_string());

    // enabling them again changes params
    operand.set_nontemporal(true);
    expect![" Bias|Lod|Grad|Nontemporal|Offsets %1 %2 %3 %4 %5"]
        .assert_eq(&operand.dis(&ctx).to_string());
    operand.set_grad(Some((IdRef(IdResult(Word(30))), IdRef(IdResult(Word(40))))));
    expect![" Bias|Lod|Grad|Nontemporal|Offsets %1 %2 %30 %40 %5"]
        .assert_eq(&operand.dis(&ctx).to_string());

    // disable 5 bits in random order
    operand.set_lod(None);
    expect![" Bias|Grad|Nontemporal|Offsets %1 %30 %40 %5"]
        .assert_eq(&operand.dis(&ctx).to_string());
    operand.set_nontemporal(false);
    expect![" Bias|Grad|Offsets %1 %30 %40 %5"].assert_eq(&operand.dis(&ctx).to_string());
    operand.set_grad(None);
    expect![" Bias|Offsets %1 %5"].assert_eq(&operand.dis(&ctx).to_string());
    operand.set_bias(None);
    expect![" Offsets %5"].assert_eq(&operand.dis(&ctx).to_string());
    operand.set_offsets(None);
    expect![" None"].assert_eq(&operand.dis(&ctx).to_string());

    // disabling them again changes nothing
    operand.set_nontemporal(false);
    expect![" None"].assert_eq(&operand.dis(&ctx).to_string());
    operand.set_grad(None);
    expect![" None"].assert_eq(&operand.dis(&ctx).to_string());

    Ok(())
}
