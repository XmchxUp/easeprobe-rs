use anyhow::Result;
use easeprobe::start;

#[tokio::main]
async fn main() -> Result<()> {
    start().await?;
    Ok(())
}
