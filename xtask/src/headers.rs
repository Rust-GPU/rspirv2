use clap::Parser;

#[derive(Clone, Debug, Parser)]
pub enum Headers {
    Update,
}

impl Headers {
    pub fn run(self) -> anyhow::Result<()> {
        Ok(())
    }
}
