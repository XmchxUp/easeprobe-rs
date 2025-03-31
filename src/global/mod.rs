use std::time::{Duration, SystemTime};
mod probe;
use anyhow::{bail, Result};
use chrono::{DateTime, Local, Utc};
pub use probe::*;
mod notify;
pub use notify::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct Retry {
    #[schemars(description = "How many times need to retry")]
    pub times: i32,
    #[schemars(description = "The interval between each retry")]
    pub interval: Duration,
}

pub const DEFAULT_CHANNEL_NAME: &str = "__EaseProbe_Channel__";
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
pub const DEFAULT_PROBE_INTERVAL: Duration = Duration::from_secs(60);
pub const DEFAULT_RETRY_INTERVAL: Duration = Duration::from_secs(5);
pub const DEFAULT_STATUS_CHANGE_THRESHOLD: i32 = 1;
pub const DEFAULT_NOTIFICATION_FACTOR: i32 = 1;
pub const DEFAULT_MAX_NOTIFICATION_TIMES: i32 = 1;
pub const DEFAULT_RETRY_TIMES: i32 = 3;

pub fn format_time(time: SystemTime) -> String {
    let datetime: DateTime<Utc> = time.into();
    let tz = Local;
    let local_time = datetime.with_timezone(&tz);
    let tf = "%Y-%m-%d %H:%M:%S";

    local_time.format(tf).to_string()
}

pub fn footer_string() -> String {
    return "EaseProbe v1.0.0 @ localhost".to_string();
}

/// Normalizes a value based on the provided logic:
/// - If both `global` and `local` are not set (invalid), return `_default`.
/// - If `global` is set (valid) but `local` is not, return `global`.
/// - If `local` is set (valid) but `global` is not, return `local`.
/// - If both `global` and `local` are set (valid), return `local`.
pub(crate) fn normalize<T>(global: T, local: T, valid: T, _default: T) -> T
where
    T: PartialOrd + Copy,
{
    if local <= valid {
        if global > valid {
            return global;
        } else {
            return _default;
        }
    }
    local
}

pub async fn do_retry<F>(kind: &str, name: &str, tag: &str, r: &Retry, func: F) -> Result<()>
where
    F: Fn() -> Result<()>,
{
    let mut last_error = anyhow::anyhow!("No error occurred");
    for i in 0..r.times {
        let res = func();
        if res.is_ok() {
            return Ok(());
        }
        if let Err(err) = res {
            log::warn!(
                "[{} / {} / {}] Retried to send {}/{} - {}",
                kind,
                name,
                tag,
                i + 1,
                r.times,
                err
            );
            last_error = err;

            if i < r.times - 1 {
                tokio::time::sleep(r.interval).await;
            }
        }
    }
    bail!(
        "[{} / {} / {}] failed after {} retries - {}",
        kind,
        name,
        tag,
        r.times,
        last_error
    )
}

pub fn get_env_or_default(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}
