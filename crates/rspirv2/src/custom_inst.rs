//! custom instructions

use crate::binary::{DecodeError, EncodeError, InstReader, WordWriter};
use crate::core::inst_meta::OP_SWITCH;
use crate::dis::DisContext;
use crate::inst::SpvInstEncoding;
use crate::meta::InstMeta;
use crate::operand::{IdRef, OperandDisContext, SpvOperandEncoding};
use OpSwitchTargetLen::{One, Two};
use rspirv2_types::Word;
use rspirv2_types::binary::{DecodeErrorKind, FnWriter, OperandReader};
use rspirv2_types::inst::{SpvInstDefUse, SpvInstDis, SpvInstMeta};
use rspirv2_types::operand::{LiteralConst, SpvOperandDis};
use smallvec::SmallVec;
use std::borrow::Cow;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};

/// [`OpSwitch`] requires a custom implementation, as it is the only SPIR-V instruction which decoding is
/// context-dependent and not resolvable by grabbing the remaining words available.
///
/// The `target` of the [`OpSwitch`] are `ZeroOrMore` tuples of [`LiteralConst`] and [`IdRef`]. The `selector` is
/// compared with every [`LiteralConst`] in `target`, and if one matches, jumps to the `label` referenced by the
/// [`IdRef`]. Otherwise, it jumps to the `default` label.
///
/// The problem is that without context, it's unknown whether [`LiteralConst`] should consume one or two words, for
/// 32bit (or lower) or 64bit literals. Unlike [`crate::core::inst::OpConstant`], you can't just consume the remaining
/// words and based on that assume the constant is one or two words (see [`LiteralConst`] docs). For example, with 6
/// words remaining, you couldn't tell whether `target` has 3 32bit entries or 2 64bit entries.
///
/// So we need context to resolve the type of `selector` to correctly decode `target`. This library has been
/// specifically designed so that instruction encoding and decoding is context-free, which greatly simplifies many
/// aspects of it, so we don't want to abandon those principles. Instead, we choose to special-case `OpSwitch` with our
/// custom instruction set API and defer the decoding of `target` until it is "used".
///
/// The `target` operand is replaced by [`OpSwitchTarget`], an enum that can store `target` as [`Unresolved`] in plain
/// words or as [`Resolved`] to a specific [`OpSwitchTargetLen`] of either [`One`] or [`Two`] word literals. While `u64`
/// requires a [`Two`] word literal, `u32` (also `u16` and `u8`) are stored in a [`One`] word literal. Use
/// [`OpSwitch::resolve`] to specify the word size and convert an [`Unresolved`] target to a [`Resolved`] one, returning
/// a `&mut` reference to the latter. This will also store the [`OpSwitchTargetLen`] used for decoding in the type, to
/// validate that future calls to resolve must use the same word len. Note that encoding and decoding the [`OpSwitch`]
/// instruction makes it lose that information.
///
/// [`Unresolved`]: OpSwitchTarget::Unresolved
/// [`Resolved`]: OpSwitchTarget::Resolved
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSwitch {
    pub selector: IdRef,
    pub default: IdRef,
    pub target: OpSwitchTarget,
}

/// Target of an [`OpSwitch`]
#[derive(Clone, Debug)]
pub enum OpSwitchTarget {
    /// The word len of the target literal is unknown and must be resolved before using it
    Unresolved(SmallVec<[Word; 6]>),
    /// An `OpSwitch` with a resolved target literal length
    Resolved(OpSwitchResolvedTarget),
}

/// Length of the target literal of an [`OpSwitch`] in words
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum OpSwitchTargetLen {
    /// one word
    One,
    /// two words
    Two,
}

impl OpSwitchTargetLen {
    pub fn words(&self) -> usize {
        match self {
            One => 1,
            Two => 2,
        }
    }

    pub fn bits(&self) -> u32 {
        match self {
            One => 32,
            Two => 64,
        }
    }
}

impl Display for OpSwitchTargetLen {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            One => write!(f, "One"),
            Two => write!(f, "Two"),
        }
    }
}

