use std::fs::OpenOptions;
use std::io::{self, Write};

use std::env;

use chrono::prelude::*;

const DEFAULT_LOG_PATH: &str = "/tmp/log.txt";


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

fn main() {
    
    // before anything get the timestamp (because we can log the time with an empty string)
    let timestamp = timestamp();

    // Load the command line arguments
    let args: Vec<String> = env::args().collect();

    let mut file_path: &str = DEFAULT_LOG_PATH;
    if args.len() >= 2 {
        file_path = &args[1];
    }

    let mut log_string: &str = "";
    if args.len() == 3 {
        log_string = &args[2];
    }
    let log_entry = format!("[{}][{}]\n", timestamp, log_string);

    match write_to_file(file_path, log_entry.as_str()) {
        Ok(size) => println!("Wrote: {} bytes", size),
        Err(e) => eprintln!("Failed to write log string to file: {}", e),
    }
}
