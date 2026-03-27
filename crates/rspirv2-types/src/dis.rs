//! Module for Disassembly

use crate::inst::InstEncoding;
use crate::operand::{ConstFmt, IdResult, LiteralStringEscape};
use crate::slice::{InstSlice, RawInstSlice};
use anstyle::Style;
use rustc_hash::FxHashMap;
use std::cell::Cell;
use std::collections::hash_map::Entry;
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
    /// Defines whether to use raw ids (`%53`) or named ids (`%my_name`) resolved from `OpName` descriptors or other
    /// sources of metadata. See [`IdNaming`].
    pub id_naming: IdNaming,
    /// How to derive names for types
    pub type_naming: TypeNaming,
}

/// Defines where names of types should be derived from.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum IdNaming {
    /// Only use raw ids (`%53`)
    RawId,
    /// Only use names for explicitly named types (`OpName`)
    ExplicitNames,
    /// Use names from both explicit naming (`OpName`) and other sources of metadata.
    ///
    /// Typical sources of names, besides `OpName`:
    /// * `OpType*`: `%u32 = OpTypeInt 32 0`
    /// * `OpConstant`: `%u32_42 = OpConstant %u32 42`
    All,
}

/// Defines how to derive names for types
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TypeNaming {
    /// Rust naming conventions, eg. `u32`, `i32`, `f32`
    Rust,
    /// C naming conventions, eg. `uint`, `int`, `float`
    ///
    /// should match spirv-dis
    C,
}

impl Default for DisOptions {
    #[inline]
    fn default() -> Self {
        Self {
            color: true,
            literal_string_escape: LiteralStringEscape::default(),
            rspirv_space: false,
            const_fmt: true,
            id_naming: IdNaming::All,
            type_naming: TypeNaming::Rust,
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
            id_naming: IdNaming::RawId,
            type_naming: TypeNaming::Rust,
        }
    }

    /// output similarly `spirv-dis` from the C++ spirv tools
    pub fn like_spirv_tools() -> Self {
        Self {
            color: true,
            literal_string_escape: LiteralStringEscape::MultiLine,
            rspirv_space: false,
            const_fmt: true,
            id_naming: IdNaming::All,
            type_naming: TypeNaming::C,
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
    /// Maps an [`IdResult`] to the "highest priority" [`IdName`]
    pub id_to_name: FxHashMap<IdResult, IdName>,
}

#[derive(Clone, Debug, Default)]
pub enum IdName {
    /// Use the raw id
    #[default]
    RawId,
    /// Use this explicitly assigned name, has priority over [`Self::DerivedName`]
    ExplicitName(String),
    /// Derived name from the definition of the id
    DerivedName(String),
}

#[derive(Copy, Clone, Debug)]
pub enum ResolvedIdName<'a> {
    Named(&'a str),
    Raw(IdResult),
}

impl Display for ResolvedIdName<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ResolvedIdName::Named(name) => write!(f, "{name}"),
            ResolvedIdName::Raw(id) => write!(f, "{}", id.0.0),
        }
    }
}

impl DisContext {
    /// Scan the supplied [`InstSlice`] for useful context
    pub fn add_context<ISA: InstSetDisCtx>(&mut self, slice: &InstSlice<ISA>) {
        ISA::add_context(slice.as_raw(), self);
    }

    /// Scan the supplied [`InstSlice`] for useful context, skip over any [`DecodeError`]s that may arise
    ///
    /// [`DecodeError`]: crate::binary::DecodeError
    pub fn add_context_raw<ISA: InstSetDisCtx>(&mut self, slice: &RawInstSlice) {
        ISA::add_context(slice, self);
    }

    /// Add a mapping from an [`IdResult`] to an [`IdName`]
    pub fn add_id_to_name(&mut self, id: IdResult, name: IdName) {
        // In the name of Hades, I accept this message!
        #[allow(clippy::match_same_arms)]
        let accept = match (self.opt.id_naming, &name) {
            // no need to store anything, default is `RawId`
            (_, IdName::RawId) => false,
            (IdNaming::RawId, _) => false,

            (IdNaming::All, _) => true,
            (IdNaming::ExplicitNames, IdName::ExplicitName(..)) => true,
            (IdNaming::ExplicitNames, IdName::DerivedName(..)) => false,
        };
        if accept {
            match self.id_to_name.entry(id) {
                Entry::Occupied(mut slot) => {
                    let overwrite = match (&name, slot.get()) {
                        // don't overwrite an explicit name
                        (IdName::DerivedName(..), IdName::ExplicitName(..)) => false,
                        (_, _) => true,
                    };
                    if overwrite {
                        slot.insert(name);
                    }
                }
                Entry::Vacant(slot) => {
                    slot.insert(name);
                }
            }
        }
    }

