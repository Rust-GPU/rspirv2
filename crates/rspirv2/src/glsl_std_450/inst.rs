use super::preamble::*;
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Round {
    pub x: IdRef,
}
impl Inst for Round {
    const META: &InstMeta = &ROUND;
}
impl InstEncoding for Round {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        profiling::function_scope!();
        let ctx = &OperandDisContext {
            id_result: None,
            id_result_type: None,
            ctx,
        };
        write!(f, "{}Round{}", ctx.id_result_writer(), self.x.dis(ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct RoundEven {
    pub x: IdRef,
}
impl Inst for RoundEven {
    const META: &InstMeta = &ROUND_EVEN;
}
impl InstEncoding for RoundEven {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        profiling::function_scope!();
        let ctx = &OperandDisContext {
            id_result: None,
            id_result_type: None,
            ctx,
        };
        write!(f, "{}RoundEven{}", ctx.id_result_writer(), self.x.dis(ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Trunc {
    pub x: IdRef,
}
impl Inst for Trunc {
    const META: &InstMeta = &TRUNC;
}
impl InstEncoding for Trunc {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        profiling::function_scope!();
        let ctx = &OperandDisContext {
            id_result: None,
            id_result_type: None,
            ctx,
        };
        write!(f, "{}Trunc{}", ctx.id_result_writer(), self.x.dis(ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FAbs {
    pub x: IdRef,
}
impl Inst for FAbs {
    const META: &InstMeta = &F_ABS;
}
impl InstEncoding for FAbs {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        profiling::function_scope!();
        let ctx = &OperandDisContext {
            id_result: None,
            id_result_type: None,
            ctx,
        };
        write!(f, "{}FAbs{}", ctx.id_result_writer(), self.x.dis(ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SAbs {
    pub x: IdRef,
}
impl Inst for SAbs {
    const META: &InstMeta = &S_ABS;
}
impl InstEncoding for SAbs {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        profiling::function_scope!();
        let ctx = &OperandDisContext {
            id_result: None,
            id_result_type: None,
            ctx,
        };
        write!(f, "{}SAbs{}", ctx.id_result_writer(), self.x.dis(ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FSign {
    pub x: IdRef,
}
impl Inst for FSign {
    const META: &InstMeta = &F_SIGN;
}
impl InstEncoding for FSign {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        profiling::function_scope!();
        let ctx = &OperandDisContext {
            id_result: None,
            id_result_type: None,
            ctx,
        };
        write!(f, "{}FSign{}", ctx.id_result_writer(), self.x.dis(ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SSign {
    pub x: IdRef,
}
impl Inst for SSign {
    const META: &InstMeta = &S_SIGN;
}
impl InstEncoding for SSign {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        profiling::function_scope!();
        let ctx = &OperandDisContext {
            id_result: None,
            id_result_type: None,
            ctx,
        };
        write!(f, "{}SSign{}", ctx.id_result_writer(), self.x.dis(ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Floor {
    pub x: IdRef,
}
impl Inst for Floor {
    const META: &InstMeta = &FLOOR;
}
impl InstEncoding for Floor {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        profiling::function_scope!();
        let ctx = &OperandDisContext {
            id_result: None,
            id_result_type: None,
            ctx,
        };
        write!(f, "{}Floor{}", ctx.id_result_writer(), self.x.dis(ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Ceil {
    pub x: IdRef,
}
impl Inst for Ceil {
    const META: &InstMeta = &CEIL;
}
impl InstEncoding for Ceil {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        profiling::function_scope!();
        let ctx = &OperandDisContext {
            id_result: None,
            id_result_type: None,
            ctx,
        };
        write!(f, "{}Ceil{}", ctx.id_result_writer(), self.x.dis(ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Fract {
    pub x: IdRef,
}
impl Inst for Fract {
    const META: &InstMeta = &FRACT;
}
impl InstEncoding for Fract {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        profiling::function_scope!();
        let ctx = &OperandDisContext {
            id_result: None,
            id_result_type: None,
            ctx,
        };
        write!(f, "{}Fract{}", ctx.id_result_writer(), self.x.dis(ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Radians {
    pub degrees: IdRef,
}
impl Inst for Radians {
    const META: &InstMeta = &RADIANS;
}
impl InstEncoding for Radians {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.degrees);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.degrees, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            degrees: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}Radians{}",
            ctx.id_result_writer(),
            self.degrees.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Degrees {
    pub radians: IdRef,
}
impl Inst for Degrees {
    const META: &InstMeta = &DEGREES;
}
impl InstEncoding for Degrees {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.radians);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.radians, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            radians: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}Degrees{}",
            ctx.id_result_writer(),
            self.radians.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Sin {
    pub x: IdRef,
}
impl Inst for Sin {
    const META: &InstMeta = &SIN;
}
impl InstEncoding for Sin {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        profiling::function_scope!();
        let ctx = &OperandDisContext {
            id_result: None,
            id_result_type: None,
            ctx,
        };
        write!(f, "{}Sin{}", ctx.id_result_writer(), self.x.dis(ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Cos {
    pub x: IdRef,
}
impl Inst for Cos {
    const META: &InstMeta = &COS;
}
impl InstEncoding for Cos {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        profiling::function_scope!();
        let ctx = &OperandDisContext {
            id_result: None,
            id_result_type: None,
            ctx,
        };
        write!(f, "{}Cos{}", ctx.id_result_writer(), self.x.dis(ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Tan {
    pub x: IdRef,
}
impl Inst for Tan {
    const META: &InstMeta = &TAN;
}
impl InstEncoding for Tan {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        profiling::function_scope!();
        let ctx = &OperandDisContext {
            id_result: None,
            id_result_type: None,
            ctx,
        };
        write!(f, "{}Tan{}", ctx.id_result_writer(), self.x.dis(ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Asin {
    pub x: IdRef,
}
impl Inst for Asin {
    const META: &InstMeta = &ASIN;
}
impl InstEncoding for Asin {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        profiling::function_scope!();
        let ctx = &OperandDisContext {
            id_result: None,
            id_result_type: None,
            ctx,
        };
        write!(f, "{}Asin{}", ctx.id_result_writer(), self.x.dis(ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Acos {
    pub x: IdRef,
}
impl Inst for Acos {
    const META: &InstMeta = &ACOS;
}
impl InstEncoding for Acos {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        profiling::function_scope!();
        let ctx = &OperandDisContext {
            id_result: None,
            id_result_type: None,
            ctx,
        };
        write!(f, "{}Acos{}", ctx.id_result_writer(), self.x.dis(ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Atan {
    pub y_over_x: IdRef,
}
impl Inst for Atan {
    const META: &InstMeta = &ATAN;
}
impl InstEncoding for Atan {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.y_over_x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.y_over_x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            y_over_x: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}Atan{}",
            ctx.id_result_writer(),
            self.y_over_x.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Sinh {
    pub x: IdRef,
}
impl Inst for Sinh {
    const META: &InstMeta = &SINH;
}
impl InstEncoding for Sinh {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        profiling::function_scope!();
        let ctx = &OperandDisContext {
            id_result: None,
            id_result_type: None,
            ctx,
        };
        write!(f, "{}Sinh{}", ctx.id_result_writer(), self.x.dis(ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Cosh {
    pub x: IdRef,
}
impl Inst for Cosh {
    const META: &InstMeta = &COSH;
}
impl InstEncoding for Cosh {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        profiling::function_scope!();
        let ctx = &OperandDisContext {
            id_result: None,
            id_result_type: None,
            ctx,
        };
        write!(f, "{}Cosh{}", ctx.id_result_writer(), self.x.dis(ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Tanh {
    pub x: IdRef,
}
impl Inst for Tanh {
    const META: &InstMeta = &TANH;
}
impl InstEncoding for Tanh {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        profiling::function_scope!();
        let ctx = &OperandDisContext {
            id_result: None,
            id_result_type: None,
            ctx,
        };
        write!(f, "{}Tanh{}", ctx.id_result_writer(), self.x.dis(ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Asinh {
    pub x: IdRef,
}
impl Inst for Asinh {
    const META: &InstMeta = &ASINH;
}
impl InstEncoding for Asinh {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        profiling::function_scope!();
        let ctx = &OperandDisContext {
            id_result: None,
            id_result_type: None,
            ctx,
        };
        write!(f, "{}Asinh{}", ctx.id_result_writer(), self.x.dis(ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Acosh {
    pub x: IdRef,
}
impl Inst for Acosh {
    const META: &InstMeta = &ACOSH;
}
impl InstEncoding for Acosh {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        profiling::function_scope!();
        let ctx = &OperandDisContext {
            id_result: None,
            id_result_type: None,
            ctx,
        };
        write!(f, "{}Acosh{}", ctx.id_result_writer(), self.x.dis(ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Atanh {
    pub x: IdRef,
}
impl Inst for Atanh {
    const META: &InstMeta = &ATANH;
}
impl InstEncoding for Atanh {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        profiling::function_scope!();
        let ctx = &OperandDisContext {
            id_result: None,
            id_result_type: None,
            ctx,
        };
        write!(f, "{}Atanh{}", ctx.id_result_writer(), self.x.dis(ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Atan2 {
    pub y: IdRef,
    pub x: IdRef,
}
impl Inst for Atan2 {
    const META: &InstMeta = &ATAN_2;
}
impl InstEncoding for Atan2 {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.y) + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.y, &mut *writer)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            y: OperandEncoding::decode(&mut op_reader)?,
            x: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}Atan2{}{}",
            ctx.id_result_writer(),
            self.y.dis(ctx),
            self.x.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Pow {
    pub x: IdRef,
    pub y: IdRef,
}
impl Inst for Pow {
    const META: &InstMeta = &POW;
}
impl InstEncoding for Pow {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x) + OperandEncoding::word_len(&self.y);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.y, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            y: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}Pow{}{}",
            ctx.id_result_writer(),
            self.x.dis(ctx),
            self.y.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Exp {
    pub x: IdRef,
}
impl Inst for Exp {
    const META: &InstMeta = &EXP;
}
impl InstEncoding for Exp {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        profiling::function_scope!();
        let ctx = &OperandDisContext {
            id_result: None,
            id_result_type: None,
            ctx,
        };
        write!(f, "{}Exp{}", ctx.id_result_writer(), self.x.dis(ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Log {
    pub x: IdRef,
}
impl Inst for Log {
    const META: &InstMeta = &LOG;
}
impl InstEncoding for Log {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        profiling::function_scope!();
        let ctx = &OperandDisContext {
            id_result: None,
            id_result_type: None,
            ctx,
        };
        write!(f, "{}Log{}", ctx.id_result_writer(), self.x.dis(ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Exp2 {
    pub x: IdRef,
}
impl Inst for Exp2 {
    const META: &InstMeta = &EXP_2;
}
impl InstEncoding for Exp2 {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        profiling::function_scope!();
        let ctx = &OperandDisContext {
            id_result: None,
            id_result_type: None,
            ctx,
        };
        write!(f, "{}Exp2{}", ctx.id_result_writer(), self.x.dis(ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Log2 {
    pub x: IdRef,
}
impl Inst for Log2 {
    const META: &InstMeta = &LOG_2;
}
impl InstEncoding for Log2 {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        profiling::function_scope!();
        let ctx = &OperandDisContext {
            id_result: None,
            id_result_type: None,
            ctx,
        };
        write!(f, "{}Log2{}", ctx.id_result_writer(), self.x.dis(ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Sqrt {
    pub x: IdRef,
}
impl Inst for Sqrt {
    const META: &InstMeta = &SQRT;
}
impl InstEncoding for Sqrt {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        profiling::function_scope!();
        let ctx = &OperandDisContext {
            id_result: None,
            id_result_type: None,
            ctx,
        };
        write!(f, "{}Sqrt{}", ctx.id_result_writer(), self.x.dis(ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct InverseSqrt {
    pub x: IdRef,
}
impl Inst for InverseSqrt {
    const META: &InstMeta = &INVERSE_SQRT;
}
impl InstEncoding for InverseSqrt {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}InverseSqrt{}",
            ctx.id_result_writer(),
            self.x.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Determinant {
    pub x: IdRef,
}
impl Inst for Determinant {
    const META: &InstMeta = &DETERMINANT;
}
impl InstEncoding for Determinant {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}Determinant{}",
            ctx.id_result_writer(),
            self.x.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct MatrixInverse {
    pub x: IdRef,
}
impl Inst for MatrixInverse {
    const META: &InstMeta = &MATRIX_INVERSE;
}
impl InstEncoding for MatrixInverse {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}MatrixInverse{}",
            ctx.id_result_writer(),
            self.x.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Modf {
    pub x: IdRef,
    pub i: IdRef,
}
impl Inst for Modf {
    const META: &InstMeta = &MODF;
}
impl InstEncoding for Modf {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x) + OperandEncoding::word_len(&self.i);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.i, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            i: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}Modf{}{}",
            ctx.id_result_writer(),
            self.x.dis(ctx),
            self.i.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ModfStruct {
    pub x: IdRef,
}
impl Inst for ModfStruct {
    const META: &InstMeta = &MODF_STRUCT;
}
impl InstEncoding for ModfStruct {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        profiling::function_scope!();
        let ctx = &OperandDisContext {
            id_result: None,
            id_result_type: None,
            ctx,
        };
        write!(f, "{}ModfStruct{}", ctx.id_result_writer(), self.x.dis(ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FMin {
    pub x: IdRef,
    pub y: IdRef,
}
impl Inst for FMin {
    const META: &InstMeta = &F_MIN;
}
impl InstEncoding for FMin {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x) + OperandEncoding::word_len(&self.y);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.y, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            y: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}FMin{}{}",
            ctx.id_result_writer(),
            self.x.dis(ctx),
            self.y.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UMin {
    pub x: IdRef,
    pub y: IdRef,
}
impl Inst for UMin {
    const META: &InstMeta = &U_MIN;
}
impl InstEncoding for UMin {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x) + OperandEncoding::word_len(&self.y);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.y, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            y: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}UMin{}{}",
            ctx.id_result_writer(),
            self.x.dis(ctx),
            self.y.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SMin {
    pub x: IdRef,
    pub y: IdRef,
}
impl Inst for SMin {
    const META: &InstMeta = &S_MIN;
}
impl InstEncoding for SMin {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x) + OperandEncoding::word_len(&self.y);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.y, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            y: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}SMin{}{}",
            ctx.id_result_writer(),
            self.x.dis(ctx),
            self.y.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FMax {
    pub x: IdRef,
    pub y: IdRef,
}
impl Inst for FMax {
    const META: &InstMeta = &F_MAX;
}
impl InstEncoding for FMax {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x) + OperandEncoding::word_len(&self.y);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.y, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            y: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}FMax{}{}",
            ctx.id_result_writer(),
            self.x.dis(ctx),
            self.y.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UMax {
    pub x: IdRef,
    pub y: IdRef,
}
impl Inst for UMax {
    const META: &InstMeta = &U_MAX;
}
impl InstEncoding for UMax {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x) + OperandEncoding::word_len(&self.y);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.y, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            y: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}UMax{}{}",
            ctx.id_result_writer(),
            self.x.dis(ctx),
            self.y.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SMax {
    pub x: IdRef,
    pub y: IdRef,
}
impl Inst for SMax {
    const META: &InstMeta = &S_MAX;
}
impl InstEncoding for SMax {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x) + OperandEncoding::word_len(&self.y);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.y, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            y: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}SMax{}{}",
            ctx.id_result_writer(),
            self.x.dis(ctx),
            self.y.dis(ctx)
        )
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
}
impl InstEncoding for FClamp {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
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
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            min_val: OperandEncoding::decode(&mut op_reader)?,
            max_val: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}FClamp{}{}{}",
            ctx.id_result_writer(),
            self.x.dis(ctx),
            self.min_val.dis(ctx),
            self.max_val.dis(ctx)
        )
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
}
impl InstEncoding for UClamp {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
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
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            min_val: OperandEncoding::decode(&mut op_reader)?,
            max_val: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}UClamp{}{}{}",
            ctx.id_result_writer(),
            self.x.dis(ctx),
            self.min_val.dis(ctx),
            self.max_val.dis(ctx)
        )
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
}
impl InstEncoding for SClamp {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
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
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            min_val: OperandEncoding::decode(&mut op_reader)?,
            max_val: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}SClamp{}{}{}",
            ctx.id_result_writer(),
            self.x.dis(ctx),
            self.min_val.dis(ctx),
            self.max_val.dis(ctx)
        )
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
}
impl InstEncoding for FMix {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
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
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            y: OperandEncoding::decode(&mut op_reader)?,
            a: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}FMix{}{}{}",
            ctx.id_result_writer(),
            self.x.dis(ctx),
            self.y.dis(ctx),
            self.a.dis(ctx)
        )
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
}
impl InstEncoding for IMix {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
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
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            y: OperandEncoding::decode(&mut op_reader)?,
            a: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}IMix{}{}{}",
            ctx.id_result_writer(),
            self.x.dis(ctx),
            self.y.dis(ctx),
            self.a.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Step {
    pub edge: IdRef,
    pub x: IdRef,
}
impl Inst for Step {
    const META: &InstMeta = &STEP;
}
impl InstEncoding for Step {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.edge) + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.edge, &mut *writer)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            edge: OperandEncoding::decode(&mut op_reader)?,
            x: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}Step{}{}",
            ctx.id_result_writer(),
            self.edge.dis(ctx),
            self.x.dis(ctx)
        )
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
}
impl InstEncoding for SmoothStep {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
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
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            edge_0: OperandEncoding::decode(&mut op_reader)?,
            edge_1: OperandEncoding::decode(&mut op_reader)?,
            x: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}SmoothStep{}{}{}",
            ctx.id_result_writer(),
            self.edge_0.dis(ctx),
            self.edge_1.dis(ctx),
            self.x.dis(ctx)
        )
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
}
impl InstEncoding for Fma {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
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
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            a: OperandEncoding::decode(&mut op_reader)?,
            b: OperandEncoding::decode(&mut op_reader)?,
            c: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}Fma{}{}{}",
            ctx.id_result_writer(),
            self.a.dis(ctx),
            self.b.dis(ctx),
            self.c.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Frexp {
    pub x: IdRef,
    pub exp: IdRef,
}
impl Inst for Frexp {
    const META: &InstMeta = &FREXP;
}
impl InstEncoding for Frexp {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x) + OperandEncoding::word_len(&self.exp);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.exp, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            exp: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}Frexp{}{}",
            ctx.id_result_writer(),
            self.x.dis(ctx),
            self.exp.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FrexpStruct {
    pub x: IdRef,
}
impl Inst for FrexpStruct {
    const META: &InstMeta = &FREXP_STRUCT;
}
impl InstEncoding for FrexpStruct {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}FrexpStruct{}",
            ctx.id_result_writer(),
            self.x.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Ldexp {
    pub x: IdRef,
    pub exp: IdRef,
}
impl Inst for Ldexp {
    const META: &InstMeta = &LDEXP;
}
impl InstEncoding for Ldexp {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x) + OperandEncoding::word_len(&self.exp);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.exp, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            exp: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}Ldexp{}{}",
            ctx.id_result_writer(),
            self.x.dis(ctx),
            self.exp.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PackSnorm4x8 {
    pub v: IdRef,
}
impl Inst for PackSnorm4x8 {
    const META: &InstMeta = &PACK_SNORM_4_X_8;
}
impl InstEncoding for PackSnorm4x8 {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.v);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.v, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            v: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}PackSnorm4x8{}",
            ctx.id_result_writer(),
            self.v.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PackUnorm4x8 {
    pub v: IdRef,
}
impl Inst for PackUnorm4x8 {
    const META: &InstMeta = &PACK_UNORM_4_X_8;
}
impl InstEncoding for PackUnorm4x8 {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.v);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.v, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            v: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}PackUnorm4x8{}",
            ctx.id_result_writer(),
            self.v.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PackSnorm2x16 {
    pub v: IdRef,
}
impl Inst for PackSnorm2x16 {
    const META: &InstMeta = &PACK_SNORM_2_X_16;
}
impl InstEncoding for PackSnorm2x16 {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.v);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.v, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            v: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}PackSnorm2x16{}",
            ctx.id_result_writer(),
            self.v.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PackUnorm2x16 {
    pub v: IdRef,
}
impl Inst for PackUnorm2x16 {
    const META: &InstMeta = &PACK_UNORM_2_X_16;
}
impl InstEncoding for PackUnorm2x16 {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.v);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.v, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            v: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}PackUnorm2x16{}",
            ctx.id_result_writer(),
            self.v.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PackHalf2x16 {
    pub v: IdRef,
}
impl Inst for PackHalf2x16 {
    const META: &InstMeta = &PACK_HALF_2_X_16;
}
impl InstEncoding for PackHalf2x16 {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.v);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.v, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            v: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}PackHalf2x16{}",
            ctx.id_result_writer(),
            self.v.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PackDouble2x32 {
    pub v: IdRef,
}
impl Inst for PackDouble2x32 {
    const META: &InstMeta = &PACK_DOUBLE_2_X_32;
}
impl InstEncoding for PackDouble2x32 {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.v);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.v, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            v: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}PackDouble2x32{}",
            ctx.id_result_writer(),
            self.v.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnpackSnorm2x16 {
    pub p: IdRef,
}
impl Inst for UnpackSnorm2x16 {
    const META: &InstMeta = &UNPACK_SNORM_2_X_16;
}
impl InstEncoding for UnpackSnorm2x16 {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.p);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.p, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            p: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}UnpackSnorm2x16{}",
            ctx.id_result_writer(),
            self.p.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnpackUnorm2x16 {
    pub p: IdRef,
}
impl Inst for UnpackUnorm2x16 {
    const META: &InstMeta = &UNPACK_UNORM_2_X_16;
}
impl InstEncoding for UnpackUnorm2x16 {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.p);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.p, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            p: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}UnpackUnorm2x16{}",
            ctx.id_result_writer(),
            self.p.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnpackHalf2x16 {
    pub v: IdRef,
}
impl Inst for UnpackHalf2x16 {
    const META: &InstMeta = &UNPACK_HALF_2_X_16;
}
impl InstEncoding for UnpackHalf2x16 {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.v);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.v, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            v: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}UnpackHalf2x16{}",
            ctx.id_result_writer(),
            self.v.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnpackSnorm4x8 {
    pub p: IdRef,
}
impl Inst for UnpackSnorm4x8 {
    const META: &InstMeta = &UNPACK_SNORM_4_X_8;
}
impl InstEncoding for UnpackSnorm4x8 {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.p);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.p, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            p: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}UnpackSnorm4x8{}",
            ctx.id_result_writer(),
            self.p.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnpackUnorm4x8 {
    pub p: IdRef,
}
impl Inst for UnpackUnorm4x8 {
    const META: &InstMeta = &UNPACK_UNORM_4_X_8;
}
impl InstEncoding for UnpackUnorm4x8 {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.p);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.p, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            p: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}UnpackUnorm4x8{}",
            ctx.id_result_writer(),
            self.p.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnpackDouble2x32 {
    pub v: IdRef,
}
impl Inst for UnpackDouble2x32 {
    const META: &InstMeta = &UNPACK_DOUBLE_2_X_32;
}
impl InstEncoding for UnpackDouble2x32 {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.v);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.v, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            v: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}UnpackDouble2x32{}",
            ctx.id_result_writer(),
            self.v.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Length {
    pub x: IdRef,
}
impl Inst for Length {
    const META: &InstMeta = &LENGTH;
}
impl InstEncoding for Length {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        profiling::function_scope!();
        let ctx = &OperandDisContext {
            id_result: None,
            id_result_type: None,
            ctx,
        };
        write!(f, "{}Length{}", ctx.id_result_writer(), self.x.dis(ctx))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Distance {
    pub p_0: IdRef,
    pub p_1: IdRef,
}
impl Inst for Distance {
    const META: &InstMeta = &DISTANCE;
}
impl InstEncoding for Distance {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.p_0) + OperandEncoding::word_len(&self.p_1);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.p_0, &mut *writer)?;
        OperandEncoding::encode(&self.p_1, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            p_0: OperandEncoding::decode(&mut op_reader)?,
            p_1: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}Distance{}{}",
            ctx.id_result_writer(),
            self.p_0.dis(ctx),
            self.p_1.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Cross {
    pub x: IdRef,
    pub y: IdRef,
}
impl Inst for Cross {
    const META: &InstMeta = &CROSS;
}
impl InstEncoding for Cross {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x) + OperandEncoding::word_len(&self.y);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.y, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            y: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}Cross{}{}",
            ctx.id_result_writer(),
            self.x.dis(ctx),
            self.y.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Normalize {
    pub x: IdRef,
}
impl Inst for Normalize {
    const META: &InstMeta = &NORMALIZE;
}
impl InstEncoding for Normalize {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        profiling::function_scope!();
        let ctx = &OperandDisContext {
            id_result: None,
            id_result_type: None,
            ctx,
        };
        write!(f, "{}Normalize{}", ctx.id_result_writer(), self.x.dis(ctx))
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
}
impl InstEncoding for FaceForward {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
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
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            n: OperandEncoding::decode(&mut op_reader)?,
            i: OperandEncoding::decode(&mut op_reader)?,
            nref: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}FaceForward{}{}{}",
            ctx.id_result_writer(),
            self.n.dis(ctx),
            self.i.dis(ctx),
            self.nref.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Reflect {
    pub i: IdRef,
    pub n: IdRef,
}
impl Inst for Reflect {
    const META: &InstMeta = &REFLECT;
}
impl InstEncoding for Reflect {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.i) + OperandEncoding::word_len(&self.n);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.i, &mut *writer)?;
        OperandEncoding::encode(&self.n, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            i: OperandEncoding::decode(&mut op_reader)?,
            n: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}Reflect{}{}",
            ctx.id_result_writer(),
            self.i.dis(ctx),
            self.n.dis(ctx)
        )
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
}
impl InstEncoding for Refract {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
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
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            i: OperandEncoding::decode(&mut op_reader)?,
            n: OperandEncoding::decode(&mut op_reader)?,
            eta: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}Refract{}{}{}",
            ctx.id_result_writer(),
            self.i.dis(ctx),
            self.n.dis(ctx),
            self.eta.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FindILsb {
    pub value: IdRef,
}
impl Inst for FindILsb {
    const META: &InstMeta = &FIND_I_LSB;
}
impl InstEncoding for FindILsb {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.value);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.value, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            value: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}FindILsb{}",
            ctx.id_result_writer(),
            self.value.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FindSMsb {
    pub value: IdRef,
}
impl Inst for FindSMsb {
    const META: &InstMeta = &FIND_S_MSB;
}
impl InstEncoding for FindSMsb {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.value);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.value, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            value: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}FindSMsb{}",
            ctx.id_result_writer(),
            self.value.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FindUMsb {
    pub value: IdRef,
}
impl Inst for FindUMsb {
    const META: &InstMeta = &FIND_U_MSB;
}
impl InstEncoding for FindUMsb {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.value);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.value, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            value: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}FindUMsb{}",
            ctx.id_result_writer(),
            self.value.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct InterpolateAtCentroid {
    pub interpolant: IdRef,
}
impl Inst for InterpolateAtCentroid {
    const META: &InstMeta = &INTERPOLATE_AT_CENTROID;
}
impl InstEncoding for InterpolateAtCentroid {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.interpolant);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.interpolant, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            interpolant: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}InterpolateAtCentroid{}",
            ctx.id_result_writer(),
            self.interpolant.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct InterpolateAtSample {
    pub interpolant: IdRef,
    pub sample: IdRef,
}
impl Inst for InterpolateAtSample {
    const META: &InstMeta = &INTERPOLATE_AT_SAMPLE;
}
impl InstEncoding for InterpolateAtSample {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1
            + OperandEncoding::word_len(&self.interpolant)
            + OperandEncoding::word_len(&self.sample);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.interpolant, &mut *writer)?;
        OperandEncoding::encode(&self.sample, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            interpolant: OperandEncoding::decode(&mut op_reader)?,
            sample: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}InterpolateAtSample{}{}",
            ctx.id_result_writer(),
            self.interpolant.dis(ctx),
            self.sample.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct InterpolateAtOffset {
    pub interpolant: IdRef,
    pub offset: IdRef,
}
impl Inst for InterpolateAtOffset {
    const META: &InstMeta = &INTERPOLATE_AT_OFFSET;
}
impl InstEncoding for InterpolateAtOffset {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1
            + OperandEncoding::word_len(&self.interpolant)
            + OperandEncoding::word_len(&self.offset);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.interpolant, &mut *writer)?;
        OperandEncoding::encode(&self.offset, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            interpolant: OperandEncoding::decode(&mut op_reader)?,
            offset: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}InterpolateAtOffset{}{}",
            ctx.id_result_writer(),
            self.interpolant.dis(ctx),
            self.offset.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct NMin {
    pub x: IdRef,
    pub y: IdRef,
}
impl Inst for NMin {
    const META: &InstMeta = &N_MIN;
}
impl InstEncoding for NMin {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x) + OperandEncoding::word_len(&self.y);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.y, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            y: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}NMin{}{}",
            ctx.id_result_writer(),
            self.x.dis(ctx),
            self.y.dis(ctx)
        )
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct NMax {
    pub x: IdRef,
    pub y: IdRef,
}
impl Inst for NMax {
    const META: &InstMeta = &N_MAX;
}
impl InstEncoding for NMax {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len = 1 + OperandEncoding::word_len(&self.x) + OperandEncoding::word_len(&self.y);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.x, &mut *writer)?;
        OperandEncoding::encode(&self.y, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            y: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}NMax{}{}",
            ctx.id_result_writer(),
            self.x.dis(ctx),
            self.y.dis(ctx)
        )
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
}
impl InstEncoding for NClamp {
    type IdResult = ();
    fn id_result(&self) -> Self::IdResult {}
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
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
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            x: OperandEncoding::decode(&mut op_reader)?,
            min_val: OperandEncoding::decode(&mut op_reader)?,
            max_val: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}NClamp{}{}{}",
            ctx.id_result_writer(),
            self.x.dis(ctx),
            self.min_val.dis(ctx),
            self.max_val.dis(ctx)
        )
    }
}
