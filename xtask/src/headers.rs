use crate::git::{GitRepo, SubmoduleBranches};
use clap::{Parser, Subcommand};

const SUBMODULE_PATH: &str = "crates/grammar/headers";
const BRANCH_PREFIX: &str = "vulkan-sdk-";

/// Interact with the SPIR-V headers submodule
#[derive(Clone, Debug, Subcommand)]
pub enum Headers {
    Update(HeadersUpdate),
    List(HeadersList),
}

impl Headers {
    pub fn run(self) -> anyhow::Result<()> {
        let repo = GitRepo::resolve_root()?;
        match self {
            Headers::Update(cmd) => cmd.run(&repo),
            Headers::List(cmd) => cmd.run(&repo),
        }
    }
}

/// Update the SPIR-V headers submodule to the most recent release or some specified rev
#[derive(Clone, Debug, Parser)]
pub struct HeadersUpdate {
    /// Branch or revision to point the submodule at. Defaults to the branch
    /// with the highest semver matching `vulkan-sdk-*.*.*`.
    rev: Option<String>,
}

impl HeadersUpdate {
    pub fn run(self, repo: &GitRepo) -> anyhow::Result<()> {
        let branch = match self.rev {
            Some(rev) => rev,
            None => SubmoduleBranches::fetch(repo, SUBMODULE_PATH, BRANCH_PREFIX)?
                .highest_semver_branch(BRANCH_PREFIX)?,
        };
        println!("Setting submodule `{SUBMODULE_PATH}` to `{branch}`");

        repo.git(&[
            "submodule",
            "set-branch",
            "--branch",
            &branch,
            "--",
            SUBMODULE_PATH,
        ])?;
        repo.git(&["submodule", "update", "--remote", SUBMODULE_PATH])?;
        Ok(())
    }
}

/// List the release branches of the SPIR-V headers repo
#[derive(Clone, Debug, Parser)]
pub struct HeadersList {}

impl HeadersList {
    #[allow(clippy::unused_self)]
    pub fn run(self, repo: &GitRepo) -> anyhow::Result<()> {
        let refs = SubmoduleBranches::fetch(repo, SUBMODULE_PATH, BRANCH_PREFIX)?;
        for branch in refs.iter() {
            println!("{}", branch);
        }
        Ok(())
    }
}
