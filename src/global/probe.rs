use std::{str::FromStr, time::Duration};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{
    normalize, DEFAULT_MAX_NOTIFICATION_TIMES, DEFAULT_NOTIFICATION_FACTOR, DEFAULT_PROBE_INTERVAL,
    DEFAULT_STATUS_CHANGE_THRESHOLD, DEFAULT_TIMEOUT,
};

#[derive(Default)]
pub struct ProbeSettings {
    pub interval: Duration,
    pub timeout: Duration,
    pub threshold: StatusChangeThresholdSettings,
    pub notification: NotificationStrategySettings,
}

#[derive(Clone, Default, Copy, Debug, Serialize, Deserialize, JsonSchema)]
pub struct StatusChangeThresholdSettings {
    #[serde(default = "default_failure")]
    #[schemars(
        title = "Failure Threshold",
        description = "The failures threshold to change the status such as 3"
    )]
    pub failure: i32,
    #[serde(default = "default_success")]
    #[schemars(
        title = "Success Threshold",
        description = "The success threshold to change the status such as 2"
    )]
    pub success: i32,
}

fn default_failure() -> i32 {
    1
}

fn default_success() -> i32 {
    1
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, JsonSchema)]
pub struct NotificationStrategySettings {
    pub strategy: IntervalStrategy,
    pub factor: i32,
    pub max_times: i32,
}

impl Default for NotificationStrategySettings {
    fn default() -> Self {
        Self {
            strategy: IntervalStrategy::Regular,
            factor: 1,
            max_times: 1,
        }
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
pub enum IntervalStrategy {
    Unknown,
    Regular,
    Increment,
    Exponential,
}

impl Default for IntervalStrategy {
    fn default() -> Self {
        Self::Regular
    }
}

impl ToString for IntervalStrategy {
    fn to_string(&self) -> String {
        match self {
            IntervalStrategy::Unknown => "unknown".to_string(),
            IntervalStrategy::Regular => "regular".to_string(),
            IntervalStrategy::Increment => "increment".to_string(),
            IntervalStrategy::Exponential => "exponent".to_string(),
        }
    }
}

impl FromStr for IntervalStrategy {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "regular" => Ok(IntervalStrategy::Regular),
            "increment" => Ok(IntervalStrategy::Increment),
            "exponent" => Ok(IntervalStrategy::Exponential),
            _ => Ok(IntervalStrategy::Unknown),
        }
    }
}

impl ProbeSettings {
    pub fn normalize_timeout(&self, t: Duration) -> Duration {
        normalize(self.timeout, t, Duration::ZERO, DEFAULT_TIMEOUT)
    }

    pub fn normalize_interval(&self, t: Duration) -> Duration {
        normalize(self.interval, t, Duration::ZERO, DEFAULT_PROBE_INTERVAL)
    }

    pub fn normalize_threshold(
        &self,
        t: StatusChangeThresholdSettings,
    ) -> StatusChangeThresholdSettings {
        StatusChangeThresholdSettings {
            failure: normalize(
                self.threshold.failure,
                t.failure,
                0,
                DEFAULT_STATUS_CHANGE_THRESHOLD,
            ),
            success: normalize(
                self.threshold.success,
                t.success,
                0,
                DEFAULT_STATUS_CHANGE_THRESHOLD,
            ),
        }
    }

    pub fn normalize_notification_strategy(
        &self,
        t: NotificationStrategySettings,
    ) -> NotificationStrategySettings {
        NotificationStrategySettings {
            strategy: normalize(
                self.notification.strategy,
                t.strategy,
                IntervalStrategy::Unknown,
                IntervalStrategy::Regular,
            ),
            factor: normalize(
                self.notification.factor,
                t.factor,
                0,
                DEFAULT_NOTIFICATION_FACTOR,
            ),
            max_times: normalize(
                self.notification.max_times,
                t.max_times,
                0,
                DEFAULT_MAX_NOTIFICATION_TIMES,
            ),
        }
    }
}
