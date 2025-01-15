use std::sync::Arc;
use std::{collections::HashMap, time::Duration};

use anyhow::Result;
use tokio::sync::{mpsc, Mutex, Notify};

use crate::{global, DefaultNotify, DefaultProbe, Format, Notifier, ProbeResult, Prober, Status};

use super::is_dry_notify;

const KIND: &str = "channel";

pub struct Channel {
    name: String,
    probers: Mutex<HashMap<String, Arc<dyn Prober>>>,
    pub(crate) notifiers: Arc<Mutex<HashMap<String, Arc<dyn Notifier>>>>,
    stop_notify: Arc<Notify>,
    channel: mpsc::Sender<ProbeResult>,
}

impl Channel {
    pub async fn new(name: &str) -> Self {
        let (channel_tx, channel_rx) = mpsc::channel(100);
        let res = Channel {
            name: name.to_string(),
            probers: Mutex::new(HashMap::new()),
            notifiers: Arc::new(Mutex::new(HashMap::new())),
            stop_notify: Arc::new(Notify::new()),
            channel: channel_tx,
        };
        res.watch_event(channel_rx).await;
        res
    }

    pub async fn get_prober(&self, name: &str) -> Option<Arc<dyn Prober>> {
        let res = {
            let probers = self.probers.lock().await;
            probers.get(name).cloned()
        };
        res
    }

    pub async fn add_prober(&self, prober: Arc<dyn Prober>) {
        let mut probers = self.probers.lock().await;
        if probers.contains_key(prober.name()) {
            log::warn!(
                "Prober [{} - {}] name is duplicated, ignored!",
                prober.kind(),
                prober.name()
            );
            return;
        }
        probers.insert(prober.name().to_string(), prober);
    }

    pub async fn add_probers(&self, probers: Vec<Arc<dyn Prober>>) {
        for p in probers {
            self.add_prober(p).await
        }
    }

    pub async fn get_notifier(&self, name: &str) -> Option<Arc<dyn Notifier>> {
        let notifiers = self.notifiers.lock().await;
        notifiers.get(name).map(|v| Arc::clone(v))
    }

    pub async fn add_notifiers(&self, notifiers: Vec<Arc<dyn Notifier>>) {
        for n in notifiers {
            self.add_notifier(n).await;
        }
    }

    pub async fn add_notifier(&self, notifier: Arc<dyn Notifier>) {
        let mut notifiers = self.notifiers.lock().await;
        if notifiers.contains_key(notifier.name()) {
            log::warn!(
                "Notifier [{} - {}] name is duplicated, ignored!",
                notifier.kind(),
                notifier.name()
            );
            return;
        }
        notifiers.insert(notifier.name().to_string(), notifier);
    }

    pub async fn stop(&self) {
        self.stop_notify.notify_waiters();
    }

    pub async fn send(&self, result: ProbeResult) {
        if let Err(e) = self.channel.send(result).await {
            log::error!(
                "[{} / {}]: Failed to send probe result: {}",
                KIND,
                self.name,
                e
            );
        }
    }

