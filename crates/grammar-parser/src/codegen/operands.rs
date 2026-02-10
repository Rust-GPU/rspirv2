use crate::codegen::GrammarWriter;
use crate::parse::{Category, Enumerant, Grammar, OperandKind, Quantifier};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
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
    let kind = OperandKind::const_ident(&operand_kind.name);
    let doc = make_doc(&operand_kind.doc);

    let symbols = enumerants
        .iter()
        .map(|e| {
            let symbol = Enumerant::variant_ident(&e.symbol);
            let params = e
                .parameters
                .iter()
                .map(|p| (p, OperandKind::type_ident(&p.kind)))
                .collect::<Vec<_>>();
            (e, symbol, params)
        })
        .collect::<Vec<_>>();

    let variants = symbols.iter().map(|(e, symbol, params)| {
        let enumerant_preamble = emit_enumerant_preamble(e);
        let members = if !e.parameters.is_empty() {
            let members = params.iter().map(|(p, ty)| {
                let docs = p
                    .name
                    .as_ref()
                    .map(|name| make_doc(name))
                    .unwrap_or(quote!());
                match p.quantifier {
                    Quantifier::One => quote!(#docs #ty),
                    Quantifier::ZeroOrOne => quote!(#docs Option<#ty>),
                    Quantifier::ZeroOrMore => quote!(#docs Vec<#ty>),
                }
            });
            quote!((#(#members),*))
        } else {
            quote!()
        };
        quote! {
            #enumerant_preamble
            #symbol #members
        }
    });

    let encode = symbols.iter().map(|(e, symbol, params)| {
        let value = e.value;
        let param_symbols = (0..params.len())
            .map(|i| format_ident!("p{}", i))
            .collect::<Vec<_>>();
        if !param_symbols.is_empty() {
            quote! {
                Self::#symbol (#(#param_symbols),*) => {
                    writer.push(Word(#value))?;
                    #(OperandEncoding::encode(#param_symbols, &mut *writer)?);*
                }
            }
        } else {
            quote!(Self::#symbol => writer.push(Word(#value))?)
        }
    });

    let decode = symbols.iter().map(|(e, symbol, params)| {
        let value = e.value;
        if !params.is_empty() {
            let members = (0..params.len()).map(|_| quote!(OperandEncoding::decode(&mut *reader)?));
            quote!(#value => Self::#symbol (#(#members),*))
        } else {
            quote!(#value => Self::#symbol)
        }
    });

    quote! {
        #doc
        #[derive(Clone, Debug, Eq, PartialEq, Hash)]
        pub enum #name {
            #(#variants),*
        }

        impl Operand for #name {
            const KIND: OperandKind = #kind;
        }

        impl OperandEncoding for #name {
            const FIXED_LEN: Option<usize> = None;

            fn encode(&self, writer: &mut impl InstructionWriter) -> Result<(), EncodeError> {
                match self {
                    #(#encode),*
                }
                Ok(())
            }

            fn decode(reader: &mut InstructionReader<'_>) -> Result<Self, DecodeError> {
                let variant = reader.pull()?.0;
                Ok(match variant {
                    #(#decode,)*
                    _ => return Err(DecodeError::UnknownEnumVariant {
                        name: stringify!(#name),
                        variant,
                    })
                })
            }
        }
    }
}

fn emit_c_like_enum(operand_kind: &OperandKind, enumerants: &[Enumerant]) -> TokenStream {
    let name = OperandKind::type_ident(&operand_kind.name);
    let kind = OperandKind::const_ident(&operand_kind.name);
    let doc = make_doc(&operand_kind.doc);

    let symbols = enumerants
        .iter()
        .map(|e| (e, Enumerant::variant_ident(&e.symbol)))
        .collect::<Vec<_>>();
    let variants = symbols.iter().map(|(e, symbol)| {
        let enumerant_preamble = emit_enumerant_preamble(e);
        let value = e.value;
        quote! {
            #enumerant_preamble
            #symbol = #value
        }
    });
    let decode = symbols.iter().map(|(e, symbol)| {
        let value = e.value;
        quote!(#value => Self::#symbol)
    });

    quote! {
        #doc
        #[repr(u32)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub enum #name {
            #(#variants),*
        }

        impl Operand for #name {
            const KIND: OperandKind = #kind;
        }

        impl OperandEncoding for #name {
            const FIXED_LEN: Option<usize> = Some(1);

            fn encode(&self, writer: &mut impl InstructionWriter) -> Result<(), EncodeError> {
                writer.push(Word(*self as u32))
            }

            fn decode(reader: &mut InstructionReader<'_>) -> Result<Self, DecodeError> {
                let variant = reader.pull()?.0;
                Ok(match variant {
                    #(#decode,)*
                    _ => return Err(DecodeError::UnknownEnumVariant {
                        name: stringify!(#name),
                        variant,
                    })
                })
            }
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
    let kind = OperandKind::const_ident(&operand_kind.name);
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

        impl Operand for #name {
            const KIND: OperandKind = #kind;
        }

        impl OperandEncoding for #name {
            const FIXED_LEN: Option<usize> = Some(1);

            fn encode(&self, writer: &mut impl InstructionWriter) -> Result<(), EncodeError> {
                writer.push(Word(self.bits()))
            }

            fn decode(reader: &mut InstructionReader<'_>) -> Result<Self, DecodeError> {
                let bits = reader.pull()?.0;
                Ok(Self::from_bits(bits).ok_or(DecodeError::invalid_bitflags::<#name>(stringify!(#name), bits))?)
            }
        }
    }
}

fn emit_composite(operand_kind: &OperandKind, bases: &[Cow<str>]) -> TokenStream {
    let name = OperandKind::type_ident(&operand_kind.name);
    let kind = OperandKind::const_ident(&operand_kind.name);
    let doc = make_doc(&operand_kind.doc);

    let member_tys = bases
        .iter()
        .map(|name| OperandKind::type_ident(name))
        .collect::<Vec<_>>();
    let len = member_tys
        .iter()
        .map(|ty| quote!(<#ty as OperandEncoding>::FIXED_LEN));
    let encode = (0..bases.len()).map(|i| {
        let i = proc_macro2::Literal::usize_unsuffixed(i);
        quote!(OperandEncoding::encode(&self.#i, &mut *writer)?)
    });
    let decode = (0..bases.len()).map(|_| quote!(OperandEncoding::decode(&mut *reader)?));

    quote! {
        #doc
        #[derive(Clone, Debug, Eq, PartialEq, Hash)]
        pub struct #name(#(#member_tys),*);

        impl Operand for #name {
            const KIND: OperandKind = #kind;
        }

        impl OperandEncoding for #name {
            const FIXED_LEN: Option<usize> = FixedLenComposer::new()#(.append(#len))*.finish();

            fn encode(&self, writer: &mut impl InstructionWriter) -> Result<(), EncodeError> {
                #(#encode;)*
                Ok(())
            }

            fn decode(reader: &mut InstructionReader<'_>) -> Result<Self, DecodeError> {
                Ok(Self(#(#decode),*))
            }
        }
    }
}

fn make_doc(doc: &str) -> TokenStream {
    if doc.is_empty() {
        quote!()
    } else {
        quote!(#[doc = #doc])
    }
}
