use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{global, probe, Prober};

pub(crate) fn to_text(r: Arc<probe::ProbeResult>) -> String {
    let rtt = r.round_trip_time.as_millis();
    format!(
        "[{}] {}\n{} - ⏱ {}\n{}\n{} at {}",
        r.title(),
        r.status.emoji(),
        r.endpoint,
        rtt,
        r.message,
        global::footer_string(),
        global::format_time(r.start_time)
    )
}
pub(crate) fn sla_text(_: Vec<Arc<RwLock<dyn Prober>>>) -> String {
    format!("{}", "[Overall SLA Report]\n\n")
}
