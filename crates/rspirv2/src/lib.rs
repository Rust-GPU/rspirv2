#![doc = include_str!("../README.md")]

pub mod core;
pub mod dis;
pub mod op_spec_op;
pub mod op_switch;

#[deprecated(note = "module renamed to `op_switch`")]
pub use op_switch as custom_inst;
pub use rspirv2_types::*;

#[cfg(test)]
mod tests {
    use crate::core::inst_set::CoreInstSet;
    use expect_test::expect;

    #[test]
    #[cfg(target_pointer_width = "64")]
    pub fn test_core_size() {
        expect!["120"].assert_eq(&format!("{}", size_of::<CoreInstSet>()));
    }
}
