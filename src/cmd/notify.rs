use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{Notifier, NotifierSetting};

pub async fn config_notifiers(notifiers: &mut Vec<Arc<RwLock<dyn Notifier>>>) {
    let setting = NotifierSetting::default();

    let mut valid_notifiers = Vec::new();
    for ele in notifiers.iter() {
        let mut e = ele.write().await;
        if let Err(err) = e.config(&setting) {
            log::error!(
                "Bad Notify Configuration for notifier {} {}: {}",
                e.kind(),
                e.name(),
                err
            );
            continue;
        }
        valid_notifiers.push(Arc::clone(&ele));
    }

    *notifiers = valid_notifiers;
}
