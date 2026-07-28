use anyhow::{Context, bail};
use std::path::{Path, PathBuf};
use std::process::Command;

/// A git repo
pub struct GitRepo {
    pub root: PathBuf,
}

impl GitRepo {
    pub fn resolve_root() -> anyhow::Result<Self> {
        let out = sh(Path::new("."), "git", &["rev-parse", "--show-toplevel"])?;
        Ok(Self {
            root: PathBuf::from(out.trim()),
        })
    }

    pub fn git(&self, args: &[&str]) -> anyhow::Result<String> {
        self.sh("git", args)
    }

    pub fn sh(&self, cmd: &str, args: &[&str]) -> anyhow::Result<String> {
        sh(&self.root, cmd, args)
    }
}

fn sh(cwd: &Path, cmd: &str, args: &[&str]) -> anyhow::Result<String> {
    let output = Command::new(cmd)
        .current_dir(cwd)
        .args(args)
        .output()
        .with_context(|| format!("failed to run `{cmd} {}`", args.join(" ")))?;
    if !output.status.success() {
        bail!(
            "`{cmd} {}` failed with {}: {}",
            args.join(" "),
            output.status,
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    String::from_utf8(output.stdout)
        .with_context(|| format!("`{cmd} {}` output was not valid UTF-8", args.join(" ")))
}

/// List of remote branches for some submodule
pub struct SubmoduleBranches {
    refs: String,
}

impl SubmoduleBranches {
    /// Fetch the submodule's remote branches
    pub fn fetch(repo: &GitRepo, submodule_path: &str, prefix: &str) -> anyhow::Result<Self> {
        let url = repo
            .git(&[
                "config",
                "--file",
                ".gitmodules",
                &format!("submodule.{submodule_path}.url"),
            ])
            .context("reading submodule url from .gitmodules")?;
        let url = url.trim();
        let refs = repo
            .git(&["ls-remote", "--heads", url, &format!("{prefix}*")])
            .context("listing remote branches")?;
        Ok(Self { refs })
    }

    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.refs
            .lines()
            .filter_map(|line| line.split('\t').nth(1))
            .filter_map(|r| r.strip_prefix("refs/heads/"))
    }

    /// Determine the branch with the highest semver
    pub fn highest_semver_branch(&self, prefix: &str) -> anyhow::Result<String> {
        let best = self
            .iter()
            .filter_map(|branch| parse_branch_semver(branch, prefix).map(|ver| (ver, branch)))
            .max_by_key(|(ver, _)| *ver)
            .map(|(_, branch)| branch.to_string());
        best.with_context(|| format!("no remote branch matching `{prefix}*.*.*` found"))
    }
}

/// Parse a `{prefix}X.Y.Z` branch name to semver tuple
pub fn parse_branch_semver(branch: &str, prefix: &str) -> Option<(u32, u32, u32)> {
    let rest = branch.strip_prefix(prefix)?;
    let mut parts = rest.split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next()?.parse().ok()?;
    let patch = parts.next()?.parse().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some((major, minor, patch))
}
