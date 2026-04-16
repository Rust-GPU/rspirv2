use crate::codegen::instructions::{write_inst, write_inst_enum};
use crate::codegen::operands::write_operands;
use crate::codegen::options::CodegenOptions;
use crate::codegen::{EmitRef, GrammarWriter};
use crate::parse::{
    Category, CoreGrammar, ExtInstSetGrammar, Extension, Grammar, InstClass, InstMeta, OperandKind,
};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashSet;
use std::ops::Deref;

/// Common writing interface between [`CoreGrammar`] and [`ExtInstSetGrammar`]
pub trait WriteableGrammar<'a>: EmitRef + Deref<Target = Grammar<'a>> {
    fn emit_grammar_def(&self) -> TokenStream;
}

impl<'a> WriteableGrammar<'a> for CoreGrammar<'a> {
    fn emit_grammar_def(&self) -> TokenStream {
        self.emit_def()
    }
}

impl<'a> WriteableGrammar<'a> for ExtInstSetGrammar<'a> {
    fn emit_grammar_def(&self) -> TokenStream {
        self.emit_def()
    }
}

pub fn write_grammar<'a>(
    mut writer: GrammarWriter,
    grammar: &impl WriteableGrammar<'a>,
    opt: &CodegenOptions<'_>,
) -> anyhow::Result<()> {
    write_extensions(&mut writer, grammar)?;
    write_operand_kinds(&mut writer, grammar)?;
    write_operands(&mut writer, grammar)?;
    write_inst_class(&mut writer, grammar)?;
    write_inst_meta(&mut writer, grammar)?;
    write_inst(&mut writer, grammar)?;
    write_inst_enum(&mut writer, grammar, opt)?;
    write_grammar_mod(&mut writer, grammar)?;
    writer.finish()?;
    Ok(())
}

fn write_operand_kinds(writer: &mut GrammarWriter, grammar: &Grammar<'_>) -> anyhow::Result<()> {
    writer.write_module(
        "operand_kinds",
        grammar
            .operand_kinds
            .iter()
            .map(OperandKind::emit_def)
            .collect(),
    )
}

fn write_inst_class(writer: &mut GrammarWriter, grammar: &Grammar<'_>) -> anyhow::Result<()> {
    writer.write_module(
        "inst_class",
        grammar.inst_class.iter().map(InstClass::emit_def).collect(),
    )
}

fn write_inst_meta(writer: &mut GrammarWriter, grammar: &Grammar<'_>) -> anyhow::Result<()> {
    writer.write_module(
        "inst_meta",
        grammar.insts.iter().map(InstMeta::emit_def).collect(),
    )
}

fn write_extensions(writer: &mut GrammarWriter, grammar: &Grammar<'_>) -> anyhow::Result<()> {
    let mut extensions = grammar
        .insts
        .iter()
        .flat_map(|i| i.extensions.iter())
        .chain(grammar.operand_kinds.iter().flat_map(|o| {
            match &o.category {
                Category::BitEnum { enumerants } | Category::ValueEnum { enumerants } => enumerants,
                _ => const { &Vec::new() },
            }
            .iter()
            .flat_map(|e| e.extensions.iter())
        }))
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    extensions.sort_by_key(|e| e.name());
    writer.write_module(
        "extensions",
        extensions
            .iter()
            .copied()
            .map(Extension::emit_def)
            .collect(),
    )
}

fn write_grammar_mod<'a>(
    writer: &mut GrammarWriter,
    grammar: &impl WriteableGrammar<'a>,
) -> anyhow::Result<()> {
    let grammar_def = grammar.emit_grammar_def();
    let other_def = Grammar::emit_def(grammar);
    writer.write_module(
        "grammar",
        quote! {
            #grammar_def
            #other_def
        },
    )?;
    Ok(())
}
