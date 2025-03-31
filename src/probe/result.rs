use std::{
    collections::HashMap,
    time::{Duration, SystemTime},
};

use serde::{Deserialize, Serialize};

use super::{NotificationStrategyData, Status, StatusCounter};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProbeResult {
    pub name: String,
    pub endpoint: String,
    pub start_time: SystemTime,
    pub start_timestamp: u128,
    pub round_trip_time: Duration,
    pub status: Status,
    pub pre_status: Status,
    pub message: String,
    pub latest_downtime: SystemTime,
    pub recovery_time: Duration,
    pub stat: Stat,
}

impl Default for ProbeResult {
    fn default() -> Self {
        Self {
            name: Default::default(),
            endpoint: Default::default(),
            start_time: SystemTime::now(),
            start_timestamp: Default::default(),
            round_trip_time: Default::default(),
            status: Default::default(),
            pre_status: Default::default(),
            message: Default::default(),
            latest_downtime: SystemTime::now(),
            recovery_time: Default::default(),
            stat: Default::default(),
        }
    }
}

impl ProbeResult {
    pub fn title(&self) -> String {
        let t: String;
        if self.pre_status == Status::Init && self.status == Status::Up {
            t = format!("Monitoring {}", self.name);
        } else if self.status != Status::Up {
            t = format!("{} Failure", self.name);
        } else {
            let rounded_recovery_duration = self.recovery_time.as_secs();
            t = format!(
                "{} Recovery - ( {} Downtime )",
                self.name, rounded_recovery_duration
            );
        }
        t
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stat {
    pub since: SystemTime,
    pub total: i64,
    pub status: HashMap<Status, i64>,
    pub uptime: Duration,
    pub downtime: Duration,
    pub notification_strategy_data: NotificationStrategyData,
    pub status_counter: StatusCounter,
}

impl Default for Stat {
    fn default() -> Self {
        Self {
            since: SystemTime::now(),
            total: Default::default(),
            status: Default::default(),
            uptime: Default::default(),
            downtime: Default::default(),
            notification_strategy_data: Default::default(),
            status_counter: StatusCounter::new(10),
        }
    }
}
