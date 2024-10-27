use std::fs;
use std::path::Path;
use std::os::unix::fs::MetadataExt;
use crate::compute_sha256::compute_sha256;

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
pub fn   is_duplicate_file(file1: &Path,file2: &Path) -> bool {
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
    let f1hash = compute_sha256(file1).unwrap();
    let f2hash = compute_sha256(file2).unwrap();
    
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