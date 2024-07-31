use std::fs::{OpenOptions, File};
use std::io::{self, BufRead, BufReader, Write};

use std::env;

use chrono::prelude::*;
use chrono::{NaiveDateTime, Utc};

use regex::Regex;


const DEFAULT_LOG_PATH: &str = "/tmp/log.md";


fn date() -> String {
    let local_time: DateTime<Local> = Local::now();
    let formatted = local_time.format("%Y-%m-%d").to_string();
    formatted
}

fn timestamp() -> String {
    let local_time: DateTime<Local> = Local::now();
    let formatted = local_time.format("%Y-%m-%d_%H:%M:%S").to_string();
    formatted
}

fn write_to_file(file_name: &str, log_string: &str) -> io::Result<usize> {
  let mut file = OpenOptions::new().append(true).create(true).open(file_name)?;
  file.write_all(log_string.as_bytes())?;

  Ok(log_string.as_bytes().len())
}

fn last_line_of_file(file_name: &str) -> io::Result<String> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    let last_line = reader.lines().filter_map(Result::ok).last();
    match last_line {
        Some(line) => Ok(line),
        None => Err(io::Error::new(io::ErrorKind::UnexpectedEof, "File is empty")),
    }
}

fn is_from_previous_day(log_line: &str) -> bool {
    let re = Regex::new(r"\[(\d{4}-\d{2}-\d{2}_\d{2}:\d{2}:\d{2})\]").unwrap();
    if let Some(caps) = re.captures(log_line) {
        let timestamp_str = &caps[1];
        let log_time = NaiveDateTime::parse_from_str(timestamp_str, "%Y-%m-%d_%H:%M:%S").expect("Invalid data format");
        let current_time = Utc::now().naive_utc();
        return log_time.date() != current_time.date();
    }
    false
}

fn main() {
    
    // before anything get the timestamp (because we can log the time with an empty string)
    let timestamp = timestamp();

    // Load the command line arguments
    let args: Vec<String> = env::args().collect();

    let mut file_path: &str = DEFAULT_LOG_PATH;
    if args.len() >= 2 {
        file_path = &args[1];
    }

    // We have a filepath, check if it is a new day because we need to handle the header
    match last_line_of_file(file_path) {
        Ok(line) => {
            if is_from_previous_day(&line) {
                let header = format!("# {}\n", date());
                let _ = write_to_file(file_path, header.as_str());
            }
        }
        Err(e) => {eprintln!("Failed to get last line of file: {}", e);}
    }

    let mut log_string: &str = "";
    if args.len() == 3 {
        log_string = &args[2];
    }
    let log_entry = format!("  * [{}][{}]\n", timestamp, log_string);

    match write_to_file(file_path, log_entry.as_str()) {
        Ok(size) => println!("Wrote: {} bytes", size),
        Err(e) => eprintln!("Failed to write log string to file: {}", e),
    }
}
