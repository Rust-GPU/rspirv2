use crate::codegen::GrammarWriter;
use crate::parse::{Grammar, InstMeta, Operand, Quantifier};
use quote::quote;

pub const SMALLVEC_LEN: usize = 4;

pub fn write_inst(writer: &mut GrammarWriter, grammar: &Grammar) -> anyhow::Result<()> {
    let insts = grammar.insts.iter().map(|inst| {
        let struct_ident = InstMeta::type_ident(&inst.opname);
        let meta = InstMeta::const_ident(&inst.opname);

        let member_operands = inst.compute_operands();
        let member_decls = member_operands.iter().map(
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
        let members = member_operands
            .iter()
            .map(|&Operand { ref name, .. }| quote!(#name))
            .collect::<Vec<_>>();
        let members_non_last = &members[..members.len().saturating_sub(1)];
        let members_last = members.last().into_iter();

        quote! {
            #[derive(Clone, Debug, Eq, PartialEq, Hash)]
            pub struct #struct_ident {
                #(#member_decls),*
            }

            impl Inst for #struct_ident {
                const META: &InstMeta = &#meta;

                fn encode(&self, writer: &mut impl InstructionWriter) -> Result<(), EncodeError> {
                    let len = 0 #(+OperandEncoding::word_len(&self.#members))*;
                    writer.write_op(Self::META.opcode, len)?;
                    #(OperandEncoding::encode(&self.#members, &mut *writer)?;)*
                    Ok(())
                }

                fn decode(reader: &mut InstructionReader) -> Result<Self, DecodeError> {
                    reader.check_opcode(Self::META)?;
                    Ok(Self {
                        #(#members_non_last: OperandEncoding::decode(&mut *reader)?,)*
                        #(#members_last: OperandEncoding::decode_last(&mut *reader)?,)*
                    })
                }
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
