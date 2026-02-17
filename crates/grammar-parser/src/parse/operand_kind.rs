use crate::parse::serde_helper::num_or_hex;
use crate::parse::{Capability, Extension, OperandSpecMeta};
use smallvec::SmallVec;
use std::borrow::Cow;

/// See [`spirv_grammar::meta::OperandKind`]
#[derive(Clone, Debug, serde::Deserialize)]
pub struct OperandKind<'a> {
    #[serde(borrow, rename = "kind")]
    pub name: Cow<'a, str>,
    #[serde(flatten)]
    pub category: Category<'a>,
    #[serde(borrow, default)]
    pub doc: Cow<'a, str>,
}

/// See [`spirv_grammar::meta::Category`]
#[derive(Clone, Debug, serde::Deserialize)]
#[serde(tag = "category")]
pub enum Category<'a> {
    BitEnum {
        #[serde(borrow)]
        enumerants: Vec<Enumerant<'a>>,
    },
    Composite {
        /// The name of the [`OperandKind`]s out of which this [`OperandKind`] is composed out of,
        /// references `Grammar.operand_kinds`
        #[serde(borrow)]
        bases: Vec<Cow<'a, str>>,
    },
    Id,
    Literal,
    ValueEnum {
        #[serde(borrow)]
        enumerants: Vec<Enumerant<'a>>,
    },
}

/// See [`spirv_grammar::meta::Enumerant`]
#[derive(Clone, Debug, Default, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Enumerant<'a> {
    #[serde(borrow, rename = "enumerant")]
    pub symbol: Cow<'a, str>,
    #[serde(deserialize_with = "num_or_hex")]
    pub value: u32,
    #[serde(borrow, default)]
    pub parameters: SmallVec<[OperandSpecMeta<'a>; 1]>,
    #[serde(borrow, default)]
    pub capabilities: SmallVec<[Capability<'a>; 2]>,
    #[serde(borrow, default)]
    pub extensions: SmallVec<[Extension<'a>; 2]>,
    #[serde(borrow, default)]
    pub version: Option<Cow<'a, str>>,
    #[serde(borrow, default, rename = "lastVersion")]
    pub last_version: Option<Cow<'a, str>>,
    #[serde(borrow, default)]
    pub aliases: SmallVec<[Cow<'a, str>; 1]>,
    #[serde(default)]
    pub provisional: bool,
}

#[cfg(feature = "codegen")]
mod codegen {
    use super::*;
    use crate::codegen::{EmitRef, OPERAND_ID_RESULT, make_const_ident, ref_ident};
    use proc_macro2::{Ident, TokenStream};
    use quote::{format_ident, quote};

    impl OperandKind<'_> {
        pub fn const_ident(name: &str) -> Ident {
            make_const_ident("OPERAND_KIND_", name)
        }

        pub fn type_ident(name: &str) -> Ident {
            if name == OPERAND_ID_RESULT {
                format_ident!("OptionIdResult")
            } else {
                format_ident!("{}", name)
            }
        }

        pub fn emit_def(&self) -> TokenStream {
            let ident = Self::const_ident(&self.name);
            let name = self.name.emit_ref();
            let category = self.category.emit_ref();
            let doc = self.doc.emit_ref();
            quote! {
                pub const #ident: OperandKind = OperandKind {
                    name: #name,
                    category: #category,
                    doc: #doc,
                };
            }
        }
    }

    impl EmitRef for OperandKind<'_> {
        fn emit_ref(&self) -> TokenStream {
            ref_ident(Self::const_ident(&self.name))
        }
    }

    impl EmitRef for Category<'_> {
        fn emit_ref(&self) -> TokenStream {
            match self {
                Category::BitEnum { enumerants } => {
                    let enumerants = enumerants.emit_ref();
                    quote!(Category::BitEnum { enumerants: #enumerants })
                }
                Category::Composite { bases } => {
                    let bases = bases
                        .iter()
                        .map(|name| ref_ident(OperandKind::const_ident(name)));
                    quote!(Category::Composite { bases: &[#(#bases),*] })
                }
                Category::Id => quote!(Category::Id),
                Category::Literal => quote!(Category::Literal),
                Category::ValueEnum { enumerants } => {
                    let enumerants = enumerants.emit_ref();
                    quote!(Category::ValueEnum { enumerants: #enumerants })
                }
            }
        }
    }

    impl Enumerant<'_> {
        pub fn variant_ident(symbol: &str) -> Ident {
            let mut chars = symbol.chars().peekable();
            if let Some(first_char) = chars.next() {
                if first_char.is_ascii_digit() {
                    // manual overwrites
                    if symbol == "2x2" {
                        // CooperativeMatrixReduce has a variant called `2x2`
                        format_ident!("TwoByTwo")
                    } else if chars.peek() == Some(&'d') || chars.peek() == Some(&'D') {
                        // 1D, 2D, 3D...
                        format_ident!("Dim{}", symbol)
                    } else {
                        // best effort
                        format_ident!("Sym{}", symbol)
                    }
                } else {
                    // regular symbol
                    format_ident!("{}", symbol)
                }
            } else {
                // empty string
                panic!("enumerant symbol must not be an empty string")
            }
        }
    }

    impl EmitRef for Enumerant<'_> {
        fn emit_ref(&self) -> TokenStream {
            let symbol = self.symbol.emit_ref();
            let value = self.value.emit_ref();
            let parameters = self.parameters.emit_ref();
            let capabilities = self.capabilities.emit_ref();
            let extensions = self.extensions.emit_ref();
            let version = self.version.emit_ref();
            let last_version = self.last_version.emit_ref();
            let aliases = self.aliases.emit_ref();
            let provisional = self.provisional.emit_ref();
            quote! {
                Enumerant {
                    symbol: #symbol,
                    value: #value,
                    parameters: #parameters,
                    capabilities: #capabilities,
                    extensions: #extensions,
                    version: #version,
                    last_version: #last_version,
                    aliases: #aliases,
                    provisional: #provisional,
                }
            }
        }
    }
}
