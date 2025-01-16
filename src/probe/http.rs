use std::{collections::HashMap, time::Duration};

use anyhow::Result;
use async_trait::async_trait;
use reqwest::Method;

use crate::ProbeSetting;

use super::{DefaultProber, ProbeBehavior, ProbeResult, Prober};

pub struct HttpProber {
    pub default_prober: DefaultProber<HttpProbeBehavior>,
}

pub struct HttpProbeBehavior {
    pub url: String,
    pub method: Method,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    pub success_codes: Vec<(u16, u16)>,
    pub proxy: Option<String>,
}

#[async_trait]
impl ProbeBehavior for HttpProbeBehavior {
    async fn do_probe(&self) -> Result<(bool, String)> {
        Ok((true, "aa".to_string()))
    }
}

impl HttpProber {
    pub fn new(
        kind: String,
        name: String,
        tag: String,
        url: String,
        method: Method,
        headers: HashMap<String, String>,
        body: Option<String>,
        timeout: Duration,
        interval: Duration,
    ) -> Self {
        let behavior = HttpProbeBehavior {
            url,
            method,
            headers,
            body,
            success_codes: vec![(0, 0)],
            proxy: None,
        };

        let default_prober = DefaultProber {
            kind,
            name,
            tag,
            channels: vec![],
            timeout,
            interval,
            result: ProbeResult::default(),
            behavior,
        };

        Self { default_prober }
    }
}

#[async_trait]
impl Prober for HttpProber {
    fn kind(&self) -> &str {
        &self.default_prober.kind
    }

    fn name(&self) -> &str {
        &self.default_prober.name
    }

    fn channels(&self) -> Vec<String> {
        self.default_prober.channels.clone()
    }

    fn timeout(&self) -> &Duration {
        &self.default_prober.timeout
    }

    fn interval(&self) -> &Duration {
        &self.default_prober.interval
    }

    fn result(&self) -> &ProbeResult {
        &self.default_prober.result
    }

    async fn probe(&mut self) -> ProbeResult {
        self.default_prober.probe().await
    }

    fn config(&mut self, setting: &ProbeSetting) {
        self.default_prober.config(setting);
    }
}
