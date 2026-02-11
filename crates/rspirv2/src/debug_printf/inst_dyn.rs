use super::preamble::*;
pub fn decode_dyn<R, F>(reader: &mut InstructionReader, f: F) -> Result<R, DecodeError>
where
    F: FnOnce(&dyn DynInst) -> R,
{
    Ok(match reader.opcode() {
        1u16 => f(&<DebugPrintf as Inst>::decode(reader)?),
        opcode => return Err(DecodeError::UnknownOpCode { opcode }),
    })
}
