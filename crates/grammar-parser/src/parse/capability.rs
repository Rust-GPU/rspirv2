use std::borrow::Cow;
use std::fmt::{Debug, Display, Formatter};

/// A SPIR-V Capability
#[derive(Clone, Eq, PartialEq, Hash, serde::Deserialize)]
pub struct Capability<'a>(#[serde(borrow)] Cow<'a, str>);

impl<'a> Capability<'a> {
    pub const fn new(name: Cow<'a, str>) -> Self {
        Self(name)
    }

    pub fn name(&self) -> &str {
        &self.0
    }
}

impl Display for Capability<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Capability({})", self.0)
    }
}

impl Debug for Capability<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

#[cfg(feature = "codegen")]
mod codegen {
    use super::*;
    use crate::codegen::{EmitRef, make_const_ident};
    use crate::parse::Enumerant;
    use proc_macro2::{Ident, TokenStream};
    use quote::quote;

    impl Capability<'_> {
        pub fn const_ident(&self) -> Ident {
            make_const_ident("CAPABILITY_", &self.0)
        }

        pub fn emit_def(&self) -> TokenStream {
            let ident = self.const_ident();
            let inner = &self.0;
            quote! {
                pub const #ident: Capability = Capability::new(#inner);
            }
        }
    }

    impl EmitRef for Capability<'_> {
        fn emit_ref(&self) -> TokenStream {
            let variant = Enumerant::variant_ident(&self.0);
            quote!(Capability::#variant)
        }
    }
}
