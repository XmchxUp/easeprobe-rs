use std::time::Duration;

#[derive(Default)]
pub struct Retry {
    pub times: i32,
    pub interval: Duration,
}
