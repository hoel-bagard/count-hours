use std::{
    fs::{self},
    path::PathBuf,
};

use anyhow::{bail, Result};
use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, Timelike};

use crate::argparse::Mode;

#[allow(clippy::expect_used)]
fn nb_days_in_month(year: i32, month: u32) -> u32 {
    u32::try_from(
        NaiveDate::from_ymd_opt(
            match month {
                12 => year + 1,
                _ => year,
            },
            match month {
                12 => 1,
                _ => month + 1,
            },
            1,
        )
        .expect("Date to be valid since we use the first of the month.")
        .signed_duration_since(
            NaiveDate::from_ymd_opt(year, month, 1)
                .expect("Date to be valid since we use the first of the month."),
        )
        .num_days(),
    )
    .expect("Number of days is small and positive")
}

/// Read a CSV written by the log command, and print out its content in a way that is easily copy/pastable into an excel sheet.
pub fn process_csv(
    log_file_path: &PathBuf,
    target_month: Option<u8>,
) -> Result<(Vec<NaiveDateTime>, Vec<NaiveDateTime>, Duration)> {
    let mut start_times: Vec<NaiveDateTime> = Vec::new();
    let mut end_times: Vec<NaiveDateTime> = Vec::new();
    let mut total_hours = Duration::minutes(0);
    let mut previous_day: Option<u32> = None;
    for line in fs::read_to_string(log_file_path)?.lines() {
        let times = line.split(',').collect::<Vec<&str>>();
        if times.len() != 2 {
            bail!("Found malformed line: {}", line)
        }
        let start_time = NaiveDateTime::parse_from_str(times[0], "%Y-%m-%d %H:%M:%S")?;
        let end_time = NaiveDateTime::parse_from_str(times[1], "%Y-%m-%d %H:%M:%S")?;
        let day = start_time.day();

        if let Some(target_month) = target_month {
            if start_time.month() != target_month.into() {
                continue;
            }
        }

        let duration = end_time - start_time;
        if duration.num_seconds() < 0 {
            bail!("Found start time later than end time: {}", line);
        }
        if duration.num_hours() > 24 {
            bail!("Found abnormally work duration: {}", line);
        }

        total_hours += duration;

        // If there are multiple timestamps for a given day, then "concatenate" them.
        if previous_day.is_some_and(|prev_day| prev_day == day) {
            if let Some(last) = end_times.last_mut() {
                *last += duration;
            }
        } else {
            start_times.push(start_time);
            end_times.push(end_time);
        }
        previous_day = Some(day);
    }

    Ok((start_times, end_times, total_hours))
}

#[allow(
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::expect_used
)]
pub fn print_report(
    mode: Mode,
    start_times: &Vec<NaiveDateTime>,
    end_times: &Vec<NaiveDateTime>,
    total_hours: &Duration,
    hourly_wage: Option<u32>,
) {
    match mode {
        Mode::Total => hourly_wage.map_or_else(
            || {
                println!(
                    "Total worked hours: {}:{:02}",
                    total_hours.num_hours(),
                    total_hours.num_minutes() % 60,
                );
            },
            |hourly_wage| {
                println!(
                    "Total worked hours: {}:{:02}, 請求金額（税込）: {}, 請求金額（税抜）: {}",
                    total_hours.num_hours(),
                    total_hours.num_minutes() % 60,
                    total_hours.num_minutes() as f64 * f64::from(hourly_wage) / 60.0,
                    ((total_hours.num_minutes() as f64 * f64::from(hourly_wage) / 60.0) / 1.1)
                        as u32
                );
            },
        ),
        Mode::Starts => {
            // Print 00:00 for days with no entry.
            let mut prev_day = 0;
            for start_time in start_times {
                while start_time.day() > prev_day + 1 {
                    prev_day += 1;
                    println!("00:00");
                }
                println!("{:02}:{:02}", start_time.hour(), start_time.minute());
                prev_day = start_time.day();
            }

            // Print 00:00 until last day of the month.
            let nb_days_in_month = nb_days_in_month(
                start_times
                    .first()
                    .expect("to have worked at least one day")
                    .year(),
                start_times
                    .first()
                    .expect("to have worked at least one day")
                    .month(),
            );
            while nb_days_in_month > prev_day {
                prev_day += 1;
                println!("00:00");
            }
        }
        Mode::Ends => {
            // Print 00:00 for days with no entry.
            let mut prev_day = 0;
            for end_time in end_times {
                // Assume that any hour between midnight and 6am corresponds to the previous day.
                // Make any early morning work belong to the previous day.
                let end_hour = if end_time.hour() > 6 {
                    end_time.hour()
                } else {
                    end_time.hour() + 24
                };
                let end_day = if end_time.hour() > 6 {
                    end_time.day()
                } else {
                    end_time.day() - 1
                };

                while end_day > prev_day + 1 {
                    prev_day += 1;
                    println!("00:00");
                }

                println!("{:02}:{:02}", end_hour, end_time.minute());

                prev_day = end_day;
            }

            // Print 00:00 until last day of the month.
            let nb_days_in_month = nb_days_in_month(
                start_times
                    .first()
                    .expect("to have worked at least one day")
                    .year(),
                start_times
                    .first()
                    .expect("to have worked at least one day")
                    .month(),
            );
            while nb_days_in_month > prev_day {
                prev_day += 1;
                println!("00:00");
            }
        }
    }
}
