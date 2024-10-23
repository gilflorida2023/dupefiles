use std::fs;
use std::path::Path;

pub fn get_file_size(path: &Path) -> std::io::Result<u64> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_get_file_size_existing_file() {
        // Create a temporary file for testing
        let path = Path::new("test_file.txt");
        fs::write(&path, b"Hello, Rust!").unwrap();

        // Test getting the file size
        let size = get_file_size(&path).unwrap();
        assert_eq!(size, 12);

        // Clean up
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_get_file_size_non_existing_file() {
        let path = Path::new("/foo/bar/non_existing_file.txt");
        let result = get_file_size(&path);
        assert!(result.is_err());
    }
}
