use std::{
    fs::OpenOptions,
    io::Write,
    sync::{Arc, Mutex},
};

use anyhow::{bail, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{
    global::NotifierSetting,
    probe::{ProbeResult, Prober},
};

use super::{DefaultNotifier, Notifier};

#[derive(Debug, Serialize, Deserialize)]
pub struct LogConfig {
    #[serde(flatten)]
    default: DefaultNotifier,
    file: String,
    #[serde(default)]
    host: String,
    #[serde(default)]
    network: String,
}

#[async_trait]
impl Notifier for LogConfig {
    fn kind(&self) -> &str {
        self.default.kind()
    }

    fn name(&self) -> &str {
        self.default.name()
    }

    fn channels(&self) -> Vec<String> {
        self.default.channels()
    }

    fn config(&mut self, g_conf: &NotifierSetting) -> Result<()> {
        if self.default.kind.is_empty() {
            self.default.kind = "log".to_string();
        }

        self.default.config(g_conf)?;

        if !self.file.is_empty() {
            let log_target = Arc::new(Mutex::new(
                OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(&self.file)
                    .map_err(|e| anyhow::anyhow!("Failed to open log file {}: {}", self.file, e))?,
            ));
            let send_fn = move |title: &str, msg: &str| {
                let mut file = log_target.lock().unwrap();
                writeln!(file, "Notification: {}", title)?;
                for line in msg.lines() {
                    writeln!(file, "{}", line)?;
                }
                file.flush()?;
                Ok(())
            };
            self.default.send_func = Some(Box::new(send_fn));
        }

        Ok(())
    }

    async fn notify(&self, result: Arc<ProbeResult>) {
        self.default.notify(result).await
    }

    async fn notify_stat(&self, probers: Vec<Arc<RwLock<dyn Prober>>>) {
        self.default.notify_stat(probers).await
    }

    fn dry_notify(&self, res: Arc<ProbeResult>) {
        self.default.dry_notify(res)
    }

    fn dry_notify_stat(&self, probers: Vec<Arc<RwLock<dyn Prober>>>) {
        self.default.dry_notify_stat(probers)
    }
}
