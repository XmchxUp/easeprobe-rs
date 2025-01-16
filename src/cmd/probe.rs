use std::{sync::Arc, time::Duration};

use tokio::sync::RwLock;

use crate::{manager::get_channel, ProbeSetting, Prober};

pub async fn config_probers(probers: &Vec<Arc<RwLock<dyn Prober>>>) {
    let setting = ProbeSetting::default();
    for ele in probers {
        ele.write().await.config(&setting);
    }
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
