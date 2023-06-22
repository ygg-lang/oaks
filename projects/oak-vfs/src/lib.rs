#![feature(new_range_api)]

use oak_core::source::Source;
use serde::{Deserialize, Serialize};

mod line_map;
pub use line_map::LineMap;

/// Type of a file in the VFS.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileType {
    File,
    Directory,
    Other,
}

/// Metadata for a file or directory in the VFS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub file_type: FileType,
    pub len: u64,
    pub modified: Option<u64>, // Unix timestamp in seconds
}

pub mod vfs;
pub use vfs::{DiskVfs, MemoryVfs};

/// A trait for a Virtual File System that can provide source content and location mapping.
pub trait Vfs: Send + Sync {
    /// The type of source returned by this VFS.
    type Source: Source + 'static;

    /// Get the source for the given URI.
    fn get_source(&self, uri: &str) -> Option<Self::Source>;

    /// Check if a path exists at the given URI.
    fn exists(&self, uri: &str) -> bool;

    /// Read the metadata for the given URI.
    fn metadata(&self, uri: &str) -> Option<FileMetadata>;

    /// Read the contents of a directory at the given URI.
    /// Returns a list of URIs or names.
    fn read_dir(&self, uri: &str) -> Option<Vec<String>>;

    /// Check if the given URI points to a file.
    fn is_file(&self, uri: &str) -> bool {
        self.metadata(uri).map(|m| m.file_type == FileType::File).unwrap_or(false)
    }

    /// Check if the given URI points to a directory.
    fn is_dir(&self, uri: &str) -> bool {
        self.metadata(uri).map(|m| m.file_type == FileType::Directory).unwrap_or(false)
    }

    fn line_map(&self, uri: &str) -> Option<LineMap> {
        self.get_source(uri).map(|s| LineMap::from_source(&s))
    }
}

/// A trait for a Virtual File System that supports writing.
pub trait WritableVfs: Vfs {
    /// Update or create a file with the given content.
    fn write_file(&self, uri: &str, content: String);

    /// Remove a file from the VFS.
    fn remove_file(&self, uri: &str);
}
