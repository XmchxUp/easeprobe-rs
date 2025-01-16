mod result;
use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
pub use result::*;
mod status;
pub use status::*;
mod notification_strategy;
pub use notification_strategy::*;
mod base;
pub use base::*;
mod http;
pub use http::*;
mod status_counter;
pub use status_counter::*;

use crate::ProbeSetting;

#[async_trait]
pub trait Prober: Send + Sync {
    fn kind(&self) -> &str;
    fn name(&self) -> &str;
    fn channels(&self) -> Vec<String>;
    fn timeout(&self) -> &Duration;
    fn interval(&self) -> &Duration;
    fn result(&self) -> &ProbeResult;
    async fn probe(&mut self) -> ProbeResult;
    fn config(&mut self, setting: &ProbeSetting);
}

#[async_trait]
pub trait ProbeBehavior {
    async fn do_probe(&self) -> Result<(bool, String)>;
}
