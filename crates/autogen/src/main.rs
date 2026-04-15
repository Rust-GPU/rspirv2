#![doc = include_str!("../README.md")]

use rspirv2_grammar::{PATH_GRAMMAR_CORE, PATH_GRAMMAR_DEBUG_PRINTF, PATH_GRAMMAR_GLSL_STD_450};
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
    core.insts
        .iter_mut()
        .filter(|i| i.opname == "OpSwitch")
        .for_each(|i| i.source = Source::MetaOnly);
    write_grammar(
        GrammarWriter::new(Path::new(PATH_GRAMMAR_CRATE_SRC).join("core"))?,
        &core,
        &CodegenOptions {
            name_suffix_type: "Core",
        },
    )?;

    write_grammar(
        GrammarWriter::new(Path::new(PATH_GRAMMAR_CRATE_SRC).join("glsl_std_450"))?,
        &PATH_GRAMMAR_GLSL_STD_450.read()?.parse_grammar()?,
        &CodegenOptions {
            name_suffix_type: "Glsl",
        },
    )?;
    write_grammar(
        GrammarWriter::new(Path::new(PATH_GRAMMAR_CRATE_SRC).join("debug_printf"))?,
        &PATH_GRAMMAR_DEBUG_PRINTF.read()?.parse_grammar()?,
        &CodegenOptions {
            name_suffix_type: "DebugPrintf",
        },
    )?;
    Ok(())
}
