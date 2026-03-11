use std::path::PathBuf;

pub const SPV_FOLDER: &str = env!("CARGO_MANIFEST_DIR");

pub fn spv_folder() -> PathBuf {
    SPV_FOLDER.into()
}

pub fn spv(name: &str) -> PathBuf {
    PathBuf::from(format!("{SPV_FOLDER}/{name}.spv"))
}
