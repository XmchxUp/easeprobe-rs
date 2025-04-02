use std::{fs, sync::Arc, time::Duration};

use anyhow::Result;
use clap::Parser;

mod probe;
pub use probe::*;
mod notify;
pub use notify::*;
use tokio::sync::RwLock;

use crate::{
    channel::manager,
    conf::{self, Conf},
    get_env_or_default,
    notify::Notifier,
    probe::Prober,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Dry notification mode
    #[arg(short = 'd', long, default_value_t = get_env_or_default("PROBE_DRY", "false")=="true")]
    dry_notify: bool,

    /// Configuration file
    #[arg(short = 'f', long, default_value_t = get_env_or_default("PROBE_CONFIG", "config.yaml"))]
    yaml_file: String,

    /// Show JSON schema
    #[arg(short = 'j', long, default_value_t = false)]
    json_schema: bool,
}

pub async fn start() -> Result<()> {
    logforth::stdout().apply();

    let args = Args::parse();
    if args.json_schema {
        let schema = conf::json_schema().expect("failed to show JSON schema: ");
        println!("{}", schema);
        std::process::exit(0);
    }

    let f = fs::read(args.yaml_file)?;
    let c: Conf = serde_yaml::from_slice(&f)?;
    println!("{:?}", c);

    manager::set_channel("test").await;

    let mut probers: Vec<Arc<RwLock<dyn Prober>>> = vec![];
    for ele in c.http {
        probers.push(Arc::new(RwLock::new(ele)));
    }
    config_probers(&mut probers, &c.settings).await;

    let mut notifiers: Vec<Arc<RwLock<dyn Notifier>>> = vec![];
    for ele in c.notify.log {
        notifiers.push(Arc::new(RwLock::new(ele)));
    }
    config_notifiers(&mut notifiers, &c.settings).await;

    manager::set_notifiers(notifiers.clone()).await;
    manager::set_probers(probers.clone()).await;

    run_probers(probers);

    tokio::time::sleep(Duration::new(30, 1000)).await;

    Ok(())
}
