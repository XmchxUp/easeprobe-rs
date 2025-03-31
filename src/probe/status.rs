use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Status {
    Init,
    Up,
    Down,
    Unknown,
    Bad,
}

impl Default for Status {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Status {
    pub fn title(&self) -> &str {
        match self {
            Status::Init => "Initialization",
            Status::Up => "Success",
            Status::Down => "Error",
            Status::Unknown => "Unknown",
            Status::Bad => "Bad",
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            Status::Init => "init",
            Status::Up => "up",
            Status::Down => "down",
            Status::Unknown => "unknown",
            Status::Bad => "bad",
        }
    }

    pub fn emoji(&self) -> &str {
        match self {
            Status::Init => "🔎",
            Status::Up => "✅",
            Status::Down => "❌",
            Status::Unknown => "⛔️",
            Status::Bad => "🚫",
        }
    }

    pub fn from_string(status: &str) -> Status {
        match status.to_lowercase().as_str() {
            "init" => Status::Init,
            "up" => Status::Up,
            "down" => Status::Down,
            "unknown" => Status::Unknown,
            "bad" => Status::Bad,
            _ => Status::Unknown,
        }
    }
}
