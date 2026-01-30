use crate::parse::{Capability, Extension};
use smallvec::SmallVec;
use std::borrow::Cow;

/// See [`spirv_grammar::meta::InstMeta`]
#[derive(Clone, Debug, serde::Deserialize)]
pub struct InstMeta<'a> {
    #[serde(borrow)]
    pub opname: Cow<'a, str>,
    /// The name of the [`InstClass`], references `Grammar.instruction_printing_class`
    #[serde(borrow)]
    pub class: Option<Cow<'a, str>>,
    pub opcode: u16,
    #[serde(borrow, default)]
    pub operands: SmallVec<[OperandMeta<'a>; 4]>,
    #[serde(borrow, default)]
    pub capabilities: SmallVec<[Capability<'a>; 2]>,
    #[serde(borrow, default)]
    pub extensions: SmallVec<[Extension<'a>; 1]>,
    #[serde(borrow, default)]
    pub version: Option<Cow<'a, str>>,
    #[serde(borrow, default, rename = "lastVersion")]
    pub last_version: Option<Cow<'a, str>>,
    #[serde(borrow, default)]
    pub aliases: SmallVec<[Cow<'a, str>; 1]>,
    #[serde(default)]
    pub provisional: bool,
}

/// See [`spirv_grammar::meta::OperandMeta`]
#[derive(Clone, Debug, serde::Deserialize)]
pub struct OperandMeta<'a> {
    /// The name of the [`OperandKind`], references `Grammar.operand_kinds`
    #[serde(borrow)]
    pub kind: Cow<'a, str>,
    #[serde(borrow, default)]
    pub name: Option<Cow<'a, str>>,
    #[serde(default)]
    pub quantifier: Quantifier,
}

/// How many times to repeat something?
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Ord, PartialOrd, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Quantifier {
    #[default]
    #[serde(rename = "")]
    One,
    #[serde(rename = "?")]
    ZeroOrOne,
    #[serde(rename = "*")]
    ZeroOrMore,
}

/// See [`spirv_grammar::meta::InstClass`]
#[derive(Clone, Debug, serde::Deserialize)]
pub struct InstClass<'a> {
    #[serde(borrow)]
    pub tag: Cow<'a, str>,
    #[serde(borrow)]
    pub heading: Option<Cow<'a, str>>,
}

#[cfg(feature = "codegen")]
mod codegen {
    use super::*;
    use crate::codegen::{Emit, make_const_ident};
    use proc_macro2::TokenStream;
    use quote::quote;

    impl Emit for Quantifier {
        fn emit_ref(&self) -> TokenStream {
            match self {
                Quantifier::One => quote!(Quantifier::One),
                Quantifier::ZeroOrOne => quote!(Quantifier::ZeroOrOne),
                Quantifier::ZeroOrMore => quote!(Quantifier::ZeroOrMore),
            }
        }

        fn emit_def(&self) -> TokenStream {
            TokenStream::default()
        }
    }

    impl Emit for InstMeta<'_> {
        fn emit_ref(&self) -> TokenStream {
            make_const_ident("INSTRUCTION_", &self.opname)
        }

        fn emit_def(&self) -> TokenStream {
            let ident = self.emit_ref();
            let opname = self.opname.emit_ref();
            let class = self.class.emit_ref();
            let opcode = self.opcode.emit_ref();
            let operands = self.operands.emit_ref();
            let capabilities = self.capabilities.emit_ref();
            let extensions = self.extensions.emit_ref();
            let version = self.version.emit_ref();
            let last_version = self.last_version.emit_ref();
            let aliases = self.aliases.emit_ref();
            let provisional = self.provisional.emit_ref();

            quote! {
                pub const #ident: InstMeta = InstMeta {
                    opname: #opname,
                    class: #class,
                    opcode: #opcode,
                    operands: #operands,
                    capabilities: #capabilities,
                    extensions: #extensions,
                    version: #version,
                    last_version: #last_version,
                    aliases: #aliases,
                    provisional: #provisional,
                };
            }
        }
    }

    impl Emit for OperandMeta<'_> {
        fn emit_ref(&self) -> TokenStream {
            let kind = self.kind.emit_ref();
            let name = self.name.emit_ref();
            let quantifier = self.quantifier.emit_ref();
            quote! {
                OperandMeta {
                    kind: #kind,
                    name: #name,
                    quantifier: #quantifier,
                }
            }
        }

        fn emit_def(&self) -> TokenStream {
            TokenStream::new()
        }
    }

    impl Emit for InstClass<'_> {
        fn emit_ref(&self) -> TokenStream {
            make_const_ident("PRINTING_CLASS_", &self.tag)
        }

        fn emit_def(&self) -> TokenStream {
            let ident = self.emit_ref();
            let tag = self.tag.emit_ref();
            let heading = self.heading.emit_ref();
            quote! {
                pub const #ident: InstClass = InstClass {
                    tag: #tag,
                    heading: #heading,
                };
            }
        }
    }
}
