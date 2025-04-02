use anyhow::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::{
    global::{NotifierSetting, ProbeSettings},
    notify, probe,
};

pub fn json_schema() -> Result<String> {
    Ok("".to_string())
    // let schema = schema_for!(Conf);
    // Ok(serde_json::to_string_pretty(&schema)?)
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
pub struct Settings {
    #[serde(default = "default_name")]
    pub name: String,
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
    pub probe: ProbeSettings,
    #[serde(default)]
    pub notify: NotifierSetting,
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
    pub http: Vec<probe::HttpProber>,
    pub notify: notify::Config,
    pub settings: Settings,
}
