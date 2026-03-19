use clap::{Parser, ValueEnum};
use rspirv2::dis::{DisOptions, InstSetDisCtx};
use rspirv2::module::Module;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

#[derive(Clone, Debug, Default, Parser)]
pub struct Args {
    /// path to SPIR-V file
    path: PathBuf,
    /// Emit disassembly like as if it was emitted by this tool
    #[arg(short, long, default_value_t)]
    profile: Profile,
    /// color
    #[clap(long, default_value_t)]
    color: clap::ColorChoice,
    /// Swap bytes of the SPIR-V module before parsing, for testing
    #[clap(skip)]
    module_swap_bytes: bool,
}

#[derive(Clone, Debug, Default, ValueEnum)]
pub enum Profile {
    #[default]
    Default,
    Rspirv,
    SpirvTools,
}

impl std::fmt::Display for Profile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl Args {
    pub fn resolve_auto_color(&mut self, stream: &impl anstream::stream::RawStream) {
        if self.color == clap::ColorChoice::Auto {
            self.color = match anstream::AutoStream::choice(stream) {
                anstream::ColorChoice::Auto => unreachable!(),
                anstream::ColorChoice::AlwaysAnsi | anstream::ColorChoice::Always => {
                    clap::ColorChoice::Always
                }
                anstream::ColorChoice::Never => clap::ColorChoice::Never,
            }
        }
    }
}

impl Args {
    pub fn run<ISA: InstSetDisCtx>(
        &mut self,
        stdout: &mut impl anstream::stream::RawStream,
    ) -> anyhow::Result<()> {
        self.resolve_auto_color(&*stdout);
        let mut writer = BufWriter::new(stdout);
        self.run_inner::<ISA>(&mut writer)
    }

    pub fn run_inner<ISA: InstSetDisCtx>(&self, stdout: &mut impl Write) -> anyhow::Result<()> {
        let mut bytes = std::fs::read(&self.path)?;
        if self.module_swap_bytes {
            for chunk in bytes.as_chunks_mut::<4>().0.iter_mut() {
                *chunk = u32::from_ne_bytes(*chunk).swap_bytes().to_ne_bytes();
            }
        }

        let module = Module::<ISA>::from_bytes_unchecked(bytes.as_slice())?;
        drop(bytes);
        let dis = module.inst.as_raw_slice().dis::<ISA>(self.to_dis_opts()?);
        write!(stdout, "{}", dis)?;
        Ok(())
    }

    pub fn to_dis_opts(&self) -> anyhow::Result<DisOptions> {
        let mut opt = match self.profile {
            Profile::Default => DisOptions::default(),
            Profile::Rspirv => DisOptions::like_rspirv(),
            Profile::SpirvTools => DisOptions::like_spirv_tools(),
        };
        opt.color = matches!(self.color, clap::ColorChoice::Always);
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
        test_disassembly(
            spv("dis_reference"),
            expect_file!["../../../spv/dis_reference.rspirv2"],
            Profile::Default,
            false,
        )
    }

    #[test]
    fn test_dis_reference_default_be() -> anyhow::Result<()> {
        test_disassembly(
            spv("dis_reference"),
            expect_file!["../../../spv/dis_reference.rspirv2"],
            Profile::Default,
            true,
        )
    }

    #[test]
    fn test_dis_reference_rspirv() -> anyhow::Result<()> {
        test_disassembly(
            spv("dis_reference"),
            expect_file!["../../../spv/dis_reference.rspirv_like"],
            Profile::Rspirv,
            false,
        )
    }

    #[test]
    fn test_dis_reference_rspirv_be() -> anyhow::Result<()> {
        test_disassembly(
            spv("dis_reference"),
            expect_file!["../../../spv/dis_reference.rspirv_like"],
            Profile::Rspirv,
            true,
        )
    }

    #[test]
    fn test_dis_reference_spirv_tools() -> anyhow::Result<()> {
        test_disassembly(
            spv("dis_reference"),
            expect_file!["../../../spv/dis_reference.spirv_tools_like"],
            Profile::SpirvTools,
            false,
        )
    }

    #[test]
    fn test_dis_reference_spirv_tools_be() -> anyhow::Result<()> {
        test_disassembly(
            spv("dis_reference"),
            expect_file!["../../../spv/dis_reference.spirv_tools_like"],
            Profile::SpirvTools,
            true,
        )
    }

    #[test]
    fn test_texture_grad_offset_default() -> anyhow::Result<()> {
        test_disassembly(
            spv("textureGradOffset"),
            expect_file!["../../../spv/textureGradOffset.rspirv2"],
            Profile::Default,
            false,
        )
    }

    #[test]
    fn test_texture_grad_offset_default_be() -> anyhow::Result<()> {
        test_disassembly(
            spv("textureGradOffset"),
            expect_file!["../../../spv/textureGradOffset.rspirv2"],
            Profile::Default,
            true,
        )
    }

    #[test]
    fn test_texture_grad_offset_rspirv() -> anyhow::Result<()> {
        test_disassembly(
            spv("textureGradOffset"),
            expect_file!["../../../spv/textureGradOffset.rspirv_like"],
            Profile::Rspirv,
            false,
        )
    }

    #[test]
    fn test_texture_grad_offset_rspirv_be() -> anyhow::Result<()> {
        test_disassembly(
            spv("textureGradOffset"),
            expect_file!["../../../spv/textureGradOffset.rspirv_like"],
            Profile::Rspirv,
            true,
        )
    }

    #[test]
    fn test_texture_grad_offset_spirv_tools() -> anyhow::Result<()> {
        test_disassembly(
            spv("textureGradOffset"),
            expect_file!["../../../spv/textureGradOffset.spirv_tools_like"],
            Profile::SpirvTools,
            false,
        )
    }

    #[test]
    fn test_texture_grad_offset_spirv_tools_be() -> anyhow::Result<()> {
        test_disassembly(
            spv("textureGradOffset"),
            expect_file!["../../../spv/textureGradOffset.spirv_tools_like"],
            Profile::SpirvTools,
            false,
        )
    }

    fn test_disassembly(
        path: PathBuf,
        expect: ExpectFile,
        profile: Profile,
        module_swap_bytes: bool,
    ) -> anyhow::Result<()> {
        let args = Args {
            path,
            profile,
            module_swap_bytes,
            color: clap::ColorChoice::Never,
        };
        let mut stdout = Vec::new();
        args.run_inner::<CoreInstSet>(&mut stdout)?;
        let stdout = String::from_utf8(stdout)?;
        expect.assert_eq(&stdout);
        Ok(())
    }
}
