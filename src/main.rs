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
use std::io::{self,ErrorKind};
use crate::find_duplicates::find_duplicates;
use crate::elapsed_time::measure_elapsed_time;
pub mod compute_sha256;
pub mod is_hidden;
pub mod is_duplicate_file;
pub mod find_duplicates;
pub mod debug_message;
pub mod elapsed_time;
pub mod human_readable_size;
use std::error::Error;
use std::panic;
use std::backtrace::Backtrace;


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
/// 
//use std::panic;
//use std::backtrace::Backtrace;
fn main() -> Result<(), Box<dyn Error>> {

    panic::set_hook(Box::new(|panic_info| {
        let backtrace = Backtrace::capture();
        eprintln!("Panic occurred: {:?}", panic_info);
        eprintln!("Stack trace:\n{:?}", backtrace);
    }));

    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("\nVersion:{}\nUsage: {} <directory>\nFinds all duplicate files in a specified sub-directory tree specified on command-line.", version, name);
        return Ok(())
    }

    let directory = Path::new(&args[1]);
    if !directory.try_exists()? {
        return Err(Box::new(io::Error::new(ErrorKind::NotFound, "File or directory not found")));
    }

    let elapsed_time = measure_elapsed_time(|| {
        let _ = find_duplicates(directory);
    });
    eprintln!("Elapsed time: {}", elapsed_time);

    Ok(())
}