use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{Notifier, NotifierSetting};

pub async fn config_notifiers(notifiers: &Vec<Arc<RwLock<dyn Notifier>>>) {
    let setting = NotifierSetting::default();
    for ele in notifiers {
        ele.write().await.config(&setting);
    }
}
