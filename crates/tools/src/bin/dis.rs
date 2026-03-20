use clap::Parser;
use rspirv2_tools::ToolsISA;

pub fn main() -> anyhow::Result<()> {
    profiling::function_scope!();
    rspirv2_tools_gen::dis::Args::parse().run::<ToolsISA>(&mut std::io::stdout().lock())
}
