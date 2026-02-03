use crate::codegen::{EmitRef, GrammarWriter};
use crate::parse::{CoreGrammar, ExtInstSetGrammar, Grammar, InstClass, InstMeta, OperandKind};
use proc_macro2::TokenStream;
use std::ops::Deref;

/// Common writing interface between [`CoreGrammar`] and [`ExtInstSetGrammar`]
pub trait WriteableGrammar<'a>: EmitRef + Deref<Target = Grammar<'a>> {
    fn requires_core_import(&self) -> bool;

    fn emit_def(&self) -> TokenStream;
}

impl<'a> WriteableGrammar<'a> for CoreGrammar<'a> {
    fn requires_core_import(&self) -> bool {
        false
    }

    fn emit_def(&self) -> TokenStream {
        self.emit_def()
    }
}

impl<'a> WriteableGrammar<'a> for ExtInstSetGrammar<'a> {
    fn requires_core_import(&self) -> bool {
        true
    }

    fn emit_def(&self) -> TokenStream {
        self.emit_def()
    }
}

pub fn write_grammar<'a>(
    mut writer: GrammarWriter,
    grammar: &impl WriteableGrammar<'a>,
) -> anyhow::Result<()> {
    write_operand_kinds(&mut writer, grammar)?;
    write_inst_class(&mut writer, grammar)?;
    write_inst(&mut writer, grammar)?;
    writer.write_const_module("grammar", grammar.emit_def())?;
    writer.finish(grammar.requires_core_import())?;
    Ok(())
}

fn write_operand_kinds(writer: &mut GrammarWriter, grammar: &Grammar) -> anyhow::Result<()> {
    writer.write_const_module(
        "operand_kinds",
        grammar
            .operand_kinds
            .iter()
            .map(OperandKind::emit_def)
            .collect(),
    )
}

fn write_inst_class(writer: &mut GrammarWriter, grammar: &Grammar) -> anyhow::Result<()> {
    writer.write_const_module(
        "inst_class",
        grammar.inst_class.iter().map(InstClass::emit_def).collect(),
    )
}

fn write_inst(writer: &mut GrammarWriter, grammar: &Grammar) -> anyhow::Result<()> {
    writer.write_const_module(
        "inst",
        grammar.insts.iter().map(InstMeta::emit_def).collect(),
    )
}
