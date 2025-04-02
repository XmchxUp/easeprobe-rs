use std::{sync::Arc, time::Duration};

use tokio::sync::RwLock;

use crate::{conf, manager::get_channel, Prober, Status};

pub async fn config_probers(probers: &mut Vec<Arc<RwLock<dyn Prober>>>, gs: &conf::Settings) {
    let mut valid_probers = Vec::new();

    for ele in probers.iter() {
        let mut e = ele.write().await;
        if let Err(err) = e.config(&gs.probe).await {
            let result = e.result();
            result.status = Status::Bad;
            result.message = format!("Bad Configuration: {}", err);
            log::error!(
                "Bad Probe Configuration for prober {} {}: {}",
                e.kind(),
                e.name(),
                err,
            );
            continue;
        }
        valid_probers.push(Arc::clone(ele));
    }

    *probers = valid_probers;
}

pub fn run_probers(probers: Vec<Arc<RwLock<dyn Prober>>>) {
    for prober in probers {
        let p = Arc::clone(&prober);
        tokio::spawn(async move {
            loop {
                let (res, channels) = {
                    let mut p = p.write().await;

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
