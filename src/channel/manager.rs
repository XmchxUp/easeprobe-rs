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
        let ch = Channel::new(name).await;
        channels.insert(name.to_string(), Arc::new(ch));
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
    set_channel(channel_name).await;
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
    set_channel(channel_name).await;
    if let Some(channel) = get_channel(channel_name).await {
        channel.add_notifier(notifier).await;
    }
}

pub async fn get_notifiers(channel_names: Vec<String>) -> HashMap<String, Arc<dyn Notifier>> {
    let mut notifiers = HashMap::new();

    for channel_name in channel_names {
        if let Some(channel) = get_channel(&channel_name).await {
            for notifier in channel.notifiers.lock().await.values() {
                notifiers.insert(notifier.name().to_string(), Arc::clone(notifier));
            }
        }
    }

    notifiers
}

/// Sends a done signal to all channels.
pub async fn all_done() {
    let channel = CHANNELS.lock().await;
    let all_channels = channel.values();
    for channel in all_channels {
        channel.stop().await;
    }
}

pub async fn get_all_channels() -> HashMap<String, Arc<Channel>> {
    let channel = CHANNELS.lock().await;
    channel.clone()
}

#[cfg(test)]
mod tests {

    use crate::{new_dummy_notify, new_dummy_prober};

    use super::*;

    #[tokio::test]
    async fn test_manager() {
        let name = "test";
        set_notifier(
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

        let test = get_channel(name).await.unwrap();

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

        assert_eq!(
            "dummy-ALL",
            test.get_prober("dummy-ALL").await.unwrap().name()
        );

        let x = get_channel("X").await.unwrap();
        assert!(x.get_prober("dummy-X").await.is_some());
        assert!(x.get_prober("dummy-XY").await.is_some());
        assert_eq!("dummy-X", x.get_prober("dummy-X").await.unwrap().name());
        assert_eq!("dummy-XY", x.get_prober("dummy-XY").await.unwrap().name());
        assert_eq!("dummy-ALL", x.get_prober("dummy-ALL").await.unwrap().name());

        let y = get_channel("Y").await.unwrap();
        assert!(y.get_prober("dummy-Y").await.is_some());
        assert!(y.get_prober("dummy-XY").await.is_some());
        assert_eq!("dummy-Y", y.get_prober("dummy-Y").await.unwrap().name());
        assert_eq!("dummy-XY", y.get_prober("dummy-XY").await.unwrap().name());
        assert_eq!("dummy-ALL", y.get_prober("dummy-ALL").await.unwrap().name());

        let notifiers: Vec<Arc<dyn Notifier>> = vec![
            Arc::new(new_dummy_notify(
                "email",
                "dummy-XY",
                vec!["X".to_string(), "Y".to_string()],
            )),
            Arc::new(new_dummy_notify("email", "dummy-X", vec!["X".to_string()])),
        ];
        set_notifiers(notifiers).await;

        assert!(x.get_notifier("dummy-X").await.is_some());
        assert!(x.get_notifier("dummy-XY").await.is_some());
        assert_eq!("dummy-X", x.get_notifier("dummy-X").await.unwrap().name());
        assert_eq!("dummy-XY", x.get_notifier("dummy-XY").await.unwrap().name());

        assert!(y.get_notifier("dummy-X").await.is_none());
        assert!(y.get_notifier("dummy-XY").await.is_some());
        assert_eq!("dummy-XY", y.get_notifier("dummy-XY").await.unwrap().name());

        let chs = get_all_channels().await;
        assert_eq!(3, chs.len());
        assert!(chs.get("test").is_some());
        assert!(chs.get("X").is_some());
        assert!(chs.get("Y").is_some());
    }
}
