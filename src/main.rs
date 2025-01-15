use std::time::Duration;

use anyhow::Result;
use easeprobe::{Channel, ProbeResult};

#[tokio::main]
async fn main() -> Result<()> {
    logforth::stdout().apply();

    let c = Channel::new("empty").await;

    c.send(ProbeResult::default()).await;

    tokio::time::sleep(Duration::new(2, 1000)).await;

    Ok(())
}
