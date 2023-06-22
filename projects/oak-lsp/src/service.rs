use crate::types::{CompletionItem, Diagnostic, FoldingRange, Hover, InitializeParams, LocationRange, StructureItem, WorkspaceEdit, WorkspaceSymbol};
use core::range::Range;
use oak_core::{language::Language, source::Source, tree::RedNode};
use oak_vfs::{Vfs, WritableVfs};
use std::future::Future;

pub trait LanguageService: Send + Sync {
    type Lang: Language;
    type Vfs: WritableVfs;

    /// Get the underlying VFS.
    fn vfs(&self) -> &Self::Vfs;

    /// Get the workspace manager.
    fn workspace(&self) -> &crate::workspace::WorkspaceManager;

    /// Helper to get source from VFS.
    fn get_source(&self, uri: &str) -> Option<Box<dyn Source + Send + Sync>> {
        self.vfs().get_source(uri).map(|s| Box::new(s) as Box<dyn Source + Send + Sync>)
    }

    /// Helper to get the root red node of a file.
    /// Implementations should override this to provide actual parsing/caching.
    fn get_root(&self, _uri: &str) -> impl Future<Output = Option<RedNode<'_, Self::Lang>>> + Send + '_ {
        async { None }
    }

    /// Helper to run logic with the root node.
    fn with_root<'a, R, F>(&'a self, uri: &'a str, f: F) -> impl Future<Output = Option<R>> + Send + 'a
    where
        R: Send,
        F: FnOnce(RedNode<'a, Self::Lang>) -> R + Send + 'a,
    {
        async move {
            let root = self.get_root(uri).await?;
            Some(f(root))
        }
    }

    /// Helper to run logic with multiple root nodes in parallel.
    fn with_roots<'a, R, F>(&'a self, uris: Vec<String>, f: F) -> impl Future<Output = Vec<R>> + Send + 'a
    where
        R: Send + 'static,
        F: Fn(RedNode<'a, Self::Lang>) -> R + Send + Sync + 'a,
    {
        let mut futures = Vec::new();
        let f = std::sync::Arc::new(f);

        for uri in uris {
            let f = f.clone();
            futures.push(async move { if let Some(root) = self.get_root(&uri).await { Some(f(root)) } else { None } });
        }

        async move { futures::future::join_all(futures).await.into_iter().flatten().collect() }
    }

    /// Provide hover information. Defaults to None.
    fn hover(&self, _uri: &str, _range: Range<usize>) -> impl Future<Output = Option<Hover>> + Send + '_ {
        async { None }
    }

    /// Provide folding ranges. Defaults to empty.
    fn folding_ranges(&self, _uri: &str) -> impl Future<Output = Vec<FoldingRange>> + Send + '_ {
        async { vec![] }
    }

    /// Provide document symbols. Defaults to empty.
    fn document_symbols(&self, _uri: &str) -> impl Future<Output = Vec<StructureItem>> + Send + '_ {
        async { vec![] }
    }

    /// Search for symbols in the entire workspace.
    fn workspace_symbols(&self, _query: &str) -> impl Future<Output = Vec<WorkspaceSymbol>> + Send + '_ {
        async { vec![] }
    }

    /// Helper to list all files recursively.
    fn list_all_files(&self, root_uri: &str) -> impl Future<Output = Vec<String>> + Send + '_ {
        let root_uri = root_uri.to_string();
        async move {
            let mut files = Vec::new();
            let mut stack = vec![root_uri];

            while let Some(uri) = stack.pop() {
                if self.vfs().is_file(&uri) {
                    files.push(uri);
                }
                else if self.vfs().is_dir(&uri) {
                    if let Some(entries) = self.vfs().read_dir(&uri) {
                        for entry in entries {
                            stack.push(entry);
                        }
                    }
                }
            }
            files
        }
    }

    /// Find definition. Defaults to empty.
    fn definition<'a>(&'a self, _uri: &'a str, _range: Range<usize>) -> impl Future<Output = Vec<LocationRange>> + Send + 'a {
        async { vec![] }
    }

    /// Find references. Defaults to empty.
    fn references<'a>(&'a self, _uri: &'a str, _range: Range<usize>) -> impl Future<Output = Vec<LocationRange>> + Send + 'a {
        async { vec![] }
    }

    /// Rename a symbol.
    fn rename<'a>(&'a self, _uri: &'a str, _range: Range<usize>, _new_name: String) -> impl Future<Output = Option<WorkspaceEdit>> + Send + 'a {
        async { None }
    }

    /// Provide completion items. Defaults to empty.
    fn completion<'a>(&'a self, _uri: &'a str, _position: usize) -> impl Future<Output = Vec<CompletionItem>> + Send + 'a {
        async { vec![] }
    }

    /// Provide diagnostics for a file. Defaults to empty.
    fn diagnostics<'a>(&'a self, _uri: &'a str) -> impl Future<Output = Vec<Diagnostic>> + Send + 'a {
        async { vec![] }
    }

    /// Called when the language server is initialized.
    fn initialize<'a>(&'a self, _params: InitializeParams) -> impl Future<Output = ()> + Send + 'a {
        async {}
    }

    /// Called when the language server is initialized (notification).
    fn initialized<'a>(&'a self) -> impl Future<Output = ()> + Send + 'a {
        async {}
    }

    /// Called when the language server is shut down.
    fn shutdown<'a>(&'a self) -> impl Future<Output = ()> + Send + 'a {
        async {}
    }

    /// Called when a file is saved.
    fn did_save<'a>(&'a self, _uri: &'a str) -> impl Future<Output = ()> + Send + 'a {
        async {}
    }

    /// Called when a file is closed.
    fn did_close<'a>(&'a self, _uri: &'a str) -> impl Future<Output = ()> + Send + 'a {
        async {}
    }
}

/// Extension trait for `LanguageService` providing additional functionality like Axum integration.
pub trait LanguageServiceExt: LanguageService {
    /// Converts the language service into an Axum router.
    #[cfg(feature = "axum")]
    fn into_axum_router(self) -> axum::Router
    where
        Self: Sized + 'static,
    {
        crate::server::axum_router(std::sync::Arc::new(self))
    }
}

impl<T: LanguageService> LanguageServiceExt for T {}
