use crate::core::inst::{
    OpConstant, OpConstantFalse, OpConstantNull, OpConstantTrue, OpTypeBool, OpTypeFloat,
    OpTypeInt, OpTypePointer, OpTypeStruct, OpTypeVector, OpTypeVoid,
};
use crate::core::inst_set::CoreInstSet;
use crate::core::preamble::OpTypeRuntimeArray;
pub use rspirv2_types::dis::*;
use rspirv2_types::operand::{ConstFmt, OperandDisContext};
use rspirv2_types::slice::{RawInstSlice, SkipDecodeErrorIteratorExt, TryDecodeIteratorExt};
use std::borrow::Cow;

impl InstSetDisCtx for CoreInstSet {
    fn add_context(slice: &RawInstSlice, ctx: &mut DisContext) {
        profiling::function_scope!();
        for inst in slice.iter().try_decode::<CoreInstSet>().skip_errors() {
            match inst {
                Self::Name(inst) => {
                    if let Some(name) = escape_id_name(Cow::Owned(inst.name.0)) {
                        ctx.add_id_to_name(inst.target.0, IdName::ExplicitName(name.into_owned()));
                    }
                }
                Self::TypeVoid(inst) => inst.add_context(ctx),
                Self::TypeBool(inst) => inst.add_context(ctx),
                Self::TypeInt(inst) => inst.add_context(ctx),
                Self::TypeFloat(inst) => inst.add_context(ctx),
                Self::TypeVector(inst) => ctx.add_id_to_name(inst.id_result, inst.derive_name(ctx)),
                Self::TypeRuntimeArray(inst) => {
                    ctx.add_id_to_name(inst.id_result, inst.derive_name(ctx));
                }
                Self::TypePointer(inst) => {
                    ctx.add_id_to_name(inst.id_result, inst.derive_name(ctx));
                }
                Self::TypeStruct(inst) => ctx.add_id_to_name(inst.id_result, inst.derive_name(ctx)),
                Self::Constant(inst) => ctx.add_id_to_name(inst.id_result, inst.derive_name(ctx)),
                Self::ConstantNull(inst) => {
                    ctx.add_id_to_name(inst.id_result, inst.derive_name(ctx));
                }
                Self::ConstantFalse(inst) => {
                    ctx.add_id_to_name(inst.id_result, inst.derive_name(ctx));
                }
                Self::ConstantTrue(inst) => {
                    ctx.add_id_to_name(inst.id_result, inst.derive_name(ctx));
                }
                _ => {}
            }
        }
    }
}

impl OpTypeVoid {
    pub fn add_context(&self, ctx: &mut DisContext) {
        profiling::function_scope!();
        ctx.add_id_to_name(self.id_result, self.derive_name(ctx));
    }

    pub fn derive_name(&self, _ctx: &DisContext) -> IdName {
        profiling::function_scope!();
        IdName::DerivedName("void".to_string())
    }
}

impl OpTypeBool {
    pub fn add_context(&self, ctx: &mut DisContext) {
        profiling::function_scope!();
        ctx.add_id_to_name(self.id_result, self.derive_name(ctx));
    }

    pub fn derive_name(&self, _ctx: &DisContext) -> IdName {
        profiling::function_scope!();
        IdName::DerivedName("bool".to_string())
    }
}

impl OpTypeInt {
    pub fn add_context(&self, ctx: &mut DisContext) {
        profiling::function_scope!();
        ctx.add_id_to_name(self.id_result, self.derive_name(ctx));
        ctx.id_to_const_fmt.insert(
            self.id_result,
            if self.signedness.to_bool() {
                ConstFmt::Signed
            } else {
                ConstFmt::Unsigned
            },
        );
    }

    pub fn derive_name(&self, ctx: &DisContext) -> IdName {
        profiling::function_scope!();
        let signed = self.signedness.to_bool();
        let width = self.width.to_u32();
        IdName::DerivedName(match ctx.type_naming {
            TypeNaming::Rust => {
                format!("{}{width}", if signed { "i" } else { "u" })
            }
            TypeNaming::C => match (signed, width) {
                (false, 32) => "uint".to_string(),
                (true, 32) => "int".to_string(),
                (false, _) => format!("uint{width}_t"),
                (true, _) => format!("int{width}_t"),
            },
        })
    }
}

impl OpTypeFloat {
    pub fn add_context(&self, ctx: &mut DisContext) {
        profiling::function_scope!();
        ctx.add_id_to_name(self.id_result, self.derive_name(ctx));
        ctx.id_to_const_fmt.insert(self.id_result, ConstFmt::Float);
    }

    pub fn derive_name(&self, ctx: &DisContext) -> IdName {
        profiling::function_scope!();
        let width = self.width.to_u32();
        IdName::DerivedName(match ctx.type_naming {
            TypeNaming::Rust => {
                format!("f{width}")
            }
            TypeNaming::C => match width {
                32 => "float".to_string(),
                64 => "double".to_string(),
                _ => format!("float{width}_t"),
            },
        })
    }
}

impl OpTypeVector {
    pub fn derive_name(&self, ctx: &DisContext) -> IdName {
        profiling::function_scope!();
        let ty_name = ctx.id_to_name(self.component_type.0);
        let count = self.component_count.to_u32();
        IdName::DerivedName(format!("v{count}{ty_name}"))
    }
}

impl OpTypeRuntimeArray {
    pub fn derive_name(&self, ctx: &DisContext) -> IdName {
        profiling::function_scope!();
        let ty_name = ctx.id_to_name(self.element_type.0);
        IdName::DerivedName(format!("_runtimearr_{ty_name}"))
    }
}

impl OpTypePointer {
    pub fn derive_name(&self, ctx: &DisContext) -> IdName {
        profiling::function_scope!();
        let ty_name = ctx.id_to_name(self.ty.0);
        let storage_class = self.storage_class;
        IdName::DerivedName(format!("_ptr_{storage_class:?}_{ty_name}"))
    }
}

impl OpTypeStruct {
    pub fn derive_name(&self, _ctx: &DisContext) -> IdName {
        profiling::function_scope!();
        IdName::DerivedName(format!("_struct_{}", self.id_result.0.0))
    }
}

impl OpConstant {
    pub fn derive_name(&self, ctx: &DisContext) -> IdName {
        profiling::function_scope!();
        let ty_name = ctx.id_to_name(self.id_result_type.0);
        let operand_ctx = OperandDisContext {
            ctx,
            id_result: Some(self.id_result),
            id_result_type: Some(self.id_result_type),
        };
        let value = self.value.fmt_value(&operand_ctx).to_string();
        let value = escape_cow(Cow::Owned(value), |c| match c {
            '0'..='9' => c,
            '-' => 'n',
            _ => '_',
        })
        .into_owned();
        IdName::DerivedName(format!("{ty_name}_{value}"))
    }
}

impl OpConstantNull {
    pub fn derive_name(&self, ctx: &DisContext) -> IdName {
        profiling::function_scope!();
        let ty_name = ctx.id_to_name(self.id_result_type.0);
        IdName::DerivedName(format!("{ty_name}_0"))
    }
}

impl OpConstantFalse {
    pub fn derive_name(&self, _ctx: &DisContext) -> IdName {
        profiling::function_scope!();
        IdName::DerivedName("bool_false".to_string())
    }
}

impl OpConstantTrue {
    pub fn derive_name(&self, _ctx: &DisContext) -> IdName {
        profiling::function_scope!();
        IdName::DerivedName("bool_true".to_string())
    }
}
