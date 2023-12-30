use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Action {
    /// Starting to work.
    Start,
    /// Finished working.
    End,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
    /// Print total hours worked.
    Total,
    /// Print all start timestamps.
    Starts,
    /// Print all end timestamps.
    Ends,
}

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Log worked hours.
    Log {
        #[arg(value_enum)]
        action: Action,
        /// Path to the file storing the start and end work times.
        log_file_path: PathBuf,
    },

    /// Report information entered in the log file in a way that is easy to copy/paste into an excel sheet.
    Report {
        /// Action to perform.
        mode: Mode,
        /// Path to the file storing the start and end work times.
        log_file_path: PathBuf,
        /// Target month, if given only the entries related to that month are printed.
        #[arg(short, long)]
        target_month: Option<u8>,
        #[arg(short('w'), long)]
        hourly_wage: Option<u32>,
    },
}