    pub fn id_to_name(&self, id: IdResult) -> ResolvedIdName<'_> {
        match self.id_to_name.get(&id).unwrap_or(&IdName::RawId) {
            IdName::RawId => ResolvedIdName::Raw(id),
            IdName::ExplicitName(name) | IdName::DerivedName(name) => ResolvedIdName::Named(name),
        }
    }

    /// Creates a new [`DisContext`] without having scanned the module for the required extra information.
    ///
    /// **WARNING**: You need to [`Self::add_context`] the instructions you want to decode before disassembling,
    /// as the disassembly generated may be invalid without the required context. For example, `OpConstant` needs to
    /// query the type of constant their value is to format it correctly as a float or an int.
    #[inline]
    pub fn no_context(opt: DisOptions) -> Self {
        Self {
            opt,
            ..Default::default()
        }
    }
}

/// An instruction set that provides additional context information for disassembly.
pub trait InstSetDisCtx: InstEncoding {
    /// Add context to the supplied [`DisContext`] by modifying the various public members of it.
    ///
    /// Only supplies a [`RawInstSlice`] instead of a full [`InstSlice`] that has been error checked, as to allow
    /// disassembly of partially corrupt instructions. Any [`DecodeError`] that arrises during decode should be silently
    /// ignored. We recommend using `slice.iter().try_decode::<CoreInstSet>().skip_errors()` to decode instructions
    /// from the [`RawInstSlice`].
    ///
    /// [`DecodeError`]: crate::binary::DecodeError
    fn add_context(slice: &RawInstSlice, ctx: &mut DisContext);
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

/// Either [`DisContext`] or [`DisOptions`] that is turned into one via [`DisContext::no_context`]. Actual context needs
/// to be gathered yourself, see warning in [`DisContext::no_context`].
pub trait IntoDisContext {
    fn into_dis_ctx(self) -> DisContext;
}

impl IntoDisContext for DisContext {
    fn into_dis_ctx(self) -> DisContext {
        self
    }
}

impl IntoDisContext for DisOptions {
    fn into_dis_ctx(self) -> DisContext {
        DisContext::no_context(self)
    }
}

/// A sequence of words that has been pre-processed and may be [`Display`]ed.
///
/// The `ISA: `[`InstEncoding`] generic determines for which instruction set these Words are disassembled.
pub struct DisInstSlice<'a, ISA: InstSetDisCtx> {
    slice: &'a RawInstSlice,
    ctx: DisContext,
    _phantom: PhantomData<ISA>,
}

impl<'a, ISA: InstSetDisCtx> DisInstSlice<'a, ISA> {
    #[inline]
    pub fn new(slice: impl Into<&'a RawInstSlice>, ctx: impl IntoDisContext) -> Self {
        let slice = slice.into();
        let mut ctx = ctx.into_dis_ctx();
        ctx.add_context_raw::<ISA>(slice);
        Self::new_no_context(slice, ctx)
    }

    #[inline]
    pub fn new_no_context(slice: impl Into<&'a RawInstSlice>, ctx: impl IntoDisContext) -> Self {
        Self {
            slice: slice.into(),
            ctx: ctx.into_dis_ctx(),
            _phantom: PhantomData,
        }
    }
}

impl<'a, ISA: InstSetDisCtx> Display for DisInstSlice<'a, ISA> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for maybe_reader in self.slice.iter() {
            match maybe_reader.and_then(|reader| ISA::decode(reader)) {
                Ok(inst) => writeln!(f, "{}", inst.dis(&self.ctx))?,
                Err(e) => writeln!(f, "Error: {}", e)?,
            }
        }
        Ok(())
    }
}

#[allow(clippy::match_same_arms)]
pub fn escape_id_name(str: &str) -> Option<String> {
    let escaped = str
        .chars()
        .map(|c| match c {
            'A'..='Z' => c,
            'a'..='z' => c,
            '0'..='9' => c,
            _ => '_',
        })
        .collect::<String>();
    if escaped.chars().all(|c| c == ' ') {
        None
    } else {
        Some(escaped)
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
