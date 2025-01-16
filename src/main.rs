use std::{sync::Arc, time::Duration};

use anyhow::Result;
use easeprobe::{
    cmd::config_probers, config_notifiers, manager, run_probers, DefaultNotifier, DefaultProber,
    Notifier, Prober,
};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() -> Result<()> {
    logforth::stdout().apply();

    manager::set_channel("test").await;

    let mut prober = DefaultProber::default();
    prober.probe_fn = Some(|| -> Option<String> { Some("Test".to_string()) });

    let probers: Vec<Arc<RwLock<dyn Prober>>> = vec![Arc::new(RwLock::new(prober))];
    config_probers(&probers).await;
    manager::set_probers(probers.clone()).await;

    let mut notifier = DefaultNotifier::default();
    notifier.send_func = Some(|a: &str, b: &str| -> Result<()> { Ok(()) });

    let notifiers: Vec<Arc<RwLock<dyn Notifier>>> = vec![Arc::new(RwLock::new(notifier))];
    config_notifiers(&notifiers).await;
    manager::set_notifiers(notifiers.clone()).await;

    run_probers(probers);

    tokio::time::sleep(Duration::new(30, 1000)).await;
    Ok(())
}
