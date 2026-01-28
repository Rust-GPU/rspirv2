#![doc = include_str!("../README.md")]

use crate::grammar_path::{
    PATH_GRAMMAR_CORE, PATH_GRAMMAR_DEBUG_PRINTF, PATH_GRAMMAR_GLSL_STD_450,
};
use spirv_grammar_parser::codegen::GrammarWriter;
use std::path::Path;

mod grammar_path;

pub const PATH_GRAMMAR_CRATE_SRC: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../grammar/src/");

pub fn main() -> anyhow::Result<()> {
    GrammarWriter::new(
        &PATH_GRAMMAR_CORE.read()?.parse_grammar()?,
        Path::new(PATH_GRAMMAR_CRATE_SRC).join("core"),
    )?
    .finish()?;

    GrammarWriter::new(
        &PATH_GRAMMAR_GLSL_STD_450.read()?.parse_grammar()?,
        Path::new(PATH_GRAMMAR_CRATE_SRC).join("glsl_std_450"),
    )?
    .finish()?;

    GrammarWriter::new(
        &PATH_GRAMMAR_DEBUG_PRINTF.read()?.parse_grammar()?,
        Path::new(PATH_GRAMMAR_CRATE_SRC).join("debug_printf"),
    )?
    .finish()?;

    Ok(())
}
