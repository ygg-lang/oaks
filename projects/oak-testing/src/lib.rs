#![feature(new_range_api)]
#![allow(missing_docs)]
//! Testing utilities for the Oak ecosystem.
//!
//! This module provides comprehensive testing infrastructure for lexers, parsers,
//! and builders, including file-based testing, expected output comparison,
//! timeout handling, and test result serialization.

pub mod building;
pub mod lexing;
pub mod parsing;

use oak_core::{errors::OakError, source::SourceText};
use std::{fs::File, path::Path};

/// Reads source text from a file path.
pub fn source_from_path(path: &Path) -> Result<SourceText, OakError> {
    match std::fs::read_to_string(path) {
        Ok(o) => Ok(SourceText::new(o)),
        Err(e) => Err(OakError::io_error(e, 0)),
    }
}

/// Reads JSON data from a file path.
pub fn json_from_path(path: &Path) -> Result<serde_json::Value, OakError> {
    let content = std::fs::read_to_string(path).map_err(|e| OakError::io_error(e, 0))?;
    serde_json::from_str(&content).map_err(|e| OakError::custom_error(e.to_string()))
}

/// Opens a file and returns a file handle.
pub fn open_file(path: &Path) -> Result<File, OakError> {
    match File::open(path) {
        Ok(o) => Ok(o),
        Err(e) => Err(OakError::io_error(e, 0)),
    }
}

/// Creates a file and its parent directories.
pub fn create_file(path: &Path) -> Result<std::fs::File, OakError> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| OakError::custom_error(e.to_string()))?
    }
    std::fs::File::create(path).map_err(|e| OakError::custom_error(e.to_string()))
}
