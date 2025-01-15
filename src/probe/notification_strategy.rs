#[derive(Debug, Default, Clone)]
pub struct NotificationStrategyData {}

impl NotificationStrategyData {
    pub fn reset(&mut self) {}

    pub fn need_to_send_notification(&self) -> bool {
        false
    }
}
