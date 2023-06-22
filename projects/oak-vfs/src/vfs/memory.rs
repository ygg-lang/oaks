use crate::{FileMetadata, FileType, Vfs, WritableVfs};
use oak_core::{
    Arc,
    source::{SourceId, SourceText},
};
use std::{
    collections::HashMap,
    sync::{
        RwLock,
        atomic::{AtomicU32, Ordering},
    },
};

/// A memory-based Virtual File System implementation.
/// Ideal for WASM environments or testing where physical disk access is not available.
#[derive(Default)]
pub struct MemoryVfs {
    files: std::sync::Arc<RwLock<HashMap<Arc<str>, FileEntry>>>,
    ids: std::sync::Arc<RwLock<HashMap<SourceId, Arc<str>>>>,
    uri_to_id: std::sync::Arc<RwLock<HashMap<Arc<str>, SourceId>>>,
    next_id: std::sync::Arc<AtomicU32>,
}

impl Clone for MemoryVfs {
    fn clone(&self) -> Self {
        Self { files: self.files.clone(), ids: self.ids.clone(), uri_to_id: self.uri_to_id.clone(), next_id: self.next_id.clone() }
    }
}

struct FileEntry {
    content: Arc<str>,
    modified: u64,
    id: SourceId,
}

impl MemoryVfs {
    /// Create a new empty MemoryVfs.
    pub fn new() -> Self {
        Self::default()
    }

    /// Upsert a file's content in the memory VFS.
    pub fn write_file(&self, uri: &str, content: impl Into<Arc<str>>) {
        let uri_arc: Arc<str> = Arc::from(uri);
        let mut files = self.files.write().unwrap();
        let mut ids = self.ids.write().unwrap();
        let mut uri_to_id = self.uri_to_id.write().unwrap();

        let id = if let Some(id) = uri_to_id.get(&uri_arc) {
            *id
        }
        else {
            let id = self.next_id.fetch_add(1, Ordering::SeqCst);
            uri_to_id.insert(uri_arc.clone(), id);
            ids.insert(id, uri_arc.clone());
            id
        };

        files.insert(uri_arc, FileEntry { content: content.into(), modified: Self::now(), id });
    }

    /// Remove a file from the memory VFS.
    pub fn remove_file(&self, uri: &str) {
        let mut files = self.files.write().unwrap();
        let mut ids = self.ids.write().unwrap();
        let mut uri_to_id = self.uri_to_id.write().unwrap();

        if let Some(entry) = files.remove(uri) {
            uri_to_id.remove(uri);
            ids.remove(&entry.id);
        }
    }

    fn now() -> u64 {
        // In WASM, we'd typically use js-sys::Date::now()
        // Here we use a simple timestamp or 0 if not available
        #[cfg(not(target_arch = "wasm32"))]
        {
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs()
        }
        #[cfg(target_arch = "wasm32")]
        {
            0 // Simplified for now
        }
    }
}

impl Vfs for MemoryVfs {
    type Source = SourceText;

    fn get_source(&self, uri: &str) -> Option<SourceText> {
        let files = self.files.read().unwrap();
        files.get(uri).map(|entry| SourceText::new_with_id(entry.content.clone(), entry.id))
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
        self.files.read().unwrap().contains_key(uri)
    }

    fn metadata(&self, uri: &str) -> Option<FileMetadata> {
        let files = self.files.read().unwrap();
        files.get(uri).map(|entry| FileMetadata { file_type: FileType::File, len: entry.content.len() as u64, modified: Some(entry.modified) })
    }

    fn read_dir(&self, _uri: &str) -> Option<Vec<Arc<str>>> {
        // Basic implementation: return all keys for now
        // A more complex one would handle directory hierarchy
        let files = self.files.read().unwrap();
        Some(files.keys().cloned().collect())
    }
}

impl WritableVfs for MemoryVfs {
    fn write_file(&self, uri: &str, content: Arc<str>) {
        self.write_file(uri, content);
    }

    fn remove_file(&self, uri: &str) {
        self.remove_file(uri);
    }
}
