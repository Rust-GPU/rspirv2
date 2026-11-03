#[test]
fn test_magic() {
    assert_eq!(
        rspirv2::module::SPIRV_MAGIC.0,
        rspirv2::core::grammar::GRAMMAR_CORE.magic_number
    );
}
