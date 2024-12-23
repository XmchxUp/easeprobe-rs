use anyhow::Result;
use easeprobe::{Channel, ProbeResult};

#[tokio::main]
async fn main() -> Result<()> {
    let mut c = Channel::new("empty");
    c.configure();

    c.send(ProbeResult {
        name: "test".to_string(),
    })?;

    let v = c.channel().unwrap().recv().await.unwrap();
    println!("{}", v.name);

    Ok(())
}
