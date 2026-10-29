#![doc = include_str!("../README.md")]

use expect_test::ExpectFile;
use rspirv2::core::inst_set::CoreInstSet;
use rspirv2_tools_gen::dis::{Args, Profile};
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

    pub fn test_disassembly(&self, expect_ext: &str, profile: Profile) -> anyhow::Result<()> {
        test_disassembly(self.spv(), self.expect(expect_ext), profile)
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
pub const SPEC_CONSTANT_OP: SpvFolder<'_> = spv("spec_constant_op");

pub fn test_disassembly(path: PathBuf, expect: ExpectFile, profile: Profile) -> anyhow::Result<()> {
    for module_swap_bytes in [false, true] {
        println!("module_swap_bytes: {module_swap_bytes}");
        let args = Args {
            path: path.clone(),
            profile,
            module_swap_bytes,
            color: clap::ColorChoice::Never,
        };
        let mut stdout = Vec::new();
        args.run_inner::<CoreInstSet>(&mut stdout)?;
        let stdout = String::from_utf8(stdout)?;
        expect.assert_eq(&stdout);
    }
    Ok(())
}
