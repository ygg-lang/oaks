//! Helper utilities for common operations in the Oak Core parsing framework.
//!
//! This module provides utility functions for file system operations, URL handling,
//! and other common tasks that are useful when working with the parsing framework.

use crate::errors::OakError;

use crate::source::SourceText;

use std::fs::File;

use url::Url;
#[cfg(feature = "testing")]
mod lexing;
#[cfg(feature = "testing")]
pub use self::lexing::LexerTester;

#[cfg(feature = "testing")]
mod parsing;
#[cfg(feature = "testing")]
pub use self::parsing::ParserTester;

#[cfg(feature = "testing")]
mod building;
#[cfg(feature = "testing")]
pub use self::building::BuilderTester;

/// Converts a file system path to a URL.
///
/// # Arguments
///
/// * `path` - The file system path to convert
///
/// # Returns
///
/// A `Result` containing the URL if successful, or an `OakError` if the path is invalid
///
/// # Examples
///
/// ```ignore
/// let path = std::path::Path::new("/home/user/file.txt");
/// let url = url_from_path(path)?;
/// ```
pub fn url_from_path(path: &std::path::Path) -> Result<Url, OakError> {
    match Url::from_file_path(path) {
        Ok(o) => Ok(o),
        Err(_) => Err(OakError::custom_error(format!("invalid url {}", path.display()))),
    }
}

/// Reads source text from a file path.
///
/// This function reads the contents of a file and creates a `SourceText` with
/// the appropriate URL metadata. It's a convenience function for loading source
/// files into the parsing system.
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
    let url = url_from_path(path)?;
    match std::fs::read_to_string(path) {
        Ok(o) => Ok(SourceText::new_with_url(o, url)),
        Err(e) => Err(OakError::io_error(e, url)),
    }
}

/// Reads JSON data from a file path.
///
/// This function reads the contents of a file and parses it as JSON.
///
/// # Arguments
///
/// * `path`: The file system path to read from
///
/// returns: Result<T, OakError>
///
/// # Examples
///
/// ```
/// ```
#[cfg(feature = "serde_json")]
pub fn json_from_path<T>(path: &std::path::Path) -> Result<T, OakError>
where
    T: for<'de> serde::Deserialize<'de>,
{
    let url = url_from_path(path)?;
    match File::open(path) {
        Ok(o) => Ok(serde_json::from_reader(o)?),
        Err(e) => Err(OakError::io_error(e, url)),
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
    let url = url_from_path(path)?;
    match File::open(path) {
        Ok(o) => Ok(o),
        Err(e) => Err(OakError::io_error(e, url)),
    }
}

/// Creates a file and returns a file handle.
///
/// # Arguments
///
/// * `path` - The file system path to create
///
/// # Returns
///
/// A `Result` containing the file handle if successful, or an `OakError` if creation fails
///
/// # Examples
///
/// ```ignore
/// let path = std::path::Path::new("output.txt");
/// let file = create_file(path)?;
/// ```
pub fn create_file(path: &std::path::Path) -> Result<File, OakError> {
    let url = url_from_path(path)?;
    match File::create(path) {
        Ok(o) => Ok(o),
        Err(e) => Err(OakError::io_error(e, url)),
    }
}
