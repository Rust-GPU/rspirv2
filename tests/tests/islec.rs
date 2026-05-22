use std::path::PathBuf;

const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

#[test]
fn islec_core() {
    let core_isle = PathBuf::from(CARGO_MANIFEST_DIR).join("../crates/rspirv2/src/core/core.isle");
    let _rust = cranelift_isle::compile::from_files(&[core_isle], &Default::default()).unwrap();
    // can't be bothered checking whether the output rust source compiles
    // if islec accepts the input, it'll most likely be fine
}
