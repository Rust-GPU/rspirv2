#![doc = include_str!("../README.md")]

use expect_test::ExpectFile;
use std::path::PathBuf;

pub const BASE_FOLDER: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/spv");

pub struct SpvFolder<'a>(&'a str);

impl SpvFolder<'_> {
    pub fn file(&self, name: &str) -> PathBuf {
        let folder = self.0;
        PathBuf::from(format!("{BASE_FOLDER}/{folder}/{name}"))
    }

    pub fn spv(&self) -> PathBuf {
        self.file(&format!("{}.spv", self.0))
    }

    pub fn expect(&self, ext: &str) -> ExpectFile {
        let folder = self.0;
        ExpectFile {
            path: self.file(&format!("{folder}.{ext}")),
            position: "",
        }
    }
}

pub const fn spv(name: &str) -> SpvFolder<'_> {
    SpvFolder(name)
}

pub const BLA: SpvFolder<'_> = spv("bla");
pub const DIS_REFERENCE: SpvFolder<'_> = spv("dis_reference");
pub const TEXTURE_GRAD_OFFSET: SpvFolder<'_> = spv("textureGradOffset");
pub const CONST_SPECS: SpvFolder<'_> = spv("const_specs");
pub const CORE_PRE_LINK: SpvFolder<'_> = spv("core_pre_link");
