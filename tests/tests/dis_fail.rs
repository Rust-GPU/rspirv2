use expect_test::expect;
use rspirv2::binary::IdResultAlloc;
use rspirv2::core::inst::{OpIAdd, OpIMul};
use rspirv2::core::inst_set::CoreInstSet;
use rspirv2::dis::DisOptions;
use rspirv2::operand::{IdRef, IdResultType};
use rspirv2::vec::InstVec;

#[test]
fn test_dis_fail() -> anyhow::Result<()> {
    let mut alloc = IdResultAlloc::default();
    let mut vec = InstVec::<CoreInstSet>::new();
    vec.push(OpIAdd {
        id_result_type: IdResultType(alloc.alloc_id()),
        id_result: alloc.alloc_id(),
        operand_1: IdRef(alloc.alloc_id()),
        operand_2: IdRef(alloc.alloc_id()),
    });
    vec.push(OpIMul {
        id_result_type: IdResultType(alloc.alloc_id()),
        id_result: alloc.alloc_id(),
        operand_1: IdRef(alloc.alloc_id()),
        operand_2: IdRef(alloc.alloc_id()),
    });
    expect![[r#"
        %1 = OpIAdd %0 %2 %3
        %5 = OpIMul %4 %6 %7
    "#]]
    .assert_eq(&vec.dis(DisOptions::simple()).to_string());

    // change opcode to similar inst
    let mut vec = vec.into_vec();
    vec[0].0 += 4;
    let vec = InstVec::<CoreInstSet>::from_words_unchecked(vec);
    expect![[r#"
        %1 = OpIMul %0 %2 %3
        %5 = OpIMul %4 %6 %7
    "#]]
    .assert_eq(&vec.dis(DisOptions::simple()).to_string());

    // change opcode to random inst
    let mut vec = vec.into_vec();
    vec[0].0 += 199;
    let vec = InstVec::<CoreInstSet>::from_words_unchecked(vec);
    expect![[r#"
        Error: The fixed-size Instruction with 4 param words has 2 Words left over after decoding.
        %5 = OpIMul %4 %6 %7
    "#]]
    .assert_eq(&vec.dis(DisOptions::simple()).to_string());

    // make inst len longer, reaching into next inst, unrecoverable
    let mut vec = vec.into_vec();
    vec[0].0 += 1 << 16;
    let vec = InstVec::<CoreInstSet>::from_words_unchecked(vec);
    expect![[r#"
        Error: The fixed-size Instruction with 5 param words has 3 Words left over after decoding.
        Error: Instruction at offset 6: Instruction has an invalid length of 0, must be least 1 word as it includes the opcode itself.
    "#]].assert_eq(&vec.dis(DisOptions::simple()).to_string());

    // make inst len shorter, unrecoverable
    let mut vec = vec.into_vec();
    vec[0].0 -= 2 << 16;
    let vec = InstVec::<CoreInstSet>::from_words_unchecked(vec);
    expect![[r#"
        Error: The fixed-size Instruction with 3 param words has 1 Words left over after decoding.
        Error: Instruction at offset 4: Instruction has an invalid length of 0, must be least 1 word as it includes the opcode itself.
    "#]].assert_eq(&vec.dis(DisOptions::simple()).to_string());

    Ok(())
}
