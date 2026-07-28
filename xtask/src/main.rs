use clap::{Parser, Subcommand};

mod git;
mod headers;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Clone, Debug, Subcommand)]
enum Commands {
    Headers {
        #[command(subcommand)]
        command: headers::Headers,
    },
}

impl Commands {
    fn run(self) -> anyhow::Result<()> {
        match self {
            Commands::Headers { command } => command.run(),
        }
    }
}

fn main() -> anyhow::Result<()> {
    Cli::parse().command.run()
}
