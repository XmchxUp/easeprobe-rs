use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{
    NotificationStrategySettings, ProbeSettings, StatusChangeThresholdSettings,
    DEFAULT_CHANNEL_NAME,
};

use super::{ProbeBehavior, ProbeResult, Prober};

pub type ProbeFuncType = fn() -> Result<(bool, String)>;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DefaultProber<B: ProbeBehavior> {
    #[serde(skip)]
    pub kind: String,
    #[serde(skip)]
    pub tag: String,
    pub name: String,
    #[serde(default)]
    pub channels: Vec<String>,
    #[serde(default = "default_timeout")]
    pub timeout: Duration,
    #[serde(default = "default_interval")]
    pub interval: Duration,
    #[serde(flatten)]
    pub behavior: B,
    #[serde(skip)]
    pub result: ProbeResult,
    #[serde(flatten)]
    pub threshold: StatusChangeThresholdSettings,
    #[serde(alias = "alert", default)]
    pub notification: NotificationStrategySettings,
}

fn default_timeout() -> Duration {
    Duration::from_secs(30) // Default from Go: 30s
}

fn default_interval() -> Duration {
    Duration::from_secs(60) // Default from Go: 1m
}

impl<B: ProbeBehavior> DefaultProber<B> {
    fn log_title(&self) -> String {
        if self.tag.is_empty() {
            format!("[{} / {} / {}]", self.kind, self.tag, self.name)
        } else {
            format!("[{} / {}]", self.kind, self.name)
        }
    }
}

#[async_trait]
impl<B: ProbeBehavior + Send + Sync> Prober for DefaultProber<B> {
    fn kind(&self) -> &str {
        &self.kind
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn channels(&self) -> Vec<String> {
        self.channels.clone()
    }

    fn timeout(&self) -> &Duration {
        &self.timeout
    }

    fn interval(&self) -> &Duration {
        &self.interval
    }

    fn result(&mut self) -> &mut ProbeResult {
        &mut self.result
    }

    async fn probe(&mut self) -> ProbeResult {
        let now = SystemTime::now();

        self.result.start_time = now;
        self.result.start_timestamp = now.duration_since(UNIX_EPOCH).unwrap().as_millis();

        let (stat, msg) = self.behavior.do_probe().await.unwrap();
        self.result.round_trip_time = now.elapsed().unwrap();
        self.result
            .stat
            .status_counter
            .append_status(stat, msg.clone());
        let title = self.result.status.title();

        if self.tag.is_empty() {
            self.result.message = format!("{} ({}): {}", title, self.kind, msg)
        } else {
            self.result.message = format!("{} ({}/{}): {}", title, self.kind, self.tag, msg)
        }

        self.result.clone()
    }

    async fn config(&mut self, setting: &ProbeSettings) -> Result<()> {
        self.threshold = setting.normalize_threshold(self.threshold);
        self.notification = setting.normalize_notification_strategy(self.notification);

        if self.channels.is_empty() {
            self.channels.push(DEFAULT_CHANNEL_NAME.to_string());
        }
        log::info!("Probe {} base options are configured!", self.log_title());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_base() {}
}
