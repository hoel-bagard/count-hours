#![allow(clippy::module_name_repetitions)]
use anyhow::Result;
use clap::Parser;

mod argparse;
mod log;
mod report;
use crate::{
    argparse::{Cli, Commands},
    log::log_timestamp,
    report::print_report,
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
            target_year,
            hourly_wage,
        } => {
            let (start_times, end_times, total_hours) =
                process_csv(log_file_path, *target_month, *target_year)?;
            print_report(*mode, &start_times, &end_times, &total_hours, *hourly_wage);
        }
    }
    Ok(())
}
