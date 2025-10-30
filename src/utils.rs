use std::env;
use std::path::PathBuf;

use anyhow::Context;
use chrono::{TimeZone, Utc};
use tokio::process::Command;

use crate::api::ApiClient;
use crate::config::ConfigManager;
use crate::vprintln;

pub async fn create_git_archive() -> anyhow::Result<String> {
    let current_dir: PathBuf = env::current_dir()?;
    let current_branch = Command::new("git")
        .current_dir(&current_dir)
        .args(["branch", "--show-current"])
        .output()
        .await
        .context("Failed to execute `git branch --show-current`")?;

    if !current_branch.status.success() {
        anyhow::bail!("Failed to get current branch")
    }

    let current_branch = String::from_utf8(current_branch.stdout)?.trim().to_string();

    let repo_name = current_dir
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or_else(|| anyhow::anyhow!("Current directory name is not valid UTF-8"))?
        .to_string();

    let archive_name = format!("{}-{}", repo_name, current_branch);
    let output_file = format!("{}/{}.zip", current_dir.display(), archive_name);

    let status = Command::new("git")
        .current_dir(&current_dir)
        .args([
            "archive",
            "--format=zip",
            &format!("--prefix={}/", archive_name),
            "-o",
            &output_file,
            &current_branch,
        ])
        .status()
        .await?;

    if !status.success() {
        anyhow::bail!("git archive failed");
    }

    vprintln!("Archive created: {}", output_file);
    Ok(output_file)
}

pub fn build_client() -> anyhow::Result<ApiClient> {
    let credentials = ConfigManager::load_credentials().with_context(|| {
        "Failed to load credentials. Check that the config file exists and is valid, or sign in again."
    })?;
    Ok(ApiClient::new(credentials.server_url, credentials.token))
}

pub fn strf_timestamp(secs: i64) -> anyhow::Result<String> {
    let dt_utc = Utc.timestamp_opt(secs, 0).unwrap();
    Ok(dt_utc.format("%Y-%m-%d %H:%M:%S").to_string())
}
