/*
  Finds all duplicate files in a specified sub-directory tree specified on command-line.
  
  Create empty hash table.
  For each file in the directory tree, 
      Create an entry in a hash table which is key'ed by sha256 of the file. 
          Associate with each key :Store absolute path and filename.
          if the hash already exists, 
              print a duplicate report consisting of :
                  dupe1.filename, dupe1.filesize,dupe2.filename,dupe2.filesize
Note: hidden files, hidden directories and zero byte files are skipped.
*/

use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use walkdir::WalkDir;
use dupefiles::{get_file_size,compute_sha256,is_hidden};
use std::io::{Error,ErrorKind};

fn find_duplicates(directory: &Path) -> Result<()> {
    /*
    Create empty hash table.
    For each file in the directory tree, 
      Note: hidden files, hidden directories and zero byte files are skipped.
      Create an entry in a hash table which is key'ed by sha256 of the file. 
          Associate with each key :Store absolute path and filename.
          if the hash already exists, 
              print a duplicate report consisting of :
                  dupe1.filename, dupe1.filesize,dupe2.filename,dupe2.filesize
     */ 
    static mut HEADER_PRINTED: bool = false;

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
            let hash = compute_sha256::compute_sha256(path)
                .with_context(|| format!("Failed to compute hash for {}", path.display()))?;

            if let Some(existing_path) = hash_map.get(&hash) {
                let existing_fsize:u64= match get_file_size::get_file_size(existing_path) {
                    Ok(size) => size,
                    Err(_e) => 0,
                };
                unsafe {
                    if ! HEADER_PRINTED {
                        println!("DUPE1.NAME,DUPE1.SIZE,DUPE2.NAME,DUPE2.SIZE");
                        HEADER_PRINTED = true;
                    }
                }
                println!("\"{}\",{},\"{}\",{}", existing_path.display(),existing_fsize,path.display(),fsize);
            } else {
                hash_map.insert(hash, path.to_path_buf());
            }
        }
    }

    Ok(())
}

fn main() -> Result<(),Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <directory>\nFinds all duplicate files in a specified sub-directory tree specified on command-line.", args[0]);
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
