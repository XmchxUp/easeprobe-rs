use std::time::Duration;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{normalize, Retry, DEFAULT_RETRY_INTERVAL, DEFAULT_RETRY_TIMES, DEFAULT_TIMEOUT};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct NotifierSetting {
    pub time_format: String,
    pub timeout: Duration,
    #[schemars(description = "The retry settings")]
    pub retry: Retry,
}

impl Default for NotifierSetting {
    fn default() -> Self {
        Self {
            time_format: Default::default(),
            timeout: Default::default(),
            retry: Retry {
                times: DEFAULT_RETRY_TIMES,
                interval: DEFAULT_RETRY_INTERVAL,
            },
        }
    }
}

impl NotifierSetting {
    pub fn normalize_timeout(&self, t: Duration) -> Duration {
        normalize(self.timeout, t, Duration::from_secs(0), DEFAULT_TIMEOUT)
    }

    pub fn normalize_retry(&self, retry: &Retry) -> Retry {
        let mut res = Retry::default();
        res.interval = normalize(
            self.retry.interval,
            retry.interval,
            Duration::from_secs(0),
            DEFAULT_RETRY_INTERVAL,
        );

        res.times = normalize(self.retry.times, retry.times, 0, DEFAULT_RETRY_TIMES);
        res
    }
}
