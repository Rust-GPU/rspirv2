use super::preamble::*;
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Round {
    pub x: IdRef,
}
impl<'a> Inst<'a> for Round {
    const META: &'static InstMeta = &ROUND;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for Round {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "Round{}", self.x.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct RoundEven {
    pub x: IdRef,
}
impl<'a> Inst<'a> for RoundEven {
    const META: &'static InstMeta = &ROUND_EVEN;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for RoundEven {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "RoundEven{}", self.x.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Trunc {
    pub x: IdRef,
}
impl<'a> Inst<'a> for Trunc {
    const META: &'static InstMeta = &TRUNC;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for Trunc {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "Trunc{}", self.x.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FAbs {
    pub x: IdRef,
}
impl<'a> Inst<'a> for FAbs {
    const META: &'static InstMeta = &F_ABS;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for FAbs {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "FAbs{}", self.x.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SAbs {
    pub x: IdRef,
}
impl<'a> Inst<'a> for SAbs {
    const META: &'static InstMeta = &S_ABS;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for SAbs {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "SAbs{}", self.x.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FSign {
    pub x: IdRef,
}
impl<'a> Inst<'a> for FSign {
    const META: &'static InstMeta = &F_SIGN;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for FSign {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "FSign{}", self.x.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SSign {
    pub x: IdRef,
}
impl<'a> Inst<'a> for SSign {
    const META: &'static InstMeta = &S_SIGN;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for SSign {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "SSign{}", self.x.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Floor {
    pub x: IdRef,
}
impl<'a> Inst<'a> for Floor {
    const META: &'static InstMeta = &FLOOR;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for Floor {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "Floor{}", self.x.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Ceil {
    pub x: IdRef,
}
impl<'a> Inst<'a> for Ceil {
    const META: &'static InstMeta = &CEIL;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for Ceil {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "Ceil{}", self.x.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Fract {
    pub x: IdRef,
}
impl<'a> Inst<'a> for Fract {
    const META: &'static InstMeta = &FRACT;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for Fract {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "Fract{}", self.x.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Radians {
    pub degrees: IdRef,
}
impl<'a> Inst<'a> for Radians {
    const META: &'static InstMeta = &RADIANS;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for Radians {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.degrees);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.degrees, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            degrees: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "Radians{}", self.degrees.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Degrees {
    pub radians: IdRef,
}
impl<'a> Inst<'a> for Degrees {
    const META: &'static InstMeta = &DEGREES;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for Degrees {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.radians);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.radians, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            radians: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "Degrees{}", self.radians.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Sin {
    pub x: IdRef,
}
impl<'a> Inst<'a> for Sin {
    const META: &'static InstMeta = &SIN;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for Sin {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "Sin{}", self.x.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Cos {
    pub x: IdRef,
}
impl<'a> Inst<'a> for Cos {
    const META: &'static InstMeta = &COS;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for Cos {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "Cos{}", self.x.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Tan {
    pub x: IdRef,
}
impl<'a> Inst<'a> for Tan {
    const META: &'static InstMeta = &TAN;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for Tan {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "Tan{}", self.x.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Asin {
    pub x: IdRef,
}
impl<'a> Inst<'a> for Asin {
    const META: &'static InstMeta = &ASIN;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for Asin {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "Asin{}", self.x.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Acos {
    pub x: IdRef,
}
impl<'a> Inst<'a> for Acos {
    const META: &'static InstMeta = &ACOS;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for Acos {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "Acos{}", self.x.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Atan {
    pub y_over_x: IdRef,
}
impl<'a> Inst<'a> for Atan {
    const META: &'static InstMeta = &ATAN;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for Atan {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.y_over_x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.y_over_x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            y_over_x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "Atan{}", self.y_over_x.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Sinh {
    pub x: IdRef,
}
impl<'a> Inst<'a> for Sinh {
    const META: &'static InstMeta = &SINH;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for Sinh {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "Sinh{}", self.x.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Cosh {
    pub x: IdRef,
}
impl<'a> Inst<'a> for Cosh {
    const META: &'static InstMeta = &COSH;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for Cosh {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "Cosh{}", self.x.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Tanh {
    pub x: IdRef,
}
impl<'a> Inst<'a> for Tanh {
    const META: &'static InstMeta = &TANH;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for Tanh {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "Tanh{}", self.x.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Asinh {
    pub x: IdRef,
}
impl<'a> Inst<'a> for Asinh {
    const META: &'static InstMeta = &ASINH;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for Asinh {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "Asinh{}", self.x.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Acosh {
    pub x: IdRef,
}
impl<'a> Inst<'a> for Acosh {
    const META: &'static InstMeta = &ACOSH;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for Acosh {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "Acosh{}", self.x.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Atanh {
    pub x: IdRef,
}
impl<'a> Inst<'a> for Atanh {
    const META: &'static InstMeta = &ATANH;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for Atanh {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "Atanh{}", self.x.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Atan2 {
    pub y: IdRef,
    pub x: IdRef,
}
impl<'a> Inst<'a> for Atan2 {
    const META: &'static InstMeta = &ATAN_2;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for Atan2 {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.y) + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.y, &mut *writer)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            y: OperandEncoding::decode(&mut op_reader)?,
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "Atan2{}{}", self.y.dis(_ctx), self.x.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Pow {
    pub x: IdRef,
    pub y: IdRef,
}
impl<'a> Inst<'a> for Pow {
    const META: &'static InstMeta = &POW;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for Pow {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x) + OperandEncoding::word_len(&self.y);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.y, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            y: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "Pow{}{}", self.x.dis(_ctx), self.y.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Exp {
    pub x: IdRef,
}
impl<'a> Inst<'a> for Exp {
    const META: &'static InstMeta = &EXP;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for Exp {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "Exp{}", self.x.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Log {
    pub x: IdRef,
}
impl<'a> Inst<'a> for Log {
    const META: &'static InstMeta = &LOG;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for Log {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "Log{}", self.x.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Exp2 {
    pub x: IdRef,
}
impl<'a> Inst<'a> for Exp2 {
    const META: &'static InstMeta = &EXP_2;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for Exp2 {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "Exp2{}", self.x.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Log2 {
    pub x: IdRef,
}
impl<'a> Inst<'a> for Log2 {
    const META: &'static InstMeta = &LOG_2;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for Log2 {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "Log2{}", self.x.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Sqrt {
    pub x: IdRef,
}
impl<'a> Inst<'a> for Sqrt {
    const META: &'static InstMeta = &SQRT;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for Sqrt {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "Sqrt{}", self.x.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct InverseSqrt {
    pub x: IdRef,
}
impl<'a> Inst<'a> for InverseSqrt {
    const META: &'static InstMeta = &INVERSE_SQRT;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for InverseSqrt {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "InverseSqrt{}", self.x.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Determinant {
    pub x: IdRef,
}
impl<'a> Inst<'a> for Determinant {
    const META: &'static InstMeta = &DETERMINANT;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for Determinant {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "Determinant{}", self.x.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct MatrixInverse {
    pub x: IdRef,
}
impl<'a> Inst<'a> for MatrixInverse {
    const META: &'static InstMeta = &MATRIX_INVERSE;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for MatrixInverse {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "MatrixInverse{}", self.x.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Modf {
    pub x: IdRef,
    pub i: IdRef,
}
impl<'a> Inst<'a> for Modf {
    const META: &'static InstMeta = &MODF;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for Modf {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x) + OperandEncoding::word_len(&self.i);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.i, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            i: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "Modf{}{}", self.x.dis(_ctx), self.i.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ModfStruct {
    pub x: IdRef,
}
impl<'a> Inst<'a> for ModfStruct {
    const META: &'static InstMeta = &MODF_STRUCT;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for ModfStruct {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "ModfStruct{}", self.x.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FMin {
    pub x: IdRef,
    pub y: IdRef,
}
impl<'a> Inst<'a> for FMin {
    const META: &'static InstMeta = &F_MIN;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for FMin {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x) + OperandEncoding::word_len(&self.y);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.y, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            y: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "FMin{}{}", self.x.dis(_ctx), self.y.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UMin {
    pub x: IdRef,
    pub y: IdRef,
}
impl<'a> Inst<'a> for UMin {
    const META: &'static InstMeta = &U_MIN;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for UMin {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x) + OperandEncoding::word_len(&self.y);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.y, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            y: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "UMin{}{}", self.x.dis(_ctx), self.y.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SMin {
    pub x: IdRef,
    pub y: IdRef,
}
impl<'a> Inst<'a> for SMin {
    const META: &'static InstMeta = &S_MIN;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for SMin {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x) + OperandEncoding::word_len(&self.y);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.y, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            y: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "SMin{}{}", self.x.dis(_ctx), self.y.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FMax {
    pub x: IdRef,
    pub y: IdRef,
}
impl<'a> Inst<'a> for FMax {
    const META: &'static InstMeta = &F_MAX;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for FMax {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x) + OperandEncoding::word_len(&self.y);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.y, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            y: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "FMax{}{}", self.x.dis(_ctx), self.y.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UMax {
    pub x: IdRef,
    pub y: IdRef,
}
impl<'a> Inst<'a> for UMax {
    const META: &'static InstMeta = &U_MAX;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for UMax {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x) + OperandEncoding::word_len(&self.y);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.y, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            y: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "UMax{}{}", self.x.dis(_ctx), self.y.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SMax {
    pub x: IdRef,
    pub y: IdRef,
}
impl<'a> Inst<'a> for SMax {
    const META: &'static InstMeta = &S_MAX;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for SMax {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x) + OperandEncoding::word_len(&self.y);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.y, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            y: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "SMax{}{}", self.x.dis(_ctx), self.y.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FClamp {
    pub x: IdRef,
    pub min_val: IdRef,
    pub max_val: IdRef,
}
impl<'a> Inst<'a> for FClamp {
    const META: &'static InstMeta = &F_CLAMP;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for FClamp {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1
            + OperandEncoding::word_len(&self.x)
            + OperandEncoding::word_len(&self.min_val)
            + OperandEncoding::word_len(&self.max_val);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.min_val, &mut *writer)?;
        OperandEncoding::encode(&self.max_val, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            min_val: OperandEncoding::decode(&mut op_reader)?,
            max_val: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(
            f,
            "FClamp{}{}{}",
            self.x.dis(_ctx),
            self.min_val.dis(_ctx),
            self.max_val.dis(_ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UClamp {
    pub x: IdRef,
    pub min_val: IdRef,
    pub max_val: IdRef,
}
impl<'a> Inst<'a> for UClamp {
    const META: &'static InstMeta = &U_CLAMP;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for UClamp {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1
            + OperandEncoding::word_len(&self.x)
            + OperandEncoding::word_len(&self.min_val)
            + OperandEncoding::word_len(&self.max_val);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.min_val, &mut *writer)?;
        OperandEncoding::encode(&self.max_val, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            min_val: OperandEncoding::decode(&mut op_reader)?,
            max_val: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(
            f,
            "UClamp{}{}{}",
            self.x.dis(_ctx),
            self.min_val.dis(_ctx),
            self.max_val.dis(_ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SClamp {
    pub x: IdRef,
    pub min_val: IdRef,
    pub max_val: IdRef,
}
impl<'a> Inst<'a> for SClamp {
    const META: &'static InstMeta = &S_CLAMP;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for SClamp {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1
            + OperandEncoding::word_len(&self.x)
            + OperandEncoding::word_len(&self.min_val)
            + OperandEncoding::word_len(&self.max_val);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.min_val, &mut *writer)?;
        OperandEncoding::encode(&self.max_val, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            min_val: OperandEncoding::decode(&mut op_reader)?,
            max_val: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(
            f,
            "SClamp{}{}{}",
            self.x.dis(_ctx),
            self.min_val.dis(_ctx),
            self.max_val.dis(_ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FMix {
    pub x: IdRef,
    pub y: IdRef,
    pub a: IdRef,
}
impl<'a> Inst<'a> for FMix {
    const META: &'static InstMeta = &F_MIX;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for FMix {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1
            + OperandEncoding::word_len(&self.x)
            + OperandEncoding::word_len(&self.y)
            + OperandEncoding::word_len(&self.a);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.y, &mut *writer)?;
        OperandEncoding::encode(&self.a, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            y: OperandEncoding::decode(&mut op_reader)?,
            a: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(
            f,
            "FMix{}{}{}",
            self.x.dis(_ctx),
            self.y.dis(_ctx),
            self.a.dis(_ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct IMix {
    pub x: IdRef,
    pub y: IdRef,
    pub a: IdRef,
}
impl<'a> Inst<'a> for IMix {
    const META: &'static InstMeta = &I_MIX;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for IMix {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1
            + OperandEncoding::word_len(&self.x)
            + OperandEncoding::word_len(&self.y)
            + OperandEncoding::word_len(&self.a);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.y, &mut *writer)?;
        OperandEncoding::encode(&self.a, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            y: OperandEncoding::decode(&mut op_reader)?,
            a: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(
            f,
            "IMix{}{}{}",
            self.x.dis(_ctx),
            self.y.dis(_ctx),
            self.a.dis(_ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Step {
    pub edge: IdRef,
    pub x: IdRef,
}
impl<'a> Inst<'a> for Step {
    const META: &'static InstMeta = &STEP;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for Step {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.edge) + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.edge, &mut *writer)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            edge: OperandEncoding::decode(&mut op_reader)?,
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "Step{}{}", self.edge.dis(_ctx), self.x.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SmoothStep {
    pub edge_0: IdRef,
    pub edge_1: IdRef,
    pub x: IdRef,
}
impl<'a> Inst<'a> for SmoothStep {
    const META: &'static InstMeta = &SMOOTH_STEP;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for SmoothStep {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1
            + OperandEncoding::word_len(&self.edge_0)
            + OperandEncoding::word_len(&self.edge_1)
            + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.edge_0, &mut *writer)?;
        OperandEncoding::encode(&self.edge_1, &mut *writer)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            edge_0: OperandEncoding::decode(&mut op_reader)?,
            edge_1: OperandEncoding::decode(&mut op_reader)?,
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(
            f,
            "SmoothStep{}{}{}",
            self.edge_0.dis(_ctx),
            self.edge_1.dis(_ctx),
            self.x.dis(_ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Fma {
    pub a: IdRef,
    pub b: IdRef,
    pub c: IdRef,
}
impl<'a> Inst<'a> for Fma {
    const META: &'static InstMeta = &FMA;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for Fma {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1
            + OperandEncoding::word_len(&self.a)
            + OperandEncoding::word_len(&self.b)
            + OperandEncoding::word_len(&self.c);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.a, &mut *writer)?;
        OperandEncoding::encode(&self.b, &mut *writer)?;
        OperandEncoding::encode(&self.c, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            a: OperandEncoding::decode(&mut op_reader)?,
            b: OperandEncoding::decode(&mut op_reader)?,
            c: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(
            f,
            "Fma{}{}{}",
            self.a.dis(_ctx),
            self.b.dis(_ctx),
            self.c.dis(_ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Frexp {
    pub x: IdRef,
    pub exp: IdRef,
}
impl<'a> Inst<'a> for Frexp {
    const META: &'static InstMeta = &FREXP;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for Frexp {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x) + OperandEncoding::word_len(&self.exp);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.exp, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            exp: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "Frexp{}{}", self.x.dis(_ctx), self.exp.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FrexpStruct {
    pub x: IdRef,
}
impl<'a> Inst<'a> for FrexpStruct {
    const META: &'static InstMeta = &FREXP_STRUCT;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for FrexpStruct {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "FrexpStruct{}", self.x.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Ldexp {
    pub x: IdRef,
    pub exp: IdRef,
}
impl<'a> Inst<'a> for Ldexp {
    const META: &'static InstMeta = &LDEXP;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for Ldexp {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x) + OperandEncoding::word_len(&self.exp);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.exp, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            exp: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "Ldexp{}{}", self.x.dis(_ctx), self.exp.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PackSnorm4x8 {
    pub v: IdRef,
}
impl<'a> Inst<'a> for PackSnorm4x8 {
    const META: &'static InstMeta = &PACK_SNORM_4_X_8;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for PackSnorm4x8 {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.v);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.v, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            v: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "PackSnorm4x8{}", self.v.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PackUnorm4x8 {
    pub v: IdRef,
}
impl<'a> Inst<'a> for PackUnorm4x8 {
    const META: &'static InstMeta = &PACK_UNORM_4_X_8;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for PackUnorm4x8 {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.v);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.v, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            v: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "PackUnorm4x8{}", self.v.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PackSnorm2x16 {
    pub v: IdRef,
}
impl<'a> Inst<'a> for PackSnorm2x16 {
    const META: &'static InstMeta = &PACK_SNORM_2_X_16;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for PackSnorm2x16 {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.v);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.v, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            v: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "PackSnorm2x16{}", self.v.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PackUnorm2x16 {
    pub v: IdRef,
}
impl<'a> Inst<'a> for PackUnorm2x16 {
    const META: &'static InstMeta = &PACK_UNORM_2_X_16;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for PackUnorm2x16 {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.v);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.v, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            v: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "PackUnorm2x16{}", self.v.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PackHalf2x16 {
    pub v: IdRef,
}
impl<'a> Inst<'a> for PackHalf2x16 {
    const META: &'static InstMeta = &PACK_HALF_2_X_16;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for PackHalf2x16 {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.v);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.v, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            v: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "PackHalf2x16{}", self.v.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PackDouble2x32 {
    pub v: IdRef,
}
impl<'a> Inst<'a> for PackDouble2x32 {
    const META: &'static InstMeta = &PACK_DOUBLE_2_X_32;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for PackDouble2x32 {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.v);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.v, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            v: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "PackDouble2x32{}", self.v.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnpackSnorm2x16 {
    pub p: IdRef,
}
impl<'a> Inst<'a> for UnpackSnorm2x16 {
    const META: &'static InstMeta = &UNPACK_SNORM_2_X_16;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for UnpackSnorm2x16 {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.p);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.p, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            p: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "UnpackSnorm2x16{}", self.p.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnpackUnorm2x16 {
    pub p: IdRef,
}
impl<'a> Inst<'a> for UnpackUnorm2x16 {
    const META: &'static InstMeta = &UNPACK_UNORM_2_X_16;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for UnpackUnorm2x16 {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.p);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.p, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            p: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "UnpackUnorm2x16{}", self.p.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnpackHalf2x16 {
    pub v: IdRef,
}
impl<'a> Inst<'a> for UnpackHalf2x16 {
    const META: &'static InstMeta = &UNPACK_HALF_2_X_16;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for UnpackHalf2x16 {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.v);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.v, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            v: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "UnpackHalf2x16{}", self.v.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnpackSnorm4x8 {
    pub p: IdRef,
}
impl<'a> Inst<'a> for UnpackSnorm4x8 {
    const META: &'static InstMeta = &UNPACK_SNORM_4_X_8;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for UnpackSnorm4x8 {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.p);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.p, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            p: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "UnpackSnorm4x8{}", self.p.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnpackUnorm4x8 {
    pub p: IdRef,
}
impl<'a> Inst<'a> for UnpackUnorm4x8 {
    const META: &'static InstMeta = &UNPACK_UNORM_4_X_8;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for UnpackUnorm4x8 {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.p);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.p, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            p: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "UnpackUnorm4x8{}", self.p.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnpackDouble2x32 {
    pub v: IdRef,
}
impl<'a> Inst<'a> for UnpackDouble2x32 {
    const META: &'static InstMeta = &UNPACK_DOUBLE_2_X_32;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for UnpackDouble2x32 {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.v);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.v, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            v: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "UnpackDouble2x32{}", self.v.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Length {
    pub x: IdRef,
}
impl<'a> Inst<'a> for Length {
    const META: &'static InstMeta = &LENGTH;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for Length {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "Length{}", self.x.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Distance {
    pub p_0: IdRef,
    pub p_1: IdRef,
}
impl<'a> Inst<'a> for Distance {
    const META: &'static InstMeta = &DISTANCE;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for Distance {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.p_0) + OperandEncoding::word_len(&self.p_1);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.p_0, &mut *writer)?;
        OperandEncoding::encode(&self.p_1, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            p_0: OperandEncoding::decode(&mut op_reader)?,
            p_1: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "Distance{}{}", self.p_0.dis(_ctx), self.p_1.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Cross {
    pub x: IdRef,
    pub y: IdRef,
}
impl<'a> Inst<'a> for Cross {
    const META: &'static InstMeta = &CROSS;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for Cross {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x) + OperandEncoding::word_len(&self.y);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.y, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            y: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "Cross{}{}", self.x.dis(_ctx), self.y.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Normalize {
    pub x: IdRef,
}
impl<'a> Inst<'a> for Normalize {
    const META: &'static InstMeta = &NORMALIZE;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for Normalize {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "Normalize{}", self.x.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FaceForward {
    pub n: IdRef,
    pub i: IdRef,
    pub nref: IdRef,
}
impl<'a> Inst<'a> for FaceForward {
    const META: &'static InstMeta = &FACE_FORWARD;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for FaceForward {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1
            + OperandEncoding::word_len(&self.n)
            + OperandEncoding::word_len(&self.i)
            + OperandEncoding::word_len(&self.nref);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.n, &mut *writer)?;
        OperandEncoding::encode(&self.i, &mut *writer)?;
        OperandEncoding::encode(&self.nref, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            n: OperandEncoding::decode(&mut op_reader)?,
            i: OperandEncoding::decode(&mut op_reader)?,
            nref: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(
            f,
            "FaceForward{}{}{}",
            self.n.dis(_ctx),
            self.i.dis(_ctx),
            self.nref.dis(_ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Reflect {
    pub i: IdRef,
    pub n: IdRef,
}
impl<'a> Inst<'a> for Reflect {
    const META: &'static InstMeta = &REFLECT;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for Reflect {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.i) + OperandEncoding::word_len(&self.n);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.i, &mut *writer)?;
        OperandEncoding::encode(&self.n, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            i: OperandEncoding::decode(&mut op_reader)?,
            n: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "Reflect{}{}", self.i.dis(_ctx), self.n.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Refract {
    pub i: IdRef,
    pub n: IdRef,
    pub eta: IdRef,
}
impl<'a> Inst<'a> for Refract {
    const META: &'static InstMeta = &REFRACT;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for Refract {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1
            + OperandEncoding::word_len(&self.i)
            + OperandEncoding::word_len(&self.n)
            + OperandEncoding::word_len(&self.eta);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.i, &mut *writer)?;
        OperandEncoding::encode(&self.n, &mut *writer)?;
        OperandEncoding::encode(&self.eta, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            i: OperandEncoding::decode(&mut op_reader)?,
            n: OperandEncoding::decode(&mut op_reader)?,
            eta: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(
            f,
            "Refract{}{}{}",
            self.i.dis(_ctx),
            self.n.dis(_ctx),
            self.eta.dis(_ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FindILsb {
    pub value: IdRef,
}
impl<'a> Inst<'a> for FindILsb {
    const META: &'static InstMeta = &FIND_I_LSB;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for FindILsb {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.value);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.value, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            value: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "FindILsb{}", self.value.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FindSMsb {
    pub value: IdRef,
}
impl<'a> Inst<'a> for FindSMsb {
    const META: &'static InstMeta = &FIND_S_MSB;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for FindSMsb {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.value);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.value, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            value: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "FindSMsb{}", self.value.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FindUMsb {
    pub value: IdRef,
}
impl<'a> Inst<'a> for FindUMsb {
    const META: &'static InstMeta = &FIND_U_MSB;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for FindUMsb {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.value);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.value, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            value: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "FindUMsb{}", self.value.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct InterpolateAtCentroid {
    pub interpolant: IdRef,
}
impl<'a> Inst<'a> for InterpolateAtCentroid {
    const META: &'static InstMeta = &INTERPOLATE_AT_CENTROID;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for InterpolateAtCentroid {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.interpolant);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.interpolant, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            interpolant: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "InterpolateAtCentroid{}", self.interpolant.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct InterpolateAtSample {
    pub interpolant: IdRef,
    pub sample: IdRef,
}
impl<'a> Inst<'a> for InterpolateAtSample {
    const META: &'static InstMeta = &INTERPOLATE_AT_SAMPLE;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for InterpolateAtSample {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1
            + OperandEncoding::word_len(&self.interpolant)
            + OperandEncoding::word_len(&self.sample);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.interpolant, &mut *writer)?;
        OperandEncoding::encode(&self.sample, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            interpolant: OperandEncoding::decode(&mut op_reader)?,
            sample: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(
            f,
            "InterpolateAtSample{}{}",
            self.interpolant.dis(_ctx),
            self.sample.dis(_ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct InterpolateAtOffset {
    pub interpolant: IdRef,
    pub offset: IdRef,
}
impl<'a> Inst<'a> for InterpolateAtOffset {
    const META: &'static InstMeta = &INTERPOLATE_AT_OFFSET;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for InterpolateAtOffset {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1
            + OperandEncoding::word_len(&self.interpolant)
            + OperandEncoding::word_len(&self.offset);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.interpolant, &mut *writer)?;
        OperandEncoding::encode(&self.offset, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            interpolant: OperandEncoding::decode(&mut op_reader)?,
            offset: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(
            f,
            "InterpolateAtOffset{}{}",
            self.interpolant.dis(_ctx),
            self.offset.dis(_ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct NMin {
    pub x: IdRef,
    pub y: IdRef,
}
impl<'a> Inst<'a> for NMin {
    const META: &'static InstMeta = &N_MIN;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for NMin {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x) + OperandEncoding::word_len(&self.y);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.y, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            y: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "NMin{}{}", self.x.dis(_ctx), self.y.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct NMax {
    pub x: IdRef,
    pub y: IdRef,
}
impl<'a> Inst<'a> for NMax {
    const META: &'static InstMeta = &N_MAX;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for NMax {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x) + OperandEncoding::word_len(&self.y);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.y, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            y: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(f, "NMax{}{}", self.x.dis(_ctx), self.y.dis(_ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct NClamp {
    pub x: IdRef,
    pub min_val: IdRef,
    pub max_val: IdRef,
}
impl<'a> Inst<'a> for NClamp {
    const META: &'static InstMeta = &N_CLAMP;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for NClamp {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1
            + OperandEncoding::word_len(&self.x)
            + OperandEncoding::word_len(&self.min_val)
            + OperandEncoding::word_len(&self.max_val);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.min_val, &mut *writer)?;
        OperandEncoding::encode(&self.max_val, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            min_val: OperandEncoding::decode(&mut op_reader)?,
            max_val: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(
            f,
            "NClamp{}{}{}",
            self.x.dis(_ctx),
            self.min_val.dis(_ctx),
            self.max_val.dis(_ctx)
        )
    }
}
