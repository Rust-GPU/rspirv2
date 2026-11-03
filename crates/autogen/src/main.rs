#![doc = include_str!("../README.md")]

use rspirv2_grammar::PATH_GRAMMAR_CORE;
use rspirv2_grammar_parser::codegen::{CodegenOptions, GrammarWriter, write_grammar};
use rspirv2_grammar_parser::parse::Source;
use rspirv2_grammar_parser::timer::TimerPrintOnDrop;
use std::path::Path;

pub const PATH_GRAMMAR_CRATE_SRC: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../rspirv2/src/");

pub fn main() -> anyhow::Result<()> {
    let _timer = TimerPrintOnDrop::new("autogen");
    autogen()
}

pub fn autogen() -> anyhow::Result<()> {
    let core_json = PATH_GRAMMAR_CORE.read()?;
    let mut core = core_json.parse_grammar()?;

    // instructions that require custom implementations
    core.insts
        .iter_mut()
        .filter(|i| i.opname == "OpSwitch" || i.opname == "OpSpecConstantOp")
        .for_each(|i| i.source = Source::MetaOnly);

    // mark instructions as OpSpecOp
    core.insts
        .iter_mut()
        .filter(|i| OP_SPEC_OP_NAMES.contains(&&*i.opname))
        .for_each(|i| i.op_spec_op = true);

    write_grammar(
        GrammarWriter::new(Path::new(PATH_GRAMMAR_CRATE_SRC).join("core"))?,
        &core,
        &CodegenOptions {
            name_suffix_type: "Core",
        },
    )?;
    Ok(())
}

/// All instructions that are allowed in `OpSpecConstantOp`, see List at
/// <https://registry.khronos.org/SPIR-V/specs/unified1/SPIRV.html#OpSpecConstantOp>
///
/// List is not yet machine parsable, see <https://github.com/KhronosGroup/SPIRV-Headers/issues/645>
const OP_SPEC_OP_NAMES: &[&str] = &[
    "OpSConvert",
    "OpUConvert",
    "OpFConvert",
    "OpSNegate",
    "OpNot",
    "OpIAdd",
    "OpISub",
    "OpIMul",
    "OpUDiv",
    "OpSDiv",
    "OpUMod",
    "OpSRem",
    "OpSMod",
    "OpShiftRightLogical",
    "OpShiftRightArithmetic",
    "OpShiftLeftLogical",
    "OpBitwiseOr",
    "OpBitwiseXor",
    "OpBitwiseAnd",
    "OpVectorShuffle",
    "OpCompositeExtract",
    "OpCompositeInsert",
    "OpLogicalOr",
    "OpLogicalAnd",
    "OpLogicalNot",
    "OpLogicalEqual",
    "OpLogicalNotEqual",
    "OpSelect",
    "OpIEqual",
    "OpINotEqual",
    "OpULessThan",
    "OpSLessThan",
    "OpUGreaterThan",
    "OpSGreaterThan",
    "OpULessThanEqual",
    "OpSLessThanEqual",
    "OpUGreaterThanEqual",
    "OpSGreaterThanEqual",
    "OpQuantizeToF16",
    "OpConvertFToS",
    "OpConvertSToF",
    "OpConvertFToU",
    "OpConvertUToF",
    "OpUConvert",
    "OpConvertPtrToU",
    "OpConvertUToPtr",
    "OpGenericCastToPtr",
    "OpPtrCastToGeneric",
    "OpBitcast",
    "OpFNegate",
    "OpFAdd",
    "OpFSub",
    "OpFMul",
    "OpFDiv",
    "OpFRem",
    "OpFMod",
    "OpAccessChain",
    "OpInBoundsAccessChain",
    "OpPtrAccessChain",
    "OpInBoundsPtrAccessChain",
];
