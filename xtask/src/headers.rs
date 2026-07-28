use crate::git::{GitRepo, SubmoduleBranches, parse_branch_semver};
use anyhow::{Context, bail};
use clap::{Parser, Subcommand};
use std::fs;
use std::process::Command;

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
    /// Skip running autogen to update generated files
    #[clap(long)]
    skip_autogen: bool,
    /// Skip updating the workspace version `+sdk-<version>` suffix
    #[clap(long)]
    skip_version: bool,
}

impl HeadersUpdate {
    pub fn run(self, repo: &GitRepo) -> anyhow::Result<()> {
        let branch = if let Some(rev) = self.rev {
            rev
        } else {
            let branch = SubmoduleBranches::fetch(repo, SUBMODULE_PATH, BRANCH_PREFIX)?
                .highest_semver_branch(BRANCH_PREFIX)?;
            println!("Newest SPIR-V header version is `{branch}`");
            branch
        };

        let old_branch = repo
            .git(&[
                "config",
                "--file",
                ".gitmodules",
                &format!("submodule.{SUBMODULE_PATH}.branch"),
            ])
            .context("reading submodule branch from .gitmodules")?;
        if old_branch.trim() == branch {
            println!("SPIR-V headers is already set to `{branch}`, skipping");
            return Ok(());
        }

        println!("Setting SPIR-V headers to `{branch}`");
        repo.git(&[
            "submodule",
            "set-branch",
            "--branch",
            &branch,
            "--",
            SUBMODULE_PATH,
        ])?;
        repo.git(&["submodule", "update", "--remote", SUBMODULE_PATH])?;

        if !self.skip_version {
            set_workspace_sdk_version(repo, &branch)
                .context("updating workspace sdk version suffix")?;
        }

        if !self.skip_autogen {
            println!("Running `cargo autogen`");
            let status = Command::new("cargo")
                .current_dir(&repo.root)
                .arg("autogen")
                .status()
                .context("failed to spawn `cargo autogen`")?;
            if !status.success() {
                bail!("`cargo autogen` failed with {}", status);
            }
        }
        Ok(())
    }
}

/// Update the workspace version `+sdk-<version>` appendix
fn set_workspace_sdk_version(repo: &GitRepo, branch: &str) -> anyhow::Result<()> {
    let sdk_version = {
        let (major, minor, patch) =
            parse_branch_semver(branch, BRANCH_PREFIX).context("failed to parse branch")?;
        format!("{major}.{minor}.{patch}")
    };

    const PREFIX: &str = "version = \"";
    const SUFFIX: &str = "\"";
    let path = repo.root.join("Cargo.toml");
    let content = fs::read_to_string(&path).context("reading Cargo.toml")?;

    let old_version = content
        .lines()
        .filter_map(|line| line.strip_prefix(PREFIX))
        .find_map(|line| line.strip_suffix(SUFFIX))
        .context("no workspace `version` line found")?;
    let base = old_version.split('+').next().unwrap_or(old_version);
    let new_version = format!("{base}+sdk-{sdk_version}");
    if old_version == new_version {
        println!("Workspace version already `{new_version}`");
        return Ok(());
    }

    println!("Setting workspace version to `{new_version}`");
    let content = content.replacen(old_version, &new_version, 1);
    fs::write(&path, content).context("writing Cargo.toml")?;
    Ok(())
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
