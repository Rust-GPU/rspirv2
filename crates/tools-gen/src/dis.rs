use clap::{Parser, ValueEnum};
use rspirv2::dis::{DisOptions, SpvInstDisCtx};
use rspirv2::inst::SpvInstEncoding;
use rspirv2::module::Module;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

#[derive(Clone, Debug, Default, Parser)]
pub struct Args {
    /// path to SPIR-V file
    pub path: PathBuf,
    /// Emit disassembly like as if it was emitted by this tool
    #[arg(short, long, default_value_t)]
    pub profile: Profile,
    /// color
    #[clap(long, default_value_t)]
    pub color: clap::ColorChoice,
    /// Swap bytes of the SPIR-V module before parsing, for testing
    #[clap(skip)]
    pub module_swap_bytes: bool,
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
    pub fn run<ISA: SpvInstDisCtx + SpvInstEncoding>(
        &mut self,
        stdout: &mut impl anstream::stream::RawStream,
    ) -> anyhow::Result<()> {
        profiling::function_scope!();
        self.resolve_auto_color(&*stdout);
        let mut writer = BufWriter::new(stdout);
        self.run_inner::<ISA>(&mut writer)
    }

    pub fn run_inner<ISA: SpvInstDisCtx + SpvInstEncoding>(
        &self,
        stdout: &mut impl Write,
    ) -> anyhow::Result<()> {
        profiling::function_scope!();
        let mut bytes = {
            profiling::scope!("std::fs::read");
            std::fs::read(&self.path)?
        };
        if self.module_swap_bytes {
            profiling::scope!("module_swap_bytes");
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
