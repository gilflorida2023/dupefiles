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
use std::os::unix::fs::MetadataExt;
use anyhow::{Context, Result};
use walkdir::WalkDir;
use dupefiles::{compute_sha256,is_hidden};

/// This function takes a Path value and returns the inode and device id from the Metadata..
/// It considers file size, sha256sum, and inode, deviceid to determine if the two paths are duplicates
/// of each other, or the same file.
/// 
/// # Arguments
///
/// * `file1` - The first standard file Path
fn get_file_info(path: &Path) -> std::io::Result<(u64, u64)> {
    let metadata = fs::metadata(path)?;
    
    let inode = metadata.ino();
    let device_id = metadata.dev();
    
    Ok((inode, device_id))
}

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
    let (f1inode, f1device_id) = get_file_info(file1).unwrap();
    let (f2inode, f2device_id) = get_file_info(file2).unwrap();

    // if two alleged files share same device id and inode, 
    //its  really only one file.
    if  f1device_id == f2device_id && f1inode == f2inode {
        return false;
    }

    // looks like a duplicate file. safe to delete one..
    true
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

       let fsize: u64 = fs::metadata(path).unwrap().len();
       /*
        let fsize:u64= match get_file_size::get_file_size(path) {
            Ok(size) => size,
            Err(_e) => 0,
        };
        */
        if is_hidden::is_hidden(path) || fsize == 0 {
            continue;
        }
        if path.is_file() {
            let hash = compute_sha256::compute_sha256(path)
                .with_context(|| format!("Failed to compute hash for {}", path.display()))?;

            if let Some(existing_path) = hash_map.get(&hash) {
                /*
                let existing_fsize:u64= match get_file_size::get_file_size(existing_path) {
                    Ok(size) => size,
                    Err(_e) => 0,
                };
                */
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
/// * argument directiry is the input file path (required)
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::Builder;
    #[test]
    fn test_no_duplicates_with_hard_link() {
        // Create a temporary directory
        let tmp_dir = Builder::new()
        .prefix("hardlink_detect_dupes")
        .tempdir()
        .unwrap();

        let dir_path = tmp_dir.path();
    
        // Create a temporary file
        let file_path = dir_path.join("test_file.txt");
        fs::write(&file_path, "duplicate content").unwrap();
    
        // Create a hard link to the temporary file
        let link_path = dir_path.join("test_file_link.txt");
        fs::hard_link(&file_path, &link_path).unwrap();
    
        // Call the duplicate detection function
        let result = is_duplicate_file(&file_path,&link_path);
    
        // Assert that no duplicates are detected since they point to the same inode
        assert_eq!(result, false, "Should not detect duplicates for hard links");
    
        // Clean up the test files explicitly
        fs::remove_file(&file_path).expect("Unable to delete test file");
        fs::remove_file(&link_path).expect("Unable to delete hard link");
    }
}