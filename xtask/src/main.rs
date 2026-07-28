use clap::Parser;

mod headers;

fn main() -> anyhow::Result<()> {
    Command::parse().run()
}

#[derive(Clone, Debug, Parser)]
pub enum Command {
    UpdateHeaders(headers::Headers),
}

impl Command {
    pub fn run(self) -> anyhow::Result<()> {
        match self {
            Command::UpdateHeaders(cmd) => cmd.run(),
        }
    }
}
