/// Converts a file size in bytes to a human-readable string.
///
/// This function takes a file size in bytes and returns a String representing
/// the size in a human-readable format (e.g., "123 B", "45 KB", "6.7 MB", "1.2 GB").
/// It uses binary prefixes (KiB, MiB, GiB, TiB) for sizes 1024 and above.
///
/// # Arguments
///
/// * `size_in_bytes` - A u64 representing the file size in bytes.
///
/// # Returns
///
/// A String containing the human-readable file size.
///
/// # Example
///
/// ```
/// use dupefiles::human_readable_size::human_readable_size;
/// assert_eq!(human_readable_size(0), "0 B");
/// assert_eq!(human_readable_size(1023), "1023 B");
/// assert_eq!(human_readable_size(1024), "1.0 KiB");
/// assert_eq!(human_readable_size(1500), "1.5 KiB");
/// assert_eq!(human_readable_size(1048576), "1.0 MiB");
/// assert_eq!(human_readable_size(1073741824), "1.0 GiB");
/// assert_eq!(human_readable_size(1099511627776), "1.0 TiB");
/// ```
pub fn human_readable_size(size_in_bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];
    if size_in_bytes == 0 {
        return "0 B".to_string();
    }
    let index = (size_in_bytes as f64).log(1024.0).floor() as usize;
    let adjusted_size = size_in_bytes as f64 / 1024_f64.powi(index as i32);
    
    if index == 0 {
        format!("{} {}", adjusted_size.round(), UNITS[index])
    } else {
        format!("{:.1} {}", adjusted_size, UNITS[index])
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    //use std::path::PathBuf;

 
    #[test]
    fn test_tebibyte() {
        assert_eq!(human_readable_size(241 * 1024 * 1024 * 1024 * 1024), "241.0 TiB");
    }


    #[test]
    fn test_gibibyte() {
        assert_eq!(human_readable_size(137 * 1024 * 1024 * 1024), "137.0 GiB");
    }

    #[test]
    fn test_mebibyte() {
        //assert_eq!(human_readable_size(1048576), "1.0 MiB");
        assert_eq!(human_readable_size(42 * 1024 * 1024), "42.0 MiB");
    }


    #[test]
    fn test_kibibyte() {
        assert_eq!(human_readable_size(911 * 1024), "911.0 KiB");
    }
}