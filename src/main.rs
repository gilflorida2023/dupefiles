/*
  Finds all duplicate files in a specified sub-directory tree specified on command-line.
  
  Associated with the sha256 is the absolute filename.
  Create empty hash table.
  For each file in the directory tree, 
      Create an entry in a hash table which is key'ed by sha256 of the file. 
          associated with key :Store absolute path and filename.
          if the hash already exists, 
              print a duplicate report consisting of :
                  filename_from_table, its filesize and
                  new_filename,its filesize
Note: Have not noticed a checksum related to different sized files, but still looking.
*/

use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use walkdir::WalkDir;
use dupefiles::is_hidden;
use dupefiles::compute_sha256;
use dupefiles::get_file_size;

fn find_duplicates(directory: &Path) -> Result<()> {
    let mut hash_map: HashMap<String, PathBuf> = HashMap::new();

    for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        
        let fsize:u64= match get_file_size::get_file_size(path) {
            Ok(size) => size,
            Err(_e) => 0,
        };
        if is_hidden::is_hidden(path) || fsize == 0 {
            continue;
        }
        if path.is_file() {
            //println!("{}",path.display());
            let hash = compute_sha256::compute_sha256(path)
                .with_context(|| format!("Failed to compute hash for {}", path.display()))?;

            if let Some(existing_path) = hash_map.get(&hash) {
                let existing_fsize:u64= match get_file_size::get_file_size(existing_path) {
                    Ok(size) => size,
                    Err(_e) => 0,
                };
                eprintln!("\"{}\",{},\"{}\",{}", existing_path.display(),existing_fsize,path.display(),fsize);
            } else {
                hash_map.insert(hash, path.to_path_buf());
            }
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <directory>\nFinds all duplicate files in a specified sub-directory tree specified on command-line.", args[0]);
        std::process::exit(1);
    }

    let directory = Path::new(&args[1]);
    find_duplicates(directory)?;

    Ok(())
}
