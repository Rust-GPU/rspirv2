use crate::codegen::GrammarWriter;
use crate::parse::{Category, Enumerant, Grammar, OperandKind};
use proc_macro2::TokenStream;
use quote::quote;

pub fn write_operands(writer: &mut GrammarWriter, grammar: &Grammar) -> anyhow::Result<()> {
    let operands = grammar.operand_kinds.iter().map(|o| match &o.category {
        // `RefId` and Literals are imported
        Category::Id | Category::Literal => quote!(),
        Category::BitEnum { .. } => quote!(),
        Category::Composite { .. } => quote!(),
        Category::ValueEnum { enumerants } => {
            let c_like = enumerants.iter().all(|e| e.parameters.is_empty());
            if c_like {
                emit_c_like_enum(o, enumerants)
            } else {
                quote!()
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

fn make_doc(doc: &str) -> TokenStream {
    if doc.is_empty() {
        quote!()
    } else {
        quote!(#[doc = #doc])
    }
}
