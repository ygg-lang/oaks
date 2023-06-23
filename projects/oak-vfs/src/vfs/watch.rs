use notify::{Event, RecursiveMode, Result, Watcher};
use std::path::PathBuf;

/// Events emitted by a VFS watcher.
pub enum VfsEvent {
    /// A file or directory was modified.
    Changed(String),
    /// A file or directory was created.
    Created(String),
    /// A file or directory was removed.
    Removed(String),
}

/// A trait for watching file system changes.
pub trait VfsWatcher {
    /// Starts watching the given URI.
    fn watch(&mut self, uri: &str) -> Result<()>;
    /// Stops watching the given URI.
    fn unwatch(&mut self, uri: &str) -> Result<()>;
    /// Stops watching all URIs.
    fn unwatch_all(&mut self) -> Result<()>;
}

/// A watcher implementation that watches files on disk.
pub struct DiskWatcher {
    watcher: notify::RecommendedWatcher,
}

impl DiskWatcher {
    /// Creates a new `DiskWatcher` with the given callback.
    pub fn new<F>(mut callback: F) -> Result<Self>
    where
        F: FnMut(VfsEvent) + Send + 'static,
    {
        let watcher = notify::recommended_watcher(move |res: Result<Event>| {
            if let Ok(event) = res {
                for path in event.paths {
                    let uri = path.to_string_lossy().to_string();
                    if event.kind.is_modify() {
                        callback(VfsEvent::Changed(uri))
                    }
                    else if event.kind.is_create() {
                        callback(VfsEvent::Created(uri))
                    }
                    else if event.kind.is_remove() {
                        callback(VfsEvent::Removed(uri))
                    }
                }
            }
        })?;
        Ok(Self { watcher })
    }
}

impl VfsWatcher for DiskWatcher {
    fn watch(&mut self, uri: &str) -> Result<()> {
        self.watcher.watch(PathBuf::from(uri).as_path(), RecursiveMode::Recursive)
    }

    fn unwatch(&mut self, uri: &str) -> Result<()> {
        self.watcher.unwatch(PathBuf::from(uri).as_path())
    }

    fn unwatch_all(&mut self) -> Result<()> {
        Ok(())
    }
}