/// An [`OpSwitch`] where the word length of the target has been resolved.
///
/// Use [`Self::add_target`] to add new targets to this [`OpSwitch`], which also validates any invariants.
#[derive(Clone, Debug)]
pub struct OpSwitchResolvedTarget {
    pub len: OpSwitchTargetLen,
    pub target: Vec<(LiteralConst, IdRef)>,
}

impl OpSwitchResolvedTarget {
    /// Create a new [`OpSwitchResolvedTarget`] with a specified literal word length
    pub fn new(len: OpSwitchTargetLen) -> Self {
        Self {
            len,
            target: Vec::new(),
        }
    }

    /// Create a [`OpSwitchResolvedTarget`] from any [`IntoIterator`] of Item `(LiteralConst, IdRef)`, while validating
    /// the literals
    pub fn from_iter(
        len: OpSwitchTargetLen,
        target: impl IntoIterator<Item = (LiteralConst, IdRef)>,
    ) -> Result<Self, OpSwitchError> {
        Self::from_vec(len, target.into_iter().collect())
    }

    /// Create a [`OpSwitchResolvedTarget`] from a `Vec` of `(LiteralConst, IdRef)`, while validating the literals
    pub fn from_vec(
        len: OpSwitchTargetLen,
        target: Vec<(LiteralConst, IdRef)>,
    ) -> Result<Self, OpSwitchError> {
        let this = Self { len, target };
        for (literal, _) in &this.target {
            this.validate_literal(literal)?;
        }
        Ok(this)
    }

    /// Add a new target to this [`OpSwitch`], validating the supplied literal
    pub fn add_target(&mut self, literal: LiteralConst, label: IdRef) -> Result<(), OpSwitchError> {
        self.validate_literal(&literal)?;
        self.target.push((literal, label));
        Ok(())
    }

    /// Validate that a literal has the appropriate word length to be used in this [`OpSwitch`]
    pub fn validate_literal(&self, literal: &LiteralConst) -> Result<(), OpSwitchError> {
        if literal.words() != self.len.words() {
            Err(OpSwitchError::ConstLiteralWordsMismatch {
                len: self.len,
                words: literal.words(),
            })
        } else {
            Ok(())
        }
    }
}

impl OpSwitch {
    /// Resolve this [`OpSwitch`] instruction to have a [`One`] word or [`Two`] words literal. Stores the decoded form
    /// and word len in the struct to make future retrieval fast and validates against mismatched `len`.
    pub fn resolve(
        &mut self,
        len: OpSwitchTargetLen,
    ) -> Result<&mut OpSwitchResolvedTarget, OpSwitchError> {
        profiling::function_scope!();
        if let OpSwitchTarget::Unresolved(words) = &self.target {
            self.target = OpSwitchTarget::Resolved(OpSwitchTarget::resolve_inner(len, words)?);
        }
        match &mut self.target {
            OpSwitchTarget::Unresolved(_) => unreachable!(),
            OpSwitchTarget::Resolved(r) => {
                if r.len == len {
                    Ok(r)
                } else {
                    Err(OpSwitchError::TargetLenMismatch {
                        previous: r.len,
                        resolve: len,
                    })
                }
            }
        }
    }

    /// Resolve this [`OpSwitch`] instruction to have a [`One`] word or [`Two`] words literal. Does **not** store the
    /// decoded form in the type, making validation on accidental reinterpretation possible.
    pub fn resolve_ref(
        &self,
        len: OpSwitchTargetLen,
    ) -> Result<Cow<'_, OpSwitchResolvedTarget>, OpSwitchError> {
        self.target.resolve_ref(len)
    }
}

