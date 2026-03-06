use proc_macro2::TokenStream;
use quote::quote;

pub struct CodegenOptions<'a> {
    pub name_suffix_type: &'a str,
    pub mod_attr: Box<dyn Fn() -> TokenStream + Send + Sync>,
    pub mod_extra: Box<dyn Fn() -> TokenStream + Send + Sync>,
    pub preamble: Box<dyn Fn() -> TokenStream + Send + Sync>,
}

impl<'a> CodegenOptions<'a> {
    pub fn mod_lints() -> TokenStream {
        quote! {
            #![allow(unused_imports)]
            #![allow(non_camel_case_types)]
            #![allow(deprecated)]
            #![allow(clippy::identity_op)]
        }
    }

    pub fn new_core() -> Self {
        Self {
            name_suffix_type: "Core",
            preamble: Box::new(|| {
                quote! {
                    pub use crate::binary::*;
                    pub use crate::inst::*;
                    pub use crate::meta::*;
                    pub use crate::operand::*;
                    pub use bitflags::bitflags;
                    pub use smallvec::SmallVec;
                }
            }),
            mod_extra: Box::new(|| {
                quote! {
                    impl preamble::AnyCapability for preamble::Capability {}
                }
            }),
            ..Default::default()
        }
    }

    pub fn new_ext_inst_set(name_suffix_type: &'a str, _path_to_core: &TokenStream) -> Self {
        Self {
            name_suffix_type,
            preamble: Box::new(|| {
                quote! {
                    pub use crate::core::preamble::*;
                }
            }),
            ..Default::default()
        }
    }
}

impl Default for CodegenOptions<'_> {
    fn default() -> Self {
        Self {
            name_suffix_type: "",
            mod_attr: Box::new(Self::mod_lints),
            mod_extra: Box::new(|| quote!()),
            preamble: Box::new(|| quote!()),
        }
    }
}
