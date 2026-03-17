//! Module for Disassembly

use crate::binary::{DecodeError, ModuleReader};
use crate::inst::InstEncoding;
use crate::operand::{ConstFmt, IdResult, LiteralStringEscape, Word};
use anstyle::Style;
use rustc_hash::FxHashMap;
use std::cell::Cell;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
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
    /// Format constants based on the `IdResultType` of the `OpConstant`, requires scanning the module for type
    /// information to provide context.
    ///
    /// **REQUIRED** for correct disassembly
    pub const_fmt: bool,
}

impl Default for DisOptions {
    #[inline]
    fn default() -> Self {
        Self {
            color: true,
            literal_string_escape: LiteralStringEscape::default(),
            rspirv_space: false,
            const_fmt: true,
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
            const_fmt: true,
        }
    }

    /// output similarly `spirv-dis` from the C++ spirv tools
    pub fn like_spirv_tools() -> Self {
        Self {
            color: true,
            literal_string_escape: LiteralStringEscape::MultiLine,
            rspirv_space: false,
            const_fmt: true,
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
///
/// Next to [`DisOptions`], contains lookup tables to aid in disassembly generation. Initializing these tables is
/// **required** to generate correct disassembly that `spirv-as` can read, otherwise, we'll generate the best possible
/// disassembly we can. Since instructions are not available within the `rspirv2-types` crate, we can't actually
/// implement the context information retrieval here. Instead, it needs to be implemented for each instruction set with
/// the [`InstSetDisCtx`] trait, which may delegate to our default implementation in
/// `rspirv2::dis::create_dis_context_core`.
#[derive(Clone, Debug, Default)]
pub struct DisContext {
    /// Options
    opt: DisOptions,
    /// Maps an [`IdResult`] of a type declaration to a [`ConstFmt`] to tell `OpConstant` instructions how to format
    /// the untyped constant value
    pub id_to_const_fmt: FxHashMap<IdResult, ConstFmt>,
}

impl DisContext {
    /// Create a new [`DisContext`] by scanning the module for useful information, based on the options provided.
    #[inline]
    pub fn new<ISA: InstSetDisCtx>(opt: DisOptions, words: &[Word]) -> Result<Self, DecodeError> {
        ISA::create_dis_context(opt, words)
    }

    /// Creates a new [`DisContext`] without having scanned the module for the required extra information.
    ///
    /// **WARNING**: Don't use this function directly, use [`Self::new`] instead, or it will result in worse and
    /// `spirv-as`-incompatible disassembly!
    #[inline]
    pub fn no_context(opt: DisOptions) -> Self {
        Self {
            opt,
            ..Default::default()
        }
    }
}

/// An instruction set that provides additional context information for disassembly, see [`DisContext`].
pub trait InstSetDisCtx: InstEncoding {
    fn create_dis_context(opt: DisOptions, words: &[Word]) -> Result<DisContext, DecodeError>;
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

/// A sequence of words that has been pre-processed and may be [`Display`]ed.
///
/// The `ISA: `[`InstEncoding`] generic determines for which instruction set these Words are disassembled.
pub struct DisModule<'a, ISA: InstSetDisCtx> {
    words: &'a [Word],
    dis: DisContext,
    _phantom: PhantomData<ISA>,
}

impl<'a, ISA: InstSetDisCtx> DisModule<'a, ISA> {
    pub fn new(words: &'a [Word], opt: DisOptions) -> Result<Self, DecodeError> {
        Ok(Self {
            words,
            dis: DisContext::new::<ISA>(opt, words)?,
            _phantom: PhantomData,
        })
    }
}

impl<'a, ISA: InstSetDisCtx> Display for DisModule<'a, ISA> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut reader = ModuleReader::new(self.words);
        while let Some(mut inst) = reader.next().map_err(|_| std::fmt::Error)? {
            let inst = ISA::decode(&mut inst).map_err(|_| std::fmt::Error)?;
            writeln!(f, "{}", inst.dis(&self.dis))?;
        }
        Ok(())
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
