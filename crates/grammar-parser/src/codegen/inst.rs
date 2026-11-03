use crate::codegen::{GrammarWriter, OPERAND_ID_RESULT, OPERAND_ID_RESULT_TYPE};
use crate::parse::{Grammar, InstMeta, Operand, Quantifier};
use anyhow::Context;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn write_inst(writer: &mut GrammarWriter, grammar: &Grammar<'_>) -> anyhow::Result<()> {
    let insts = grammar.insts.iter().map(|inst| {
        let struct_ident = InstMeta::type_ident(&inst.opname);
        if !inst.source.codegen_impl() {
            return Ok(quote!(pub use super::preamble::#struct_ident;));
        }

        let meta = InstMeta::const_ident(&inst.opname);
        let member_operands = inst.compute_operands();
        let id_result = member_operands
            .iter()
            .find(|op| op.meta.kind == OPERAND_ID_RESULT);
        let id_result_type = member_operands
            .iter()
            .find(|op| op.meta.kind == OPERAND_ID_RESULT_TYPE);

        // struct decl
        let member_decls = member_operands.iter().map(
            |&Operand {
                 meta,
                 ref name,
                 ref ty,
             }| match meta.quantifier {
                Quantifier::One => quote!(pub #name: #ty),
                Quantifier::ZeroOrOne => quote!(pub #name: ZeroOrOne<#ty>),
                Quantifier::ZeroOrMore => quote!(pub #name: ZeroOrMore<#ty>),
            },
        );

        // aliases
        let aliases = inst.aliases.iter().map(|alias| {
            let alias = format_ident!("{}", alias);
            quote!(pub type #alias = #struct_ident;)
        });

        // id_result
        let (id_result_ty, id_result_get) = if let Some(id_result) = id_result {
            let name = &id_result.name;
            (quote!(IdResult), quote!(self.#name))
        } else {
            (quote!(()), quote!())
        };

        // id_result_type
        let (id_result_type_ty, id_result_type_get) = if let Some(id_result_type) = id_result_type {
            let name = &id_result_type.name;
            (quote!(IdResultType), quote!(self.#name))
        } else {
            (quote!(()), quote!())
        };

        // OpSpecOp
        let op_spec_op_encoding = if inst.op_spec_op {
            let missing_operands = || {
                format!("Instruction {} is marked as `op_spec_op` compatible but doesn't have required `IdResult` and \
                `IdResultType` operands. All operands: {:?}", inst.opname, inst.operands)
            };
            let id_result = id_result.with_context(missing_operands)?;
            let id_result_type = id_result_type.with_context(missing_operands)?;

            // members that are not IdResult or IdResultType, since those need special casing.
            let members = member_operands
                .iter()
                .filter(|&op| {
                    !core::ptr::eq(op, id_result)
                        && !core::ptr::eq(op, id_result_type)
                })
                .map(|Operand { name, .. }| name)
                .collect::<Vec<_>>();
            let members_non_last = &members[..members.len().saturating_sub(1)];
            let members_last = members.last().into_iter();
            let member_id_result = &id_result.name;
            let member_id_result_type = &id_result_type.name;

            quote! {
                impl OpSpecOpInst for #struct_ident {
                    fn op_spec_op_encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
                        profiling::function_scope!();
                        OperandEncoding::encode(&self.#member_id_result_type, &mut *writer)?;
                        OperandEncoding::encode(&self.#member_id_result, &mut *writer)?;
                        writer.write_op(Self::META.opcode, 0)?;
                        #(OperandEncoding::encode(&self.#members, &mut *writer)?;)*
                        Ok(())
                    }

                    fn op_spec_op_decode(reader: &[Word]) -> Result<Self, DecodeError> {
                        profiling::function_scope!();
                        let mut op_reader = OperandReader::new(reader);
                        let #member_id_result_type = OperandEncoding::decode(&mut op_reader)?;
                        let #member_id_result = OperandEncoding::decode(&mut op_reader)?;
                        op_reader.pull_expected_opcode(Self::META)?;
                        Ok(Self {
                            #member_id_result_type,
                            #member_id_result,
                            #(#members_non_last: OperandEncoding::decode(&mut op_reader)?,)*
                            #(#members_last: OperandEncoding::decode_last(&mut op_reader)?,)*
                        })
                    }
                }
            }
        } else {
            quote!()
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
        let dis_operand_ctx = {
            let id_result_opt = if let Some(id_result) = id_result {
                let name = &id_result.name;
                quote!(Some(self.#name))
            } else {
                quote!(None)
            };
            let id_result_type_opt = if let Some(id_result_type) = id_result_type {
                let name = &id_result_type.name;
                quote!(Some(self.#name))
            } else {
                quote!(None)
            };
            quote! {
                let ctx = OperandDisContext {
                    id_result: #id_result_opt,
                    id_result_type: #id_result_type_opt,
                    ctx,
                };
            }
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
                quote!(, self.#name.dis(ctx))
            })
            .collect::<Vec<_>>();
        let pat = ["{}", inst.opname.as_ref()]
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
            quote!(let rspirv_space = ctx.rspirv_space();)
        } else {
            quote!()
        };

        let struct_def = if member_operands.is_empty() {
            quote! {
                #[derive(Clone, Debug, Eq, PartialEq, Hash)]
                pub struct #struct_ident;
            }
        } else {
            quote! {
                #[derive(Clone, Debug, Eq, PartialEq, Hash)]
                pub struct #struct_ident {
                    #(#member_decls),*
                }
            }
        };

        Ok(quote! {
            #struct_def
            #(#aliases)*

            impl Inst for #struct_ident {
                const META: &InstMeta = &#meta;
            }

            impl InstEncoding for #struct_ident {
                type IdResult = #id_result_ty;
                type IdResultType = #id_result_type_ty;

                fn id_result(&self) -> Self::IdResult {
                    #id_result_get
                }

                fn id_result_type(&self) -> Self::IdResultType {
                    #id_result_type_get
                }

                fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
                    profiling::function_scope!();
                    let len = 1 #(+OperandEncoding::word_len(&self.#members))*;
                    writer.write_op(Self::META.opcode, len)?;
                    #(OperandEncoding::encode(&self.#members, &mut *writer)?;)*
                    Ok(())
                }

                fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
                    profiling::function_scope!();
                    #reader reader.check_opcode(Self::META)?;
                    Ok(Self {
                        #(#members_non_last: OperandEncoding::decode(&mut op_reader)?,)*
                        #(#members_last: OperandEncoding::decode_last(&mut op_reader)?,)*
                    })
                }

                fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
                    profiling::function_scope!();
                    #dis_operand_ctx
                    #rspirv_spaces_prefix
                    write!(f, #pat, ctx.id_result_writer() #(#dis_operands_value)*)
                }
            }

            #op_spec_op_encoding
        })
    }).collect::<anyhow::Result<TokenStream>>()?;
    writer.write_module("inst", insts)
}
