use crate::api::ApiClient;
use crate::config::ConfigManager;
use crate::iprintln;
use crate::utils::create_git_archive;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[allow(dead_code)]
pub struct Plugin {
    pub name: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub repo: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub author: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub desc: String,
    pub version: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub reserved: bool,
    pub activated: bool,
    #[serde(skip_serializing, skip_deserializing)]
    pub online_version: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub handlers: serde_json::Value,
}

impl Plugin {
    pub fn pretty_print(&self) {
        println!("Name(id): {}", self.name);
        println!("Version: {}", self.version);
        println!("Activated: {}", self.activated);
        println!()
    }
}

fn build_client() -> anyhow::Result<ApiClient> {
    let credentials = ConfigManager::load_credentials()?;
    Ok(ApiClient::new(credentials.server_url, credentials.token))
}

pub async fn handle_plugin_get() -> anyhow::Result<Vec<Plugin>> {
    iprintln!("Fetching plugin list...");

    let api_client = build_client()?;
    let plugins = api_client.get_plugins().await?;
    Ok(plugins)
}

pub async fn handle_plugin_install_from_github(git_repo: &String) -> anyhow::Result<String> {
    let api_client = build_client()?;
    let ret = api_client.install_remote_plugin(git_repo).await?;
    Ok(ret)
}

pub async fn handle_plugin_install_from_local() -> anyhow::Result<String> {
    let local_plugin_path = create_git_archive().await?;
    let api_client = build_client()?;
    let ret = api_client.install_local_plugin(&local_plugin_path).await?;
    Ok(ret)
}

pub async fn handle_plugin_common_actions(
    plugin_name: &str,
    action_name: &str,
) -> anyhow::Result<String> {
    let api_client = build_client()?;
    let ret = api_client
        .plugin_common_actions_request(plugin_name, action_name)
        .await?;
    Ok(ret)
}
