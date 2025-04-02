use serde::{Deserialize, Serialize};

use super::LogConfig;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub log: Vec<LogConfig>,
}
