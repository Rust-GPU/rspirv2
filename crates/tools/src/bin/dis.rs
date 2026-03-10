use clap::Parser;
use rspirv2_tools::ToolsISA;

pub fn main() -> anyhow::Result<()> {
    rspirv2_tools_gen::dis::Args::parse().run::<ToolsISA>(&mut std::io::stdout().lock())
}
