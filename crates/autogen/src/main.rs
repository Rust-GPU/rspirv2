#![doc = include_str!("../README.md")]

use rspirv2_codegen::{CodegenOptions, write_all};
use rspirv2_grammar::PATH_GRAMMAR_CORE;
use rspirv2_grammar_parser::codegen::GrammarWriter;
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

    let mut writer = GrammarWriter::new(Path::new(PATH_GRAMMAR_CRATE_SRC).join("core"))?;
    let opt = CodegenOptions {
        name_suffix_type: "Core",
    };
    write_all(&mut writer, &core, &opt)?;
    writer.finish()?;
    Ok(())
}
