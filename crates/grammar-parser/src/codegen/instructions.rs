use crate::codegen::GrammarWriter;
use crate::parse::{Grammar, InstMeta, Operand, Quantifier};
use quote::quote;

pub const SMALLVEC_LEN: usize = 4;

pub fn write_inst(writer: &mut GrammarWriter, grammar: &Grammar) -> anyhow::Result<()> {
    let insts = grammar.insts.iter().map(|inst| {
        let struct_ident = InstMeta::type_ident(&inst.opname);
        let members = inst.compute_operands();
        let member_decls = members.iter().map(
            |&Operand {
                 meta,
                 ref name,
                 ref ty,
             }| match meta.quantifier {
                Quantifier::One => quote!(pub #name: #ty),
                Quantifier::ZeroOrOne => quote!(pub #name: Option<#ty>),
                Quantifier::ZeroOrMore => quote!(pub #name: SmallVec<[#ty; #SMALLVEC_LEN]>),
            },
        );
        quote! {
            #[derive(Clone, Debug, Eq, PartialEq, Hash)]
            pub struct #struct_ident {
                #(#member_decls),*
            }
        }
    });
    writer.write_const_module(
        "inst",
        quote! {
            #(#insts)*
        },
    )
}
