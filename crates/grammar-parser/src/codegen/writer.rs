use crate::codegen::EmitRef;
use crate::parse::{CoreGrammar, ExtInstSetGrammar, Grammar, InstClass, InstMeta, OperandKind};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::fs;
use std::ops::Deref;
use std::path::PathBuf;
use std::process::Command;

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

/// a use statement that imports symbols from other files
pub fn use_super() -> TokenStream {
    quote! {
        use super::preamble::*;
    }
}

pub struct GrammarWriter {
    folder: PathBuf,
    submodules: Vec<String>,
    requires_core_import: bool,
}

impl GrammarWriter {
    pub fn new<'a>(grammar: &impl WriteableGrammar<'a>, folder: PathBuf) -> anyhow::Result<Self> {
        fs::create_dir_all(&folder)?;
        let mut this = Self {
            folder,
            submodules: Vec::new(),
            requires_core_import: grammar.requires_core_import(),
        };
        this.write_grammar(grammar)?;
        Ok(this)
    }

    /// Get the file path for a submodule name
    pub fn submodule_file(&self, submodule: &str) -> PathBuf {
        self.folder.join(format!("{submodule}.rs"))
    }

    /// Write a module file and "link" it from `mod.rs`.
    ///
    /// The contents must contain the tokens that [`use_super`] returned.
    pub fn write_module(&mut self, submodule: &str, content: TokenStream) -> anyhow::Result<()> {
        fs::write(self.submodule_file(submodule), content.to_string())?;
        self.submodules.push(submodule.to_string());
        Ok(())
    }

    /// Write the definitions ([`EmitRef::emit_def`]) of some emittable struct to a module
    fn write_const_module(&mut self, submodule: &str, content: TokenStream) -> anyhow::Result<()> {
        if content.is_empty() {
            return Ok(());
        }
        let use_super = use_super();
        self.write_module(
            submodule,
            quote! {
                #use_super
                #content
            },
        )
    }

    fn write_grammar<'a>(&mut self, grammar: &impl WriteableGrammar<'a>) -> anyhow::Result<()> {
        self.write_const_module(
            "inst_class",
            grammar.inst_class.iter().map(InstClass::emit_def).collect(),
        )?;
        self.write_const_module(
            "operant_kinds",
            grammar
                .operand_kinds
                .iter()
                .map(OperandKind::emit_def)
                .collect(),
        )?;
        self.write_const_module(
            "inst",
            grammar.insts.iter().map(InstMeta::emit_def).collect(),
        )?;
        self.write_const_module("grammar", grammar.emit_def())?;
        Ok(())
    }

    /// Finish writing the grammar
    pub fn finish(self) -> anyhow::Result<()> {
        self.write_mod_rs()?;
        self.format_submodules()?;
        Ok(())
    }

    /// always write mod.rs and don't add to `submodules`
    fn write_mod_rs(&self) -> anyhow::Result<()> {
        fs::write(
            self.submodule_file("mod"),
            self.codegen_mod_rs()?.to_string(),
        )?;
        Ok(())
    }

    /// see [`use_super`]
    fn codegen_mod_rs(&self) -> anyhow::Result<TokenStream> {
        let core_import = if self.requires_core_import {
            quote!(
                pub use super::super::core::preamble;
            )
        } else {
            TokenStream::default()
        };

        let (mods, imports): (Vec<_>, Vec<_>) = self
            .submodules
            .iter()
            .map(|s| format_ident!("{}", s))
            .map(|s| (quote!(pub mod #s;), quote!(pub use super::#s::*;)))
            .unzip();

        Ok(quote! {
            #(#mods)*
            pub mod preamble {
                pub use crate::meta::*;
                #core_import
                #(#imports)*
            }
        })
    }

    fn format_submodules(&self) -> anyhow::Result<()> {
        let files = self
            .submodules
            .iter()
            .map(String::as_str)
            .chain(std::iter::once("mod"))
            .map(|s| self.submodule_file(s));
        let status = Command::new("rustfmt")
            .args(["--edition", "2024"])
            .args(files)
            .status()?;
        if !status.success() {
            // we assume if we fail to rustfmt, it'll also fail to compile later
            // so don't error, just emit a warning
            println!("cargo:warning=failed to rustfmt {:?}", self.folder);
        }
        Ok(())
    }
}
