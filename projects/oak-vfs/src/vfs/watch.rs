use notify::{Event, RecursiveMode, Result, Watcher};
use std::path::PathBuf;

pub enum VfsEvent {
    Changed(String),
    Created(String),
    Removed(String),
}

pub trait VfsWatcher {
    fn watch(&mut self, uri: &str) -> Result<()>;
    fn unwatch(&mut self, uri: &str) -> Result<()>;
}

pub struct DiskWatcher {
    watcher: notify::RecommendedWatcher,
}

impl DiskWatcher {
    pub fn new<F>(mut callback: F) -> Result<Self>
    where
        F: FnMut(VfsEvent) + Send + 'static,
    {
        let watcher = notify::recommended_watcher(move |res: Result<Event>| {
            if let Ok(event) = res {
                for path in event.paths {
                    let uri = path.to_string_lossy().to_string();
                    if event.kind.is_modify() {
                        callback(VfsEvent::Changed(uri));
                    }
                    else if event.kind.is_create() {
                        callback(VfsEvent::Created(uri));
                    }
                    else if event.kind.is_remove() {
                        callback(VfsEvent::Removed(uri));
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
}
