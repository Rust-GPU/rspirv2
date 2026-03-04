#![doc = include_str!("../README.md")]

use rspirv2_grammar::{PATH_GRAMMAR_CORE, PATH_GRAMMAR_DEBUG_PRINTF, PATH_GRAMMAR_GLSL_STD_450};
use rspirv2_grammar_parser::codegen::{CodegenOptions, GrammarWriter, write_grammar};
use rspirv2_grammar_parser::quote::quote;
use std::path::Path;

pub const PATH_GRAMMAR_CRATE_SRC: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../rspirv2/src/");

pub fn main() -> anyhow::Result<()> {
    write_grammar(
        GrammarWriter::new(Path::new(PATH_GRAMMAR_CRATE_SRC).join("core"))?,
        &PATH_GRAMMAR_CORE.read()?.parse_grammar()?,
        &CodegenOptions::new_core(),
    )?;
    let path_to_core = quote!(crate::core);
    write_grammar(
        GrammarWriter::new(Path::new(PATH_GRAMMAR_CRATE_SRC).join("glsl_std_450"))?,
        &PATH_GRAMMAR_GLSL_STD_450.read()?.parse_grammar()?,
        &CodegenOptions::new_ext_inst_set("Glsl", &path_to_core),
    )?;
    write_grammar(
        GrammarWriter::new(Path::new(PATH_GRAMMAR_CRATE_SRC).join("debug_printf"))?,
        &PATH_GRAMMAR_DEBUG_PRINTF.read()?.parse_grammar()?,
        &CodegenOptions::new_ext_inst_set("DebugPrintf", &path_to_core),
    )?;
    Ok(())
}
