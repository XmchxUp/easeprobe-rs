use std::time::Duration;

pub struct ProbeSetting {
    pub interval: Duration,
    pub timeout: Duration,
}

impl Default for ProbeSetting {
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(2),
            timeout: Duration::from_secs(5),
        }
    }
}
