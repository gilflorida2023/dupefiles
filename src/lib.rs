//! Duplicate file finder library
//! 
//! This library provides functionality to find duplicate files in a directory tree.
//! It uses SHA256 hashing for file comparison and supports filtering by file extension.
//! 
//! # Examples
//! 
//! ```no_run
//! use std::path::Path;
//! use dupefiles::find_duplicates::find_duplicates;
//! 
//! # fn main() -> anyhow::Result<()> {
//! let directory = Path::new("test_data");
//! let extensions = Some(vec!["txt".to_string()]);
//! let mut output_file = None;
//! 
//! find_duplicates(directory, extensions.as_ref(), output_file.as_mut())?;
//! # Ok(())
//! # }
//! ```
//! 
//! # Features
//! 
//! - SHA256 hashing for reliable file comparison
//! - Optional file extension filtering
//! - Skips hidden files and directories
//! - Supports CSV output format
//! - Handles symlinks safely
//! - Provides human-readable file sizes

pub mod compute_sha256;
pub mod is_hidden;
pub mod is_duplicate_file;
pub mod find_duplicates;
pub mod debug_message;
pub mod elapsed_time;
pub mod human_readable_size;