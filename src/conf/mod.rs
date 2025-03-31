use anyhow::Result;
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::{global, probe};

pub fn json_schema() -> Result<String> {
    let schema = schema_for!(Notify);
    Ok(serde_json::to_string_pretty(&schema)?)
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Schedule {
    None,
    Minutely,
    Hourly,
    Daily,
    Weekly,
    Monthly,
}

impl Default for Schedule {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]
pub struct Notify {
    #[schemars(description = "The retry settings")]
    pub retry: global::Retry,
    #[schemars(description = "Set true to make the notification dry run and not send the message")]
    pub dry: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Probe {
    #[serde(with = "humantime_serde")]
    // #[schemars(description = "The interval of probe")]
    pub interval: Duration,
    #[serde(with = "humantime_serde")]
    // #[schemars(description = "The timeout of probe")]
    pub timeout: Duration,
    #[serde(flatten, default)]
    pub threshold: global::StatusChangeThresholdSettings,
    #[serde(default, alias = "alert")]
    pub alert: global::NotificationStrategySettings,
}

impl Default for Probe {
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(60),
            timeout: Duration::from_secs(30),
            threshold: Default::default(),
            alert: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SLAReport {
    schedule: Schedule,
    time: String,
    data_file: String,
    backups: i32,
    channels: Vec<String>,
}

impl Default for SLAReport {
    fn default() -> Self {
        Self {
            schedule: Default::default(),
            time: "00:00".to_string(),
            data_file: Default::default(),
            backups: 5,
            channels: Default::default(),
        }
    }
}

// HTTP Server settings
#[derive(Debug, Clone, Serialize, Deserialize)]
struct HTTPServer {
    ip: String,
    port: String,
    refresh: Duration,
    // #[serde(default)]
    // access_log: Log,
}

impl Default for HTTPServer {
    fn default() -> Self {
        Self {
            ip: Default::default(),
            port: "8181".to_string(),
            refresh: Default::default(),
        }
    }
}

// Global Settings
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Settings {
    #[serde(default = "default_name")]
    name: String,
    #[serde(default)]
    icon: String,
    #[serde(default)]
    pid: String,
    // #[serde(default)]
    // log: Log,
    #[serde(default = "default_time_format")]
    timeformat: String,
    #[serde(default = "default_time_zone")]
    timezone: String,
    #[serde(default)]
    probe: Probe,
    #[serde(default)]
    notify: Notify,
    #[serde(default)]
    sla: SLAReport,
    #[serde(default)]
    http: HTTPServer,
}

fn default_name() -> String {
    "EaseProbe".to_string()
}

fn default_time_format() -> String {
    "2006-01-02 15:04:05Z07:00".to_string()
}

fn default_time_zone() -> String {
    "UTC".to_string()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Conf {
    #[serde(default)]
    version: String,
    #[serde(default)]
    http: Vec<probe::HttpProber>,
    // #[serde(default)]
    // notify: notify::Config,
    settings: Settings,
}
