use anyhow::{bail, Result};
use async_trait::async_trait;
use reqwest::{Client, Method, Url};
use std::{collections::HashMap, time::Duration};

use crate::{NotificationStrategySettings, ProbeSettings, StatusChangeThresholdSettings};

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
    pub client: Option<Client>,
}

#[async_trait]
impl ProbeBehavior for HttpProbeBehavior {
    async fn do_probe(&self) -> Result<(bool, String)> {
        if let Some(client) = &self.client {
            let mut request = client.request(self.method.clone(), &self.url);

            if let Some(body) = &self.body {
                request = request.body(body.clone());
            }

            let response = request.send().await?;
            let status = response.status();

            let valid = self
                .success_codes
                .iter()
                .any(|&(start, end)| start <= status.as_u16() && status.as_u16() <= end);

            // let body = response.text().await?;
            let message = if valid {
                format!("HTTP Status Code is {}", status)
            } else {
                format!(
                    "HTTP Status Code is {}. It missed in {:?}",
                    status, self.success_codes
                )
            };

            return Ok((valid, message));
        }
        bail!("probe error")
    }
}

impl HttpProber {
    pub fn new(
        name: &str,
        url: &str,
        method: Method,
        headers: HashMap<String, String>,
        body: Option<String>,
        timeout: Duration,
        interval: Duration,
    ) -> Self {
        let behavior = HttpProbeBehavior {
            method,
            headers,
            body,
            success_codes: vec![(200, 299)],
            proxy: None,
            client: None,
            url: url.to_string(),
        };

        let default_prober = DefaultProber {
            kind: "http".to_string(),
            tag: "".to_string(),
            channels: vec![],
            result: ProbeResult::default(),
            name: name.to_string(),
            timeout,
            interval,
            behavior,
            threshold: StatusChangeThresholdSettings::default(),
            notification: NotificationStrategySettings::default(),
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

    fn result(&mut self) -> &mut ProbeResult {
        &mut self.default_prober.result
    }

    async fn probe(&mut self) -> ProbeResult {
        self.default_prober.probe().await
    }

    async fn config(&mut self, setting: &ProbeSettings) -> Result<()> {
        self.default_prober.config(setting).await?;

        let b = &self.default_prober.behavior;

        if let Err(err) = Url::parse(&b.url) {
            log::error!(
                "[{} / {}] URL is not valid - {} url={}",
                self.kind(),
                self.name(),
                err,
                b.url,
            );
            bail!(err)
        }

        let mut client_builder = Client::builder().timeout(setting.timeout);

        // proxy server
        if let Some(proxy_url) = &b.proxy {
            if let Err(err) = Url::parse(&proxy_url) {
                log::error!(
                    "[{} / {}] proxy URL is not valid - {} url={}",
                    self.kind(),
                    self.name(),
                    err,
                    proxy_url,
                );
                bail!(err)
            }

            let proxy = reqwest::Proxy::http(proxy_url.trim())?;

            client_builder = client_builder.proxy(proxy);
            log::debug!(
                "[{} / {}] proxy server is {}",
                self.kind(),
                self.name(),
                proxy_url,
            );
        }

        let b = &mut self.default_prober.behavior;
        b.client = Some(client_builder.build()?);

        Ok(())
    }
}
