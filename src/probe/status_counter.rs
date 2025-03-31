use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusHistory {
    pub status: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusCounter {
    pub status_history: VecDeque<StatusHistory>,
    max_len: usize,
    pub current_status: bool,
    pub status_count: usize,
}

impl StatusCounter {
    pub fn new(max_len: usize) -> Self {
        Self {
            status_history: VecDeque::with_capacity(max_len),
            max_len,
            current_status: true,
            status_count: 0,
        }
    }

    pub fn append_status(&mut self, status: bool, message: String) {
        if status != self.current_status {
            self.status_count = 0;
            self.current_status = status;
        }

        if self.status_count < self.max_len {
            self.status_count += 1;
        }

        self.status_history
            .push_back(StatusHistory { status, message });

        // Remove the oldest status if the history exceeds the max length
        if self.status_history.len() > self.max_len {
            self.status_history.pop_front();
        }
    }

    pub fn set_max_len(&mut self, max_len: usize) {
        self.max_len = max_len;

        if self.status_history.len() > self.max_len {
            self.status_history = self
                .status_history
                .iter()
                .skip(self.status_history.len() - self.max_len)
                .cloned()
                .collect();
        }
    }
}
