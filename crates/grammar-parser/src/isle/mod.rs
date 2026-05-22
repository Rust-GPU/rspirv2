use crate::codegen::CodegenOptions;
use crate::isle::inst::{isle_inst, isle_inst_set};
use crate::isle::operands::isle_operands;
use crate::parse::Grammar;
use std::fmt::Write;

mod inst;
mod operands;

pub fn isle(grammar: &Grammar<'_>, opt: &CodegenOptions<'_>) -> anyhow::Result<String> {
    let mut out = String::new();
    isle_operands(&mut out, grammar)?;
    writeln!(&mut out)?;
    isle_inst(&mut out, grammar)?;
    writeln!(&mut out)?;
    isle_inst_set(&mut out, grammar, opt)?;
    Ok(out)
}
