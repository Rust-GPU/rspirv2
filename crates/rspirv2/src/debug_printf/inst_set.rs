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
    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        let opcode = reader.opcode();
        Ok(match opcode {
            1u16 => Self::DebugPrintf(<DebugPrintf as InstEncoding>::decode(reader)?),
            _ => {
                return Err(DecodeError::UnknownOpCode { opcode });
            }
        })
    }
    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        match self {
            Self::DebugPrintf(inst) => InstEncoding::dis_fmt(inst, f, ctx),
        }
    }
}
impl From<DebugPrintf> for DebugPrintfInstSet {
    fn from(inst: DebugPrintf) -> Self {
        Self::DebugPrintf(inst)
    }
}
