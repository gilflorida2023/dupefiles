use std::fs::File;
use std::io::{BufReader, Read, Result,Error, ErrorKind};
use std::path::Path;
use sha2::{Sha256, Digest};

/// Computes the SHA256 hash of a file at the given path.
///
/// # Arguments
///
/// * `path` - A reference to a `Path` representing the file to hash.
///
/// # Returns
///
/// * `Result<String>` - The SHA256 hash as a hexadecimal string if successful, or an error if the file
///   doesn't exist or cannot be read.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// // use your_crate_name::compute_sha256;
/// use dupefiles::compute_sha256::compute_sha256;
///
/// let path = Path::new("example.txt");
/// match compute_sha256(path) {
///     Ok(hash) => println!("SHA256: {}", hash),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
pub fn compute_sha256(path: &Path) -> Result<String> {
    if ! path.try_exists()? {
        // Path does not exist, return an error
        return Err(Error::new(ErrorKind::NotFound, "Path does not exist"))
    }
    let file = File::open(path)?;
    let mut reader = BufReader::with_capacity(1024 * 1024, file); // 1MB buffer
    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024 * 1024]; // Also increase the read buffer to 1MB

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 { 
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }   

    Ok(format!("{:x}", hasher.finalize()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_compute_sha256() {
        // Create a temporary file for testing
        let test_file_path = Path::new("/tmp/test_file.txt");
        let data = b"Hello, world!";
        fs::write(&test_file_path, data).expect("Unable to write test file");

        // Compute the SHA256 hash
        let hash = compute_sha256(&test_file_path).expect("Failed to compute SHA256");

        // Check against expected hash
        assert_eq!(hash, "315f5bdb76d078c43b8ac0064e4a0164612b1fce77c869345bfc94c75894edd3");

        // Clean up the test file
        fs::remove_file(&test_file_path).expect("Unable to delete test file");
    }
    #[test]
    fn test_compute_sha_256_checksum_non_existing_file() {
        let path = Path::new("/non/existing/file.txt");
        let result= compute_sha256(&path) ;
        assert!(result.is_err());
    }
}
