use expect_test::expect;
use rspirv2::core::inst::{OpName, OpNop, OpTypeInt, OpVariable};
use rspirv2::core::inst_set::CoreInstSet;
use rspirv2::core::operands::StorageClass;
use rspirv2_types::Word;
use rspirv2_types::dis::{DisInstSlice, DisOptions};
use rspirv2_types::inst::Inst;
use rspirv2_types::operand::{IdRef, IdResult, IdResultType, LiteralInteger, LiteralString};
use rspirv2_types::slice::RawInstSlice;
use rspirv2_types::vec::InstVec;

fn dis(raw: &RawInstSlice) -> String {
    DisInstSlice::<CoreInstSet>::new_no_context(raw, DisOptions::simple()).to_string()
}

fn record_inst() -> InstVec<CoreInstSet> {
    let mut vec = InstVec::new();
    let id_u32 = IdResult(Word(42));
    let id_var = IdResult(Word(69));
    vec.push(OpTypeInt {
        id_result: id_u32,
        width: LiteralInteger::new(32),
        signedness: LiteralInteger::new(0),
    });
    vec.push(OpName {
        target: IdRef(id_var),
        name: LiteralString::new("abcdef".to_string()),
    });
    vec.push(OpVariable {
        id_result_type: IdResultType(id_u32),
        id_result: id_var,
        storage_class: StorageClass::StorageBuffer,
        initializer: None,
    });
    vec
}

fn op_name_str_offset(words: &[Word]) -> usize {
    words
        .iter()
        .enumerate()
        .find(|(_, w)| **w == Word::from_le_bytes(*b"abcd"))
        .unwrap()
        .0
}

/// baseline is valid
#[test]
fn test_valid_dis() {
    let words = record_inst().into_vec();
    expect![[r#"
        %42 = OpTypeInt 32 0
        OpName %69 "abcdef"
        %69 = OpVariable %42 StorageBuffer
    "#]]
    .assert_eq(&dis(RawInstSlice::from_words(&words)));
}

/// even if one op fails to decode, as long as `op_len` is fine, we can jump over it and try to continue decoding
#[test]
fn test_no_null_term() {
    let mut words = record_inst().into_vec();
    let str_offset = op_name_str_offset(&words);
    words[str_offset + 1] = Word::from_le_bytes(*b"efgh");
    expect![[r#"
        %42 = OpTypeInt 32 0
        Error: String is not null-terminated.
        %69 = OpVariable %42 StorageBuffer
    "#]]
    .assert_eq(&dis(RawInstSlice::from_words(&words)));
}

/// even if one op fails to decode, as long as `op_len` is fine, we can jump over it and try to continue decoding
#[test]
fn test_shorter_str() {
    let mut words = record_inst().into_vec();
    let str_offset = op_name_str_offset(&words);
    words[str_offset] = Word::from_le_bytes([b'a', b'b', b'c', 0]);
    words[str_offset + 1] = Word(0);
    expect![[r#"
        %42 = OpTypeInt 32 0
        Error: The fixed-size Instruction with 3 param words has 1 Words left over after decoding.
        %69 = OpVariable %42 StorageBuffer
    "#]]
    .assert_eq(&dis(RawInstSlice::from_words(&words)));
}

/// zero len op, no way to jump over inst, so stop the disassembly
#[test]
fn test_zero_len_op() {
    let mut words = record_inst().into_vec();
    let str_offset = op_name_str_offset(&words);
    words[str_offset - 2] = Word::new_op(OpName::META.opcode, 3).unwrap();
    words[str_offset] = Word::from_le_bytes([b'a', b'b', b'c', 0]);
    words[str_offset + 1] = Word(0);
    expect![[r#"
        %42 = OpTypeInt 32 0
        OpName %69 "abc"
        Error: Instruction at offset 7: Instruction has an invalid length of 0, must be least 1 word as it includes the opcode itself.
    "#]]
    .assert_eq(&dis(RawInstSlice::from_words(&words)));
}

/// replacing with `OpNop` is not zeroing out the word, but replacing it with `0x10000`
#[test]
fn test_replace_nop() {
    let mut words = record_inst().into_vec();
    let str_offset = op_name_str_offset(&words);
    words[str_offset - 2] = Word::new_op(OpName::META.opcode, 3).unwrap();
    words[str_offset] = Word::from_le_bytes([b'a', b'b', b'c', 0]);
    words[str_offset + 1] = Word::new_op(OpNop::META.opcode, 1).unwrap();
    expect![[r#"
        %42 = OpTypeInt 32 0
        OpName %69 "abc"
        OpNop
        %69 = OpVariable %42 StorageBuffer
    "#]]
    .assert_eq(&dis(RawInstSlice::from_words(&words)));
}

/// op longer than remaining words, no way to jump over inst, so stop the disassembly
#[test]
fn test_too_long_op() {
    let mut words = record_inst().into_vec();
    let str_offset = op_name_str_offset(&words);
    words[str_offset - 2] = Word::new_op(OpName::META.opcode, 0xDEAD).unwrap();
    expect![[r#"
        %42 = OpTypeInt 32 0
        Error: Instruction at offset 4: Instruction has a supposed length of 57005 but the module only has 8 words remaining.
    "#]]
    .assert_eq(&dis(RawInstSlice::from_words(&words)));
}

/// the string is longer than the instruction length:
/// * first string fails to decode
/// * then the remaining string is interpreted as a garbage opcode word, manipulated here to be of length 2,
///   fails due to unknown opcode
/// * jumps to `OpVariable + 1` and reinterprets the `IdResult` into a garbage opcode word of 0 length
#[test]
fn test_op_shorter_than_string() {
    let mut words = record_inst().into_vec();
    let str_offset = op_name_str_offset(&words);
    words[str_offset - 2] = Word::new_op(OpName::META.opcode, 3).unwrap();
    words[str_offset + 1] = Word::from_le_bytes([b'e', b'f', 2, 0]);
    expect![[r#"
        %42 = OpTypeInt 32 0
        Error: String is not null-terminated.
        Error: Instruction Set couldn't decode instruction with unknown opcode 26213
        Error: Instruction at offset 9: Instruction has an invalid length of 0, must be least 1 word as it includes the opcode itself.
    "#]]
    .assert_eq(&dis(RawInstSlice::from_words(&words)));
}
