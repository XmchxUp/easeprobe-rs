use anyhow::Result;
use easeprobe::{Channel, ProbeResult};

#[tokio::main]
async fn main() -> Result<()> {
    logforth::stdout().apply();

    let mut c = Channel::new("empty");
    c.configure();

    c.send(ProbeResult {
        name: "test".to_string(),
        endpoint: todo!(),
        start_time: todo!(),
        start_timestamp: todo!(),
        round_trip_time: todo!(),
        status: todo!(),
        pre_status: todo!(),
        message: todo!(),
        latest_downtime: todo!(),
        recovery_time: todo!(),
        stat: todo!(),
    })?;

    let v = c.channel().unwrap().recv().await.unwrap();
    log::info!("{}", v.name);

    Ok(())
}
