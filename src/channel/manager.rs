use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, LazyLock,
    },
};

use dashmap::DashMap;
use tokio::sync::RwLock;

use crate::{Notify, Prober};

use super::Channel;

static CHANNELS: LazyLock<DashMap<String, Arc<RwLock<Channel>>>> = LazyLock::new(|| DashMap::new());

static DRY_NOTIFY: AtomicBool = AtomicBool::new(false);

pub fn set_dry_notify(dry: bool) {
    DRY_NOTIFY.store(dry, Ordering::SeqCst);
}

pub fn is_dry_notify() -> bool {
    DRY_NOTIFY.load(Ordering::SeqCst)
}

pub fn get_all_channels() -> &'static DashMap<String, Arc<RwLock<Channel>>> {
    &CHANNELS
}

pub fn get_channel(name: &str) -> Option<Arc<RwLock<Channel>>> {
    CHANNELS.get(name).map(|entry| Arc::clone(entry.value()))
}

pub fn set_channel(name: &str) {
    CHANNELS
        .entry(name.to_string())
        .or_insert_with(|| Arc::new(RwLock::new(Channel::new(name))));
}

pub async fn set_prober(channel: &str, prober: Arc<dyn Prober>) {
    set_channel(&channel);
    if let Some(channel_entry) = CHANNELS.get(channel) {
        let mut channel = channel_entry.write().await;
        channel.set_prober(prober);
    }
}

pub async fn set_probers(probers: Vec<Arc<dyn Prober>>) {
    for p in probers {
        for channel in p.channels() {
            set_prober(&channel, Arc::clone(&p)).await;
        }
    }
}

pub async fn set_notify(channel: &str, notifier: Arc<dyn Notify>) {
    set_channel(channel);
    if let Some(channel_entry) = CHANNELS.get(channel) {
        let mut channel = channel_entry.write().await;
        channel.set_notify(notifier);
    }
}

pub async fn set_notifiers(notifiers: Vec<Arc<dyn Notify>>) {
    for notifier in notifiers {
        for channel in notifier.channels() {
            set_notify(&channel, Arc::clone(&notifier)).await;
        }
    }
}

pub async fn get_notifiers(channels: Vec<String>) -> HashMap<String, Arc<dyn Notify>> {
    let mut notifiers = HashMap::new();
    for channel in channels {
        if let Some(channel_entry) = CHANNELS.get(&channel) {
            let channel = channel_entry.read().await;

            for (name, notify) in &channel.notifiers {
                notifiers.insert(name.clone(), Arc::clone(notify));
            }
        }
    }
    notifiers
}

pub async fn config_all_channels() {
    for channel_entry in CHANNELS.iter() {
        let mut channel = channel_entry.value().write().await;

        channel.configure();
    }
}

pub async fn watch_for_all_events() {
    for channel_entry in CHANNELS.iter() {
        let channel = Arc::clone(channel_entry.value());
        tokio::spawn(async move {
            let mut channel = channel.write().await;
            channel.watch_event().await;
        });
    }
}

pub async fn all_done() {
    for channel_entry in CHANNELS.iter() {
        let channel = channel_entry.value().read().await;
        channel.done();
    }
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
