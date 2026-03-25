use crate::core::inst_set::CoreInstSet;
use crate::core::inst_set::CoreInstSet::{TypeFloat, TypeInt};
pub use rspirv2_types::dis::*;
use rspirv2_types::operand::ConstFmt;
use rspirv2_types::slice::{RawInstSlice, SkipDecodeErrorIteratorExt, TryDecodeIteratorExt};

impl InstSetDisCtx for CoreInstSet {
    fn add_context(slice: &RawInstSlice, ctx: &mut DisContext) {
        for inst in slice.iter().try_decode::<CoreInstSet>().skip_errors() {
            match inst {
                TypeFloat(inst) => {
                    ctx.id_to_const_fmt
                        .insert(inst.id_result.unwrap(), ConstFmt::Float);
                }
                TypeInt(inst) => {
                    ctx.id_to_const_fmt.insert(
                        inst.id_result.unwrap(),
                        if inst.signedness.to_bool() {
                            ConstFmt::Signed
                        } else {
                            ConstFmt::Unsigned
                        },
                    );
                }
                _ => {}
            }
        }
    }
}
