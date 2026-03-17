use crate::Word;
use crate::binary::{DecodeError, EncodeError, OperandReader, WordWriter};
use crate::meta::{Category, OperandKind};
use crate::operand::{Operand, OperandDisContext, OperandEncoding};
use anstyle::AnsiColor;
use smallvec::SmallVec;
use std::fmt::Formatter;

pub const OPERAND_KIND_LITERAL_CONTEXT_DEPENDENT_NUMBER: OperandKind = OperandKind {
    name: "LiteralContextDependentNumber",
    category: Category::Literal,
    doc: "A literal number whose size and format are determined by a previous operand in the enclosing instruction",
};

/// A `LiteralContextDependentNumber`, or [`LiteralConst`] for short since it's only used by `OpConstant` (and
/// `OpSpecConstant`) instructions.
///
/// A number of content dependent length, as defined by SPIR-V spec. May represent any number of [`Word`]s, depending on
/// the type of the constant.
///
/// # Parsing Assumption
/// > We assume that [`LiteralConst`] is always the last [`Operand`] in an [`Instruction`] and never has a quantity of
/// > [`Quantifier::ZeroOrMore`].
///
/// The current spec satisfies this requirement. `OpConstant` / `OpSpecConstant` are the only instructions to consume
/// a [`LiteralConst`] as the last operant and expect exactly one operant.
///
/// This greatly simplifies parsing, as we can simply assume the remaining words of this instruction all contribute to
/// the constant operand. The "correct" way to handle this would be to parse the "result type id", resolve its type
/// from previously parsed instructions and compute its size from the type layout specification. This would make the
/// implementation vastly more complex and likely have a negative impact on decoding performance, which is why we
/// decided to make this assumption about SPIR-V grammars.
///
/// A [`LiteralConst`] can only be decoded with [`OperandEncoding::decode_last`]. Decoding an `LiteralInteger` not as
/// the last operand, aka. calling [`OperandEncoding::decode`], will always return an Error.
///
/// [`Instruction`]: crate::meta::InstMeta
/// [`Quantifier::ZeroOrMore`]: `crate::meta::Quantifier`
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct LiteralConst(SmallVec<[Word; 2]>);

pub type LiteralContextDependentNumber = LiteralConst;

impl From<u64> for LiteralConst {
    #[inline]
    fn from(value: u64) -> Self {
        LiteralConst(SmallVec::from_slice(&[
            Word(value as u32),
            Word((value >> 32) as u32),
        ]))
    }
}

impl From<u32> for LiteralConst {
    #[inline]
    fn from(value: u32) -> Self {
        LiteralConst(SmallVec::from_slice(&[Word(value)]))
    }
}

macro_rules! impl_cast {
    ($ty:ty => $to:ty) => {
        impl From<$ty> for LiteralConst {
            #[inline]
            fn from(value: $ty) -> Self {
                Self::from(value as $to)
            }
        }
    };
}

impl_cast!(u8 => u32);
impl_cast!(u16 => u32);
impl_cast!(i8 => i32);
impl_cast!(i16 => i32);
impl_cast!(i32 => u32);
impl_cast!(i64 => u64);

macro_rules! impl_float {
    ($ty:ty) => {
        impl From<$ty> for LiteralConst {
            #[inline]
            fn from(value: $ty) -> Self {
                Self::from(value.to_bits())
            }
        }
    };
}

impl_float!(f32);
impl_float!(f64);

impl LiteralConst {
    pub fn as_u64(&self) -> Result<u64, DecodeError> {
        if self.0.len() == 2 {
            Ok(self.0[0].0 as u64 | (self.0[1].0 as u64) << 32)
        } else {
            Err(DecodeError::LiteralConstOfWrongWordSize {
                expected_size: 2,
                actual_size: self.0.len(),
            })
        }
    }

    pub fn as_u32(&self) -> Result<u32, DecodeError> {
        if self.0.len() == 1 {
            Ok(self.0[0].0)
        } else {
            Err(DecodeError::LiteralConstOfWrongWordSize {
                expected_size: 1,
                actual_size: self.0.len(),
            })
        }
    }

