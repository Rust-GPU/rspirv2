use super::preamble::*;
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct DebugPrintf {
    pub format: IdRef,
    pub id_ref: SmallVec<[IdRef; 4usize]>,
}
impl Inst for DebugPrintf {
    const META: &InstMeta = &DEBUG_PRINTF;
    fn encode(&self, writer: &mut impl InstructionWriter) -> Result<(), EncodeError> {
        let len =
            1 + OperandEncoding::word_len(&self.format) + OperandEncoding::word_len(&self.id_ref);
        writer.push(Word::new_op(Self::META.opcode, len)?)?;
        OperandEncoding::encode(&self.format, &mut *writer)?;
        OperandEncoding::encode(&self.id_ref, &mut *writer)?;
        Ok(())
    }
    fn decode(reader: &mut InstructionReader) -> Result<Self, DecodeError> {
        reader.check_opcode(Self::META)?;
        Ok(Self {
            format: OperandEncoding::decode(&mut *reader)?,
            id_ref: OperandEncoding::decode_last(&mut *reader)?,
        })
    }
}
