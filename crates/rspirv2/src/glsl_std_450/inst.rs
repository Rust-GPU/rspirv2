use super::preamble::*;
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Round {
    pub x: IdRef,
}
impl Inst for Round {
    const META: &InstMeta = &ROUND;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for Round {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct RoundEven {
    pub x: IdRef,
}
impl Inst for RoundEven {
    const META: &InstMeta = &ROUND_EVEN;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for RoundEven {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Trunc {
    pub x: IdRef,
}
impl Inst for Trunc {
    const META: &InstMeta = &TRUNC;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for Trunc {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FAbs {
    pub x: IdRef,
}
impl Inst for FAbs {
    const META: &InstMeta = &F_ABS;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for FAbs {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SAbs {
    pub x: IdRef,
}
impl Inst for SAbs {
    const META: &InstMeta = &S_ABS;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for SAbs {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FSign {
    pub x: IdRef,
}
impl Inst for FSign {
    const META: &InstMeta = &F_SIGN;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for FSign {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SSign {
    pub x: IdRef,
}
impl Inst for SSign {
    const META: &InstMeta = &S_SIGN;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for SSign {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Floor {
    pub x: IdRef,
}
impl Inst for Floor {
    const META: &InstMeta = &FLOOR;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for Floor {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Ceil {
    pub x: IdRef,
}
impl Inst for Ceil {
    const META: &InstMeta = &CEIL;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for Ceil {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Fract {
    pub x: IdRef,
}
impl Inst for Fract {
    const META: &InstMeta = &FRACT;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for Fract {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Radians {
    pub degrees: IdRef,
}
impl Inst for Radians {
    const META: &InstMeta = &RADIANS;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for Radians {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.degrees);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.degrees, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            degrees: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Degrees {
    pub radians: IdRef,
}
impl Inst for Degrees {
    const META: &InstMeta = &DEGREES;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for Degrees {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.radians);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.radians, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            radians: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Sin {
    pub x: IdRef,
}
impl Inst for Sin {
    const META: &InstMeta = &SIN;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for Sin {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Cos {
    pub x: IdRef,
}
impl Inst for Cos {
    const META: &InstMeta = &COS;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for Cos {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Tan {
    pub x: IdRef,
}
impl Inst for Tan {
    const META: &InstMeta = &TAN;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for Tan {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Asin {
    pub x: IdRef,
}
impl Inst for Asin {
    const META: &InstMeta = &ASIN;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for Asin {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Acos {
    pub x: IdRef,
}
impl Inst for Acos {
    const META: &InstMeta = &ACOS;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for Acos {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Atan {
    pub y_over_x: IdRef,
}
impl Inst for Atan {
    const META: &InstMeta = &ATAN;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for Atan {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.y_over_x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.y_over_x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            y_over_x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Sinh {
    pub x: IdRef,
}
impl Inst for Sinh {
    const META: &InstMeta = &SINH;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for Sinh {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Cosh {
    pub x: IdRef,
}
impl Inst for Cosh {
    const META: &InstMeta = &COSH;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for Cosh {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Tanh {
    pub x: IdRef,
}
impl Inst for Tanh {
    const META: &InstMeta = &TANH;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for Tanh {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Asinh {
    pub x: IdRef,
}
impl Inst for Asinh {
    const META: &InstMeta = &ASINH;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for Asinh {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Acosh {
    pub x: IdRef,
}
impl Inst for Acosh {
    const META: &InstMeta = &ACOSH;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for Acosh {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Atanh {
    pub x: IdRef,
}
impl Inst for Atanh {
    const META: &InstMeta = &ATANH;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for Atanh {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Atan2 {
    pub y: IdRef,
    pub x: IdRef,
}
impl Inst for Atan2 {
    const META: &InstMeta = &ATAN_2;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for Atan2 {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.y) + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.y, &mut *writer)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            y: OperandEncoding::decode(&mut op_reader)?,
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Pow {
    pub x: IdRef,
    pub y: IdRef,
}
impl Inst for Pow {
    const META: &InstMeta = &POW;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for Pow {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x) + OperandEncoding::word_len(&self.y);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.y, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            y: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Exp {
    pub x: IdRef,
}
impl Inst for Exp {
    const META: &InstMeta = &EXP;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for Exp {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Log {
    pub x: IdRef,
}
impl Inst for Log {
    const META: &InstMeta = &LOG;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for Log {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Exp2 {
    pub x: IdRef,
}
impl Inst for Exp2 {
    const META: &InstMeta = &EXP_2;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for Exp2 {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Log2 {
    pub x: IdRef,
}
impl Inst for Log2 {
    const META: &InstMeta = &LOG_2;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for Log2 {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Sqrt {
    pub x: IdRef,
}
impl Inst for Sqrt {
    const META: &InstMeta = &SQRT;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for Sqrt {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct InverseSqrt {
    pub x: IdRef,
}
impl Inst for InverseSqrt {
    const META: &InstMeta = &INVERSE_SQRT;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for InverseSqrt {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Determinant {
    pub x: IdRef,
}
impl Inst for Determinant {
    const META: &InstMeta = &DETERMINANT;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for Determinant {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct MatrixInverse {
    pub x: IdRef,
}
impl Inst for MatrixInverse {
    const META: &InstMeta = &MATRIX_INVERSE;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for MatrixInverse {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Modf {
    pub x: IdRef,
    pub i: IdRef,
}
impl Inst for Modf {
    const META: &InstMeta = &MODF;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for Modf {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x) + OperandEncoding::word_len(&self.i);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.i, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            i: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ModfStruct {
    pub x: IdRef,
}
impl Inst for ModfStruct {
    const META: &InstMeta = &MODF_STRUCT;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for ModfStruct {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FMin {
    pub x: IdRef,
    pub y: IdRef,
}
impl Inst for FMin {
    const META: &InstMeta = &F_MIN;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for FMin {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x) + OperandEncoding::word_len(&self.y);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.y, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            y: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UMin {
    pub x: IdRef,
    pub y: IdRef,
}
impl Inst for UMin {
    const META: &InstMeta = &U_MIN;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for UMin {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x) + OperandEncoding::word_len(&self.y);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.y, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            y: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SMin {
    pub x: IdRef,
    pub y: IdRef,
}
impl Inst for SMin {
    const META: &InstMeta = &S_MIN;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for SMin {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x) + OperandEncoding::word_len(&self.y);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.y, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            y: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FMax {
    pub x: IdRef,
    pub y: IdRef,
}
impl Inst for FMax {
    const META: &InstMeta = &F_MAX;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for FMax {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x) + OperandEncoding::word_len(&self.y);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.y, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            y: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UMax {
    pub x: IdRef,
    pub y: IdRef,
}
impl Inst for UMax {
    const META: &InstMeta = &U_MAX;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for UMax {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x) + OperandEncoding::word_len(&self.y);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.y, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            y: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SMax {
    pub x: IdRef,
    pub y: IdRef,
}
impl Inst for SMax {
    const META: &InstMeta = &S_MAX;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for SMax {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x) + OperandEncoding::word_len(&self.y);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.y, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            y: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FClamp {
    pub x: IdRef,
    pub min_val: IdRef,
    pub max_val: IdRef,
}
impl Inst for FClamp {
    const META: &InstMeta = &F_CLAMP;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for FClamp {
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
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            min_val: OperandEncoding::decode(&mut op_reader)?,
            max_val: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UClamp {
    pub x: IdRef,
    pub min_val: IdRef,
    pub max_val: IdRef,
}
impl Inst for UClamp {
    const META: &InstMeta = &U_CLAMP;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for UClamp {
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
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            min_val: OperandEncoding::decode(&mut op_reader)?,
            max_val: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SClamp {
    pub x: IdRef,
    pub min_val: IdRef,
    pub max_val: IdRef,
}
impl Inst for SClamp {
    const META: &InstMeta = &S_CLAMP;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for SClamp {
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
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            min_val: OperandEncoding::decode(&mut op_reader)?,
            max_val: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FMix {
    pub x: IdRef,
    pub y: IdRef,
    pub a: IdRef,
}
impl Inst for FMix {
    const META: &InstMeta = &F_MIX;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for FMix {
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
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            y: OperandEncoding::decode(&mut op_reader)?,
            a: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct IMix {
    pub x: IdRef,
    pub y: IdRef,
    pub a: IdRef,
}
impl Inst for IMix {
    const META: &InstMeta = &I_MIX;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for IMix {
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
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            y: OperandEncoding::decode(&mut op_reader)?,
            a: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Step {
    pub edge: IdRef,
    pub x: IdRef,
}
impl Inst for Step {
    const META: &InstMeta = &STEP;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for Step {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.edge) + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.edge, &mut *writer)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            edge: OperandEncoding::decode(&mut op_reader)?,
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SmoothStep {
    pub edge_0: IdRef,
    pub edge_1: IdRef,
    pub x: IdRef,
}
impl Inst for SmoothStep {
    const META: &InstMeta = &SMOOTH_STEP;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for SmoothStep {
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
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            edge_0: OperandEncoding::decode(&mut op_reader)?,
            edge_1: OperandEncoding::decode(&mut op_reader)?,
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Fma {
    pub a: IdRef,
    pub b: IdRef,
    pub c: IdRef,
}
impl Inst for Fma {
    const META: &InstMeta = &FMA;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for Fma {
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
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            a: OperandEncoding::decode(&mut op_reader)?,
            b: OperandEncoding::decode(&mut op_reader)?,
            c: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Frexp {
    pub x: IdRef,
    pub exp: IdRef,
}
impl Inst for Frexp {
    const META: &InstMeta = &FREXP;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for Frexp {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x) + OperandEncoding::word_len(&self.exp);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.exp, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            exp: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FrexpStruct {
    pub x: IdRef,
}
impl Inst for FrexpStruct {
    const META: &InstMeta = &FREXP_STRUCT;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for FrexpStruct {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Ldexp {
    pub x: IdRef,
    pub exp: IdRef,
}
impl Inst for Ldexp {
    const META: &InstMeta = &LDEXP;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for Ldexp {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x) + OperandEncoding::word_len(&self.exp);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.exp, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            exp: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PackSnorm4x8 {
    pub v: IdRef,
}
impl Inst for PackSnorm4x8 {
    const META: &InstMeta = &PACK_SNORM_4_X_8;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for PackSnorm4x8 {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.v);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.v, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            v: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PackUnorm4x8 {
    pub v: IdRef,
}
impl Inst for PackUnorm4x8 {
    const META: &InstMeta = &PACK_UNORM_4_X_8;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for PackUnorm4x8 {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.v);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.v, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            v: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PackSnorm2x16 {
    pub v: IdRef,
}
impl Inst for PackSnorm2x16 {
    const META: &InstMeta = &PACK_SNORM_2_X_16;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for PackSnorm2x16 {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.v);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.v, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            v: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PackUnorm2x16 {
    pub v: IdRef,
}
impl Inst for PackUnorm2x16 {
    const META: &InstMeta = &PACK_UNORM_2_X_16;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for PackUnorm2x16 {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.v);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.v, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            v: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PackHalf2x16 {
    pub v: IdRef,
}
impl Inst for PackHalf2x16 {
    const META: &InstMeta = &PACK_HALF_2_X_16;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for PackHalf2x16 {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.v);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.v, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            v: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PackDouble2x32 {
    pub v: IdRef,
}
impl Inst for PackDouble2x32 {
    const META: &InstMeta = &PACK_DOUBLE_2_X_32;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for PackDouble2x32 {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.v);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.v, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            v: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnpackSnorm2x16 {
    pub p: IdRef,
}
impl Inst for UnpackSnorm2x16 {
    const META: &InstMeta = &UNPACK_SNORM_2_X_16;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for UnpackSnorm2x16 {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.p);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.p, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            p: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnpackUnorm2x16 {
    pub p: IdRef,
}
impl Inst for UnpackUnorm2x16 {
    const META: &InstMeta = &UNPACK_UNORM_2_X_16;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for UnpackUnorm2x16 {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.p);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.p, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            p: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnpackHalf2x16 {
    pub v: IdRef,
}
impl Inst for UnpackHalf2x16 {
    const META: &InstMeta = &UNPACK_HALF_2_X_16;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for UnpackHalf2x16 {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.v);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.v, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            v: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnpackSnorm4x8 {
    pub p: IdRef,
}
impl Inst for UnpackSnorm4x8 {
    const META: &InstMeta = &UNPACK_SNORM_4_X_8;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for UnpackSnorm4x8 {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.p);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.p, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            p: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnpackUnorm4x8 {
    pub p: IdRef,
}
impl Inst for UnpackUnorm4x8 {
    const META: &InstMeta = &UNPACK_UNORM_4_X_8;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for UnpackUnorm4x8 {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.p);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.p, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            p: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnpackDouble2x32 {
    pub v: IdRef,
}
impl Inst for UnpackDouble2x32 {
    const META: &InstMeta = &UNPACK_DOUBLE_2_X_32;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for UnpackDouble2x32 {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.v);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.v, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            v: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Length {
    pub x: IdRef,
}
impl Inst for Length {
    const META: &InstMeta = &LENGTH;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for Length {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Distance {
    pub p_0: IdRef,
    pub p_1: IdRef,
}
impl Inst for Distance {
    const META: &InstMeta = &DISTANCE;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for Distance {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.p_0) + OperandEncoding::word_len(&self.p_1);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.p_0, &mut *writer)?;
        OperandEncoding::encode(&self.p_1, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            p_0: OperandEncoding::decode(&mut op_reader)?,
            p_1: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Cross {
    pub x: IdRef,
    pub y: IdRef,
}
impl Inst for Cross {
    const META: &InstMeta = &CROSS;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for Cross {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x) + OperandEncoding::word_len(&self.y);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.y, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            y: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Normalize {
    pub x: IdRef,
}
impl Inst for Normalize {
    const META: &InstMeta = &NORMALIZE;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for Normalize {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FaceForward {
    pub n: IdRef,
    pub i: IdRef,
    pub nref: IdRef,
}
impl Inst for FaceForward {
    const META: &InstMeta = &FACE_FORWARD;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for FaceForward {
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
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            n: OperandEncoding::decode(&mut op_reader)?,
            i: OperandEncoding::decode(&mut op_reader)?,
            nref: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Reflect {
    pub i: IdRef,
    pub n: IdRef,
}
impl Inst for Reflect {
    const META: &InstMeta = &REFLECT;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for Reflect {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.i) + OperandEncoding::word_len(&self.n);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.i, &mut *writer)?;
        OperandEncoding::encode(&self.n, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            i: OperandEncoding::decode(&mut op_reader)?,
            n: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Refract {
    pub i: IdRef,
    pub n: IdRef,
    pub eta: IdRef,
}
impl Inst for Refract {
    const META: &InstMeta = &REFRACT;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for Refract {
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
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            i: OperandEncoding::decode(&mut op_reader)?,
            n: OperandEncoding::decode(&mut op_reader)?,
            eta: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FindILsb {
    pub value: IdRef,
}
impl Inst for FindILsb {
    const META: &InstMeta = &FIND_I_LSB;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for FindILsb {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.value);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.value, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            value: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FindSMsb {
    pub value: IdRef,
}
impl Inst for FindSMsb {
    const META: &InstMeta = &FIND_S_MSB;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for FindSMsb {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.value);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.value, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            value: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FindUMsb {
    pub value: IdRef,
}
impl Inst for FindUMsb {
    const META: &InstMeta = &FIND_U_MSB;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for FindUMsb {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.value);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.value, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            value: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct InterpolateAtCentroid {
    pub interpolant: IdRef,
}
impl Inst for InterpolateAtCentroid {
    const META: &InstMeta = &INTERPOLATE_AT_CENTROID;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for InterpolateAtCentroid {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.interpolant);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.interpolant, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            interpolant: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct InterpolateAtSample {
    pub interpolant: IdRef,
    pub sample: IdRef,
}
impl Inst for InterpolateAtSample {
    const META: &InstMeta = &INTERPOLATE_AT_SAMPLE;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for InterpolateAtSample {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1
            + OperandEncoding::word_len(&self.interpolant)
            + OperandEncoding::word_len(&self.sample);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.interpolant, &mut *writer)?;
        OperandEncoding::encode(&self.sample, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            interpolant: OperandEncoding::decode(&mut op_reader)?,
            sample: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct InterpolateAtOffset {
    pub interpolant: IdRef,
    pub offset: IdRef,
}
impl Inst for InterpolateAtOffset {
    const META: &InstMeta = &INTERPOLATE_AT_OFFSET;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for InterpolateAtOffset {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1
            + OperandEncoding::word_len(&self.interpolant)
            + OperandEncoding::word_len(&self.offset);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.interpolant, &mut *writer)?;
        OperandEncoding::encode(&self.offset, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            interpolant: OperandEncoding::decode(&mut op_reader)?,
            offset: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct NMin {
    pub x: IdRef,
    pub y: IdRef,
}
impl Inst for NMin {
    const META: &InstMeta = &N_MIN;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for NMin {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x) + OperandEncoding::word_len(&self.y);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.y, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            y: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct NMax {
    pub x: IdRef,
    pub y: IdRef,
}
impl Inst for NMax {
    const META: &InstMeta = &N_MAX;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for NMax {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len = 1 + OperandEncoding::word_len(&self.x) + OperandEncoding::word_len(&self.y);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.y, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            y: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct NClamp {
    pub x: IdRef,
    pub min_val: IdRef,
    pub max_val: IdRef,
}
impl Inst for NClamp {
    const META: &InstMeta = &N_CLAMP;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl InstEncoding for NClamp {
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
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            min_val: OperandEncoding::decode(&mut op_reader)?,
            max_val: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
}
