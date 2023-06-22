#![feature(new_range_api)]

use oak_core::{
    Arc,
    source::{Source, SourceId},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

mod line_map;
pub use line_map::LineMap;

/// Type of a file in the VFS.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FileType {
    File,
    Directory,
    Other,
}

/// Metadata for a file or directory in the VFS.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FileMetadata {
    pub file_type: FileType,
    pub len: u64,
    pub modified: Option<u64>, // Unix timestamp in seconds
}

pub mod vfs;
pub use vfs::MemoryVfs;
#[cfg(feature = "disk")]
pub use vfs::{DiskVfs, DiskWatcher, VfsEvent, VfsWatcher};

/// A trait for a Virtual File System that can provide source content and location mapping.
pub trait Vfs: Send + Sync {
    /// The type of source returned by this VFS.
    type Source: Source + 'static;

    /// Get the source for the given URI.
    fn get_source(&self, uri: &str) -> Option<Self::Source>;

    /// Get the URI for the given SourceId.
    fn get_uri(&self, id: SourceId) -> Option<Arc<str>>;

    /// Get the SourceId for the given URI.
    fn get_id(&self, uri: &str) -> Option<SourceId>;

    /// Check if a path exists at the given URI.
    fn exists(&self, uri: &str) -> bool;

    /// Read the metadata for the given URI.
    fn metadata(&self, uri: &str) -> Option<FileMetadata>;

    /// Read the contents of a directory at the given URI.
    /// Returns a list of URIs or names.
    fn read_dir(&self, uri: &str) -> Option<Vec<Arc<str>>>;

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
    fn write_file(&self, uri: &str, content: Arc<str>);

    /// Remove a file from the VFS.
    fn remove_file(&self, uri: &str);
}
