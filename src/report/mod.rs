use crate::{probe, ProbeResult};
use std::{
    collections::HashMap,
    sync::{Arc, LazyLock},
};

mod result;
use result::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Format {
    Unknown,
    MarkdownSocial, // *text* is bold
    Markdown,       // **text** is bold
    HTML,
    JSON,
    Text,
    Log,
    Slack,
    Discord,
    Lark,
    SMS,
    Shell,
}

impl Format {
    #[allow(dead_code)]
    fn to_string(&self) -> &'static str {
        match self {
            Format::Unknown => "unknown",
            Format::MarkdownSocial => "markdown-social",
            Format::Markdown => "markdown",
            Format::HTML => "html",
            Format::JSON => "json",
            Format::Text => "text",
            Format::Log => "log",
            Format::Slack => "slack",
            Format::Discord => "discord",
            Format::Lark => "lark",
            Format::SMS => "sms",
            Format::Shell => "shell",
        }
    }

    #[allow(dead_code)]
    fn from_string(s: &str) -> Format {
        match s.to_lowercase().as_str() {
            "unknown" => Format::Unknown,
            "markdown-social" => Format::MarkdownSocial,
            "markdown" => Format::Markdown,
            "html" => Format::HTML,
            "json" => Format::JSON,
            "text" => Format::Text,
            "log" => Format::Log,
            "slack" => Format::Slack,
            "discord" => Format::Discord,
            "lark" => Format::Lark,
            "sms" => Format::SMS,
            "shell" => Format::Shell,
            _ => Format::Unknown,
        }
    }
}

pub type FormatFuncType = fn(Arc<ProbeResult>) -> String;
pub type StatFormatFuncType = fn(Vec<Arc<dyn probe::Prober>>) -> String;

#[derive(Debug)]
pub struct FormatFuncStruct {
    pub result_fn: FormatFuncType,
    pub stat_fn: StatFormatFuncType,
}

pub static FORMAT_FUNCS: LazyLock<HashMap<Format, FormatFuncStruct>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert(
        Format::Unknown,
        FormatFuncStruct {
            result_fn: to_text,
            stat_fn: sla_text,
        },
    );

    m
});
