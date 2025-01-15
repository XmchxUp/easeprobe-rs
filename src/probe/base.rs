use std::time::{Duration, SystemTime, UNIX_EPOCH};

use async_trait::async_trait;

use super::{ProbeResult, Prober};

pub type ProbeFuncType = fn() -> Option<String>;

#[derive(Default)]
pub struct DefaultProber {
    pub kind: String,
    pub name: String,
    pub tag: String,
    pub channels: Vec<String>,
    pub timeout: Duration,
    pub interval: Duration,
    pub probe_result: ProbeResult,
    pub probe_fn: Option<ProbeFuncType>,
}

#[async_trait]
impl Prober for DefaultProber {
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

    fn result(&self) -> &ProbeResult {
        &self.probe_result
    }

    async fn probe(&mut self) -> ProbeResult {
        if self.probe_fn.is_none() {
            return self.probe_result.clone();
        }

        let now = SystemTime::now();

        self.probe_result.start_time = now;
        self.probe_result.start_timestamp = now.duration_since(UNIX_EPOCH).unwrap().as_millis();

        let res = self.probe_fn.unwrap()();
        self.probe_result.round_trip_time = now.elapsed().unwrap();
        println!("{:?}", res);

        self.probe_result.clone()
    }

    fn config(&mut self) {
        self.probe_fn = Some(|| -> Option<String> { Some("probe_fn".to_string()) });
        self.channels.push("test".to_string());
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_base() {}
}
