use super::preamble::*;
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum DebugPrintfInstSet {
    DebugPrintf(DebugPrintf),
}
impl From<DebugPrintf> for DebugPrintfInstSet {
    fn from(inst: DebugPrintf) -> Self {
        Self::DebugPrintf(inst)
    }
}