    pub async fn watch_event(&self, mut channel: mpsc::Receiver<ProbeResult>) {
        let stop_notify = Arc::clone(&self.stop_notify);
        let channel_name = self.name.clone();
        let notifiers = Arc::clone(&self.notifiers);

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = stop_notify.notified() => {
                        log::info!("[{} / {}]: Received the done signal, channel exiting...", KIND, channel_name);
                        break;
                    }
                    result = channel.recv() => {
                        if let Some(result) = result {
                            Self::handle_result(&channel_name, result,&notifiers).await;
                        }
                    }
                }
            }
        });
    }

    async fn handle_result(
        channel_name: &String,
        mut result: ProbeResult,
        notifiers: &Arc<Mutex<HashMap<String, Arc<dyn Notifier>>>>,
    ) {
        // if it is the first time, and the status is UP, no need notify
        if result.pre_status == Status::Init && result.status == Status::Up {
            log::debug!(
                "[{} / {}]: {} ({}) - Initial Status [{}] == [{}], no notification.",
                KIND,
                channel_name,
                result.name,
                result.endpoint,
                result.pre_status,
                result.status
            );
            return;
        }

        // if the status has no change for UP or Init, no need notify
        if result.pre_status == result.status
            && (result.status == Status::Up || result.status == Status::Init)
        {
            log::debug!(
                "[{} / {}]: {} ({}) - Status no change [{}] == [{}], no notification.",
                KIND,
                channel_name,
                result.name,
                result.endpoint,
                result.pre_status,
                result.status
            );
            return;
        }

        // if the status changed to UP, reset the notification strategy
        let nsd = &mut result.stat.notification_strategy_data;
        if result.status == Status::Up {
            nsd.reset();
        }

        // if the status is DOWN, check the notification strategy
        if result.status == Status::Down && !nsd.need_to_send_notification() {
            log::debug!(
                "[{} / {}]: {} ({}) - Don't meet the notification condition, no notification.",
                KIND,
                channel_name,
                result.name,
                result.endpoint
            );
            return;
        }

        if result.pre_status != result.status {
            log::info!(
                "[{} / {}]: {} ({}) - Status changed [{}] ==> [{}], sending notification...",
                KIND,
                channel_name,
                result.name,
                result.endpoint,
                result.pre_status,
                result.status
            );
        } else {
            log::debug!(
                "[{} / {}]: {} ({}) - Meet the notification condition, sending notification...",
                KIND,
                channel_name,
                result.name,
                result.endpoint
            );
        }

        let result = Arc::new(result);
        let notifiers = notifiers.lock().await;
        for notifier in notifiers.values() {
            let t = Arc::clone(&result);
            if is_dry_notify() {
                notifier.dry_notify(t);
            } else {
                let notifier_clone = Arc::clone(notifier);
                tokio::spawn(async move {
                    notifier_clone.notify(t).await;
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_basic() {
        let ch = Channel::new("test").await;

        let probers: Vec<Arc<dyn Prober>> = vec![
            Arc::new(new_dummy_prober(
                "http",
                "XY",
                "dummy-XY",
                vec!["X".to_string(), "Y".to_string()],
            )),
            Arc::new(new_dummy_prober(
                "http",
                "X",
                "dummy-X",
                vec!["X".to_string()],
            )),
        ];
        ch.add_probers(probers).await;
        assert_eq!(ch.probers.lock().await.len(), 2);
        assert_eq!(ch.get_prober("dummy-XY").await.unwrap().kind(), "http");

        let notifiers: Vec<Arc<dyn Notifier>> = vec![
            Arc::new(new_dummy_notify(
                "email",
                "dummy-XY",
                vec!["X".to_string(), "Y".to_string()],
            )),
            Arc::new(new_dummy_notify("email", "dummy-X", vec!["X".to_string()])),
        ];
        ch.add_notifiers(notifiers).await;
        assert_eq!(ch.notifiers.lock().await.len(), 2);
        assert_eq!(ch.get_notifier("dummy-XY").await.unwrap().kind(), "email");

        // test duplicate name
        let n = Arc::new(new_dummy_notify(
            "discord",
            "dummy-X",
            vec!["X".to_string()],
        ));
        ch.add_notifier(n).await;
        assert_eq!(ch.get_notifier("dummy-X").await.unwrap().kind(), "email");

        let p = Arc::new(new_dummy_prober(
            "ssh",
            "XY",
            "dummy-XY",
            vec!["X".to_string()],
        ));
        ch.add_prober(p).await;
        assert_eq!(ch.get_prober("dummy-XY").await.unwrap().kind(), "http");
    }
}

#[allow(dead_code)]
pub(crate) fn new_dummy_notify(kind: &str, name: &str, channels: Vec<String>) -> impl Notifier {
    let send_func = |_: String, _: String| -> Result<()> { Ok(()) };

    DefaultNotify {
        kind: kind.to_string(),
        name: name.to_string(),
        format: Format::Text,
        send_func,
        channels,
        dry: false,
        timeout: Duration::default(),
        retry: global::Retry::default(),
    }
}

#[allow(dead_code)]
pub(crate) fn new_dummy_prober(
    kind: &str,
    tag: &str,
    name: &str,
    channels: Vec<String>,
) -> impl Prober {
    DefaultProbe {
        kind: kind.to_string(),
        name: name.to_string(),
        tag: tag.to_string(),
        channels,
        timeout: Duration::new(1, 0),
        interval: Duration::new(5, 0),
        result: ProbeResult::default(),
    }
}
