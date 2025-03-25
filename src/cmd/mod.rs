mod probe;
use anyhow::Result;
use clap::Parser;
pub use probe::*;
mod notify;
pub use notify::*;

use crate::{conf, get_env_or_default};

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

pub fn start() -> Result<()> {
    logforth::stdout().apply();

    let args = Args::parse();
    if args.json_schema {
        let schema = conf::json_schema().expect("failed to show JSON schema: ");
        println!("{}", schema);
        std::process::exit(0);
    }

    Ok(())
}
