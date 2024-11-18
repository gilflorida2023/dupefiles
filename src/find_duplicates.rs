use std::collections::HashMap;
use std::fs;
use std::env;
use std::path::{Path, PathBuf};
use anyhow::Result;
use std::io::Write;
use walkdir::WalkDir;
use crate::is_duplicate_file::is_duplicate_file;
use crate::compute_sha256::compute_sha256;
use crate::is_hidden::is_hidden;
use crate::human_readable_size::human_readable_size;
use std::io::ErrorKind;
use std::io::Error;

/// Write a line either to the output file or stdout
fn write_line(output_file: &mut Option<&mut fs::File>, line: &str) -> Result<()> {
    match output_file {
        Some(file) => writeln!(file, "{}", line)?,
        None => println!("{}", line),
    }
    Ok(())
}

/// This function takes a directory Path value and prints duplicates identified to the specified output.
/// It skips zero byte files as well as hidden files and hidden directories.
/// 
/// # Arguments
///
/// * `directory` - The directory Path where the search for duplicates begins
/// * `extensions` - Optional list of file extensions to filter by (e.g., ["mp4", "jpg"])
/// * `output_file` - Optional file to write results to (if None, writes to stdout)
///
/// # Returns
///
/// Result
///
/// # Example
///
/// ```no_run
/// use std::path::Path;
/// use dupefiles::find_duplicates::find_duplicates;
/// 
/// # fn main() -> anyhow::Result<()> {
/// let directory = Path::new("test_data");
/// let extensions = Some(vec!["txt".to_string()]);
/// let mut output_file = None;
/// 
/// find_duplicates(directory, extensions.as_ref(), output_file.as_mut())?;
/// # Ok(())
/// # }
/// ```
pub fn find_duplicates(directory: &Path, extensions: Option<&Vec<String>>, mut output_file: Option<&mut fs::File>) -> Result<()> {
    static mut HEADER_PRINTED_ONCE: bool = false;
    let mut hash_map: HashMap<String, PathBuf> = HashMap::new();
    let mut found_duplicates = false;

    let current_dir = env::current_dir()?;
    let absolute_path = current_dir.join(directory);
    let canonical_directory = absolute_path.canonicalize().map_err(|e| {
        Error::new(ErrorKind::NotFound, format!("Failed to canonicalize directory path: {}", e))
    })?;

    if !canonical_directory.exists() {
        return Err(Error::new(ErrorKind::NotFound, "Directory does not exist").into());
    }

    // Write CSV header if needed
    unsafe {
        if !HEADER_PRINTED_ONCE {
            write_line(&mut output_file, "DUPE1.NAME,DUPE1.SIZE,DUPE1.HRSIZE,DUPE2.NAME,DUPE2.SIZE,DUPE2.HRSIZE")?;
            HEADER_PRINTED_ONCE = true;
        }
    }

    for entry in WalkDir::new(&canonical_directory)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !is_hidden(e.path()))
    {
        let path = entry.path();

        // Skip symlinks that point to non-existent targets
        if path.is_symlink() {
            if let Ok(link_target) = fs::read_link(path) {
                if !link_target.exists() {
                    eprintln!("Skipping broken symlink: {} -> {}", path.display(), link_target.display());
                    continue;
                }
            } else {
                eprintln!("Failed to read symlink: {}", path.display());
                continue;
            }
        }

        // Get file metadata
        let metadata = match fs::metadata(path) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Error accessing metadata for {}: {}", path.display(), e);
                continue;
            }
        };

        // Skip if not a file or zero size
        if !metadata.is_file() || metadata.len() == 0 {
            continue;
        }

        // Check file extension if filters are specified
        if let Some(exts) = extensions {
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                if !exts.iter().any(|e| e == &ext_str) {
                    continue;
                }
            } else {
                continue; // Skip files without extensions when filtering
            }
        }

        // Compute file hash
        let hash = match compute_sha256(path) {
            Ok(h) => h,
            Err(e) => {
                eprintln!("Failed to compute hash for {}: {}", path.display(), e);
                continue;
            }
        };

        // Check for duplicates
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

            let existing_size = match fs::metadata(existing_path) {
                Ok(m) => m.len(),
                Err(e) => {
                    eprintln!("Error accessing metadata for {}: {}", existing_path.display(), e);
                    continue;
                }
            };

            let current_size = metadata.len();
            found_duplicates = true;
            
            // Write duplicate file information
            let output = format!("\"{}\",{},\"{}\",\"{}\",{},\"{}\"",
                existing_path.display(), existing_size, human_readable_size(existing_size),
                path.display(), current_size, human_readable_size(current_size));
            write_line(&mut output_file, &output)?;
        } else {
            hash_map.insert(hash, path.to_path_buf());
        }
    }

    if !found_duplicates {
        write_line(&mut output_file, "No duplicate files found.")?;
    }

    Ok(())
}