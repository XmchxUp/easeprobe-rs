use std::{sync::Arc, time::Duration};

use anyhow::Result;
use async_trait::async_trait;

use crate::{global, probe, report, ProbeResult, FORMAT_FUNCS};

use super::Notifier;

pub struct DefaultNotify {
    pub kind: String,
    pub name: String,
    pub format: report::Format,
    pub send_func: fn(String, String) -> Result<()>,
    pub channels: Vec<String>,
    pub dry: bool,
    pub timeout: Duration,
    pub retry: global::Retry,
}

#[async_trait]
impl Notifier for DefaultNotify {
    fn kind(&self) -> &str {
        &self.kind
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn channels(&self) -> &Vec<String> {
        &self.channels
    }

    async fn notify(&self, result: Arc<ProbeResult>) {
        if self.dry {
            self.dry_notify(result);
            return;
        }
        let title = result.title();
    }

    fn notify_stat(&self, probers: Vec<Box<dyn crate::Prober>>) {
        todo!()
    }

    fn dry_notify(&self, res: Arc<ProbeResult>) {
        log::info!(
            "[{} / {} / dry_notify] - {}",
            self.kind,
            self.name,
            (FORMAT_FUNCS.get(&self.format).unwrap().result_fn)(res)
        );
    }

    fn dry_notify_stat(&self, probers: Vec<Box<dyn crate::Prober>>) {
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
