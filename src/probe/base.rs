use std::time::Duration;

use super::{ProbeResult, Prober};

pub struct DefaultProbe {
    pub kind: String,
    pub name: String,
    pub tag: String,
    pub channels: Vec<String>,
    pub timeout: Duration,
    pub interval: Duration,
    pub result: ProbeResult,
}

impl Prober for DefaultProbe {
    fn kind(&self) -> &str {
        &self.kind
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn channels(&self) -> &Vec<String> {
        &self.channels
    }

    fn timeout(&self) -> &Duration {
        &self.timeout
    }

    fn interval(&self) -> &Duration {
        &self.interval
    }

    fn result(&self) -> &ProbeResult {
        &self.result
    }

    fn probe(&self) -> ProbeResult {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_base() {}
}
