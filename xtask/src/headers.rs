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
    /// Proceed updating things anyway, even if the version didn't change. Useful if submodules are in a weird state.
    #[clap(long)]
    force: bool,
    /// Skip running autogen to update generated files
    #[clap(long)]
    skip_autogen: bool,
    /// Skip updating the workspace version `+sdk-<version>` suffix
    #[clap(long)]
    skip_version: bool,
    /// Skip updating the changelog
    #[clap(long)]
    skip_changelog: bool,
}

impl HeadersUpdate {
    pub fn run(self, repo: &GitRepo) -> anyhow::Result<()> {
        let branch = if let Some(rev) = self.rev {
            rev
        } else {
            let branch = SubmoduleBranches::fetch(repo, SUBMODULE_PATH, BRANCH_PREFIX)?
                .highest_semver_branch(BRANCH_PREFIX)?;
            println!("Resolved newest SPIR-V header version `{branch}`");
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
        let old_branch = old_branch.trim();
        if !self.force && old_branch == branch {
            println!("SPIR-V headers is already set to `{branch}`, skipping");
            return Ok(());
        }

        println!("Updating SPIR-V headers from `{old_branch}` to `{branch}`");
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
            set_workspace_sdk_version(repo, &branch, self.force)
                .context("updating workspace sdk version suffix")?;
        }

        if !self.skip_changelog {
            update_changelog(repo, &branch).context("updating changelog")?;
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
fn set_workspace_sdk_version(repo: &GitRepo, branch: &str, force: bool) -> anyhow::Result<()> {
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
    if !force && old_version == new_version {
        println!("Workspace version already set to `{new_version}`");
        return Ok(());
    }

    println!("Updating workspace version from `{old_version}` to `{new_version}`");
    let content = content.replacen(old_version, &new_version, 1);
    fs::write(&path, content).context("writing Cargo.toml")?;
    Ok(())
}

/// Update the changelog to include a sdk update message
fn update_changelog(repo: &GitRepo, branch: &str) -> anyhow::Result<()> {
    const UNRELEASED_TAG: &str = "## [Unreleased]\n";
    let path = repo.root.join("CHANGELOG.md");
    let replacement = format!(
        "{UNRELEASED_TAG}- Update Vulkan sdk to [{branch}](https://github.com/KhronosGroup/Vulkan-Headers/tree/{branch})\n"
    );
    let content = fs::read_to_string(&path).context("reading CHANGELOG.md")?;
    let content = content.replacen(UNRELEASED_TAG, &replacement, 1);
    fs::write(&path, content).context("writing CHANGELOG.md")?;
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
