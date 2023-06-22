use crate::{FileMetadata, FileType, Vfs, WritableVfs};
use oak_core::source::SourceText;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

/// A memory-based Virtual File System implementation.
/// Ideal for WASM environments or testing where physical disk access is not available.
#[derive(Default, Clone)]
pub struct MemoryVfs {
    files: Arc<RwLock<HashMap<String, FileEntry>>>,
}

struct FileEntry {
    content: String,
    modified: u64,
}

impl MemoryVfs {
    /// Create a new empty MemoryVfs.
    pub fn new() -> Self {
        Self::default()
    }

    /// Upsert a file's content in the memory VFS.
    pub fn write_file(&self, uri: &str, content: String) {
        let mut files = self.files.write().unwrap();
        files.insert(uri.to_string(), FileEntry { content, modified: Self::now() });
    }

    /// Remove a file from the memory VFS.
    pub fn remove_file(&self, uri: &str) {
        let mut files = self.files.write().unwrap();
        files.remove(uri);
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
        files.get(uri).map(|entry| SourceText::new(entry.content.clone()))
    }

    fn exists(&self, uri: &str) -> bool {
        self.files.read().unwrap().contains_key(uri)
    }

    fn metadata(&self, uri: &str) -> Option<FileMetadata> {
        let files = self.files.read().unwrap();
        files.get(uri).map(|entry| FileMetadata { file_type: FileType::File, len: entry.content.len() as u64, modified: Some(entry.modified) })
    }

    fn read_dir(&self, _uri: &str) -> Option<Vec<String>> {
        // Basic implementation: return all keys for now
        // A more complex one would handle directory hierarchy
        let files = self.files.read().unwrap();
        Some(files.keys().cloned().collect())
    }
}

impl WritableVfs for MemoryVfs {
    fn write_file(&self, uri: &str, content: String) {
        self.write_file(uri, content);
    }

    fn remove_file(&self, uri: &str) {
        self.remove_file(uri);
    }
}
