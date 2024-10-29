use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::Result;
use walkdir::WalkDir;
#[allow(unused_imports)]
use std::arch::asm;
use crate::is_duplicate_file::is_duplicate_file;
use crate::compute_sha256::compute_sha256;
use crate::is_hidden::is_hidden;
use crate::debug_message::debug_message;
use crate::human_readable_size::human_readable_size;
pub use crate::log;

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
pub fn find_duplicates(directory: &Path) -> Result<()> {
    static mut HEADER_PRINTED_ONCE: bool = false;
    let mut hash_map: HashMap<String, PathBuf> = HashMap::new();

    for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        log!("{}", path.display());

        if is_hidden(path) {
            continue;
        }

        /*
        if path.is_symlink() {
            continue;
        }
        */
        let fsize: u64 = match fs::metadata(path) {
            Ok(metadata) => metadata.len(),
            Err(e) => {
                eprintln!("Error accessing metadata for {}: {}", path.display(), e);
                continue;
            }
        };

        if fsize == 0 {
            continue;
        }
        

        if path.is_file() {

            let hash = match compute_sha256(path) {
                Ok(h) => h,
                Err(e) => {
                    eprintln!("Failed to compute hash for {}: {}", path.display(), e);
                    continue;
                }
            };

            if let Some(existing_path) = hash_map.get(&hash) {
                match is_duplicate_file(existing_path, path) {
                    Ok(is_duplicate) => {
                        if !is_duplicate {
                            continue;
                        }
                    },
                    Err(e) => {
                        eprintln!("Error checking for duplicate file: {}", e);
                        continue;
                    }
                }
            
                unsafe {
                    if !HEADER_PRINTED_ONCE {
                        println!("DUPE1.NAME,DUPE1.SIZE,DUPE1.HRSIZE,DUPE2.NAME,DUPE2.SIZE,DUPE2.HRSIZE");
                        HEADER_PRINTED_ONCE = true;
                    }
                }

                let existing_fsize: u64 = match fs::metadata(existing_path) {
                    Ok(metadata) => metadata.len(),
                    Err(e) => {
                        eprintln!("Error accessing metadata for {}: {}", existing_path.display(), e);
                        continue;
                    }
                };

                println!("\"{}\",{},\"{}\",\"{}\",{},\"{}\"", 
                    existing_path.display(), existing_fsize, human_readable_size(existing_fsize),
                    path.display(), fsize, human_readable_size(fsize));
            } else {
                hash_map.insert(hash, path.to_path_buf());
            }
        }
    }

    Ok(())
}