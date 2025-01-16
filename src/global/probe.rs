use std::time::Duration;

#[derive(Default)]
pub struct ProbeSetting {
    pub interval: Duration,
    pub timeout: Duration,
}
