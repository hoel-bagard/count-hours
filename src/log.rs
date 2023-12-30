use std::path::PathBuf;

use crate::argparse::Action;

use anyhow::{bail, Result};
use chrono::Local;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

pub fn log_timestamp(action: &Action, log_file_path: &PathBuf) -> Result<()> {
    // Get current time.
    let current_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    match action {
        Action::Start => {
            let mut file = if log_file_path.try_exists()? {
                // Check if the last line has both start and end times.
                let file = File::open(&log_file_path)?;
                let last_line = BufReader::new(file).lines().last().transpose()?;

                if let Some(line) = last_line {
                    if line.split(',').count() != 2 {
                        bail!(
                            "Last line was malformed, expected a single timestamp: {}",
                            line
                        );
                    }
                }

                // Open the file in append mode.
                OpenOptions::new().append(true).open(log_file_path)?
            } else {
                println!("Log file does not exist, should it be created ? (Y/n)");
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;

                if input.trim().eq_ignore_ascii_case("y") || input.trim().is_empty() {
                    File::create(&log_file_path)?
                } else {
                    println!("File not created, exiting.");
                    return Ok(());
                }
            };

            // Write current time to file.
            write!(file, "{}", current_time)?;
        }
        Action::End => {
            if !log_file_path.try_exists()? {
                bail!("File not found {}", log_file_path.to_str().unwrap());
            }

            // Check that there is a start time.
            let file = File::open(&log_file_path)?;
            let last_line = BufReader::new(&file).lines().last().transpose()?;

            if let Some(line) = last_line {
                if line.split(',').count() != 1 {
                    bail!("No start time found: {}.", line);
                }
            } else {
                bail!("Log file exists but is empty. Cannot start with an end time.");
            }

            // Add a "," to the last line, then write current time to file.
            let mut file = OpenOptions::new().append(true).open(log_file_path)?;
            writeln!(file, ",{}", current_time)?;
        }
    }

    Ok(())
}
