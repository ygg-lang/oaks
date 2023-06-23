use crate::{FileMetadata, FileType, Vfs, WritableVfs};
use oak_core::{
    Arc,
    source::{SourceId, SourceText},
};
use std::{collections::HashMap, fs, path::PathBuf, sync::RwLock};

/// A virtual file system that reads from and writes to the physical disk.
pub struct DiskVfs {
    /// Mapping from file URIs to internal source IDs.
    uri_to_id: RwLock<HashMap<Arc<str>, SourceId>>,
    /// Mapping from source IDs back to file URIs.
    id_to_uri: RwLock<HashMap<SourceId, Arc<str>>>,
    /// Base path for relative URIs.
    root_path: PathBuf,
}

impl DiskVfs {
    /// Creates a new DiskVfs instance with the given root path.
    pub fn new(root_path: PathBuf) -> Self {
        Self { uri_to_id: RwLock::new(HashMap::new()), id_to_uri: RwLock::new(HashMap::new()), root_path }
    }

    /// Converts a URI to a physical file path.
    fn uri_to_path(&self, uri: &str) -> Option<PathBuf> {
        let path = PathBuf::from(uri);
        if path.is_absolute() { Some(path) } else { Some(self.root_path.join(path)) }
    }

    /// Converts a physical file path to a URI.
    fn path_to_uri(&self, path: PathBuf) -> Arc<str> {
        let uri = if path.is_absolute() { path.to_string_lossy().to_string() } else { path.strip_prefix(&self.root_path).unwrap_or(&path).to_string_lossy().to_string() };
        Arc::from(uri)
    }
}

impl Vfs for DiskVfs {
    type Source = SourceText;

    /// Reads the content of a file from disk given its URI.
    fn get_source(&self, uri: &str) -> Option<Self::Source> {
        let path = self.uri_to_path(uri)?;
        let content = fs::read_to_string(path).ok()?;
        Some(SourceText::new(&*content))
    }

    /// Returns the URI for a given source ID.
    fn get_uri(&self, id: SourceId) -> Option<Arc<str>> {
        let id_to_uri = self.id_to_uri.read().unwrap();
        id_to_uri.get(&id).cloned()
    }

    /// Returns the source ID for a given URI, if it exists.
    fn get_id(&self, uri: &str) -> Option<SourceId> {
        let uri_to_id = self.uri_to_id.read().unwrap();
        uri_to_id.get(uri).cloned()
    }

    /// Checks if a file or directory exists at the given URI on disk.
    fn exists(&self, uri: &str) -> bool {
        self.uri_to_path(uri).map(|p| p.exists()).unwrap_or(false)
    }

    /// Retrieves metadata for a file or directory from the disk.
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

    /// Reads the contents of a directory on disk and returns their URIs.
    fn read_dir(&self, uri: &str) -> Option<Vec<Arc<str>>> {
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
    /// Writes the given content to a file on disk at the specified URI.
    fn write_file(&self, uri: &str, content: Arc<str>) {
        if let Some(path) = self.uri_to_path(uri) {
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let _ = fs::write(path, content.as_ref());
        }
    }

    /// Removes a file from disk at the specified URI.
    fn remove_file(&self, uri: &str) {
        if let Some(path) = self.uri_to_path(uri) {
            let _ = fs::remove_file(path);
        }
    }
}
