mod emit;
mod instructions;
mod modules;
mod operands;
mod writer;

pub use emit::*;
pub use modules::*;
pub use writer::*;

/// Copied from `proc_macro2::fallback::validate_ident`
pub fn ident_ok(string: &str) -> bool {
    let mut chars = string.chars();
    let first = chars.next().unwrap();
    if !is_ident_start(first) {
        return false;
    }
    for ch in chars {
        if !is_ident_continue(ch) {
            return false;
        }
    }
    true
}
pub(crate) fn is_ident_start(c: char) -> bool {
    c == '_' || unicode_ident::is_xid_start(c)
}

pub(crate) fn is_ident_continue(c: char) -> bool {
    unicode_ident::is_xid_continue(c)
}
