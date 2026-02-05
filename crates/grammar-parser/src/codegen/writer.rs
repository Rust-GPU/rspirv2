use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// a use statement that imports symbols from other files
pub fn use_super() -> TokenStream {
    quote! {
        use super::preamble::*;
    }
}

#[derive(Clone, Debug, Default)]
pub struct ModOptions {
    pub mod_attr: TokenStream,
    pub preamble: TokenStream,
}

pub struct GrammarWriter {
    folder: PathBuf,
    submodules: Vec<String>,
}

impl GrammarWriter {
    pub fn new<'a>(folder: PathBuf) -> anyhow::Result<Self> {
        fs::create_dir_all(&folder)?;
        Ok(Self {
            folder,
            submodules: Vec::new(),
        })
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
    pub fn write_const_module(
        &mut self,
        submodule: &str,
        content: TokenStream,
    ) -> anyhow::Result<()> {
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

    /// Finish writing the grammar
    pub fn finish(self, mod_options: ModOptions) -> anyhow::Result<()> {
        self.write_mod_rs(mod_options)?;
        self.format_submodules()?;
        Ok(())
    }

    /// always write mod.rs and don't add to `submodules`
    fn write_mod_rs(&self, mod_options: ModOptions) -> anyhow::Result<()> {
        fs::write(
            self.submodule_file("mod"),
            self.codegen_mod_rs(mod_options)?.to_string(),
        )?;
        Ok(())
    }

    /// see [`use_super`]
    fn codegen_mod_rs(&self, mod_options: ModOptions) -> anyhow::Result<TokenStream> {
        let (mods, imports): (Vec<_>, Vec<_>) = self
            .submodules
            .iter()
            .map(|s| format_ident!("{}", s))
            .map(|s| (quote!(pub mod #s;), quote!(pub use super::#s::*;)))
            .unzip();
        let ModOptions {
            mod_attr: lints_extra,
            preamble: preamble_extra,
        } = mod_options;
        Ok(quote! {
            #lints_extra
            #(#mods)*
            pub mod preamble {
                #preamble_extra
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
