use std::fs::File;
use std::io::{self, ErrorKind};
use std::path::PathBuf;
use std::time::Instant;
use clap::Parser;
use anyhow::Result;
use std::process;
use std::thread;
use std::panic;

use dupefiles::find_duplicates::find_duplicates;

/// Duplicate file finder - finds duplicate files in a directory tree
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help = true)]
struct Args {
    /// Directory to scan for duplicates
    #[arg(value_name = "DIRECTORY")]
    directory: PathBuf,

    /// Optional comma-separated list of file extensions to filter by (e.g., "mp4,jpg")
    #[arg(short, long)]
    extensions: Option<String>,

    /// Optional output file path (if not specified, prints to stdout)
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn measure_elapsed_time<F>(f: F) -> String
where
    F: FnOnce() -> Result<()>
{
    let start = Instant::now();
    if let Err(e) = f() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
    let duration = start.elapsed();

    let secs = duration.as_secs();
    let millis = duration.subsec_millis();

    if secs == 0 {
        format!("{} milliseconds", millis)
    } else {
        format!("{} seconds {} milliseconds", secs, millis)
    }
}

fn main() -> Result<()> {
    // Set up custom panic handler
    panic::set_hook(Box::new(|panic_info| {
        let thread = thread::current();
        let thread_name = thread.name().unwrap_or("<unnamed>");

        let msg = match panic_info.payload().downcast_ref::<&str>() {
            Some(s) => *s,
            None => match panic_info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<Any>",
            },
        };

        let location = panic_info.location().unwrap();

        eprintln!(
            "thread '{}' panicked at '{}', {}:{}:{}",
            thread_name,
            msg,
            location.file(),
            location.line(),
            location.column(),
        );
        process::exit(1);
    }));

    let args = Args::parse();

    let directory = args.directory.as_path();
    if !directory.exists() {
        return Err(io::Error::new(
            ErrorKind::NotFound,
            format!("Directory does not exist: {}", directory.display())
        ).into());
    }

    if !directory.is_dir() {
        return Err(io::Error::new(
            ErrorKind::InvalidInput,
            format!("Not a directory: {}", directory.display())
        ).into());
    }

    // Convert extensions to Vec<String> if provided
    let extensions: Option<Vec<String>> = args.extensions.map(|ext| {
        ext.trim_start_matches('*')
           .trim_start_matches('.')
           .split(',')
           .map(|s| s.trim_start_matches('*')
                     .trim_start_matches('.')
                     .to_lowercase())
           .filter(|s| !s.is_empty())
           .collect()
    });

    // Set up output file if specified
    let mut output_file = args.output.map(|path| {
        File::create(path).map_err(|e| {
            io::Error::new(ErrorKind::Other, format!("Failed to create output file: {}", e))
        })
    }).transpose()?;

    let elapsed_time = measure_elapsed_time(|| {
        find_duplicates(directory, extensions.as_ref(), output_file.as_mut())
    });
    eprintln!("Elapsed time: {}", elapsed_time);

    Ok(())
}