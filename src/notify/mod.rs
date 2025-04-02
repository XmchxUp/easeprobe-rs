use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;

mod base;
pub use base::*;
mod config;
pub use config::*;
mod log;
pub use log::*;

use crate::{NotifierSetting, ProbeResult, Prober};

#[async_trait]
pub trait Notifier: Send + Sync {
    fn kind(&self) -> &str;
    fn name(&self) -> &str;
    fn channels(&self) -> Vec<String>;
    fn config(&mut self, setting: &NotifierSetting) -> Result<()>;
    async fn notify(&self, res: Arc<ProbeResult>);
    async fn notify_stat(&self, probers: Vec<Arc<RwLock<dyn Prober>>>);
    fn dry_notify(&self, res: Arc<ProbeResult>);
    fn dry_notify_stat(&self, probers: Vec<Arc<RwLock<dyn Prober>>>);
}
