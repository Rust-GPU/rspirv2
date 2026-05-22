use crate::codegen::CodegenOptions;
use crate::parse::Grammar;
use quote::format_ident;
use std::fmt::Write;

pub fn isle_inst(mut out: impl Write, grammar: &Grammar<'_>) -> anyhow::Result<()> {
    for inst in &grammar.insts {
        let member_operands = inst.compute_operands();
        let opname = &inst.opname;
        write!(out, "(type {opname} extern (struct")?;
        for op in &member_operands {
            let name = &op.name;
            let ty = &op.ty;
            write!(out, " ({name} {ty})")?;
        }
        writeln!(out, "))")?;
    }
    Ok(())
}

pub fn isle_inst_set(
    mut out: impl Write,
    grammar: &Grammar<'_>,
    opt: &CodegenOptions<'_>,
) -> anyhow::Result<()> {
    let name = format_ident!("{}InstSet", opt.name_suffix_type);
    writeln!(out, "(type {name} extern")?;
    write!(out, "    (enum")?;
    for inst in &grammar.insts {
        let name = &inst.opname;
        write!(out, "\n        ({name} {name})")?;
    }
    write!(out, "))")?;
    Ok(())
}
