# Changelog

All notable changes to the dupefiles project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2024-01-21

### Added
- Initial release of dupefiles
- Core functionality to find duplicate files using SHA256 hashing
- Command-line interface using clap (v4.5.21)
- File extension filtering with support for multiple extensions
- CSV output format with file sizes in bytes and human-readable format
- Support for writing results to file or stdout
- Error handling using anyhow
- Hidden file and directory skipping
- Safe symlink handling
- Zero-byte file skipping
- Performance timing output
- Documentation and examples

### Changed
- Improved extension handling to properly strip '*' and '.' prefixes
- Refactored find_duplicates function for better organization
- Removed debug logging for cleaner output
- Updated error messages to be more descriptive

### Fixed
- Extension filtering now correctly handles *.jpg style inputs
- Fixed duplicate detection logic
- Improved error handling for broken symlinks
- Fixed output formatting for better CSV compatibility

## [0.1.0] - 2024-01-20

### Added
- Project initialization
- Basic project structure
- Initial dependency setup
