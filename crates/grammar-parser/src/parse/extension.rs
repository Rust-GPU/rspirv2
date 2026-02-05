use std::borrow::Cow;
use std::fmt::{Debug, Display, Formatter};

/// A SPIR-V Extension
#[derive(Clone, Eq, PartialEq, Hash, serde::Deserialize)]
pub struct Extension<'a>(#[serde(borrow)] Cow<'a, str>);

impl<'a> Extension<'a> {
    pub const fn new(name: Cow<'a, str>) -> Self {
        Self(name)
    }

    pub fn name(&self) -> &str {
        &self.0
    }
}

impl Display for Extension<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Extension({})", self.0)
    }
}

impl Debug for Extension<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

#[cfg(feature = "codegen")]
mod codegen {
    use super::*;
    use crate::codegen::{EmitRef, make_const_ident};
    use proc_macro2::{Ident, TokenStream};
    use quote::{ToTokens, quote};

    impl Extension<'_> {
        pub fn const_ident(&self) -> Ident {
            make_const_ident("EXTENSION_", &self.0)
        }

        pub fn emit_def(&self) -> TokenStream {
            let ident = self.const_ident();
            let inner = &self.0;
            quote! {
                pub const #ident: Extension = Extension::new(#inner);
            }
        }
    }

    impl EmitRef for Extension<'_> {
        fn emit_ref(&self) -> TokenStream {
            self.const_ident().into_token_stream()
        }
    }
}
