use crate::parse::serde_helper::num_or_hex;
use crate::parse::{InstClass, InstMeta, OperandKind};
use std::ops::Deref;

/// See [`spirv_grammar::meta::Grammar`]
#[derive(Clone, Debug, Default, serde::Deserialize)]
pub struct Grammar<'a> {
    // ignore the copyright
    // #[serde(borrow, default)]
    // pub copyright: Vec<&'a str>,
    #[serde(borrow, default, rename = "instructions")]
    pub insts: Vec<InstMeta<'a>>,
    #[serde(borrow, default)]
    pub operand_kinds: Vec<OperandKind<'a>>,
    #[serde(borrow, default, rename = "instruction_printing_class")]
    pub inst_class: Vec<InstClass<'a>>,
}

/// See [`spirv_grammar::meta::CoreGrammar`]
#[derive(Clone, Debug, serde::Deserialize)]
pub struct CoreGrammar<'a> {
    #[serde(borrow, flatten)]
    pub grammar: Grammar<'a>,
    #[serde(deserialize_with = "num_or_hex")]
    pub magic_number: u32,
    pub major_version: u8,
    pub minor_version: u8,
    pub revision: u32,
}

impl<'a> Deref for CoreGrammar<'a> {
    type Target = Grammar<'a>;

    fn deref(&self) -> &Self::Target {
        &self.grammar
    }
}

/// See [`spirv_grammar::meta::ExtInstSetGrammar`]
#[derive(Clone, Debug, serde::Deserialize)]
pub struct ExtInstSetGrammar<'a> {
    #[serde(borrow, flatten)]
    pub grammar: Grammar<'a>,
    pub version: Option<u32>,
    pub revision: Option<u32>,
}

impl<'a> Deref for ExtInstSetGrammar<'a> {
    type Target = Grammar<'a>;

    fn deref(&self) -> &Self::Target {
        &self.grammar
    }
}

/// A kind of grammar, without any lifetimes, so it can be used as a marker. Use with [`CoreGrammar`] or
/// [`ExtInstSetGrammar`] and a `'static` lifetime. When deserializing, use [`Self::Grammar`] to get the proper
/// lifetime.
pub trait GrammarKind {
    type Grammar<'a>: Deref<Target = Grammar<'a>> + serde::Deserialize<'a>;
}
impl GrammarKind for CoreGrammar<'_> {
    type Grammar<'a> = CoreGrammar<'a>;
}
impl GrammarKind for ExtInstSetGrammar<'_> {
    type Grammar<'a> = ExtInstSetGrammar<'a>;
}

#[cfg(feature = "codegen")]
mod codegen {
    use super::*;
    use crate::codegen::EmitRef;
    use proc_macro2::{Ident, TokenStream};
    use quote::{ToTokens, format_ident, quote};

    impl Grammar<'_> {
        pub fn all_insts_ident() -> Ident {
            format_ident!("ALL_INSTS")
        }

        pub fn all_operand_kinds_ident() -> Ident {
            format_ident!("ALL_OPERAND_KINDS")
        }

        pub fn all_inst_classes_ident() -> Ident {
            format_ident!("ALL_INST_CLASSES")
        }

        pub fn emit_def(&self) -> TokenStream {
            let all_insts_ident = Self::all_insts_ident();
            let all_insts = self.insts.iter().map(|i| i.emit_ref());
            let all_operand_kinds_ident = Self::all_operand_kinds_ident();
            let all_operand_kinds = self.operand_kinds.iter().map(|i| i.emit_ref());
            let all_inst_classes_ident = Self::all_inst_classes_ident();
            let all_inst_classes = self.inst_class.iter().map(|i| i.emit_ref());
            quote! {
                const #all_insts_ident: &[&InstMeta] = &[#(#all_insts),*];
                const #all_operand_kinds_ident: &[&OperandKind] = &[#(#all_operand_kinds),*];
                const #all_inst_classes_ident: &[&InstClass] = &[#(#all_inst_classes),*];
            }
        }
    }

    impl EmitRef for Grammar<'_> {
        fn emit_ref(&self) -> TokenStream {
            let all_insts_ident = Self::all_insts_ident();
            let all_operand_kinds_ident = Self::all_operand_kinds_ident();
            let all_inst_classes_ident = Self::all_inst_classes_ident();
            quote! {
                Grammar {
                    insts: #all_insts_ident,
                    operand_kinds: #all_operand_kinds_ident,
                    inst_class: #all_inst_classes_ident,
                }
            }
        }
    }

    impl EmitRef for CoreGrammar<'_> {
        fn emit_ref(&self) -> TokenStream {
            format_ident!("GRAMMAR_CORE").into_token_stream()
        }
    }

    impl CoreGrammar<'_> {
        pub fn emit_def(&self) -> TokenStream {
            let ident = self.emit_ref();
            let grammar = self.grammar.emit_ref();
            let magic_number = self.magic_number.emit_ref();
            let major_version = self.major_version.emit_ref();
            let minor_version = self.minor_version.emit_ref();
            let revision = self.revision.emit_ref();
            quote! {
                pub const #ident: CoreGrammar = CoreGrammar {
                    grammar: #grammar,
                    magic_number: #magic_number,
                    major_version: #major_version,
                    minor_version: #minor_version,
                    revision: #revision,
                };
            }
        }
    }

    impl EmitRef for ExtInstSetGrammar<'_> {
        fn emit_ref(&self) -> TokenStream {
            format_ident!("GRAMMAR_EXTINST").into_token_stream()
        }
    }

    impl ExtInstSetGrammar<'_> {
        pub fn emit_def(&self) -> TokenStream {
            let ident = self.emit_ref();
            let grammar = self.grammar.emit_ref();
            let version = self.version.emit_ref();
            let revision = self.revision.emit_ref();
            quote! {
                pub const #ident: ExtInstSetGrammar = ExtInstSetGrammar {
                    grammar: #grammar,
                    version: #version,
                    revision: #revision,
                };
            }
        }
    }
}
