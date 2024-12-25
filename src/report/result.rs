use std::sync::Arc;

use crate::probe;

pub(crate) fn to_text(r: Arc<probe::ProbeResult>) -> String {
    let rtt = r.round_trip_time.as_millis();
    format!(
        "[{}] {}\n{} - ⏱ {}\n{}\n{}",
        r.title(),
        r.status.emoji(),
        r.endpoint,
        rtt,
        r.message,
        ""
    )
}
pub(crate) fn sla_text(_: Vec<Arc<dyn probe::Prober>>) -> String {
    format!("{}", "[Overall SLA Report]\n\n")
}
