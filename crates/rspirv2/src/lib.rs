#![doc = include_str!("../README.md")]

pub mod core;
pub mod custom_inst;
pub mod dis;

pub use rspirv2_types::*;

#[cfg(test)]
mod tests {
    use crate::core::inst_set::CoreInstSet;
    use expect_test::expect;

    #[test]
    pub fn test_core_size() {
        expect!["120"].assert_eq(&format!("{}", size_of::<CoreInstSet>()));
    }
}
