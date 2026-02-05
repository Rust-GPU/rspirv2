use super::preamble::*;
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct DebugPrintf {
    pub format: IdRef,
    pub id_ref: SmallVec<[IdRef; 4usize]>,
}
