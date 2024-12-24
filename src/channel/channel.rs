use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::{collections::HashMap, sync::atomic::AtomicBool, time::Duration};

use anyhow::Result;
use tokio::sync::{mpsc, oneshot};

use crate::{global, DefaultNotify, DefaultProbe, Format, Notifier, ProbeResult, Prober, Status};

pub struct Channel {
    name: String,
    probers: HashMap<String, Box<dyn Prober>>,
    notifiers: HashMap<String, Arc<dyn Notifier + Send + Sync>>,
    is_watch: AtomicBool,
    done_tx: Option<oneshot::Sender<()>>,
    done_rx: Option<oneshot::Receiver<()>>,
    results_tx: Option<mpsc::UnboundedSender<ProbeResult>>,
    results_rx: Option<mpsc::UnboundedReceiver<ProbeResult>>,
}

impl Channel {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            probers: HashMap::new(),
            notifiers: HashMap::new(),
            is_watch: AtomicBool::new(false),
            done_tx: None,
            results_tx: None,
            done_rx: None,
            results_rx: None,
        }
    }

    pub fn configure(&mut self) {
        let (done_tx, done_rx) = oneshot::channel();
        let (results_tx, results_rx) = mpsc::unbounded_channel();

        self.done_tx = Some(done_tx);
        self.done_rx = Some(done_rx);

        self.results_tx = Some(results_tx);
        self.results_rx = Some(results_rx);
    }

    pub fn done(&mut self) -> Option<&mut oneshot::Receiver<()>> {
        self.done_rx.as_mut()
    }

    pub fn channel(&mut self) -> Option<&mut mpsc::UnboundedReceiver<ProbeResult>> {
        self.results_rx.as_mut()
    }

    pub fn send(&self, result: ProbeResult) -> Result<()> {
        if let Some(results_tx) = self.results_tx.as_ref() {
            results_tx.send(result)?;
        }
        Ok(())
    }

    pub fn get_prober(&self, name: &str) -> Option<&Box<dyn Prober>> {
        self.probers.get(name)
    }

    pub fn set_prober(&mut self, prober: Box<dyn Prober>) {
        if self.probers.contains_key(prober.name()) {
            log::warn!(
                "Prober [{} - {}] name is duplicated, ignored!",
                prober.kind(),
                prober.name()
            );
            return;
        }
        self.probers.insert(prober.name().to_string(), prober);
    }

    pub fn set_probers(&mut self, probers: Vec<Box<dyn Prober>>) {
        for p in probers {
            self.set_prober(p)
        }
    }

    pub fn get_notifier(&self, name: &str) -> Option<&Arc<dyn Notifier + Send + Sync>> {
        self.notifiers.get(name)
    }

    pub fn set_notifiers(&mut self, notifiers: Vec<Arc<dyn Notifier + Send + Sync>>) {
        for n in notifiers {
            self.set_notifier(n);
        }
    }

    pub fn set_notifier(&mut self, notifier: Arc<dyn Notifier + Send + Sync>) {
        if self.notifiers.contains_key(notifier.name()) {
            log::warn!(
                "Notifier [{} - {}] name is duplicated, ignored!",
                notifier.kind(),
                notifier.name()
            );
            return;
        }
        self.notifiers.insert(notifier.name().to_string(), notifier);
    }

    pub fn watch_event(&mut self) {
        if let Err(_) =
            self.is_watch
                .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::SeqCst)
        {
            log::warn!("[{}]: Channel is already watching!", self.name);
            return;
        }

        let _guard = scopeguard::guard((), |_| {
            self.is_watch.store(false, Ordering::SeqCst);
        });

        loop {
            if let Some(ref mut done_rx) = self.done_rx {
                if done_rx.try_recv().is_ok() {
                    log::info!(
                        "[{}]: Received the done signal, channel exiting...",
                        self.name
                    );
                    return;
                }
            }

            if let Some(ref mut results_rx) = self.results_rx {
                match results_rx.try_recv() {
                    Ok(result) => {
                        self.handle_result(result);
                    }
                    Err(_) => {
                        continue;
                    }
                }
            }
        }
    }

    fn handle_result(&self, mut result: ProbeResult) {
        // if it is the first time, and the status is UP, no need notify
        if result.pre_status == Status::Init && result.status == Status::Up {
            log::debug!(
                "[{}]: {} - Initial Status [{}] == [{}], no notification.",
                self.name,
                result.name,
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
                "[{}]: {} - Status no change [{}] == [{}], no notification.",
                self.name,
                result.name,
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
                "[{}]: {} - Don't meet the notification condition, no notification.",
                self.name,
                result.name
            );
            return;
        }

        if result.pre_status != result.status {
            log::info!(
                "[{}]: {} - Status changed [{}] ==> [{}], sending notification...",
                self.name,
                result.name,
                result.pre_status,
                result.status
            );
        } else {
            log::debug!(
                "[{}]: {} - Meet the notification condition, sending notification...",
                self.name,
                result.name
            );
        }

        let result = Arc::new(result);
        for notifier in self.notifiers.values() {
            let t = Arc::clone(&result);
            if true {
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
    #[test]
    fn test_basic() {
        let mut ch = Channel::new("test");

        ch.configure();

        assert!(ch.done().is_some());
        assert!(ch.channel().is_some());

        let probers: Vec<Box<dyn Prober>> = vec![
            Box::new(new_dummy_prober(
                "http",
                "XY",
                "dummy-XY",
                vec!["X".to_string(), "Y".to_string()],
            )),
            Box::new(new_dummy_prober(
                "http",
                "X",
                "dummy-X",
                vec!["X".to_string()],
            )),
        ];
        ch.set_probers(probers);
        assert_eq!(ch.probers.len(), 2);
        assert_eq!(ch.get_prober("dummy-XY").unwrap().kind(), "http");

        let notifiers: Vec<Arc<dyn Notifier + Send + Sync>> = vec![
            Arc::new(new_dummy_notify(
                "email",
                "dummy-XY",
                vec!["X".to_string(), "Y".to_string()],
            )),
            Arc::new(new_dummy_notify("email", "dummy-X", vec!["X".to_string()])),
        ];
        ch.set_notifiers(notifiers);
        assert_eq!(ch.notifiers.len(), 2);
        assert_eq!(ch.get_notifier("dummy-XY").unwrap().kind(), "email");

        // test duplicate name
        let n = Arc::new(new_dummy_notify(
            "discord",
            "dummy-X",
            vec!["X".to_string()],
        ));
        ch.set_notifier(n);
        assert_eq!(ch.get_notifier("dummy-X").unwrap().kind(), "email");

        let p = Box::new(new_dummy_prober(
            "ssh",
            "XY",
            "dummy-XY",
            vec!["X".to_string()],
        ));
        ch.set_prober(p);
        assert_eq!(ch.get_prober("dummy-XY").unwrap().kind(), "http");
    }
}

fn new_dummy_notify(kind: &str, name: &str, channels: Vec<String>) -> impl Notifier {
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

fn new_dummy_prober(kind: &str, tag: &str, name: &str, channels: Vec<String>) -> impl Prober {
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
