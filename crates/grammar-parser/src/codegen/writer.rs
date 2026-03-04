use crate::codegen::options::CodegenOptions;
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

pub struct GrammarWriter {
    folder: PathBuf,
    submodules: Vec<String>,
}

impl GrammarWriter {
    pub fn new(folder: PathBuf) -> anyhow::Result<Self> {
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
    pub fn write_module_str(&mut self, submodule: &str, content: &str) -> anyhow::Result<()> {
        fs::write(self.submodule_file(submodule), content)?;
        self.submodules.push(submodule.to_string());
        Ok(())
    }

    /// Write some `content` to a module file called `submodule`
    pub fn write_module(&mut self, submodule: &str, content: TokenStream) -> anyhow::Result<()> {
        if content.is_empty() {
            return Ok(());
        }
        let use_super = use_super();
        let content = quote! {
            #use_super
            #content
        };
        let content_str;
        #[cfg(feature = "prettyplease")]
        {
            content_str = prettyplease::unparse(&syn::parse2(content)?);
        }
        #[cfg(not(feature = "prettyplease"))]
        {
            content_str = content.to_string();
        }
        self.write_module_str(submodule, &content_str)
    }

    /// Finish writing the grammar
    pub fn finish(self, mod_options: &CodegenOptions) -> anyhow::Result<()> {
        self.write_mod_rs(mod_options)?;
        self.format_submodules()?;
        Ok(())
    }

    /// always write mod.rs and don't add to `submodules`
    fn write_mod_rs(&self, mod_options: &CodegenOptions) -> anyhow::Result<()> {
        fs::write(
            self.submodule_file("mod"),
            self.codegen_mod_rs(mod_options)?.to_string(),
        )?;
        Ok(())
    }

    /// see [`use_super`]
    fn codegen_mod_rs(&self, mod_options: &CodegenOptions) -> anyhow::Result<TokenStream> {
        let (mods, imports): (Vec<_>, Vec<_>) = self
            .submodules
            .iter()
            .map(|s| format_ident!("{}", s))
            .map(|s| (quote!(pub mod #s;), quote!(pub use super::#s::*;)))
            .unzip();
        let CodegenOptions {
            mod_attr,
            mod_extra,
            preamble,
            ..
        } = mod_options;
        Ok(quote! {
            #mod_attr
            #(#mods)*
            #mod_extra
            pub mod preamble {
                #preamble
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
