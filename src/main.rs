#![allow(clippy::module_name_repetitions)]
use anyhow::Result;
use clap::Parser;

mod argparse;
mod log;
mod report;
use crate::{
    argparse::{Cli, Commands},
    log::log_timestamp,
    report::process_csv,
};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Log {
            action,
            log_file_path,
        } => {
            log_timestamp(*action, log_file_path)?;
        }
        Commands::Report {
            mode,
            log_file_path,
            target_month,
            hourly_wage,
        } => {
            dbg!(mode, log_file_path, target_month, hourly_wage);
            process_csv(*mode, log_file_path, *target_month, *hourly_wage)?;
        }
    }
    Ok(())
}
