use crate::types::InitializeParams;
use dashmap::DashMap;
use std::path::PathBuf;
use url::Url;

/// A manager for workspace folders and path resolution.
pub struct WorkspaceManager {
    folders: DashMap<String, PathBuf>,
}

impl WorkspaceManager {
    pub fn new() -> Self {
        Self { folders: DashMap::new() }
    }

    /// Initialize the workspace manager with parameters from the client.
    pub fn initialize(&self, params: &InitializeParams) {
        if let Some(uri_str) = &params.root_uri {
            if let Ok(uri) = Url::parse(uri_str) {
                if let Ok(path) = uri.to_file_path() {
                    self.folders.insert(uri_str.clone(), path);
                }
            }
        }

        for folder in &params.workspace_folders {
            if let Ok(uri) = Url::parse(&folder.uri) {
                if let Ok(path) = uri.to_file_path() {
                    self.folders.insert(folder.uri.clone(), path);
                }
            }
        }
    }

    /// Add a workspace folder.
    pub fn add_folder(&self, uri: String, _name: String) {
        if let Ok(url) = Url::parse(&uri) {
            if let Ok(path) = url.to_file_path() {
                self.folders.insert(uri, path);
            }
        }
    }

    /// Remove a workspace folder.
    pub fn remove_folder(&self, uri: &str) {
        self.folders.remove(uri);
    }

    /// Get the local path for a URI if it's within a workspace folder.
    pub fn get_path(&self, uri: &str) -> Option<PathBuf> {
        if let Ok(url) = Url::parse(uri) {
            if let Ok(path) = url.to_file_path() {
                return Some(path);
            }
        }
        None
    }

    /// Convert a local path to a file URI.
    pub fn path_to_uri(&self, path: PathBuf) -> Option<String> {
        Url::from_file_path(path).ok().map(|url| url.to_string())
    }

    /// Find the workspace folder URI that contains the given URI.
    pub fn find_root(&self, uri: &str) -> Option<String> {
        let path = self.get_path(uri)?;
        let mut best_match: Option<(String, usize)> = None;

        for entry in self.folders.iter() {
            let folder_uri = entry.key();
            let folder_path = entry.value();

            if path.starts_with(folder_path) {
                let len = folder_path.as_os_str().len();
                if best_match.as_ref().map_or(true, |(_, best_len)| len > *best_len) {
                    best_match = Some((folder_uri.clone(), len));
                }
            }
        }

        best_match.map(|(uri, _)| uri)
    }

    /// Check if a URI is within any workspace folder.
    pub fn is_within_workspace(&self, uri: &str) -> bool {
        self.find_root(uri).is_some()
    }

    /// List all workspace folders.
    pub fn list_folders(&self) -> Vec<(String, PathBuf)> {
        self.folders.iter().map(|entry| (entry.key().clone(), entry.value().clone())).collect()
    }
}

impl Default for WorkspaceManager {
    fn default() -> Self {
        Self::new()
    }
}
