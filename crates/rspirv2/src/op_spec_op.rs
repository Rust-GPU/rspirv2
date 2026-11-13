use crate::core::inst_meta::OP_SPEC_CONSTANT_OP;
use crate::core::op_spec_op::CoreOpSpecOpInstSet;
use rspirv2_types::binary::{DecodeError, InstReader, WordCounter, WordWriter};
use rspirv2_types::dis::DisContext;
use rspirv2_types::inst::{Inst, InstEncoding};
use rspirv2_types::meta::InstMeta;
pub use rspirv2_types::op_spec_op::*;
use rspirv2_types::operand::{IdResult, IdResultType, OperandDisContext, OperandEncoding};
use std::fmt::Formatter;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OpSpecConstantOp(pub CoreOpSpecOpInstSet);

impl Inst for OpSpecConstantOp {
    const META: &InstMeta = &OP_SPEC_CONSTANT_OP;
}

impl InstEncoding for OpSpecConstantOp {
    type IdResult = IdResult;
    type IdResultType = IdResultType;

    fn id_result(&self) -> Self::IdResult {
        // unwrap is unreachable
        self.0.id_result().unwrap()
    }

    fn id_result_type(&self) -> Self::IdResultType {
        // unwrap is unreachable
        self.0.id_result_type().unwrap()
    }

    fn encode(&self, writer: &mut impl WordWriter) {
        profiling::function_scope!();
        // we don't have a good way of computing the len ourselves, let's hope this gets inlined and optimized nicely
        let mut len = WordCounter::default();
        self.0.op_spec_op_encode(&mut len);
        // +1 as len must include the `OpSpecConstantOp` opcode word as well
        writer.write_op(Self::META.opcode, *len + 1);
        self.0.op_spec_op_encode(writer)
    }

    fn decode(reader: InstReader<'_>) -> Result<Self, DecodeError> {
        profiling::function_scope!();
        let op_reader = reader.check_opcode(Self::META)?;
        Ok(Self(CoreOpSpecOpInstSet::op_spec_op_decode(
            op_reader.as_slice(),
        )?))
    }

    fn dis_fmt(&self, f: &mut Formatter<'_>, ctx: &DisContext) -> std::fmt::Result {
        profiling::function_scope!();
        let ctx = OperandDisContext {
            id_result: Some(self.id_result()),
            id_result_type: Some(self.id_result_type()),
            ctx,
        };
        let rspirv_space = ctx.rspirv_space();
        write!(
            f,
            "{}OpSpecConstantOp{rspirv_space}{}{rspirv_space} {}",
            ctx.id_result_writer(),
            self.id_result_type().dis(ctx),
            self.0.op_spec_op_dis(ctx)
        )
    }
}
