use anyhow::Result;
use easeprobe::{Channel, ProbeResult};

#[tokio::main]
async fn main() -> Result<()> {
    logforth::stdout().apply();

    let mut c = Channel::new("empty");
    c.configure();

    c.send(ProbeResult::default())?;

    let v = c.channel().unwrap().recv().await.unwrap();
    log::info!("{}", v.name);

    Ok(())
}
