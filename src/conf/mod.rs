use std::time::Duration;

use anyhow::Result;
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};

use crate::global;

pub fn json_schema() -> Result<String> {
    let schema = schema_for!(Notify);
    Ok(serde_json::to_string_pretty(&schema)?)
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Schedule {
    None,
    Minutely,
    Hourly,
    Daily,
    Weekly,
    Monthly,
}

#[derive(Debug, Serialize, Deserialize, Default, JsonSchema)]
pub struct Notify {
    #[schemars(description = "The retry settings")]
    pub retry: global::Retry,
    #[schemars(description = "Set true to make the notification dry run and not send the message")]
    #[serde(default)]
    pub dry: bool,
}

#[derive(Debug, Serialize, Deserialize, Default, JsonSchema)]
pub struct Probe {
    #[schemars(description = "The interval of probe")]
    pub interval: Duration,
    #[schemars(description = "The timeout of probe")]
    pub timeout: Duration,
    #[serde(flatten)]
    #[schemars(flatten)]
    pub threshold: global::StatusChangeThresholdSettings,
    #[schemars(title = "Alert", description = "The alert settings")]
    pub alert: global::NotificationStrategySettings,
}
