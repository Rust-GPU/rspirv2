use crate::core::inst_set::CoreInstSet;
use crate::core::inst_set::CoreInstSet::TypeFloat;
pub use rspirv2_types::dis::*;
use rspirv2_types::operand::ConstFmt;
use rspirv2_types::slice::InstSlice;

impl InstSetDisCtx for CoreInstSet {
    fn add_context(slice: &InstSlice<Self>, ctx: &mut DisContext) {
        for inst in slice.iter() {
            if let TypeFloat(inst) = inst {
                ctx.id_to_const_fmt
                    .insert(inst.id_result.unwrap(), ConstFmt::Float);
            }
        }
    }
}
