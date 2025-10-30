use crate::{iprintln, utils::build_client};
use tokio::io::AsyncWriteExt;

pub async fn handle_live_log(flush: bool) -> anyhow::Result<()> {
    iprintln!("Fetching live log...");
    let api_client = build_client()?;
    api_client.get_live_log(flush).await?;
    Ok(())
}

pub async fn handle_history_log(output_file: String) -> anyhow::Result<()> {
    iprintln!("Fetching log history...");
    let api_client = build_client()?;
    let response = api_client.get_log_history().await?;
    let mut file = tokio::fs::File::create(&output_file).await?;
    for log in response.logs {
        let line = log.into_line();
        file.write_all(line.as_bytes()).await?;
        file.write_all(b"\n").await?;
    }
    iprintln!("Log history saved to {}", output_file);
    Ok(())
}
