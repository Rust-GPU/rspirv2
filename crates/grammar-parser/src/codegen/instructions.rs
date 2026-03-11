use crate::codegen::options::CodegenOptions;
use crate::codegen::{GrammarWriter, OPERAND_ID_RESULT};
use crate::parse::{Grammar, InstMeta, Operand, Quantifier};
use quote::{format_ident, quote};

pub const SMALLVEC_LEN: usize = 4;

pub fn write_inst(writer: &mut GrammarWriter, grammar: &Grammar<'_>) -> anyhow::Result<()> {
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

        let (maybe_id_result, id_result_ref) = if let Some(op_id_result) = member_operands
            .iter()
            .find(|op| op.meta.kind == OPERAND_ID_RESULT)
        {
            let name = &op_id_result.name;
            (quote!(OptionIdResult), quote!(&mut self.#name))
        } else {
            (quote!(()), quote!(make_mut_ref_unit()))
        };

        let members = member_operands
            .iter()
            .map(|Operand { name, .. }| name)
            .collect::<Vec<_>>();
        let members_non_last = &members[..members.len().saturating_sub(1)];
        let members_last = members.last().into_iter();
        let reader = (!members.is_empty()).then(|| quote!(let mut op_reader = ));

        quote! {
            #[derive(Clone, Debug, Eq, PartialEq, Hash)]
            pub struct #struct_ident {
                #(#member_decls),*
            }

            impl Inst for #struct_ident {
                const META: &InstMeta = &#meta;

                type MaybeIdResult = #maybe_id_result;

                fn id_result(&mut self) -> &mut Self::MaybeIdResult {
                    #id_result_ref
                }
            }

            impl InstEncoding for #struct_ident {
                fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
                    let len = 1 #(+OperandEncoding::word_len(&self.#members))*;
                    writer.write_op(Self::META.opcode, len)?;
                    #(OperandEncoding::encode(&self.#members, &mut *writer)?;)*
                    Ok(())
                }

                fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
                    #reader reader.check_opcode(Self::META)?;
                    Ok(Self {
                        #(#members_non_last: OperandEncoding::decode(&mut op_reader)?,)*
                        #(#members_last: OperandEncoding::decode_last(&mut op_reader)?,)*
                    })
                }
            }
        }
    });
    writer.write_module(
        "inst",
        quote! {
            #(#insts)*
        },
    )
}

pub fn write_inst_enum(
    writer: &mut GrammarWriter,
    grammar: &Grammar<'_>,
    opt: &CodegenOptions<'_>,
) -> anyhow::Result<()> {
    let name = format_ident!("{}InstSet", opt.name_suffix_type);
    let insts = grammar
        .insts
        .iter()
        .map(|inst| {
            let type_ident = InstMeta::type_ident(&inst.opname);
            let enum_ident = InstMeta::enum_ident(&inst.opname);
            (inst, type_ident, enum_ident)
        })
        .collect::<Vec<_>>();
    let enum_variants = insts
        .iter()
        .map(|(_, type_ident, enum_ident)| quote!(#enum_ident(#type_ident),));
    let encode_match = insts.iter().map(
        |(_, _, enum_ident)| quote!(Self::#enum_ident(inst) => InstEncoding::encode(inst, writer),),
    );
    let decode_match = insts.iter().map(|(inst, type_ident, enum_ident)| {
        let opcode = inst.opcode;
        quote!(#opcode => Self::#enum_ident(<#type_ident as InstEncoding>::decode(reader)?),)
    });
    let from_impls = insts.iter().map(|(_, type_ident, enum_ident)| {
        quote! {
            impl From<#type_ident> for #name {
                fn from(inst: #type_ident) -> Self {
                    Self::#enum_ident(inst)
                }
            }
        }
    });
    writer.write_module(
        "inst_set",
        quote! {
            #[derive(Clone, Debug, Eq, PartialEq, Hash)]
            pub enum #name {
                #(#enum_variants)*
            }

            impl InstEncoding for #name {
                fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
                    match self {
                        #(#encode_match)*
                    }
                }

                fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
                    let opcode = reader.opcode();
                    Ok(match opcode {
                        #(#decode_match)*
                        _ => return Err(DecodeError::UnknownOpCode { opcode }),
                    })
                }
            }

            #(#from_impls)*
        },
    )
}
