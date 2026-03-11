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
        let module = Module::from_bytes(std::fs::read(&self.path)?.as_slice())?;
        write!(stdout, "{}", module.dis::<ISA>(self.to_dis_opts()?)?)?;
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
    use crate::test::BLESS;
    use rspirv2::core::inst_set::CoreInstSet;
    use spv::{spv, spv_folder};
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
        test_dis_reference("dis_reference.rspirv2", Like::Default)
    }

    #[test]
    fn test_dis_reference_rspirv() -> anyhow::Result<()> {
        test_dis_reference("dis_reference.rspirv_like", Like::Rspirv)
    }

    #[test]
    fn test_dis_reference_spirv_tools() -> anyhow::Result<()> {
        test_dis_reference("dis_reference.spirv_tools_like", Like::SpirvTools)
    }

    fn test_dis_reference(ref_path: &str, like: Like) -> anyhow::Result<()> {
        let args = Args {
            path: spv("dis_reference"),
            like,
        };
        let mut stdout = Vec::new();
        args.run::<CoreInstSet>(&mut stdout)?;
        let stdout = String::from_utf8(stdout)?;
        let ref_path = spv_folder().join(ref_path);
        if BLESS {
            std::fs::write(&ref_path, &stdout)?;
        }
        let reference = std::fs::read_to_string(ref_path)?;
        assert_eq!(
            reference, stdout,
            "`dis_reference.rspirv` and `rspirv2-dis dis_reference.spv` don't match, please run \
            `cargo dis ./dis/dis_reference.spv &> ./dis/dis_reference.rspirv2`"
        );
        Ok(())
    }
}
