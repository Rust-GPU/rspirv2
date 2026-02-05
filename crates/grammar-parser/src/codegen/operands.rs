use crate::codegen::GrammarWriter;
use crate::parse::{Category, Enumerant, Grammar, OperandKind, Quantifier};
use proc_macro2::TokenStream;
use quote::quote;
use std::borrow::Cow;

pub fn write_operands(writer: &mut GrammarWriter, grammar: &Grammar) -> anyhow::Result<()> {
    let operands = grammar.operand_kinds.iter().map(|o| match &o.category {
        // `RefId` and Literals are imported
        Category::Id | Category::Literal => quote!(),
        Category::BitEnum { enumerants } => emit_bitflags_enum(o, enumerants),
        Category::Composite { bases } => emit_composite(o, bases),
        Category::ValueEnum { enumerants } => {
            let c_like = enumerants.iter().all(|e| e.parameters.is_empty());
            if c_like {
                emit_c_like_enum(o, enumerants)
            } else {
                emit_rust_like_enum(o, enumerants)
            }
        }
    });
    writer.write_const_module(
        "operands",
        quote! {
            #(#operands)*
        },
    )
}

fn emit_rust_like_enum(operand_kind: &OperandKind, enumerants: &[Enumerant]) -> TokenStream {
    let name = OperandKind::type_ident(&operand_kind.name);
    let doc = make_doc(&operand_kind.doc);

    let variants = enumerants.iter().map(|e| {
        let enumerant_preamble = emit_enumerant_preamble(e);
        let symbol = Enumerant::variant_ident(&e.symbol);
        let params = if !e.parameters.is_empty() {
            let params = e.parameters.iter().map(|p| {
                let docs = p
                    .name
                    .as_ref()
                    .map(|name| make_doc(name))
                    .unwrap_or(quote!());
                let ty = OperandKind::type_ident(&p.kind);
                match p.quantifier {
                    Quantifier::One => quote!(#docs #ty),
                    Quantifier::ZeroOrOne => quote!(#docs Option<#ty>),
                    Quantifier::ZeroOrMore => quote!(#docs Vec<#ty>),
                }
            });
            quote!((#(#params),*))
        } else {
            quote!()
        };
        quote! {
            #enumerant_preamble
            #symbol #params
        }
    });

    quote! {
        #doc
        #[derive(Clone, Debug, Eq, PartialEq, Hash)]
        pub enum #name {
            #(#variants),*
        }
    }
}

fn emit_c_like_enum(operand_kind: &OperandKind, enumerants: &[Enumerant]) -> TokenStream {
    let name = OperandKind::type_ident(&operand_kind.name);
    let doc = make_doc(&operand_kind.doc);

    let variants = enumerants.iter().map(|e| {
        let enumerant_preamble = emit_enumerant_preamble(e);
        let symbol = Enumerant::variant_ident(&e.symbol);
        let value = e.value;
        quote! {
            #enumerant_preamble
            #symbol = #value
        }
    });

    quote! {
        #doc
        #[repr(u32)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub enum #name {
            #(#variants),*
        }
    }
}

fn emit_enumerant_preamble(e: &Enumerant) -> TokenStream {
    let docs_since = e
        .version
        .as_ref()
        .filter(|v| !matches!(v.as_ref(), "1.0" | "None"))
        .map(|v| make_doc(&format!("Since SPIR-V {v}")));
    let docs_deprecation = e
        .last_version
        .as_ref()
        .map(|v| make_doc(&format!("Deprecated in SPIR-V {v}")));
    let deprecation = e.last_version.is_some().then(|| quote!(#[deprecated]));
    quote! {
        #docs_since
        #docs_deprecation
        #deprecation
    }
}

fn emit_bitflags_enum(operand_kind: &OperandKind, enumerants: &[Enumerant]) -> TokenStream {
    let name = OperandKind::type_ident(&operand_kind.name);
    let doc = make_doc(&operand_kind.doc);

    let variants = enumerants.iter().map(|e| {
        let enumerant_preamble = emit_enumerant_preamble(e);
        let symbol = Enumerant::variant_ident(&e.symbol);
        let value = e.value;
        quote! {
            #enumerant_preamble
            const #symbol = #value;
        }
    });

    quote! {
        bitflags! {
            #doc
            #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
            pub struct #name: u32 {
                #(#variants)*
            }
        }
    }
}

fn emit_composite(operand_kind: &OperandKind, bases: &[Cow<str>]) -> TokenStream {
    let name = OperandKind::type_ident(&operand_kind.name);
    let doc = make_doc(&operand_kind.doc);
    let member_tys = bases.iter().map(|name| OperandKind::type_ident(name));
    quote! {
        #doc
        #[derive(Clone, Debug, Eq, PartialEq, Hash)]
        pub struct #name(#(#member_tys),*);
    }
}

fn make_doc(doc: &str) -> TokenStream {
    if doc.is_empty() {
        quote!()
    } else {
        quote!(#[doc = #doc])
    }
}
