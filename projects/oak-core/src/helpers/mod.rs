//! Helper utilities for common operations in the Oak Core parsing framework.
//!
//! This module provides utility functions for file system operations
//! and other common tasks that are useful when working with the parsing framework.

use crate::errors::OakError;

use crate::source::SourceText;

use std::fs::File;

/// Reads source text from a file path.
///
/// This function reads the contents of a file and creates a `SourceText`.
///
/// # Arguments
///
/// * `path` - The file system path to read from
///
/// # Returns
///
/// A `Result` containing the `SourceText` if successful, or an `OakError` if reading fails
///
/// # Examples
///
/// ```ignore
/// let path = std::path::Path::new("source.rs");
/// let source = source_from_path(path)?;
/// ```
pub fn source_from_path(path: &std::path::Path) -> Result<SourceText, OakError> {
    match std::fs::read_to_string(path) {
        Ok(o) => Ok(SourceText::new(o)),
        Err(e) => Err(OakError::custom_error(format!("failed to read file {}: {}", path.display(), e))),
    }
}

/// Opens a file and returns a file handle.
///
/// # Arguments
///
/// * `path` - The file system path to open
///
/// # Returns
///
/// A `Result` containing the file handle if successful, or an `OakError` if opening fails
///
/// # Examples
///
/// ```ignore
/// let path = std::path::Path::new("source.rs");
/// let file = open_file(path)?;
/// ```
pub fn open_file(path: &std::path::Path) -> Result<File, OakError> {
    match File::open(path) {
        Ok(o) => Ok(o),
        Err(e) => Err(OakError::custom_error(format!("failed to open file {}: {}", path.display(), e))),
    }
}

/// Creates a file and its parent directories.
pub fn create_file(path: &std::path::Path) -> Result<std::fs::File, OakError> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| OakError::custom_error(e.to_string()))?;
    }
    std::fs::File::create(path).map_err(|e| OakError::custom_error(e.to_string()))
}