    pub fn as_u16(&self) -> Result<u16, DecodeError> {
        let value = self.as_u32()?;
        u16::try_from(value).map_err(|_| DecodeError::LiteralConstTooLarge { value, bits: 16 })
    }

    pub fn as_u8(&self) -> Result<u8, DecodeError> {
        let value = self.as_u32()?;
        u8::try_from(value).map_err(|_| DecodeError::LiteralConstTooLarge { value, bits: 8 })
    }
}

macro_rules! as_signed {
    ($uname:ident => $name:ident: $ty:ty) => {
        pub fn $name(&self) -> Result<$ty, DecodeError> {
            Ok(self.$uname()? as $ty)
        }
    };
}

macro_rules! as_float {
    ($uname:ident => $name:ident: $ty:ty) => {
        pub fn $name(&self) -> Result<$ty, DecodeError> {
            Ok(<$ty>::from_bits(self.$uname()?))
        }
    };
}

impl LiteralConst {
    as_signed!(as_u8 => as_i8: i8);
    as_signed!(as_u16 => as_i16: i16);
    as_signed!(as_u32 => as_i32: i32);
    as_signed!(as_u64 => as_i64: i64);
    as_float!(as_u32 => as_f32: f32);
    as_float!(as_u64 => as_f64: f64);
}

unsafe impl Operand for LiteralConst {
    const KIND: &OperandKind = &OPERAND_KIND_LITERAL_CONTEXT_DEPENDENT_NUMBER;
}

unsafe impl OperandEncoding for LiteralConst {
    const FIXED_LEN: Option<usize> = None;

    #[inline]
    fn word_len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        writer.write_iter(self.0.iter().copied());
        Ok(())
    }

    #[inline]
    fn decode(_: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        Err(DecodeError::LiteralConstNotLastOperand)
    }

    #[inline]
    fn decode_last(reader: &mut OperandReader<'_>) -> Result<Self, DecodeError> {
        Ok(Self(reader.collect()))
    }

    #[inline]
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &OperandDisContext<'_>) -> std::fmt::Result {
        let color = ctx.color(AnsiColor::Red.on_default());
        let fmt = ctx
            .id_result_type
            .and_then(|ty| ctx.id_to_const_fmt.get(&ty.0))
            .copied()
            .unwrap_or_default();
        match (self.0.len(), fmt) {
            (1, ConstFmt::Decimal) => write!(f, " {color}{}{color:#}", self.as_u32().unwrap()),
            (2, ConstFmt::Decimal) => write!(f, " {color}{}{color:#}", self.as_u64().unwrap()),
            (1, ConstFmt::LowerHex) => write!(f, " {color}{:x}{color:#}", self.as_u32().unwrap()),
            (2, ConstFmt::LowerHex) => write!(f, " {color}{:x}{color:#}", self.as_u64().unwrap()),
            (1, ConstFmt::UpperHex) => write!(f, " {color}{:X}{color:#}", self.as_u32().unwrap()),
            (2, ConstFmt::UpperHex) => write!(f, " {color}{:X}{color:#}", self.as_u64().unwrap()),
            (1, ConstFmt::Float) => write!(f, " {color}{}{color:#}", self.as_f32().unwrap()),
            (2, ConstFmt::Float) => write!(f, " {color}{}{color:#}", self.as_f64().unwrap()),
            (_, _) => write!(f, " {color}{:?}{color:#}", self.0.as_slice()),
        }
    }
}

/// How the untyped constant value of a [`LiteralConst`] should be formatted
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub enum ConstFmt {
    /// Format constant as a decimal integer
    #[default]
    Decimal,
    /// **INCOMPLIANT** Format constant as a lower case hex value
    ///
    /// Hex values in disassembly are not supported by `spirv-as`
    LowerHex,
    /// **INCOMPLIANT** Format constant as an upper case hex value
    ///
    /// Hex values in disassembly are not supported by `spirv-as`
    UpperHex,
    /// Format constant as a float
    Float,
}
