use super::preamble::*;
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct DebugPrintf {
    pub format: IdRef,
    pub id_ref: SmallVec<[IdRef; 4usize]>,
}
impl<'a> Inst<'a> for DebugPrintf {
    const META: &'static InstMeta = &DEBUG_PRINTF;
    type MaybeIdResult = ();
    fn id_result(&mut self) -> &mut Self::MaybeIdResult {
        make_mut_ref_unit()
    }
}
impl<'a> InstEncoding<'a> for DebugPrintf {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        let len =
            1 + OperandEncoding::word_len(&self.format) + OperandEncoding::word_len(&self.id_ref);
        writer.write_op(Self::META.opcode, len)?;
        OperandEncoding::encode(&self.format, &mut *writer)?;
        OperandEncoding::encode(&self.id_ref, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: InstReader<'a>) -> Result<Self, DecodeError> {
        let mut op_reader = reader.check_opcode(Self::META)?;
        Ok(Self {
            format: OperandEncoding::decode(&mut op_reader)?,
            id_ref: OperandEncoding::decode_last(&mut op_reader)?,
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, _ctx: &DisContext) -> std::fmt::Result {
        write!(
            f,
            "DebugPrintf{}{}",
            self.format.dis(_ctx),
            self.id_ref.dis(_ctx)
        )
    }
}
