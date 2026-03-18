use clap::{Parser, ValueEnum};
use rspirv2::dis::DisOptions;
use rspirv2::inst::InstEncoding;
use rspirv2::module::Module;
use std::io::Write;
use std::path::PathBuf;

#[derive(Clone, Debug, Default, Parser)]
pub struct Args {
    /// path to SPIR-V file
    path: PathBuf,
    /// Emit disassembly like as if it was emitted by this tool
    #[clap(short, long)]
    like: Like,
    /// Swap bytes of the SPIR-V module before parsing, for testing
    #[clap(skip)]
    module_swap_bytes: bool,
}

#[derive(Clone, Debug, Default, ValueEnum)]
pub enum Like {
    #[default]
    Default,
    Rspirv,
    SpirvTools,
}

impl Args {
    pub fn run<ISA: InstEncoding>(&self, stdout: &mut impl Write) -> anyhow::Result<()> {
        let mut slice = std::fs::read(&self.path)?;
        if self.module_swap_bytes {
            for chunk in slice.as_chunks_mut::<4>().0.iter_mut() {
                *chunk = u32::from_ne_bytes(*chunk).swap_bytes().to_ne_bytes();
            }
        }

        let module = Module::<ISA>::from_bytes(slice.as_slice())?;
        write!(stdout, "{}", module.dis(self.to_dis_opts()?)?)?;
        Ok(())
    }

    pub fn to_dis_opts(&self) -> anyhow::Result<DisOptions> {
        let opt = match self.like {
            Like::Default => DisOptions::default(),
            Like::Rspirv => DisOptions::like_rspirv(),
            Like::SpirvTools => DisOptions::like_spirv_tools(),
        };
        Ok(opt)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use expect_test::{ExpectFile, expect_file};
    use rspirv2::core::inst_set::CoreInstSet;
    use spv::spv;
    use std::io::stdout;

    #[test]
    fn test_bla() -> anyhow::Result<()> {
        Args {
            path: spv("bla"),
            ..Default::default()
        }
        .run::<CoreInstSet>(&mut stdout())
    }

    #[test]
    fn test_dis_reference_default() -> anyhow::Result<()> {
        test_dis_reference(
            expect_file!["../../../spv/dis_reference.rspirv2"],
            Like::Default,
            false,
        )
    }

    #[test]
    fn test_dis_reference_default_be() -> anyhow::Result<()> {
        test_dis_reference(
            expect_file!["../../../spv/dis_reference.rspirv2"],
            Like::Default,
            true,
        )
    }

    #[test]
    fn test_dis_reference_rspirv() -> anyhow::Result<()> {
        test_dis_reference(
            expect_file!["../../../spv/dis_reference.rspirv_like"],
            Like::Rspirv,
            false,
        )
    }

    #[test]
    fn test_dis_reference_rspirv_be() -> anyhow::Result<()> {
        test_dis_reference(
            expect_file!["../../../spv/dis_reference.rspirv_like"],
            Like::Rspirv,
            true,
        )
    }

    #[test]
    fn test_dis_reference_spirv_tools() -> anyhow::Result<()> {
        test_dis_reference(
            expect_file!["../../../spv/dis_reference.spirv_tools_like"],
            Like::SpirvTools,
            false,
        )
    }

    #[test]
    fn test_dis_reference_spirv_tools_be() -> anyhow::Result<()> {
        test_dis_reference(
            expect_file!["../../../spv/dis_reference.spirv_tools_like"],
            Like::SpirvTools,
            true,
        )
    }

    fn test_dis_reference(
        expect: ExpectFile,
        like: Like,
        module_swap_bytes: bool,
    ) -> anyhow::Result<()> {
        let args = Args {
            path: spv("dis_reference"),
            like,
            module_swap_bytes,
        };
        let mut stdout = Vec::new();
        args.run::<CoreInstSet>(&mut anstream::AutoStream::never(&mut stdout))?;
        let stdout = String::from_utf8(stdout)?;
        expect.assert_eq(&stdout);
        Ok(())
    }
}
