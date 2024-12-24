mod base;
use std::sync::Arc;

use async_trait::async_trait;
pub use base::*;

use crate::{ProbeResult, Prober};

#[async_trait]
pub trait Notifier {
    fn kind(&self) -> &str;
    fn name(&self) -> &str;
    fn channels(&self) -> &Vec<String>;
    async fn notify(&self, res: Arc<ProbeResult>);
    fn notify_stat(&self, probers: Vec<Box<dyn Prober>>);
    fn dry_notify(&self, res: Arc<ProbeResult>);
    fn dry_notify_stat(&self, probers: Vec<Box<dyn Prober>>);
}
