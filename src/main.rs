//! Find all duplicate files in a specified sub-directory tree specified on command-line.
//! Note: hidden files, hidden directories and zero byte files are skipped.
//!
//! # Usage
//!
//! ```
//! $ cargo run -- <input_directory>
//! ```
//!
//! Results will be printed to stdout.

use std::env;
use std::path::Path;
use std::io::{Error,ErrorKind};
use anyhow::Result;
use crate::find_duplicates::find_duplicates;
pub mod compute_sha256;
pub mod is_hidden;
pub mod is_duplicate_file;
pub mod find_duplicates;
pub mod debug_message;
use std::time::Instant;

fn measure_elapsed_time<F>(f: F) -> String
where
    F: FnOnce(),
{
    let start = Instant::now();
    f();
    let duration = start.elapsed();
    format_duration(duration)
}

fn format_duration(duration: std::time::Duration) -> String {
    let total_seconds = duration.as_secs();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    let milliseconds = duration.subsec_millis();

    if hours > 0 {
        format!("{}h {}m {}s {}ms", hours, minutes, seconds, milliseconds)
    } else if minutes > 0 {
        format!("{}m {}s {}ms", minutes, seconds, milliseconds)
    } else if seconds > 0 {
        format!("{}s {}ms", seconds, milliseconds)
    } else {
        format!("{}ms", milliseconds)
    }
}

/// The main entry point for the program dupefiles.
///
/// # Arguments
///
/// The program expects a command-line arguments:
/// * argument directory is the input file path (required)
///
/// # Errors
///
/// This function will return an error if:
/// * The required input file argument is missing
/// * The input file cannot be read
fn main() -> Result<(),Error> {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("\nVersion:{}\nUsage: {} <directory>\nFinds all duplicate files in a specified sub-directory tree specified on command-line.", version, name);
        std::process::exit(1);
    }

    let directory = Path::new(&args[1]);
    if ! directory.try_exists()? {
        return Err(Error::new(ErrorKind::NotFound, "File or directory not found"));
    }
    let elapsed_time = measure_elapsed_time(|| {
        let _ = find_duplicates(directory);
    });
    eprintln!("Elapsed time: {}", elapsed_time);


    Ok(())
}
