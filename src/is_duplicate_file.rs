use std::fs;
use std::path::Path;
use std::os::unix::fs::MetadataExt;
use crate::compute_sha256::compute_sha256;

use std::io::Error;


/// Determines if two files are duplicates based on their content and metadata.
///
/// This function checks if two files are duplicates by comparing their size, SHA256 hash,
/// and inode information. It considers files as duplicates if they have the same content
/// but are stored as separate files on the filesystem.
///
/// # Arguments
///
/// * `file1` - A reference to the `Path` of the first file to compare.
/// * `file2` - A reference to the `Path` of the second file to compare.
///
/// # Returns
///
/// Returns `true` if the files are duplicates, `false` otherwise.
///
/// # Panics
///
/// This function will panic if:
/// - File existence checks fail.
/// - File metadata cannot be retrieved.
/// - SHA256 hash computation fails.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use std::fs::File;
/// use std::io::Write;
/// use dupefiles::is_duplicate_file::is_duplicate_file;
///
/// // Create two files with the same content
/// let file1_path = Path::new("test_file1.txt");
/// let file2_path = Path::new("test_file2.txt");
/// let content = b"Test content";
///
/// File::create(file1_path).unwrap().write_all(content).unwrap();
/// File::create(file2_path).unwrap().write_all(content).unwrap();
///
/// // Use unwrap() to get the boolean result, or handle the error appropriately
/// assert!(is_duplicate_file(file1_path, file2_path).unwrap());
///
/// // Clean up: remove the test files
/// std::fs::remove_file(file1_path).unwrap();
/// std::fs::remove_file(file2_path).unwrap();
/// ```
/// # Note
///
/// This function considers files as non-duplicates if they are actually the same file
/// (i.e., same inode and device ID). This is to distinguish between true duplicates
/// and hard links.
pub fn is_duplicate_file(file1: &Path, file2: &Path) -> Result<bool, Error> {
    if !file1.try_exists().map_err(|e| {
        eprintln!("Error checking existence of file1: {}", e);
        e
    })? || !file2.try_exists().map_err(|e| {
        eprintln!("Error checking existence of file2: {}", e);
        e
    })? {
        return Ok(false);
    }

    let f1size = fs::metadata(file1).map_err(|e| {
        eprintln!("Error accessing metadata of file1: {}", e);
        e
    })?.len();
    let f2size = fs::metadata(file2).map_err(|e| {
        eprintln!("Error accessing metadata of file2: {}", e);
        e
    })?.len();

    if f1size != f2size {
        return Ok(false);
    }

    let f1hash = compute_sha256(file1).map_err(|e| {
        eprintln!("Error computing SHA256 for file1: {}", e);
        e
    })?;
    let f2hash = compute_sha256(file2).map_err(|e| {
        eprintln!("Error computing SHA256 for file2: {}", e);
        e
    })?;

    if f1hash != f2hash {
        return Ok(false);
    }

    let f1metadata = fs::metadata(file1).map_err(|e| {
        eprintln!("Error accessing metadata of file1: {}", e);
        e
    })?;
    let f2metadata = fs::metadata(file2).map_err(|e| {
        eprintln!("Error accessing metadata of file2: {}", e);
        e
    })?;

    let f1inode = f1metadata.ino();
    let f1device_id = f1metadata.dev();
    let f2inode = f2metadata.ino();
    let f2device_id = f2metadata.dev();

    if f1device_id == f2device_id && f1inode == f2inode {
        return Ok(false);
    }

    Ok(true)
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
        assert!(result.is_ok(), "is_duplicate_file should not return an error");
        assert_eq!(result.unwrap(), false, "Should not detect duplicates for hard links");
    
        // Clean up the test files explicitly
        fs::remove_file(&file_path).expect("Unable to delete test file");
        fs::remove_file(&link_path).expect("Unable to delete hard link");
    }
}
    