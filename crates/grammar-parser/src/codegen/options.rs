use proc_macro2::TokenStream;
use quote::quote;

#[derive(Clone, Debug, Default)]
pub struct CodegenOptions<'a> {
    pub name_suffix_type: &'a str,
    pub mod_attr: TokenStream,
    pub mod_extra: TokenStream,
    pub preamble: TokenStream,
}

impl<'a> CodegenOptions<'a> {
    pub fn mod_lints() -> TokenStream {
        quote! {
            #![allow(unused_imports)]
            #![allow(non_camel_case_types)]
            #![allow(deprecated)]
            #![allow(missing_docs)]
            #![allow(clippy::identity_op)]
            #![allow(clippy::semicolon_if_nothing_returned)]
            #![allow(clippy::doc_markdown)]
        }
    }

    pub fn new_core() -> Self {
        Self {
            name_suffix_type: "Core",
            preamble: quote! {
                pub use crate::binary::*;
                pub use crate::inst::*;
                pub use crate::meta::*;
                pub use crate::operand::*;
                pub use bitflags::bitflags;
                pub use smallvec::SmallVec;
            },
            mod_attr: Self::mod_lints(),
            mod_extra: quote! {
                impl preamble::AnyCapability for preamble::Capability {}
            },
        }
    }

    pub fn new_ext_inst_set(name_suffix_type: &'a str, path_to_core: &TokenStream) -> Self {
        Self {
            name_suffix_type,
            preamble: quote! {
                pub use #path_to_core::preamble::*;
            },
            mod_attr: Self::mod_lints(),
            ..Default::default()
        }
    }
}
