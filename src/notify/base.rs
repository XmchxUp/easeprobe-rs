use std::{sync::Arc, time::Duration};

use anyhow::{bail, Result};
use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::{
    global, report, Format, NotifierSetting, ProbeResult, Prober, Retry, DEFAULT_CHANNEL_NAME,
    FORMAT_FUNCS,
};

use super::Notifier;
pub type SendFunc = fn(&str, &str) -> Result<()>;

#[derive(Default)]
pub struct DefaultNotifier {
    pub kind: String,
    pub name: String,
    pub format: Format,
    pub send_func: Option<SendFunc>,
    pub channels: Vec<String>,
    pub dry: bool,
    pub timeout: Duration,
    pub retry: Retry,
}

impl DefaultNotifier {
    async fn send_with_retry(&self, title: &str, msg: &str, tag: &str) {
        let func = || -> Result<()> {
            log::debug!("[{} / {} / {}] - {}", self.kind, self.name, tag, title);
            if let Some(send_func) = self.send_func {
                send_func(title, msg)
            } else {
                log::error!(
                    "[{} / {} / {}] - {} SendFunc is none",
                    self.kind,
                    self.name,
                    tag,
                    title
                );
                bail!("SendFunc is none")
            }
        };
        let err = global::do_retry(&self.kind, &self.name, tag, &self.retry, func).await;
        report::log_send(&self.kind, &self.name, tag, &msg, err);
    }
}

#[async_trait]
impl Notifier for DefaultNotifier {
    fn config(&mut self, conf: &NotifierSetting) -> Result<()> {
        let mut mode = "Live";

        if self.dry {
            mode = "Dry";
        }

        log::info!(
            "Notification [{}] - [{}] is running on {} mode!",
            self.kind,
            self.name,
            mode,
        );

        self.timeout = conf.normalize_timeout(self.timeout);
        self.retry = conf.normalize_retry(&self.retry);

        if self.channels.is_empty() {
            self.channels.push(DEFAULT_CHANNEL_NAME.to_string());
        }

        log::info!(
            "Notification [{}] - [{}] is configured!",
            self.kind,
            self.name,
        );
        Ok(())
    }

    fn kind(&self) -> &str {
        &self.kind
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn channels(&self) -> Vec<String> {
        self.channels.clone()
    }

    async fn notify(&self, result: Arc<ProbeResult>) {
        if self.dry {
            self.dry_notify(result);
            return;
        }
        let title = result.title();
        let msg = (FORMAT_FUNCS.get(&self.format).unwrap().result_fn)(result);

        self.send_with_retry(&title, &msg, "Notification").await;
    }

    async fn notify_stat(&self, probers: Vec<Arc<RwLock<dyn Prober>>>) {
        if self.dry {
            self.dry_notify_stat(probers);
            return;
        }
        let title = "Overall SLA Report";
        let msg = (FORMAT_FUNCS.get(&self.format).unwrap().stat_fn)(probers);

        self.send_with_retry(title, &msg, "SLA").await;
    }

    fn dry_notify(&self, res: Arc<ProbeResult>) {
        log::info!(
            "[{} / {} / dry_notify] - {}",
            self.kind,
            self.name,
            (FORMAT_FUNCS.get(&self.format).unwrap().result_fn)(res)
        );
    }

    fn dry_notify_stat(&self, probers: Vec<Arc<RwLock<dyn Prober>>>) {
        log::info!(
            "[{} / {} / dry_notify_stat] - {}",
            self.kind,
            self.name,
            (FORMAT_FUNCS.get(&self.format).unwrap().stat_fn)(probers)
        );
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_base() {}
}
