use crate::{iprintln, utils::build_client};

pub async fn handle_live_log() -> anyhow::Result<()> {
    iprintln!("Fetching plugin list...");
    let api_client = build_client()?;
    api_client.get_live_log().await?;
    Ok(())
}
