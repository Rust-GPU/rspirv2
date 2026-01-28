use convert_case::{Case, Casing};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::borrow::Cow;

/// Central location to convert a name to a const ident
pub fn make_const_ident(prefix: &str, name: &str) -> Ident {
    // filter out the `@` in the `@exclude` printing class
    let name = name
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect::<String>();
    let name = name.to_case(Case::Constant);
    format_ident!("{}{}", prefix, name)
}

/// Trait for codegen emission for the various grammar types.
///
/// There are two possible ways to implement this trait:
/// 1. [`Self::emit_ref`] emits the value as-is, and [`Self::emit_def`] returns an empty [`TokenStream`].
/// 2. [`Self::emit_ref`] emits a reference to a constant, and [`Self::emit_def`] emits said constant.
///
/// The first should be used for trivial types such as `str` and any values wrapped in [`Cow`]. The second allows user
/// code to easily reference any definition within the grammar, and should be preferred for most non-trivial objects.
/// It also deduplicates values across the grammar, e.g. allowing multiple [`Instructions`] to reference the same
/// [`OperandKind`].
///
/// To allow both parsing and the codegen'ed declaration of the grammar to use the same data structure, we make
/// ubiquitous use of [`Cow`]. You can assume that the codegen'ed grammar is using [`Cow::Borrowed`] with `'static`
/// lifetimes exclusively, making equality operations and cloning cheap.
///
/// You should assume that any constants you use are present within the same module, or the appropriate glob use
/// statements have imported it.
///
/// [`Instructions`]: `crate::meta::InstructionMeta`
/// [`OperandKind`]: `crate::meta::OperandKind`
pub trait Emit {
    /// Emit a reference to this type, to be used within other `quote!`s.
    fn emit_ref(&self) -> TokenStream;

    /// Generate the definition of any idents that [`self.emit_ref`] may use.
    ///
    /// May return an empty [`TokenStream`] if none are required.
    fn emit_def(&self) -> TokenStream;
}

impl<T: Emit + ToOwned + ?Sized> Emit for Cow<'_, T> {
    fn emit_ref(&self) -> TokenStream {
        let inner = self.as_ref().emit_ref();
        quote!(Cow::Borrowed(#inner))
    }

    fn emit_def(&self) -> TokenStream {
        TokenStream::default()
    }
}

impl Emit for str {
    fn emit_ref(&self) -> TokenStream {
        quote!(#self)
    }

    fn emit_def(&self) -> TokenStream {
        TokenStream::default()
    }
}

impl<T: Emit> Emit for Option<T> {
    fn emit_ref(&self) -> TokenStream {
        match self {
            Some(v) => {
                let inner = v.emit_ref();
                quote!(Some(#inner))
            }
            None => quote!(None),
        }
    }

    fn emit_def(&self) -> TokenStream {
        TokenStream::default()
    }
}

impl<T: Emit> Emit for [T] {
    fn emit_ref(&self) -> TokenStream {
        let refs: Vec<_> = self.iter().map(|v| v.emit_ref()).collect();
        quote!(&[#(#refs),*])
    }

    fn emit_def(&self) -> TokenStream {
        TokenStream::default()
    }
}

impl Emit for bool {
    fn emit_ref(&self) -> TokenStream {
        if *self { quote!(true) } else { quote!(false) }
    }

    fn emit_def(&self) -> TokenStream {
        TokenStream::default()
    }
}

impl Emit for u8 {
    fn emit_ref(&self) -> TokenStream {
        quote!(#self)
    }

    fn emit_def(&self) -> TokenStream {
        TokenStream::default()
    }
}

impl Emit for u16 {
    fn emit_ref(&self) -> TokenStream {
        quote!(#self)
    }

    fn emit_def(&self) -> TokenStream {
        TokenStream::default()
    }
}

impl Emit for u32 {
    fn emit_ref(&self) -> TokenStream {
        quote!(#self)
    }

    fn emit_def(&self) -> TokenStream {
        TokenStream::default()
    }
}
