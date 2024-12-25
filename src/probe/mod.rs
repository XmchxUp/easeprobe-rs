mod result;
use std::time::Duration;

pub use result::*;
mod status;
pub use status::*;
mod notification_strategy;
pub use notification_strategy::*;
mod base;
pub use base::*;

pub trait Prober: Send + Sync {
    fn kind(&self) -> &str;
    fn name(&self) -> &str;
    fn channels(&self) -> &Vec<String>;
    fn timeout(&self) -> &Duration;
    fn interval(&self) -> &Duration;
    fn result(&self) -> &ProbeResult;
    fn probe(&self) -> ProbeResult;
}
