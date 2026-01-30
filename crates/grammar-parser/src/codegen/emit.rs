use convert_case::{Case, Casing};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::borrow::Cow;

/// Central location to convert a name to a const ident
pub fn make_const_ident(prefix: &str, mut name: &str) -> Ident {
    // filter out the `@` in the `@exclude` printing class
    if name.starts_with("@") {
        name = &name[1..];
    }
    let name = name.to_case(Case::Constant);
    format_ident!("{}{}", prefix, name)
}

pub fn ref_ident(ident: Ident) -> TokenStream {
    quote!(&#ident)
}

/// Trait for codegen emission for the various grammar types.
///
/// There are two possible ways to implement this trait:
/// 1. [`Self::emit_ref`] emits the value as-is.
/// 2. [`Self::emit_ref`] emits a reference to a constant, and they have a function [`Self::emit_def`] that emits said
///    constant, defined outside of this trait.
///
/// The first should be used for trivial types such as `str`, integers and any kind of slice. The second allows user
/// code to easily reference any definition within the grammar, and should be preferred for most non-trivial objects.
/// It also deduplicates values across the grammar, e.g. allowing multiple [`Instructions`] to reference the same
/// [`OperandKind`].
///
/// You should assume that any constants you use are present within the same module, or the appropriate glob use
/// statements have imported it.
///
/// [`Instructions`]: `crate::meta::InstructionMeta`
/// [`OperandKind`]: `crate::meta::OperandKind`
pub trait EmitRef {
    /// Emit a reference to this type, to be used within other `quote!`s.
    fn emit_ref(&self) -> TokenStream;
}

/// No [`Cow`]s in emitted code
impl<T: EmitRef + ToOwned + ?Sized> EmitRef for Cow<'_, T> {
    fn emit_ref(&self) -> TokenStream {
        self.as_ref().emit_ref()
    }
}

impl<T: EmitRef> EmitRef for Option<T> {
    fn emit_ref(&self) -> TokenStream {
        match self {
            Some(v) => {
                let inner = v.emit_ref();
                quote!(Some(#inner))
            }
            None => quote!(None),
        }
    }
}

impl<T: EmitRef> EmitRef for [T] {
    fn emit_ref(&self) -> TokenStream {
        let refs: Vec<_> = self.iter().map(|v| v.emit_ref()).collect();
        quote!(&[#(#refs),*])
    }
}

impl EmitRef for str {
    fn emit_ref(&self) -> TokenStream {
        quote!(#self)
    }
}

impl EmitRef for bool {
    fn emit_ref(&self) -> TokenStream {
        quote!(#self)
    }
}

impl EmitRef for u8 {
    fn emit_ref(&self) -> TokenStream {
        quote!(#self)
    }
}

impl EmitRef for u16 {
    fn emit_ref(&self) -> TokenStream {
        quote!(#self)
    }
}

impl EmitRef for u32 {
    fn emit_ref(&self) -> TokenStream {
        quote!(#self)
    }
}
