use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use walkdir::WalkDir;
use std::fmt;
use std::arch::asm;
use crate::is_duplicate_file::is_duplicate_file;
use crate::compute_sha256::compute_sha256;
use crate::is_hidden::is_hidden;


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
pub fn find_duplicates(directory: &Path) -> Result<()> {
    static mut HEADER_PRINTED_ONCE: bool = false;
    let mut hash_map: HashMap<String, PathBuf> = HashMap::new();

    for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
       let path = entry.path();
       log!("{}",path.display());


       if path.is_symlink() {
           continue; 
       }

       let fsize: u64 = fs::metadata(path).unwrap().len();
        if is_hidden(path) || fsize == 0 {
            continue;
        }
        if path.is_file() {
            let hash = compute_sha256(path)
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
