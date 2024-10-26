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

use std::collections::HashMap;
use std::{env,fs};
use std::path::{Path, PathBuf};
use std::io::{Error,ErrorKind};
//use std::os::unix::fs::MetadataExt;
use anyhow::{Context, Result};
use walkdir::WalkDir;
use std::fmt;
use std::arch::asm;

use crate::is_duplicate_file::is_duplicate_file;
pub mod compute_sha256;
pub mod is_hidden;
pub mod is_duplicate_file;
/*
/// This function takes two Path values and returns boolean indicating whether the files are duplicates.
/// It considers file size, sha256sum, and inode, deviceid to determine if the two paths are duplicates
/// of each other, or the same file.
/// 
/// # Arguments
///
/// * `file1` - The first standard file Path
/// * `file2` - The second standard file Path
///
/// # Returns
///
/// bool reflecting whether the two paths are duplicates.
///
fn is_duplicate_file(file1: &Path,file2: &Path) -> bool {
    // validate two files exist
    if ! file1.try_exists().unwrap() || ! file2.try_exists().unwrap() {
        return false;
    }

    // validate file sizes equal
    let f1size = fs::metadata(file1).unwrap().len();
    let f2size: u64 = fs::metadata(file2).unwrap().len();
    
    if f1size != f2size {
        return false;
    }

    // vereify two files have same hash
    let f1hash = compute_sha256::compute_sha256(file1).with_context(|| format!("Failed to compute hash for {}", file1.display())).unwrap();
    let f2hash = compute_sha256::compute_sha256(file2).with_context(|| format!("Failed to compute hash for {}", file2.display())).unwrap();
    
    if f1hash != f2hash {
        return false;   
    }

    // dont be tricked by external devices or hardlinks
    // if two alleged files share same device id and inode, 
    // its  really only one file.
    let f1inode: u64 = fs::metadata(file1).unwrap().ino();
    let f1device_id: u64 = fs::metadata(file1).unwrap().dev();
    
    let f2inode: u64 = fs::metadata(file2).unwrap().ino();
    let f2device_id: u64 = fs::metadata(file2).unwrap().dev();

    if  f1device_id == f2device_id && f1inode == f2inode {
        return false;
    }

    // looks like a duplicate file. safe to delete one..
    true
}
*/
// Debug routines
#[cfg(feature = "debug")]
fn debug_message(args: fmt::Arguments) {
    println!("{}", args);
}

#[cfg(not(feature = "debug"))]
fn debug_message(_args: fmt::Arguments) {
    unsafe {
        asm!("nop");
    }
}

// Macro to make it easier to use debug_message with format strings
macro_rules! log {
    ($($arg:tt)*) => {
        debug_message(format_args!($($arg)*))
    };
}


/// This function takes a directory Path value and prints to stdout, a csv file indicating duplicates identified.
/// It skips zero byte files as well as hidden files and hidden directories. It calls ['crate::dupefiles::is_duplicate_file()'] to make sure 
/// its safe to delete  one copy of identified duplicate. Prints to stdout.
/// 
/// # Arguments
///
/// * `directory` - The directory Path where the search for duplicates begins.
///
/// # Returns
///
/// Result
///
fn find_duplicates(directory: &Path) -> Result<()> {
    static mut HEADER_PRINTED_ONCE: bool = false;
    let mut hash_map: HashMap<String, PathBuf> = HashMap::new();

    for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
       let path = entry.path();
       log!("{}",path.display());


       if path.is_symlink() {
           continue; 
       }

       let fsize: u64 = fs::metadata(path).unwrap().len();
        if is_hidden::is_hidden(path) || fsize == 0 {
            continue;
        }
        if path.is_file() {
            let hash = compute_sha256::compute_sha256(path)
                .with_context(|| format!("Failed to compute hash for {}", path.display()))?;

            if let Some(existing_path) = hash_map.get(&hash) {
                if ! is_duplicate_file(&existing_path,&path) {
                    continue;
                }

                unsafe { // ony print header once
                    if ! HEADER_PRINTED_ONCE {
                        println!("DUPE1.NAME,DUPE1.SIZE,DUPE2.NAME,DUPE2.SIZE");
                        HEADER_PRINTED_ONCE = true;
                    }
                }
                let existing_fsize: u64 = fs::metadata(existing_path).unwrap().len();
                println!("\"{}\",{},\"{}\",{}", existing_path.display(),existing_fsize,path.display(),fsize);
            } else {
                hash_map.insert(hash, path.to_path_buf());
            }
        }
    }

    Ok(())
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
    let _ = find_duplicates(directory);
    eprintln!("Success");
    Ok(())
}
