use std::{sync::Arc, time::Duration};

use anyhow::Result;
use easeprobe::{get_channel, set_channel, set_probers, DefaultProber, Prober};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<()> {
    logforth::stdout().apply();

    set_channel("test").await;

    let probers: Vec<Arc<Mutex<dyn Prober>>> = vec![Arc::new(Mutex::new(DefaultProber::default()))];
    config_probers(&probers).await;

    set_probers(probers.clone()).await;

    run_probers(probers);

    tokio::time::sleep(Duration::new(30, 1000)).await;

    Ok(())
}

async fn config_probers(probers: &Vec<Arc<Mutex<dyn Prober>>>) {
    for ele in probers {
        ele.lock().await.config();
    }
}

fn run_probers(probers: Vec<Arc<Mutex<dyn Prober>>>) {
    for prober in probers {
        let p = Arc::clone(&prober);
        tokio::spawn(async move {
            loop {
                let (res, channels) = {
                    let mut p = p.lock().await;

                    (p.probe().await, p.channels())
                };

                for ch in channels {
                    if let Some(ch) = get_channel(&ch).await {
                        ch.send(res.clone()).await;
                    }
                }

                tokio::time::sleep(Duration::new(1, 1000)).await;
            }
        });
    }
}
