#![doc = include_str!("../README.md")]
#![allow(clippy::needless_update)]

use rspirv2_grammar::{PATH_GRAMMAR_CORE, PATH_GRAMMAR_DEBUG_PRINTF, PATH_GRAMMAR_GLSL_STD_450};
use rspirv2_grammar_parser::codegen::{GrammarWriter, ModOptions, write_grammar};
use rspirv2_grammar_parser::quote::quote;
use std::path::Path;

pub const PATH_GRAMMAR_CRATE_SRC: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../rspirv2/src/");

pub fn main() -> anyhow::Result<()> {
    let mod_attr = quote! {
        #![allow(unused_imports)]
        #![allow(non_camel_case_types)]
        #![allow(deprecated)]
        #![allow(clippy::identity_op)]
    };

    write_grammar(
        GrammarWriter::new(Path::new(PATH_GRAMMAR_CRATE_SRC).join("core"))?,
        &PATH_GRAMMAR_CORE.read()?.parse_grammar()?,
        ModOptions {
            preamble: quote! {
                pub use crate::binary::*;
                pub use crate::inst::*;
                pub use crate::meta::*;
                pub use crate::operand::*;
                pub use bitflags::bitflags;
                pub use smallvec::SmallVec;
            },
            mod_attr: mod_attr.clone(),
            mod_extra: quote! {
                impl preamble::AnyCapability for preamble::Capability {}
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
            mod_attr: mod_attr.clone(),
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
            mod_attr: mod_attr.clone(),
            ..Default::default()
        },
    )?;
    Ok(())
}
