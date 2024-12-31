use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, LazyLock,
    },
};

use tokio::sync::Mutex;

use crate::{Notifier, Prober};

use super::Channel;

static CHANNELS: LazyLock<Mutex<HashMap<String, Arc<Channel>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

static DRY_NOTIFY: AtomicBool = AtomicBool::new(false);

pub fn set_dry_notify(dry: bool) {
    DRY_NOTIFY.store(dry, Ordering::SeqCst);
}

pub fn is_dry_notify() -> bool {
    DRY_NOTIFY.load(Ordering::SeqCst)
}

pub async fn get_channel(name: &str) -> Option<Arc<Channel>> {
    let channels = CHANNELS.lock().await;
    channels.get(name).cloned()
}

pub async fn set_channel(name: &str) {
    let mut channels = CHANNELS.lock().await;
    if !channels.contains_key(name) {
        channels.insert(name.to_string(), Arc::new(Channel::new(name)));
    }
}

pub async fn set_probers(probers: Vec<Arc<dyn Prober>>) {
    for prober in probers {
        for channel_name in prober.channels() {
            set_prober(&channel_name, prober.clone()).await;
        }
    }
}

pub async fn set_prober(channel_name: &str, prober: Arc<dyn Prober>) {
    set_channel(channel_name);
    if let Some(channel) = get_channel(channel_name).await {
        channel.add_prober(prober).await;
    }
}
pub async fn set_notifiers(notifiers: Vec<Arc<dyn Notifier>>) {
    for notifier in notifiers {
        for channel_name in notifier.channels() {
            set_notifier(&channel_name, notifier.clone()).await;
        }
    }
}

pub async fn set_notifier(channel_name: &str, notifier: Arc<dyn Notifier>) {
    set_channel(channel_name);
    if let Some(channel) = get_channel(channel_name).await {
        channel.add_notifier(notifier).await;
    }
}

pub async fn get_notifiers(channel_names: Vec<String>) -> Vec<Arc<dyn Notifier>> {
    let mut notifiers = HashMap::new();

    for channel_name in channel_names {
        if let Some(channel) = get_channel(&channel_name).await {
            let t = channel.notifiers.lock().await;
            for notifier in t.values() {
                notifiers.insert(notifier.name().to_string(), notifier);
            }
        }
    }
    notifiers.values().map(|v| Arc::clone(v)).collect()
}

/// Watches for events on all channels.
pub async fn watch_for_all_events() {
    let all_channels = get_all_channels();
    let notify_done = ALL_DONE.clone();

    tokio::spawn(async move {
        for channel in all_channels {
            let notify_done = notify_done.clone();
            tokio::spawn(async move {
                channel.watch_event().await;
                notify_done.notify_one();
            });
        }
    });
}

/// Sends a done signal to all channels.
pub async fn all_done() {
    for channel in get_all_channels() {
        channel.stop().await;
    }
    ALL_DONE.notified().await;
}

#[cfg(test)]
mod tests {

    use crate::{new_dummy_notify, new_dummy_prober};

    use super::*;

    #[tokio::test]
    async fn test_manager() {
        let name = "test";
        set_notify(
            name,
            Arc::new(new_dummy_notify("email", "dummy", vec!["test".to_string()])),
        )
        .await;

        let notifiers = get_notifiers(vec!["nil-channel".to_string()]).await;
        assert_eq!(notifiers.len(), 0);

        let notifiers = get_notifiers(vec!["test".to_string()]).await;
        assert_eq!(notifiers.len(), 1);
        assert_eq!(notifiers.get("dummy").unwrap().name(), "dummy");

        set_prober(
            name,
            Arc::new(new_dummy_prober(
                "http",
                "",
                "dummy",
                vec!["test".to_string()],
            )),
        )
        .await;

        let test = get_channel(name);
        assert!(test.is_some());

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
            Arc::new(new_dummy_prober(
                "http",
                "Y",
                "dummy-Y",
                vec!["Y".to_string()],
            )),
            Arc::new(new_dummy_prober(
                "http",
                "ALL",
                "dummy-ALL",
                vec!["X".to_string(), "Y".to_string(), "test".to_string()],
            )),
        ];
        set_probers(probers).await;

        let x = get_channel("X").unwrap();
    }
}
