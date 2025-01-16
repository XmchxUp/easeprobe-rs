mod result;
use std::time::Duration;

use async_trait::async_trait;
pub use result::*;
mod status;
pub use status::*;
mod notification_strategy;
pub use notification_strategy::*;
mod base;
pub use base::*;

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
