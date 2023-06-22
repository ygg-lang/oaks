use crate::{FileMetadata, FileType, Vfs, WritableVfs};
use oak_core::{
    Arc,
    source::{SourceId, SourceText},
};
use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    sync::{
        RwLock,
        atomic::{AtomicU32, Ordering},
    },
};
use url::Url;

/// A disk-based Virtual File System implementation.
/// Handles file URIs and interacts with the physical file system.
#[derive(Default)]
pub struct DiskVfs {
    ids: std::sync::Arc<RwLock<HashMap<SourceId, Arc<str>>>>,
    uri_to_id: std::sync::Arc<RwLock<HashMap<Arc<str>, SourceId>>>,
    next_id: std::sync::Arc<AtomicU32>,
}

impl Clone for DiskVfs {
    fn clone(&self) -> Self {
        Self { ids: self.ids.clone(), uri_to_id: self.uri_to_id.clone(), next_id: self.next_id.clone() }
    }
}

impl DiskVfs {
    /// Create a new DiskVfs.
    pub fn new() -> Self {
        Self::default()
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
    fn path_to_uri(&self, path: PathBuf) -> Arc<str> {
        if let Ok(url) = Url::from_file_path(path) { Arc::from(url.to_string()) } else { Arc::from("") }
    }

    fn get_or_assign_id(&self, uri: &str) -> SourceId {
        let mut uri_to_id = self.uri_to_id.write().unwrap();
        if let Some(id) = uri_to_id.get(uri) {
            return *id;
        }

        let mut ids = self.ids.write().unwrap();
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let uri_arc: Arc<str> = Arc::from(uri);
        uri_to_id.insert(uri_arc.clone(), id);
        ids.insert(id, uri_arc);
        id
    }
}

impl Vfs for DiskVfs {
    type Source = SourceText;

    fn get_source(&self, uri: &str) -> Option<SourceText> {
        let path = self.uri_to_path(uri)?;
        let id = self.get_or_assign_id(uri);
        fs::read_to_string(path).ok().map(|s| SourceText::new_with_id(s, id))
    }

    fn get_uri(&self, id: SourceId) -> Option<Arc<str>> {
        let ids = self.ids.read().unwrap();
        ids.get(&id).cloned()
    }

    fn get_id(&self, uri: &str) -> Option<SourceId> {
        let uri_to_id = self.uri_to_id.read().unwrap();
        uri_to_id.get(uri).cloned()
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
    fn write_file(&self, uri: &str, content: Arc<str>) {
        if let Some(path) = self.uri_to_path(uri) {
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let _ = fs::write(path, content.as_ref());
        }
    }

    fn remove_file(&self, uri: &str) {
        if let Some(path) = self.uri_to_path(uri) {
            let _ = fs::remove_file(path);
        }
    }
}
