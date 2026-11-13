use crate::Word;
use crate::binary::{DecodeError, WordWriter};
use crate::inst::InstEncoding;
use crate::operand::OperandDisContext;
use std::fmt::{Display, Formatter};

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
    fn op_spec_op_encode(&self, writer: &mut impl WordWriter);

    /// Decode this instruction from a slice of words.
    ///
    /// `InstReader` assumes the opcode is at offset 0, but here it's at offset 2.
    fn op_spec_op_decode(reader: &[Word]) -> Result<Self, DecodeError>;

    /// Like [`InstEncoding::dis_fmt`] but printing everything after `OpSpecConstantOp`, eg. `OpIAdd %1 %2`.
    fn op_spec_op_dis_fmt(
        &self,
        f: &mut Formatter<'_>,
        ctx: OperandDisContext<'_>,
    ) -> std::fmt::Result;

    /// Like [`InstEncoding::dis`] but printing everything after `OpSpecConstantOp`, eg. `OpIAdd %1 %2`.
    ///
    /// You shouldn't overwrite this function but [`Self::op_spec_op_dis`] instead.
    #[inline]
    fn op_spec_op_dis<'a>(&'a self, ctx: OperandDisContext<'a>) -> OpSpecOpDis<'a, Self> {
        OpSpecOpDis(self, ctx)
    }
}

pub struct OpSpecOpDis<'a, T: OpSpecOpInst>(&'a T, OperandDisContext<'a>);

impl<'a, T: OpSpecOpInst> Display for OpSpecOpDis<'a, T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.op_spec_op_dis_fmt(f, self.1)
    }
}
