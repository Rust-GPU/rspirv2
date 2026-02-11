#[test]
fn test_magic() {
    assert_eq!(
        rspirv2_types::module::SPIRV_MAGIC.0,
        rspirv2::core::grammar::GRAMMAR_CORE.magic_number
    );
}
