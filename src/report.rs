// - report
//   -> total + file + hourly wage
//   -> start_hours + file      (print in XX:XX format, one time per line, print blank or 00:00 for days not work so that it's easy to copy/paste into an excel document)
//   -> end_hours + file

use std::{
    fs::{self},
    path::PathBuf,
};

use anyhow::{bail, Result};
use chrono::{Datelike, Duration, NaiveDateTime, Timelike};

use crate::argparse::Mode;

pub fn process_csv(
    mode: Mode,
    log_file_path: &PathBuf,
    target_month: Option<u8>,
    hourly_wage: Option<u32>,
) -> Result<()> {
    let mut start_times: Vec<NaiveDateTime> = Vec::new();
    let mut end_times: Vec<NaiveDateTime> = Vec::new();
    let mut total_hours = Duration::minutes(0);
    for line in fs::read_to_string(log_file_path)?.lines() {
        let times = line.split(',').collect::<Vec<&str>>();
        if times.len() != 2 {
            bail!("Found malformed line: {}", line)
        }
        let start_time = NaiveDateTime::parse_from_str(times[0], "%Y-%m-%d %H:%M:%S")?;
        let end_time = NaiveDateTime::parse_from_str(times[1], "%Y-%m-%d %H:%M:%S")?;

        if let Some(target_month) = target_month {
            if start_time.month() != target_month.into() {
                continue;
            }
        }

        if (end_time - start_time).num_seconds() < 0 {
            bail!("Found start time later than end time: {}", line);
        } else if (end_time - start_time).num_hours() > 24 {
            bail!("Found abnormally work duration: {}", line);
        } else {
            total_hours = total_hours + (end_time - start_time).into()
        }

        start_times.push(start_time);
        end_times.push(end_time);
    }

    match mode {
        Mode::Total => {
            if let Some(hourly_wage) = hourly_wage {
                println!(
                    "Total worked hours: {}:{:02}, 請求金額（税込）: {}, 請求金額（税抜）: {}",
                    total_hours.num_hours(),
                    total_hours.num_minutes() % 60,
                    total_hours.num_minutes() as f64 * f64::from(hourly_wage) / 60.0,
                    ((total_hours.num_minutes() as f64 * f64::from(hourly_wage) / 60.0) / 1.1)
                        as u32
                )
            } else {
                println!(
                    "Total worked hours: {}:{:02}",
                    total_hours.num_hours(),
                    total_hours.num_minutes() % 60,
                )
            }
        }
        Mode::Starts => {
            // Print 00:00 for days with no entry.
            let mut prev_day = 0;
            for start_time in start_times.iter() {
                while start_time.day() > prev_day + 1 {
                    prev_day += 1;
                    println!("00:00");
                }
                println!("{:02}:{:02}", start_time.hour(), start_time.minute());
                prev_day = start_time.day();
            }
        }
        Mode::Ends => {
            // Print 00:00 for days with no entry.
            let mut prev_day = 0;
            for end_time in end_times.iter() {
                while end_time.day() > prev_day + 1 {
                    prev_day += 1;
                    println!("00:00");
                }
                println!("{:02}:{:02}", end_time.hour(), end_time.minute());
                prev_day = end_time.day();
            }
        }
    }

    Ok(())
}
