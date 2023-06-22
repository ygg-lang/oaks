use dashmap::DashMap;
use oak_symbols::SymbolInformation;
use std::path::PathBuf;
use url::Url;

use std::sync::RwLock;

/// Trait for resolving module imports to file URIs.
pub trait ModuleResolver: Send + Sync {
    /// Resolve an import path relative to a base URI.
    fn resolve(&self, base_uri: &str, import_path: &str) -> Option<String>;
}

/// A standard resolver that handles relative and absolute file paths.
pub struct StandardResolver {
    root_dirs: RwLock<Vec<PathBuf>>,
}

impl StandardResolver {
    pub fn new(root_dirs: Vec<PathBuf>) -> Self {
        Self { root_dirs: RwLock::new(root_dirs) }
    }

    /// Update the root directories used for resolution.
    pub fn set_root_dirs(&self, root_dirs: Vec<PathBuf>) {
        if let Ok(mut dirs) = self.root_dirs.write() {
            *dirs = root_dirs;
        }
    }
}

impl ModuleResolver for StandardResolver {
    fn resolve(&self, base_uri: &str, import_path: &str) -> Option<String> {
        let root_dirs = self.root_dirs.read().ok()?;

        if let Ok(base_url) = Url::parse(base_uri) {
            if let Ok(base_path) = base_url.to_file_path() {
                let base_dir = base_path.parent()?;
                let resolved_path = base_dir.join(import_path);

                if resolved_path.exists() {
                    return Url::from_file_path(resolved_path).ok().map(|u| u.to_string());
                }
            }
        }

        // Try root directories (like PYTHONPATH or node_modules logic)
        for root in root_dirs.iter() {
            let resolved_path = root.join(import_path);
            if resolved_path.exists() {
                return Url::from_file_path(resolved_path).ok().map(|u| u.to_string());
            }
        }

        None
    }
}

/// Global symbol table that stores symbols across the entire workspace.
pub struct GlobalSymbolTable {
    /// Map of URI to symbols defined in that file.
    file_symbols: DashMap<String, Vec<SymbolInformation>>,
    /// Map of fully qualified name to symbol information.
    qualified_symbols: DashMap<String, SymbolInformation>,
}

impl GlobalSymbolTable {
    pub fn new() -> Self {
        Self { file_symbols: DashMap::new(), qualified_symbols: DashMap::new() }
    }

    /// Add or update symbols for a file.
    pub fn update_file_symbols(&self, uri: String, symbols: Vec<SymbolInformation>) {
        // Remove old qualified symbols for this file
        if let Some((_, old_symbols)) = self.file_symbols.remove(&uri) {
            for sym in old_symbols {
                let fqn = self.make_qualified_name(&sym);
                self.qualified_symbols.remove(&fqn);
            }
        }

        // Add new symbols
        for sym in &symbols {
            let fqn = self.make_qualified_name(sym);
            self.qualified_symbols.insert(fqn, sym.clone());
        }
        self.file_symbols.insert(uri, symbols);
    }

    fn make_qualified_name(&self, sym: &SymbolInformation) -> String {
        match &sym.container_name {
            Some(container) => format!("{}::{}", container, sym.name),
            None => sym.name.clone(),
        }
    }

    /// Lookup a symbol by its fully qualified name.
    pub fn lookup(&self, fqn: &str) -> Option<SymbolInformation> {
        self.qualified_symbols.get(fqn).map(|r| r.value().clone())
    }

    /// Get all symbols defined in a specific file.
    pub fn query_file(&self, uri: &str) -> Vec<SymbolInformation> {
        self.file_symbols.get(uri).map(|r| r.value().clone()).unwrap_or_default()
    }

    /// Find all symbols matching a query (for workspace/symbol).
    pub fn query(&self, query: &str) -> Vec<SymbolInformation> {
        self.qualified_symbols.iter().filter(|r| r.key().contains(query)).map(|r| r.value().clone()).collect()
    }
}

impl Default for GlobalSymbolTable {
    fn default() -> Self {
        Self::new()
    }
}
