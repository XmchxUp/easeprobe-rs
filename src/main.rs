use std::time::Duration;

use anyhow::Result;
use easeprobe::{Channel, ProbeResult};

#[tokio::main]
async fn main() -> Result<()> {
    logforth::stdout().apply();

    let mut c = Channel::new("empty");
    c.configure().await;

    c.send(ProbeResult::default()).await;

    tokio::time::sleep(Duration::new(2, 1000)).await;

    Ok(())
}
