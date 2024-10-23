use std::path::Path;

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
        assert!(is_hidden(&PathBuf::from("/home/minty/Othermachine/home/minty/.cargo/bin/cargo-fmt")));
        assert!(is_hidden(&PathBuf::from("/home/minty/.cargo/bin/cargo-fmt")));
        assert!(!is_hidden(&PathBuf::from("/home/minty/Othermachine/home/minty/cargo-fmt")));
        assert!(is_hidden(&PathBuf::from("/home/minty/Othermachine/home/minty/.hidden_dir/file.txt")));
        assert!(!is_hidden(&PathBuf::from("/home/minty/Othermachine/home/minty/visible_dir/file.txt")));
        assert!(is_hidden(&PathBuf::from("/home/minty/.hidden_file")));
        assert!(!is_hidden(&PathBuf::from("/home/minty/visible_file")));
        assert!(is_hidden(&PathBuf::from("/home/minty/Othermachine/home/minty/directory/.hidden_file")));
        assert!(!is_hidden(&PathBuf::from("/home/minty/Othermachine/home/minty/directory/visible_file")));
    }
}
