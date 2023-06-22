use crate::{FileMetadata, FileType, Vfs, WritableVfs};
use oak_core::source::SourceText;
use std::{fs, path::PathBuf};
use url::Url;

/// A disk-based Virtual File System implementation.
/// Handles file URIs and interacts with the physical file system.
#[derive(Default, Clone)]
pub struct DiskVfs;

impl DiskVfs {
    /// Create a new DiskVfs.
    pub fn new() -> Self {
        Self
    }

    /// Convert a URI string to a PathBuf.
    fn uri_to_path(&self, uri: &str) -> Option<PathBuf> {
        if let Ok(url) = Url::parse(uri) {
            if url.scheme() == "file" {
                return url.to_file_path().ok();
            }
        }
        // Fallback for relative paths or non-URI strings
        Some(PathBuf::from(uri))
    }

    /// Convert a PathBuf to a URI string.
    fn path_to_uri(&self, path: PathBuf) -> String {
        if let Ok(url) = Url::from_file_path(path) { url.to_string() } else { "".to_string() }
    }
}

impl Vfs for DiskVfs {
    type Source = SourceText;

    fn get_source(&self, uri: &str) -> Option<SourceText> {
        let path = self.uri_to_path(uri)?;
        fs::read_to_string(path).ok().map(SourceText::new)
    }

    fn exists(&self, uri: &str) -> bool {
        self.uri_to_path(uri).map(|p| p.exists()).unwrap_or(false)
    }

    fn metadata(&self, uri: &str) -> Option<FileMetadata> {
        let path = self.uri_to_path(uri)?;
        let meta = fs::metadata(path).ok()?;

        let file_type = if meta.is_file() {
            FileType::File
        }
        else if meta.is_dir() {
            FileType::Directory
        }
        else {
            FileType::Other
        };

        let modified = meta.modified().ok().and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok()).map(|d| d.as_secs());

        Some(FileMetadata { file_type, len: meta.len(), modified })
    }

    fn read_dir(&self, uri: &str) -> Option<Vec<String>> {
        let path = self.uri_to_path(uri)?;
        if !path.is_dir() {
            return None;
        }

        let mut entries = Vec::new();
        for entry in fs::read_dir(path).ok()? {
            if let Ok(entry) = entry {
                entries.push(self.path_to_uri(entry.path()));
            }
        }
        Some(entries)
    }
}

impl WritableVfs for DiskVfs {
    fn write_file(&self, uri: &str, content: String) {
        if let Some(path) = self.uri_to_path(uri) {
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let _ = fs::write(path, content);
        }
    }

    fn remove_file(&self, uri: &str) {
        if let Some(path) = self.uri_to_path(uri) {
            let _ = fs::remove_file(path);
        }
    }
}
