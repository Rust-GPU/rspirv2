use crate::codegen::{CodegenOptions, GrammarWriter};
use crate::parse::{Grammar, InstMeta};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

pub fn write_inst_enum(
    writer: &mut GrammarWriter,
    grammar: &Grammar<'_>,
    opt: &CodegenOptions<'_>,
) -> anyhow::Result<()> {
    writer.write_module(
        "inst_set",
        quote_inst_set(
            &prep_insts(grammar.insts.iter()),
            &format_ident!("{}InstSet", opt.name_suffix_type),
        ),
    )
}

pub fn write_op_spec_op_inst_enum(
    writer: &mut GrammarWriter,
    grammar: &Grammar<'_>,
    opt: &CodegenOptions<'_>,
) -> anyhow::Result<()> {
    let name = format_ident!("{}OpSpecOpInstSet", opt.name_suffix_type);
    let insts = prep_insts(grammar.insts.iter().filter(|i| i.op_spec_op));
    let inst_set = quote_inst_set(&insts, &name);

    let op_spec_op_inst_set = {
        let encode_match = insts.iter().map(
            |(_, _, enum_ident)| quote!(Self::#enum_ident(inst) => OpSpecOpInst::op_spec_op_encode(inst, writer),),
        );
        let decode_match = insts.iter().map(|(inst, type_ident, enum_ident)| {
            let opcode = inst.opcode;
            quote!(#opcode => Self::#enum_ident(<#type_ident as OpSpecOpInst>::op_spec_op_decode(reader)?),)
        });
        let dis_match = insts.iter().map(
            |(_, _, enum_ident)| quote!(Self::#enum_ident(inst) => OpSpecOpInst::op_spec_op_dis_fmt(inst, f, ctx),),
        );
        quote! {
            impl OpSpecOpInst for #name {
                fn op_spec_op_encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
                    profiling::function_scope!();
                    match self {
                        #(#encode_match)*
                    }
                }

                fn op_spec_op_decode(reader: &[Word]) -> Result<Self, DecodeError> {
                    profiling::function_scope!();
                    let opcode = reader[2].to_op().0;
                    Ok(match opcode {
                        #(#decode_match)*
                        _ => return Err(DecodeErrorKind::UnknownOpCode { opcode }.into()),
                    })
                }

                fn op_spec_op_dis_fmt(&self, f: &mut Formatter<'_>, ctx: OperandDisContext<'_>) -> std::fmt::Result {
                    profiling::function_scope!();
                    match self {
                        #(#dis_match)*
                    }
                }
            }
        }
    };

    writer.write_module(
        "op_spec_op",
        quote! {
            #inst_set
            #op_spec_op_inst_set
        },
    )
}

pub fn prep_insts<'a, 'b: 'a>(
    insts: impl IntoIterator<Item = &'a InstMeta<'b>>,
) -> Vec<(&'a InstMeta<'b>, Ident, Ident)> {
    insts
        .into_iter()
        .map(|inst| {
            let type_ident = InstMeta::type_ident(&inst.opname);
            let enum_ident = InstMeta::enum_ident(&inst.opname);
            (inst, type_ident, enum_ident)
        })
        .collect::<Vec<_>>()
}

pub fn quote_inst_set<'a, 'b: 'a>(
    insts: &[(&'a InstMeta<'b>, Ident, Ident)],
    name: &Ident,
) -> TokenStream {
    let enum_variants = insts
        .iter()
        .map(|(_, type_ident, enum_ident)| quote!(#enum_ident(#type_ident),));
    let id_result_match = insts.iter().map(
        |(_, _, enum_ident)| quote!(Self::#enum_ident(inst) => InstEncoding::id_result(inst).to_optional(),),
    );
    let id_result_type_match = insts.iter().map(
        |(_, _, enum_ident)| quote!(Self::#enum_ident(inst) => InstEncoding::id_result_type(inst).to_optional(),),
    );
    let encode_match = insts.iter().map(
        |(_, _, enum_ident)| quote!(Self::#enum_ident(inst) => InstEncoding::encode(inst, writer),),
    );
    let decode_match = insts.iter().map(|(inst, type_ident, enum_ident)| {
        let opcode = inst.opcode;
        quote!(#opcode => Self::#enum_ident(<#type_ident as InstEncoding>::decode(reader)?),)
    });
    let dis_match = insts.iter().map(
        |(_, _, enum_ident)| quote!(Self::#enum_ident(inst) => InstEncoding::dis_fmt(inst, f, ctx),),
    );
    let from_impls = insts.iter().map(|(_, type_ident, enum_ident)| {
        quote! {
            impl From<#type_ident> for #name {
                fn from(inst: #type_ident) -> Self {
                    Self::#enum_ident(inst)
                }
            }
        }
    });

    quote! {
        #[derive(Clone, Debug, Eq, PartialEq, Hash)]
        pub enum #name {
            #(#enum_variants)*
        }

        impl InstEncoding for #name {
            type IdResult = Option<IdResult>;
            type IdResultType = Option<IdResultType>;

            fn id_result(&self) -> Self::IdResult {
                profiling::function_scope!();
                match self {
                    #(#id_result_match)*
                }
            }

            fn id_result_type(&self) -> Self::IdResultType {
                profiling::function_scope!();
                match self {
                    #(#id_result_type_match)*
                }
            }

            fn name() -> &'static str {
                stringify!(#name)
            }

            fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
                profiling::function_scope!();
                match self {
                    #(#encode_match)*
                }
            }

            fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
                profiling::function_scope!();
                let opcode = reader.opcode();
                Ok(match opcode {
                    #(#decode_match)*
                    _ => return Err(DecodeErrorKind::UnknownOpCode { opcode }.into()),
                })
            }

            fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
                profiling::function_scope!();
                match self {
                    #(#dis_match)*
                }
            }
        }

        #(#from_impls)*
    }
}
