//! custom instructions

use crate::binary::{DecodeError, EncodeError, InstReader, WordWriter};
use crate::core::inst_meta::OP_SWITCH;
use crate::dis::DisContext;
use crate::inst::{Inst, InstEncoding};
use crate::meta::InstMeta;
use crate::operand::{IdRef, OperandDisContext, OperandEncoding};
use OpSwitchTargetLen::{One, Two};
use rspirv2_types::Word;
use rspirv2_types::binary::{DecodeErrorKind, FnWriter, OperandReader};
use rspirv2_types::operand::{IdResult, LiteralConst};
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
                OperandEncoding::encode(resolved, &mut writer).unwrap();
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

unsafe impl OperandEncoding for OpSwitchTarget {
    const FIXED_LEN: Option<usize> = None;

    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        match self {
            Self::Unresolved(words) => writer.write_iter(words.iter().copied()),
            Self::Resolved(resolved) => OperandEncoding::encode(resolved, writer)?,
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

    fn dis_fmt(&self, _f: &mut Formatter<'_>, _ctx: &OperandDisContext<'_>) -> std::fmt::Result {
        unreachable!()
    }
}

impl OpSwitchTarget {
    fn dis_fmt(
        &self,
        f: &mut Formatter<'_>,
        selector: IdResult,
        ctx: &DisContext,
    ) -> std::fmt::Result {
        profiling::function_scope!();
        let resolved = match self {
            OpSwitchTarget::Unresolved(_) => {
                let len = {
                    let width = ctx.id_to_int_width.get(&selector).copied();
                    match width {
                        Some(64) => Some(Two),
                        Some(32 | 16 | 8) => Some(One),
                        _ => None,
                    }
                }
                // TODO How to handle this error nicely? All other cases we could just get away with not erroring
                .expect("Disassembly missing context");
                self.resolve_ref(len).unwrap()
            }
            OpSwitchTarget::Resolved(r) => Cow::Borrowed(r),
        };
        resolved.dis_fmt(f, &OperandDisContext::new(ctx))
    }
}

pub struct OpSwitchTargetDis<'a> {
    target: &'a OpSwitchTarget,
    selector: IdResult,
    ctx: &'a DisContext,
}

impl Display for OpSwitchTargetDis<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.target.dis_fmt(f, self.selector, self.ctx)
    }
}

unsafe impl OperandEncoding for OpSwitchResolvedTarget {
    const FIXED_LEN: Option<usize> = None;

    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        self.target.encode(writer)
    }

    fn decode(_: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        Err(DecodeErrorKind::LiteralConstNotLastOperand.into())
    }

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
impl Inst for OpSwitch {
    const META: &InstMeta = &OP_SWITCH;
}

/// copied from autogen
impl InstEncoding for OpSwitch {
    type IdResult = ();
    type IdResultType = ();

    fn id_result(&self) -> Self::IdResult {}

    fn id_result_type(&self) -> Self::IdResultType {}

    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1
            + OperandEncoding::word_len(&self.selector)
            + OperandEncoding::word_len(&self.default)
            + OperandEncoding::word_len(&self.target);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.selector, &mut *writer)?;
        OperandEncoding::encode(&self.default, &mut *writer)?;
        OperandEncoding::encode(&self.target, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            selector: OperandEncoding::decode(&mut op_reader)?,
            default: OperandEncoding::decode(&mut op_reader)?,
            target: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        profiling::function_scope!();
        let ctx = &OperandDisContext {
            id_result: None,
            id_result_type: None,
            ctx,
        };
        write!(
            f,
            "{}OpSwitch{}{}{}",
            ctx.id_result_writer(),
            self.selector.dis(ctx),
            self.default.dis(ctx),
            OpSwitchTargetDis {
                target: &self.target,
                selector: self.selector.0,
                ctx,
            }
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