impl OpSwitchTarget {
    /// Resolve this [`OpSwitch`] instruction to have a [`One`] word or [`Two`] words literal. Does **not** store the
    /// decoded form in the type, making validation on accidental reinterpretation possible.
    pub fn resolve_ref(
        &self,
        len: OpSwitchTargetLen,
    ) -> Result<Cow<'_, OpSwitchResolvedTarget>, OpSwitchError> {
        profiling::function_scope!();
        match &self {
            OpSwitchTarget::Unresolved(words) => Ok(Cow::Owned(Self::resolve_inner(len, words)?)),
            OpSwitchTarget::Resolved(r) => {
                if r.len == len {
                    Ok(Cow::Borrowed(r))
                } else {
                    Err(OpSwitchError::TargetLenMismatch {
                        previous: r.len,
                        resolve: len,
                    })
                }
            }
        }
    }

    pub fn as_words(&self) -> Cow<'_, [Word]> {
        profiling::function_scope!();
        match self {
            OpSwitchTarget::Unresolved(words) => Cow::Borrowed(words),
            OpSwitchTarget::Resolved(resolved) => {
                let mut writer = Vec::new();
                SpvOperandEncoding::encode(resolved, &mut writer).unwrap();
                Cow::Owned(writer)
            }
        }
    }

    fn resolve_inner(
        len: OpSwitchTargetLen,
        words: &[Word],
    ) -> Result<OpSwitchResolvedTarget, OpSwitchError> {
        profiling::function_scope!();
        // LiteralConst: len.words()
        // IdRef: 1
        let element_len = len.words() + 1;
        if !words.len().is_multiple_of(element_len) {
            return Err(DecodeErrorKind::InstructionWithMismatchedVariableOperants {
                op_len: words.len(),
                expected_multiple: element_len,
            }
            .into());
        }
        let mut target = Vec::with_capacity(words.len() / element_len);
        for chunk in words.chunks_exact(element_len) {
            let (literal, label) = chunk.split_at(chunk.len() - 1);
            target.push((
                LiteralConst::decode_last(&mut OperandReader::new(literal))?,
                IdRef::decode(&mut OperandReader::new(label))?,
            ));
        }
        Ok(OpSwitchResolvedTarget { len, target })
    }
}

unsafe impl SpvOperandEncoding for OpSwitchTarget {
    const FIXED_LEN: Option<usize> = None;

    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        match self {
            Self::Unresolved(words) => writer.write_iter(words.iter().copied()),
            Self::Resolved(resolved) => SpvOperandEncoding::encode(resolved, writer)?,
        }
        Ok(())
    }

    fn decode(_: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        Err(DecodeErrorKind::LiteralConstNotLastOperand.into())
    }

    fn decode_last(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        Ok(Self::Unresolved(reader.collect()))
    }
}

impl SpvOperandDis for OpSwitchTarget {
    fn dis_fmt(&self, _f: &mut Formatter<'_>, _ctx: &OperandDisContext<'_>) -> std::fmt::Result {
        panic!("OpSwitchTarget can't be trivially disassembled")
    }
}

unsafe impl SpvOperandEncoding for OpSwitchResolvedTarget {
    const FIXED_LEN: Option<usize> = None;

    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        self.target.encode(writer)
    }

    fn decode(_: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        Err(DecodeErrorKind::LiteralConstNotLastOperand.into())
    }
}

impl SpvOperandDis for OpSwitchResolvedTarget {
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &OperandDisContext<'_>) -> std::fmt::Result {
        profiling::function_scope!();
        self.target.dis_fmt(f, ctx)
    }
}

impl Hash for OpSwitchTarget {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.encode(&mut FnWriter(|w| w.hash(state))).unwrap();
    }
}

impl PartialEq<Self> for OpSwitchTarget {
    fn eq(&self, other: &Self) -> bool {
        self.as_words() == other.as_words()
    }
}

impl Eq for OpSwitchTarget {}

/// copied from autogen
impl SpvInstMeta for OpSwitch {
    const META: &InstMeta = &OP_SWITCH;
}

/// copied from autogen
impl SpvInstDefUse for OpSwitch {
    type IdResult = ();
    type IdResultType = ();

    fn id_result(&self) -> Self::IdResult {}

    fn id_result_type(&self) -> Self::IdResultType {}
}

