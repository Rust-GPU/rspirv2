use crate::codegen::{CodegenOptions, GrammarWriter};
use crate::parse::{Grammar, InstMeta};
use quote::{format_ident, quote};

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
    let id_result_match = insts.iter().map(
        |(_, _, enum_ident)| quote!(Self::#enum_ident(inst) => InstEncoding::id_result(inst).to_optional(),),
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
    writer.write_module(
        "inst_set",
        quote! {
            #[derive(Clone, Debug, Eq, PartialEq, Hash)]
            pub enum #name {
                #(#enum_variants)*
            }

            impl InstEncoding for #name {
                type IdResult = Option<IdResult>;

                fn id_result(&self) -> Self::IdResult {
                    profiling::function_scope!();
                    match self {
                        #(#id_result_match)*
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
        },
    )
}
