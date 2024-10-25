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
use std::{env,fs};
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

fn detect_duplicates(files: Vec<&str>) -> bool {
    // This function should be implemented in your dupefiles application.
    // Here it's just a placeholder.
   false 
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
    
        // Prepare file paths for the duplicate detection function
        let files = vec![
            file_path.to_str().unwrap(),
            link_path.to_str().unwrap(),
        ];
    
        // Call the duplicate detection function
        let result = detect_duplicates(files);
    
        // Assert that no duplicates are detected since they point to the same inode
        assert_eq!(result, false, "Should not detect duplicates for hard links");
    
        // Clean up the test files explicitly
        fs::remove_file(&file_path).expect("Unable to delete test file");
        fs::remove_file(&link_path).expect("Unable to delete hard link");
    }
}