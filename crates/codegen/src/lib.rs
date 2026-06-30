mod inst;
mod inst_set;
mod operands;
mod options;

use crate::inst::write_inst;
use crate::inst_set::write_inst_enum;
use crate::operands::write_operands;
pub use options::*;
use rspirv2_grammar_parser::codegen::meta::{WriteableGrammar, write_meta};
pub use rspirv2_grammar_parser::codegen::writer::*;

pub const OPERAND_ID_RESULT: &str = "IdResult";
pub const OPERAND_ID_RESULT_TYPE: &str = "IdResultType";

pub fn write_all<'a>(
    writer: &mut GrammarWriter,
    grammar: &impl WriteableGrammar<'a>,
    opt: &CodegenOptions<'_>,
) -> anyhow::Result<()> {
    write_meta(&mut *writer, grammar)?;
    write_operands(&mut *writer, grammar)?;
    write_inst(&mut *writer, grammar)?;
    write_inst_enum(&mut *writer, grammar, opt)?;
    Ok(())
}
