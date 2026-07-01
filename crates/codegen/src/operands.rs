use crate::GrammarWriter;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use rspirv2_grammar_parser::parse::{Capability, Category, Extension, Grammar, OperandKind, OperandSpecMeta, Quantifier, Source};
use std::borrow::Cow;
use smallvec::SmallVec;

pub struct Operand<'a> {
    pub name: Cow<'a, str>,
    pub ty: OperandType<'a>,
    pub docs: Cow<'a, str>,
    pub source: Source,
}

pub enum OperandType<'a> {
    Extern,
    Enum,
    EnumWithData,
    Bitflags,
    BitflagsWithData,
}

#[derive(Clone, Debug, Default)]
pub struct Enumerant<'a> {
    pub docs: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub value: u32,
    pub parameters: SmallVec<[OperandSpecMeta<'a>; 1]>,
    pub aliases: SmallVec<[Cow<'a, str>; 1]>,
}

pub fn write_operands(writer: &mut GrammarWriter, grammar: &Grammar<'_>) -> anyhow::Result<()> {
    let operands = grammar.operand_kinds.iter().map(|o| {
        if !o.source.codegen_impl() {
            let name = OperandKind::type_ident(&o.name);
            return quote!(pub use super::preamble::#name;);
        }

        let has_no_params =
            |enumerants: &[Enumerant<'_>]| enumerants.iter().all(|e| e.parameters.is_empty());
        match &o.category {
            // `RefId` and Literals are imported
            Category::Id | Category::Literal => quote!(),
            Category::BitEnum { enumerants } => {
                if has_no_params(enumerants) {
                    emit_bitflags_enum(o, enumerants)
                } else {
                    emit_parameterised_bitmask(o, enumerants)
                }
            }
            Category::Composite { bases } => emit_composite(o, bases),
            Category::ValueEnum { enumerants } => {
                if has_no_params(enumerants) {
                    emit_c_like_enum(o, enumerants)
                } else {
                    emit_rust_like_enum(o, enumerants)
                }
            }
        }
    });
    writer.write_module(
        "operands",
        quote! {
            #(#operands)*
        },
    )
}

fn param_to_member_decl(p: &OperandSpecMeta<'_>) -> TokenStream {
    let docs = p
        .name
        .as_ref()
        .map(|name| make_doc(name))
        .unwrap_or_default();
    let ty = OperandKind::type_ident(&p.kind);
    match p.quantifier {
        Quantifier::One => quote!(#docs #ty),
        Quantifier::ZeroOrOne => quote!(#docs Option<#ty>),
        Quantifier::ZeroOrMore => quote!(#docs Vec<#ty>),
    }
}

fn emit_rust_like_enum(
    operand_kind: &OperandKind<'_>,
    enumerants: &[Enumerant<'_>],
) -> TokenStream {
    let name = OperandKind::type_ident(&operand_kind.name);
    let kind = OperandKind::const_ident(&operand_kind.name);
    let doc = make_doc(&operand_kind.doc);

    let symbols = enumerants
        .iter()
        .map(|e| (e, Enumerant::variant_ident(&e.symbol)))
        .collect::<Vec<_>>();

    let variants = symbols.iter().map(|(e, symbol)| {
        let enumerant_preamble = emit_enumerant_preamble(e);
        let members = if !e.parameters.is_empty() {
            let members = e.parameters.iter().map(param_to_member_decl);
            quote!((#(#members),*))
        } else {
            quote!()
        };
        quote! {
            #enumerant_preamble
            #symbol #members
        }
    });

    let encode = symbols.iter().map(|(e, symbol)| {
        let value = e.value;
        let param_symbols = (0..e.parameters.len())
            .map(|i| format_ident!("p{}", i))
            .collect::<Vec<_>>();
        if !param_symbols.is_empty() {
            quote! {
                Self::#symbol (#(#param_symbols),*) => {
                    writer.write(Word(#value));
                    #(SpvOperandEncoding::encode(#param_symbols, &mut *writer)?);*
                }
            }
        } else {
            quote!(Self::#symbol => writer.write(Word(#value)))
        }
    });

    let decode = symbols.iter().map(|(e, symbol)| {
        let value = e.value;
        if !e.parameters.is_empty() {
            let members =
                (0..e.parameters.len()).map(|_| quote!(SpvOperandEncoding::decode(&mut *reader)?));
            quote!(#value => Self::#symbol (#(#members),*))
        } else {
            quote!(#value => Self::#symbol)
        }
    });

    let dis = symbols.iter().map(|(e, symbol)| {
        let param_symbols = (0..e.parameters.len())
            .map(|i| format_ident!("p{}", i))
            .collect::<Vec<_>>();
        let fmt = [" ", &e.symbol].into_iter()
            .chain((0..param_symbols.len()).map(|_| "{}"))
            .collect::<String>();
        if !param_symbols.is_empty() {
            quote!(Self::#symbol (#(#param_symbols),*) => write!(f, #fmt, #(#param_symbols.dis(_ctx)),*))
        } else {
            quote!(Self::#symbol => write!(f, #fmt))
        }
    });

    quote! {
        #doc
        #[derive(Clone, Debug, Eq, PartialEq, Hash)]
        pub enum #name {
            #(#variants),*
        }

        unsafe impl SpvOperandMeta for #name {
            const KIND: &OperandKind = &#kind;
        }

        unsafe impl SpvOperandEncoding for #name {
            const FIXED_LEN: Option<usize> = None;

            fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
                profiling::function_scope!();
                match self {
                    #(#encode),*
                }
                Ok(())
            }

            fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
                profiling::function_scope!();
                let variant = reader.pull()?.0;
                Ok(match variant {
                    #(#decode,)*
                    _ => return Err(DecodeErrorKind::UnknownEnumVariant {
                        name: stringify!(#name),
                        variant,
                    }.into())
                })
            }
        }

        impl SpvOperandDis for #name {
            #[inline]
            fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &OperandDisContext<'_>) -> std::fmt::Result {
                profiling::function_scope!();
                match self {
                    #(#dis),*
                }
            }
        }
    }
}

fn emit_c_like_enum(operand_kind: &OperandKind<'_>, enumerants: &[Enumerant<'_>]) -> TokenStream {
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
    let dis = symbols.iter().map(|(e, symbol)| {
        let fmt = format!(" {}", e.symbol);
        quote!(Self::#symbol => write!(f, #fmt))
    });

    quote! {
        #doc
        #[repr(u32)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub enum #name {
            #(#variants),*
        }

        #[cfg(feature = "bytemuck")]
        unsafe impl bytemuck::Zeroable for #name {}
        #[cfg(feature = "bytemuck")]
        unsafe impl bytemuck::Pod for #name {}

        unsafe impl SpvOperandMeta for #name {
            const KIND: &OperandKind = &#kind;
        }

        unsafe impl SpvOperandEncoding for #name {
            const FIXED_LEN: Option<usize> = Some(1);

            fn encode(&self, writer: &mut impl WordWriter)  -> Result<(), EncodeError>{
                profiling::function_scope!();
                writer.write(Word(*self as u32));
                Ok(())
            }

            fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
                profiling::function_scope!();
                let variant = reader.pull()?.0;
                Ok(match variant {
                    #(#decode,)*
                    _ => return Err(DecodeErrorKind::UnknownEnumVariant {
                        name: stringify!(#name),
                        variant,
                    }.into())
                })
            }
        }

        impl SpvOperandDis for #name {
            #[inline]
            fn dis_fmt(&self, f: &mut Formatter<'_>, _: &OperandDisContext<'_>) -> std::fmt::Result {
                profiling::function_scope!();
                match self {
                    #(#dis),*
                }
            }
        }
    }
}

fn emit_enumerant_preamble(e: &Enumerant<'_>) -> TokenStream {
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

fn emit_bitflags_enum(operand_kind: &OperandKind<'_>, enumerants: &[Enumerant<'_>]) -> TokenStream {
    let name = OperandKind::type_ident(&operand_kind.name);
    let kind = OperandKind::const_ident(&operand_kind.name);
    let (decl, encoding) = emit_bitflags_enum_common(operand_kind, &name, enumerants);
    quote! {
        #decl
        unsafe impl SpvOperandMeta for #name {
            const KIND: &OperandKind = &#kind;
        }
        #encoding
    }
}

fn emit_bitflags_enum_common(
    operand_kind: &OperandKind<'_>,
    name: &Ident,
    enumerants: &[Enumerant<'_>],
) -> (TokenStream, TokenStream) {
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

    let dis = enumerants.iter().filter(|e| e.value != 0).map(|e| {
        let symbol = Enumerant::variant_ident(&e.symbol);
        let fmt = format!("{{sep}}{}", e.symbol);
        quote! {
            if self.contains(Self::#symbol) {
                write!(f, #fmt)?;
            }
        }
    });

    let decl = quote! {
        bitflags! {
            #doc
            #[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
            pub struct #name: u32 {
                #(#variants)*
            }
        }
    };

    let encoding = quote! {
        unsafe impl SpvOperandEncoding for #name {
            const FIXED_LEN: Option<usize> = Some(1);

            fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
                profiling::function_scope!();
                writer.write(Word(self.bits()));
                Ok(())
            }

            fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
                profiling::function_scope!();
                let bits = reader.pull()?.0;
                Self::from_bits(bits).ok_or(DecodeError::invalid_bitflags::<#name>(stringify!(#name), bits))
            }
        }

        impl SpvOperandDis for #name {
            #[inline]
            fn dis_fmt(&self, f: &mut Formatter<'_>, _: &OperandDisContext<'_>) -> std::fmt::Result {
                profiling::function_scope!();
                if self.is_empty() {
                    write!(f, " None")
                } else {
                    write!(f, " ")?;
                    let sep = SeparatorJoiner::new("|");
                    #(#dis)*
                    Ok(())
                }
            }
        }
    };
    (decl, encoding)
}

/// Example: `ImageOperands`
fn emit_parameterised_bitmask(
    operand_kind: &OperandKind<'_>,
    enumerants: &[Enumerant<'_>],
) -> TokenStream {
    let name = OperandKind::type_ident(&operand_kind.name);
    let name_bits = OperandKind::type_bits_ident(&operand_kind.name);
    let kind = OperandKind::const_ident(&operand_kind.name);
    let doc = make_doc(&operand_kind.doc);

    let (decl, encoding) = emit_bitflags_enum_common(operand_kind, &name_bits, enumerants);

    let mut bit_to_enumerants: Vec<Option<&Enumerant<'_>>> = vec![None; 32];
    for e in enumerants {
        if e.value == 0 {
            // ignore `None`
        } else if !e.value.is_power_of_two() {
            panic!(
                "Operand {} enumerant {} has a non-power-of-two value of {}",
                operand_kind.name, e.symbol, e.value
            )
        } else {
            let bit = e.value.checked_ilog2().unwrap();
            if let Some(old) = bit_to_enumerants[bit as usize].replace(e) {
                panic!(
                    "Operand {} has duplicate bitmask value {} in enumerant {} and {}",
                    operand_kind.name, e.value, e.symbol, old.symbol
                );
            }
        }
    }

    let param_word_size_array = bit_to_enumerants.iter().map(|e| {
        if let Some(e) = e
            && !e.parameters.is_empty()
        {
            let params = e
                .parameters
                .iter()
                .map(|p| OperandKind::type_ident(&p.kind));
            quote! {
                FixedLenComposer::new()
                    #(.append(#params::FIXED_LEN))*
                    .finish()
                    .expect(PARAMETERIZED_BITMASK_REQUIRES_FIXED_LEN)
            }
        } else {
            quote!(0)
        }
    });
    let getter_setter = bit_to_enumerants.iter().enumerate().map(|(bit, e)| {
        let bit = bit as u32;
        if let Some(e) = e {
            let (getter, setter) = e.parameterized_bitmask_getter_setter();
            if e.parameters.is_empty() {
                quote! {
                    pub fn #getter(&self) -> bool {
                        self.0.get_bool(#bit)
                    }

                    pub fn #setter(&mut self, enabled: bool) {
                        self.0.set_bool(#bit, enabled);
                    }
                }
            } else {
                let param_ty = {
                    let members = e.parameters.iter().map(param_to_member_decl);
                    if e.parameters.len() == 1 {
                        quote!(#(#members)*)
                    } else {
                        quote!((#(#members),*))
                    }
                };
                quote! {
                    pub fn #getter(&self) -> Option<#param_ty> {
                        self.0.get(#bit)
                    }

                    pub fn #setter(&mut self, opt: Option<#param_ty>) {
                        self.0.set(#bit, opt);
                    }
                }
            }
        } else {
            quote!()
        }
    });

    let dis_extra = bit_to_enumerants.iter().map(|e| {
        if let Some(e) = e
            && !e.parameters.is_empty()
        {
            let (getter, _) = e.parameterized_bitmask_getter_setter();
            quote! {
                if let Some(extra) = self.#getter() {
                    extra.dis_fmt(f, ctx)?;
                }
            }
        } else {
            quote!()
        }
    });

    quote! {
        #decl
        #encoding

        impl ParameterizedBitmaskBits for #name_bits {
            const BIT_TO_EXTRA_LEN: &[usize] = &[#(#param_word_size_array),*];
        }

        #doc
        #[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
        pub struct #name(ParameterizedBitmask<#name_bits>);

        impl #name {
            pub fn new() -> Self {
                Self::default()
            }

            #(#getter_setter)*
        }

        unsafe impl SpvOperandMeta for #name {
            const KIND: &OperandKind = &#kind;
        }

        unsafe impl SpvOperandEncoding for #name {
            const FIXED_LEN: Option<usize> = None;

            fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
                self.0.encode(writer)
            }

            fn decode(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
                Ok(Self(ParameterizedBitmask::<#name_bits>::decode(reader)?))
            }
        }

        impl SpvOperandDis for #name {
            #[inline]
            fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &OperandDisContext<'_>) -> std::fmt::Result {
                self.0.dis_fmt(f, ctx)?;
                #(#dis_extra)*
                Ok(())
            }
        }
    }
}

fn emit_composite(operand_kind: &OperandKind<'_>, bases: &[Cow<'_, str>]) -> TokenStream {
    let name = OperandKind::type_ident(&operand_kind.name);
    let doc = make_doc(&operand_kind.doc);

    let member_tys = bases
        .iter()
        .map(|name| OperandKind::type_ident(name))
        .collect::<Vec<_>>();

    quote! {
        #doc
        pub type #name = (#(#member_tys),*);
    }
}

fn make_doc(doc: &str) -> TokenStream {
    if doc.is_empty() {
        quote!()
    } else {
        quote!(#[doc = #doc])
    }
}
