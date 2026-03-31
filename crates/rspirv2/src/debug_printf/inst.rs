use super::preamble::*;
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct DebugPrintf {
    pub format: IdRef,
    pub id_ref: ZeroOrMore<IdRef>,
}
impl Inst for DebugPrintf {
    const META: &InstMeta = &DEBUG_PRINTF;
    type MaybeIdResult = ();
    fn id_result(&self) -> Self::MaybeIdResult {}
}
impl InstEncoding for DebugPrintf {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        profiling::function_scope!();
        let len =
            1 + OperandEncoding::word_len(&self.format) + OperandEncoding::word_len(&self.id_ref);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.format, &mut *writer)?;
        OperandEncoding::encode(&self.id_ref, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            format: OperandEncoding::decode(&mut op_reader)?,
            id_ref: OperandEncoding::decode_last(&mut op_reader)?,
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
            "{}DebugPrintf{}{}",
            ctx.id_result_writer(),
            self.format.dis(ctx),
            self.id_ref.dis(ctx)
        )
    }
}
