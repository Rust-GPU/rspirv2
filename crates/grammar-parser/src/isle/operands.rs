use crate::parse::{Category, Enumerant, Grammar};
use std::fmt::Write;

pub fn isle_operands(mut out: impl Write, grammar: &Grammar<'_>) -> anyhow::Result<()> {
    for o in &grammar.operand_kinds {
        let name = &o.name;
        match &o.category {
            Category::Id | Category::Literal | Category::BitEnum { .. } => {
                writeln!(out, "(type {name} (primitive {name}))")?;
            }
            Category::Composite { bases } => {
                write!(out, "(type {name} (struct")?;
                for member in bases {
                    write!(out, " {member}")?;
                }
                writeln!(out, "))")?;
            }
            Category::ValueEnum { enumerants } => {
                let symbols = enumerants
                    .iter()
                    .map(|e| (e, Enumerant::variant_ident(&e.symbol)))
                    .collect::<Vec<_>>();

                writeln!(out, "(type {name} extern")?;
                write!(out, "    (enum")?;
                for (variant, variant_name) in &symbols {
                    write!(out, "\n        ({variant_name}")?;
                    for params in &variant.parameters {
                        let kind = &params.kind;
                        write!(out, " {kind}")?;
                    }
                    write!(out, ")")?;
                }
                writeln!(out, "))")?;
            }
        }
    }
    Ok(())
}
