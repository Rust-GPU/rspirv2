#![doc = include_str!("../README.md")]

use crate::grammar_path::{
    PATH_GRAMMAR_CORE, PATH_GRAMMAR_DEBUG_PRINTF, PATH_GRAMMAR_GLSL_STD_450,
};
use spirv_grammar_parser::codegen::{GrammarWriter, ModOptions, write_grammar};
use spirv_grammar_parser::quote::quote;
use std::path::Path;

mod grammar_path;

pub const PATH_GRAMMAR_CRATE_SRC: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../grammar/src/");

pub fn main() -> anyhow::Result<()> {
    write_grammar(
        GrammarWriter::new(Path::new(PATH_GRAMMAR_CRATE_SRC).join("core"))?,
        &PATH_GRAMMAR_CORE.read()?.parse_grammar()?,
        ModOptions {
            preamble: quote! {
                pub use crate::meta::*;
            },
            mod_attr: quote! {
                #![allow(unused_imports)]
                #![allow(non_camel_case_types)]
            },
            ..Default::default()
        },
    )?;
    write_grammar(
        GrammarWriter::new(Path::new(PATH_GRAMMAR_CRATE_SRC).join("glsl_std_450"))?,
        &PATH_GRAMMAR_GLSL_STD_450.read()?.parse_grammar()?,
        ModOptions {
            preamble: quote! {
                pub use crate::core::preamble::*;
            },
            ..Default::default()
        },
    )?;
    write_grammar(
        GrammarWriter::new(Path::new(PATH_GRAMMAR_CRATE_SRC).join("debug_printf"))?,
        &PATH_GRAMMAR_DEBUG_PRINTF.read()?.parse_grammar()?,
        ModOptions {
            preamble: quote! {
                pub use crate::core::preamble::*;
            },
            ..Default::default()
        },
    )?;
    Ok(())
}
