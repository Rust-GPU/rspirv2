use super::preamble::*;
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum DebugPrintfInstSet {
    DebugPrintf(DebugPrintf),
}
impl InstEncoding for DebugPrintfInstSet {
    fn encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError> {
        match self {
            Self::DebugPrintf(inst) => InstEncoding::encode(inst, writer),
        }
    }
    fn decode(reader: &mut InstReader<'_>) -> Result<Self, DecodeError> {
        let opcode = reader.opcode();
        Ok(match opcode {
            1u16 => Self::DebugPrintf(<DebugPrintf as InstEncoding>::decode(reader)?),
            _ => {
                return Err(DecodeError::UnknownOpCode { opcode });
            }
        })
    }
}
impl From<DebugPrintf> for DebugPrintfInstSet {
    fn from(inst: DebugPrintf) -> Self {
        Self::DebugPrintf(inst)
    }
}
