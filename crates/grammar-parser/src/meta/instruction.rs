#[cfg(feature = "serde")]
use crate::meta::serde_helper::{operand_kind_from_str, printing_class_from_str};
use crate::meta::{Capability, Extension, OperandKind};
use std::borrow::Cow;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct InstructionMeta<'a> {
    /// The name of the instruction
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub opname: Cow<'a, str>,
    /// The [`Class`] of this instruction, more informational than anything
    #[cfg_attr(
        feature = "serde",
        serde(default, deserialize_with = "printing_class_from_str")
    )]
    pub class: Option<Cow<'a, InstructionPrintingClass<'a>>>,
    /// The u16 opcode for this instruction
    pub opcode: u16,
    /// The operands of this instruction
    #[cfg_attr(feature = "serde", serde(borrow, default))]
    pub operands: Cow<'a, [OperandMeta<'a>]>,
    /// required capabilities
    #[cfg_attr(feature = "serde", serde(borrow, default))]
    pub capabilities: Cow<'a, [Capability<'a>]>,
    /// required extensions
    #[cfg_attr(feature = "serde", serde(borrow, default))]
    pub extensions: Cow<'a, [Extension<'a>]>,
    /// The SPIR-V version this instruction was introduced in
    #[cfg_attr(feature = "serde", serde(borrow, default))]
    pub version: Option<Cow<'a, str>>,
    /// The last SPIR-V version this instruction is valid in
    #[cfg_attr(feature = "serde", serde(borrow, default, rename = "lastVersion"))]
    pub last_version: Option<Cow<'a, str>>,
    /// Aliases for this instruction
    #[cfg_attr(feature = "serde", serde(borrow, default))]
    pub aliases: Cow<'a, [Cow<'a, str>]>,
    /// Whether this instruction is provisional
    #[cfg_attr(feature = "serde", serde(default))]
    pub provisional: bool,
}

/// An operand of an instruction
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct OperandMeta<'a> {
    /// The kind of operand, referencing the [`OperandKind`]s defined in [`Grammar`]
    #[cfg_attr(feature = "serde", serde(deserialize_with = "operand_kind_from_str"))]
    pub kind: Cow<'a, OperandKind<'a>>,
    /// Operand name
    #[cfg_attr(feature = "serde", serde(borrow, default))]
    pub name: Cow<'a, str>,
    /// The repetition [`Quantifier`]
    #[cfg_attr(feature = "serde", serde(default))]
    pub quantifier: Quantifier,
}

/// How many times to repeat something?
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub enum Quantifier {
    #[default]
    #[cfg_attr(feature = "serde", serde(rename = ""))]
    One,
    #[cfg_attr(feature = "serde", serde(rename = "?"))]
    ZeroOrOne,
    #[cfg_attr(feature = "serde", serde(rename = "*"))]
    ZeroOrMore,
}

/// Informational metadata about the class of instruction
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct InstructionPrintingClass<'a> {
    /// the tag, or primary key
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub tag: Cow<'a, str>,
    /// a human name for the class
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub heading: Option<Cow<'a, str>>,
}

#[cfg(feature = "codegen")]
mod codegen {
    use super::*;
    use crate::codegen::{Emit, make_const_ident};
    use proc_macro2::{Ident, TokenStream};
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

    impl InstructionMeta<'_> {
        pub fn const_ident(&self) -> Ident {
            make_const_ident("INSTRUCTION_", &self.opname)
        }
    }

    impl Emit for InstructionMeta<'_> {
        fn emit_ref(&self) -> TokenStream {
            let ident = self.const_ident();
            quote!(&#ident)
        }

        fn emit_def(&self) -> TokenStream {
            let ident = self.const_ident();
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
                pub const #ident: InstructionMeta = InstructionMeta {
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

    impl InstructionPrintingClass<'_> {
        pub fn const_ident(&self) -> Ident {
            make_const_ident("PRINTING_CLASS_", &self.tag)
        }
    }

    impl Emit for InstructionPrintingClass<'_> {
        fn emit_ref(&self) -> TokenStream {
            let ident = self.const_ident();
            quote!(&#ident)
        }

        fn emit_def(&self) -> TokenStream {
            let ident = self.const_ident();
            let tag = self.tag.emit_ref();
            let heading = self.heading.emit_ref();
            quote! {
                pub const #ident: InstructionPrintingClass = InstructionPrintingClass {
                    tag: #tag,
                    heading: #heading,
                };
            }
        }
    }
}
