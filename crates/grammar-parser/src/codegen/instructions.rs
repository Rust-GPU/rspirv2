use crate::codegen::options::CodegenOptions;
use crate::codegen::{GrammarWriter, OPERAND_ID_RESULT, OPERAND_ID_RESULT_TYPE};
use crate::parse::{Grammar, InstMeta, Operand, OperandKind, Quantifier};
use quote::{format_ident, quote};

pub const SMALLVEC_LEN: usize = 4;

pub fn write_inst(writer: &mut GrammarWriter, grammar: &Grammar<'_>) -> anyhow::Result<()> {
    let insts = grammar.insts.iter().map(|inst| {
        let struct_ident = InstMeta::type_ident(&inst.opname);
        let meta = InstMeta::const_ident(&inst.opname);
        let member_operands = inst.compute_operands();
        let id_result = member_operands
            .iter()
            .find(|op| op.meta.kind == OPERAND_ID_RESULT);
        let id_result_type = member_operands
            .iter()
            .find(|op| op.meta.kind == OPERAND_ID_RESULT_TYPE);

        // struct decl
        let lifetime = if inst.has_lifetime() {
            quote!(<'a>)
        } else {
            quote!()
        };
        let member_decls = member_operands.iter().map(
            |&Operand {
                 meta,
                 ref name,
                 ref ty,
             }| {
                let lifetime = if OperandKind::has_lifetime(&meta.kind) {
                    &lifetime
                } else {
                    &quote!()
                };
                match meta.quantifier {
                    Quantifier::One => quote!(pub #name: #ty #lifetime),
                    Quantifier::ZeroOrOne => quote!(pub #name: Option<#ty #lifetime>),
                    Quantifier::ZeroOrMore => {
                        quote!(pub #name: SmallVec<[#ty #lifetime; #SMALLVEC_LEN]>)
                    }
                }
            },
        );

        // id_result(&mut self) -> &mut OptionIdResult
        let (maybe_id_result, id_result_mut_ref) = if let Some(id_result) = id_result {
            let name = &id_result.name;
            (quote!(OptionIdResult), quote!(&mut self.#name))
        } else {
            (quote!(()), quote!(make_mut_ref_unit()))
        };

        // encode decode
        let members = member_operands
            .iter()
            .map(|Operand { name, .. }| name)
            .collect::<Vec<_>>();
        let members_non_last = &members[..members.len().saturating_sub(1)];
        let members_last = members.last().into_iter();
        let reader = (!members.is_empty()).then(|| quote!(let mut op_reader = ));

        // disassembly
        let (dis_id_result_pat, dis_id_result_value) = if let Some(id_result) = id_result {
            let name = &id_result.name;
            ("{} = ", Some(quote!(, self.#name.dis(_ctx))))
        } else {
            ("", None)
        };
        // `name: Ident` is a good key to filter out the id_result, as names must be unique anyway
        let operands_without_result_id = member_operands
            .iter()
            .filter(|op| id_result.is_none_or(|id_result| id_result.name != op.name))
            .collect::<Vec<_>>();
        let dis_operands_value = operands_without_result_id
            .iter()
            .map(|op| {
                let name = &op.name;
                quote!(, self.#name.dis(_ctx))
            })
            .collect::<Vec<_>>();
        let pat = [dis_id_result_pat, inst.opname.as_ref()]
            .into_iter()
            .chain(
                operands_without_result_id
                    .iter()
                    .enumerate()
                    .map(|(i, op)| {
                        let last = i == operands_without_result_id.len() - 1;
                        let is_result_type = op.meta.kind == OPERAND_ID_RESULT_TYPE;
                        match (is_result_type, last) {
                            (true, false) => "{rspirv_space}{}{rspirv_space}",
                            (true, true) => "{rspirv_space}{}",
                            (false, _) => "{}",
                        }
                    }),
            )
            .collect::<String>();
        let rspirv_spaces_prefix = if id_result_type.is_some() {
            quote!(let rspirv_space = _ctx.rspirv_space();)
        } else {
            quote!()
        };

        quote! {
            #[derive(Clone, Debug, Eq, PartialEq, Hash)]
            pub struct #struct_ident #lifetime {
                #(#member_decls),*
            }

            impl<'a> Inst<'a> for #struct_ident #lifetime {
                const META: &'static InstMeta = &#meta;

                type MaybeIdResult = #maybe_id_result;

                fn id_result(&mut self) -> &mut Self::MaybeIdResult {
                    #id_result_mut_ref
                }
            }

            impl<'a> InstEncoding<'a> for #struct_ident #lifetime {
                fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
                    let len = 1 #(+OperandEncoding::word_len(&self.#members))*;
                    writer.write_op(Self::META.opcode, len)?;
                    #(OperandEncoding::encode(&self.#members, &mut *writer)?;)*
                    Ok(())
                }

                fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
                    #reader reader.check_opcode(Self::META)?;
                    Ok(Self {
                        #(#members_non_last: OperandEncoding::decode(&mut op_reader)?,)*
                        #(#members_last: OperandEncoding::decode_last(&mut op_reader)?,)*
                    })
                }

                fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
                    #rspirv_spaces_prefix
                    write!(f, #pat #dis_id_result_value #(#dis_operands_value)*)
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
            let lifetime = if inst.has_lifetime() {
                quote!(<'a>)
            } else {
                quote!()
            };
            (inst, type_ident, enum_ident, lifetime)
        })
        .collect::<Vec<_>>();
    let enum_variants = insts
        .iter()
        .map(|(_, type_ident, enum_ident, lifetime)| quote!(#enum_ident(#type_ident #lifetime),));
    let encode_match = insts.iter().map(
        |(_, _, enum_ident, _)| quote!(Self::#enum_ident(inst) => InstEncoding::encode(inst, writer),),
    );
    let decode_match = insts.iter().map(|(inst, type_ident, enum_ident, lifetime)| {
        let opcode = inst.opcode;
        quote!(#opcode => Self::#enum_ident(<#type_ident #lifetime as InstEncoding>::decode(reader)?),)
    });
    let dis_match = insts.iter().map(
        |(_, _, enum_ident, _)| quote!(Self::#enum_ident(inst) => InstEncoding::dis_fmt(inst, f, ctx),),
    );
    let from_impls = insts.iter().map(|(_, type_ident, enum_ident, lifetime)| {
        quote! {
            impl From<#type_ident #lifetime> for #name #lifetime {
                fn from(inst: #type_ident #lifetime) -> Self {
                    Self::#enum_ident(inst)
                }
            }
        }
    });
    writer.write_module(
        "inst_set",
        quote! {
            #[derive(Clone, Debug, Eq, PartialEq, Hash)]
            pub enum #name<'a> {
                #(#enum_variants)*
            }

            impl InstEncoding<'a> for #name<'a> {
                fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
                    match self {
                        #(#encode_match)*
                    }
                }

                fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
                    let opcode = reader.opcode();
                    Ok(match opcode {
                        #(#decode_match)*
                        _ => return Err(DecodeError::UnknownOpCode { opcode }),
                    })
                }

                fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
                    match self {
                        #(#dis_match)*
                    }
                }
            }

            #(#from_impls)*
        },
    )
}
