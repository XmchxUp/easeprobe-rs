use std::collections::HashMap;

use anyhow::Result;
use tokio::sync::{mpsc, oneshot};

use crate::{Notifier, ProbeResult, Prober};

pub struct Channel {
    name: String,
    probers: HashMap<String, Box<dyn Prober>>,
    notifiers: HashMap<String, Box<dyn Notifier>>,
    is_watch: bool,
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
            is_watch: false,
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
            println!(
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
}
