use crate::core::inst::OpTypeFloat;
use crate::core::inst_set::CoreInstSet;
use rspirv2_types::binary::{DecodeError, ModuleReader};
pub use rspirv2_types::dis::*;
use rspirv2_types::inst::InstEncoding;
use rspirv2_types::operand::{ConstFmt, Word};

pub fn create_dis_context_core(opt: DisOptions, words: &[Word]) -> Result<DisContext, DecodeError> {
    let mut ctx = DisContext::no_context(opt);
    let mut reader = ModuleReader::new(words);
    while let Some(mut inst) = reader.next()? {
        if let Some(inst) = OpTypeFloat::try_decode(&mut inst)? {
            ctx.id_to_const_fmt
                .insert(inst.id_result.unwrap(), ConstFmt::Float);
        }
    }
    Ok(ctx)
}

impl InstSetDisCtx for CoreInstSet {
    fn create_dis_context(opt: DisOptions, words: &[Word]) -> Result<DisContext, DecodeError> {
        create_dis_context_core(opt, words)
    }
}