impl SpvInstEncoding for OpSwitch {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1
            + SpvOperandEncoding::word_len(&self.selector)
            + SpvOperandEncoding::word_len(&self.default)
            + SpvOperandEncoding::word_len(&self.target);
        writer.write_op(Self::META.opcode, len)?;
        SpvOperandEncoding::encode(&self.selector, &mut *writer)?;
        SpvOperandEncoding::encode(&self.default, &mut *writer)?;
        SpvOperandEncoding::encode(&self.target, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            selector: SpvOperandEncoding::decode(&mut op_reader)?,
            default: SpvOperandEncoding::decode(&mut op_reader)?,
            target: SpvOperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}

impl SpvInstDis for OpSwitch {
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        profiling::function_scope!();
        let ctx = &OperandDisContext {
            id_result: None,
            id_result_type: None,
            ctx,
        };
        let resolved = match &self.target {
            OpSwitchTarget::Unresolved(words) => {
                let width = ctx.id_to_int_width.get(&self.selector.0).copied();
                let len = match width {
                    Some(64) => Two,
                    Some(32 | 16 | 8) => One,
                    _ => {
                        // we're missing context to know how to decode this OpSwitch...
                        // The best we can do is take an educated guess
                        let prefix = "# ERROR: Missing context to resolve whether OpSwitch has 32 or 64bit constants!";
                        let words = words.len();
                        let can_be_64 = words % 3 == 0;
                        let can_be_32 = words % 2 == 0;
                        match (can_be_32, can_be_64) {
                            (false, false) => {
                                writeln!(
                                    f,
                                    "{prefix} With {words} words neither 32bit or 64bit make sense, refusing to decode entry table!"
                                )?;
                                write!(
                                    f,
                                    "{}OpSwitch{}{} ???",
                                    ctx.id_result_writer(),
                                    self.selector.dis(ctx),
                                    self.default.dis(ctx)
                                )?;
                                return Ok(());
                            }
                            (true, false) => {
                                writeln!(f, "{prefix} With {words} words must be 32bit")?;
                                One
                            }
                            (false, true) => {
                                writeln!(f, "{prefix} With {words} words must be 64bit")?;
                                Two
                            }
                            (true, true) => {
                                writeln!(
                                    f,
                                    "{prefix} With {words} words can be 32 or 64bit, *guessing 32bit*"
                                )?;
                                One
                            }
                        }
                    }
                };
                // panic is unreachable: can only panic if `target` is `Self::Resolved`
                self.resolve_ref(len).unwrap()
            }
            OpSwitchTarget::Resolved(r) => Cow::Borrowed(r),
        };
        write!(
            f,
            "{}OpSwitch{}{}{}",
            ctx.id_result_writer(),
            self.selector.dis(ctx),
            self.default.dis(ctx),
            resolved.dis(ctx),
        )
    }
}

#[derive(Clone, PartialEq)]
pub enum OpSwitchError {
    DecodeError(DecodeError),
    TargetLenMismatch {
        previous: OpSwitchTargetLen,
        resolve: OpSwitchTargetLen,
    },
    ConstLiteralWordsMismatch {
        len: OpSwitchTargetLen,
        words: usize,
    },
}

impl Debug for OpSwitchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for OpSwitchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OpSwitchError::DecodeError(e) => write!(f, "Decode Error: {e}"),
            OpSwitchError::TargetLenMismatch { previous, resolve } => {
                write!(
                    f,
                    "Target len previously resolved to {previous} cannot be resolved to a different len of {resolve}"
                )
            }
            OpSwitchError::ConstLiteralWordsMismatch { len, words } => {
                write!(
                    f,
                    "Literal was expected to be {} word(s) but was {words} word(s) long",
                    len.words()
                )
            }
        }
    }
}

impl Error for OpSwitchError {}

impl From<DecodeError> for OpSwitchError {
    fn from(value: DecodeError) -> Self {
        Self::DecodeError(value)
    }
}

impl From<DecodeErrorKind> for OpSwitchError {
    fn from(value: DecodeErrorKind) -> Self {
        Self::DecodeError(value.into())
    }
}
