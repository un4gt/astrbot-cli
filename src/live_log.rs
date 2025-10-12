use crate::{iprintln, utils::build_client};

pub async fn handle_live_log(flush: bool) -> anyhow::Result<()> {
    iprintln!("Fetching live log...");
    let api_client = build_client()?;
    api_client.get_live_log(flush).await?;
    Ok(())
}
