use std::error;

use clap::Parser;

mod argparse;
use crate::argparse::{Cli, Commands};

fn main() -> Result<(), Box<dyn error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Log {
            action,
            log_file_path,
        } => {
            dbg!(action, log_file_path);
        }
        Commands::Report {
            mode,
            log_file_path,
            target_month,
            hourly_wage,
        } => {
            dbg!(mode, log_file_path, target_month, hourly_wage);
        }
    }
    Ok(())
}
