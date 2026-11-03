use crate::Word;
use crate::binary::{DecodeError, EncodeError, WordWriter};
use crate::inst::InstEncoding;

/// Like [`InstEncoding`] but encodes supported instruction in the slightly different binary format that
/// `OpSpecConstantOp` requires.
///
/// The `ResultType` and `Result` operands are moved to the front and the `opcode` is the 3rd operand instead of the
/// first:
/// ```plain
/// # normal encoding:
/// [(IAdd.opcode | len << 16),             ResultType, Result,              IdRef, IdRef]
/// # OpSpecConstantOp encoding:
/// [(OpSpecConstantOp.opcode | len << 16), ResultType, Result, IAdd.opcode, IdRef, IdRef]
/// ```
///
/// Also see:
/// * <https://github.com/KhronosGroup/SPIRV-Headers/issues/645>
/// * <https://github.com/KhronosGroup/SPIRV-Headers/issues/124>
pub trait OpSpecOpInst: InstEncoding {
    /// Encode this instruction to a [`WordWriter`]
    fn op_spec_op_encode(&self, writer: &mut impl WordWriter) -> Result<(), EncodeError>;

    /// Decode this instruction from a slice of words.
    ///
    /// `InstReader` assumes the opcode is at offset 0, but here it's at offset 2.
    fn op_spec_op_decode(reader: &[Word]) -> Result<Self, DecodeError>;
}
