use std::path::Path;

/// Evaluate supplied &Path to determine if the referenced object is hidden or not.
///
/// This function accepts a &Path reference.
///
/// # Arguments
///
/// * `path` - path to evaluate its hidden status. 
///
/// # Returns
///
/// true if referenced object is hidden, and false otherwise.
///
/// # Examples
///
///
/// # Panics
///
/// This function does not panic.
///
/// # Errors
///
/// This function does not return a `Result`, so it cannot produce an `Err`.
///
/// # Safety
///
/// This function is always safe to call.
pub fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.starts_with('.'))
        .unwrap_or(false)
        || path.ancestors()
            .any(|ancestor| ancestor.file_name()
                .and_then(|name| name.to_str())
                .map(|name| name.starts_with('.'))
                .unwrap_or(false))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_is_hidden() {
        assert!(is_hidden(&PathBuf::from("/Othermachine/home/minty/.cargo/bin/cargo-fmt")));
        assert!(is_hidden(&PathBuf::from("/.cargo/bin/cargo-fmt")));
        assert!(!is_hidden(&PathBuf::from("/Othermachine/home/minty/cargo-fmt")));
        assert!(is_hidden(&PathBuf::from("/Othermachine/home/minty/.hidden_dir/file.txt")));
        assert!(!is_hidden(&PathBuf::from("/Othermachine/home/minty/visible_dir/file.txt")));
        assert!(is_hidden(&PathBuf::from("/.hidden_file")));
        assert!(!is_hidden(&PathBuf::from("/visible_file")));
        assert!(is_hidden(&PathBuf::from("/Othermachine/home/minty/directory/.hidden_file")));
        assert!(!is_hidden(&PathBuf::from("/Othermachine/home/minty/directory/visible_file")));
    }
}
