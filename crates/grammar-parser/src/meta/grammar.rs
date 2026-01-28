#[cfg(feature = "serde")]
use crate::meta::serde_helper::num_or_hex;
use crate::meta::{InstructionMeta, InstructionPrintingClass, OperandKind};
use std::borrow::Cow;
use std::ops::Deref;

/// A SPIR-V Grammar of any kind. There are only minor differences between the core SPIR-V specification and an
/// extended instruction set, such as versioning.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct Grammar<'a> {
    #[cfg_attr(feature = "serde", serde(borrow, default))]
    pub copyright: Cow<'a, [Cow<'a, str>]>,
    /// all [`Instructions`] defined by the grammar
    ///
    /// [`Instructions`]: [`InstructionMeta`]
    #[cfg_attr(feature = "serde", serde(borrow, default))]
    pub instructions: Cow<'a, [InstructionMeta<'a>]>,
    /// all [`OperandKind`]s defined by the grammar
    #[cfg_attr(feature = "serde", serde(borrow, default))]
    pub operand_kinds: Cow<'a, [OperandKind<'a>]>,
    #[cfg_attr(feature = "serde", serde(borrow, default))]
    pub instruction_printing_class: Cow<'a, [InstructionPrintingClass<'a>]>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct CoreGrammar<'a> {
    /// The Grammar
    #[cfg_attr(feature = "serde", serde(borrow, flatten))]
    pub grammar: Grammar<'a>,
    /// The SPIR-V magic number
    #[cfg_attr(feature = "serde", serde(deserialize_with = "num_or_hex"))]
    pub magic_number: u32,
    /// The major version, only used in the core spec
    pub major_version: u8,
    /// The major version, only used in the core spec
    pub minor_version: u8,
    /// The revision, used in both spec kinds
    pub revision: u32,
}

impl<'a> Deref for CoreGrammar<'a> {
    type Target = Grammar<'a>;

    fn deref(&self) -> &Self::Target {
        &self.grammar
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct ExtInstSetGrammar<'a> {
    /// The Grammar
    #[cfg_attr(feature = "serde", serde(borrow, flatten))]
    pub grammar: Grammar<'a>,
    /// The version, only used in extended instruction sets
    pub version: Option<u32>,
    /// The revision, used in both spec kinds
    pub revision: Option<u32>,
}

impl<'a> Deref for ExtInstSetGrammar<'a> {
    type Target = Grammar<'a>;

    fn deref(&self) -> &Self::Target {
        &self.grammar
    }
}

/// A kind of grammar, without any lifetimes, so it can be used as a marker. Use with [`CoreGrammar`] or
/// [`ExtInstSetGrammar`] and a `'static` lifetime. When deserializing, use [`Self::Grammar`] to setup the lifetime
/// properly.
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
    use crate::codegen::Emit;
    use proc_macro2::TokenStream;
    use quote::{ToTokens, format_ident, quote};

    impl Emit for Grammar<'_> {
        fn emit_ref(&self) -> TokenStream {
            let copyright = self.copyright.emit_ref();
            let instructions = self.instructions.emit_ref();
            let operand_kinds = self.operand_kinds.emit_ref();
            let instruction_printing_class = self.instruction_printing_class.emit_ref();
            quote! {
                Grammar {
                    copyright: #copyright,
                    instructions: #instructions,
                    operand_kinds: #operand_kinds,
                    instruction_printing_class: #instruction_printing_class,
                }
            }
        }

        fn emit_def(&self) -> TokenStream {
            TokenStream::new()
        }
    }

    impl Emit for CoreGrammar<'_> {
        fn emit_ref(&self) -> TokenStream {
            format_ident!("GRAMMAR_CORE").into_token_stream()
        }

        fn emit_def(&self) -> TokenStream {
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

    impl Emit for ExtInstSetGrammar<'_> {
        fn emit_ref(&self) -> TokenStream {
            format_ident!("GRAMMAR_EXTINST").into_token_stream()
        }

        fn emit_def(&self) -> TokenStream {
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
