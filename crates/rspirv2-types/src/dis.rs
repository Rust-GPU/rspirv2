//! Module for Disassembly

use crate::operand::LiteralStringEscape;
use anstyle::Style;
use std::cell::Cell;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

/// Options for disassembly
#[derive(Clone, Debug)]
pub struct DisOptions {
    /// Whether to emit ANSI escape sequences for colorful output
    pub color: bool,
    /// Describes how to escape string sequences
    pub literal_string_escape: LiteralStringEscape,
    /// Add extra spaces around [`crate::operand::IdResultType`] to match `rspirv`'s behaviour
    pub rspirv_space: bool,
}

impl Default for DisOptions {
    #[inline]
    fn default() -> Self {
        Self {
            color: true,
            literal_string_escape: LiteralStringEscape::default(),
            rspirv_space: false,
        }
    }
}

impl DisOptions {
    /// simple tries to be as basic as possible, similar to [`Self::like_rspirv`], but without its quirks
    pub fn simple() -> Self {
        Self {
            color: false,
            ..Default::default()
        }
    }

    /// output similarly to the `rspirv` crate
    pub fn like_rspirv() -> Self {
        Self {
            color: false,
            literal_string_escape: LiteralStringEscape::EscapeNewlines,
            rspirv_space: true,
        }
    }

    /// output similarly `spirv-dis` from the C++ spirv tools
    pub fn like_spirv_tools() -> Self {
        Self {
            color: true,
            literal_string_escape: LiteralStringEscape::MultiLine,
            rspirv_space: false,
        }
    }

    /// Disable the style if `color == false`
    #[inline]
    pub fn color(&self, style: Style) -> Style {
        if self.color {
            style
        } else {
            Default::default()
        }
    }

    /// Return a space " " if `rspirv_spaces` is on
    pub fn rspirv_space(&self) -> &str {
        if self.rspirv_space { " " } else { "" }
    }
}

/// Context object for disassembly generation
#[derive(Clone, Debug, Default)]
pub struct DisContext {
    opt: DisOptions,
}

impl DisContext {
    pub fn new(opt: DisOptions) -> Self {
        Self { opt }
    }
}

impl Deref for DisContext {
    type Target = DisOptions;
    fn deref(&self) -> &Self::Target {
        &self.opt
    }
}

impl DerefMut for DisContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.opt
    }
}

/// Utility struct for inserting separators between variants. Use it like `format!({sep}{value})`, will skip the
/// separator on first write.
///
/// Used by bitmasks to add `|` in disassembly.
pub struct SeparatorJoiner<'a> {
    first: Cell<bool>,
    sep: &'a str,
}

impl<'a> SeparatorJoiner<'a> {
    #[inline]
    pub fn new(sep: &'a str) -> Self {
        Self {
            first: Cell::new(false),
            sep,
        }
    }
}

impl<'a> Display for SeparatorJoiner<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.first.replace(true) {
            write!(f, "{}", self.sep)
        } else {
            Ok(())
        }
    }
}
