use std::path::Path;


/// Determines if a given path is hidden.
///
/// A path is considered hidden if:
/// - Its file name starts with a dot (.), or
/// - Any of its parent directories' names start with a dot (.)
///
/// # Arguments
///
/// * `path` - A reference to a `Path` to check
///
/// # Returns
///
/// * `true` if the path is hidden, `false` otherwise
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use dupefiles::is_hidden::is_hidden;
///
/// let visible_file = Path::new("/home/user/documents/file.txt");
/// assert_eq!(is_hidden(visible_file), false);
///
/// let hidden_file = Path::new("/home/user/.config/settings.ini");
/// assert_eq!(is_hidden(hidden_file), true);
///
/// let file_in_hidden_dir = Path::new("/home/user/.hidden_dir/file.txt");
/// assert_eq!(is_hidden(file_in_hidden_dir), true);
///
/// let empty_path = Path::new("");
/// assert_eq!(is_hidden(empty_path), false);
/// ```
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

    #[test]
    fn test_visible_file() {
        let path = Path::new("/home/user/documents/file.txt");
        assert!(!is_hidden(path));
    }

    #[test]
    fn test_hidden_file() {
        let path = Path::new("/home/user/.hidden_file.txt");
        assert!(is_hidden(path));
    }

    #[test]
    fn test_file_in_hidden_directory() {
        let path = Path::new("/home/user/.hidden_dir/file.txt");
        assert!(is_hidden(path));
    }

    #[test]
    fn test_hidden_directory() {
        let path = Path::new("/home/user/.hidden_dir");
        assert!(is_hidden(path));
    }

    #[test]
    fn test_empty_path() {
        let path = Path::new("");
        assert!(!is_hidden(path));
    }

    #[test]
    fn test_root_path() {
        let path = Path::new("/");
        assert!(!is_hidden(path));
    }

}
