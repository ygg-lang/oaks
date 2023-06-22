use crate::types::{CodeAction, CompletionItem, Diagnostic, DocumentHighlight, FoldingRange, Hover, InitializeParams, InlayHint, LocationRange, SelectionRange, SemanticTokens, SignatureHelp, StructureItem, TextEdit, WorkspaceEdit, WorkspaceSymbol};
use core::range::Range;
use oak_core::{
    language::{ElementRole, ElementType, Language},
    source::Source,
    tree::RedNode,
};
use oak_resolver::ModuleResolver;
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
    fn get_source(&self, uri: &str) -> Option<<Self::Vfs as Vfs>::Source> {
        self.vfs().get_source(uri)
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
    fn document_symbols<'a>(&'a self, uri: &'a str) -> impl Future<Output = Vec<StructureItem>> + Send + 'a {
        let uri = uri.to_string();
        async move {
            let _source = match self.get_source(&uri) {
                Some(s) => s,
                None => return vec![],
            };
            let _root = match self.get_root(&uri).await {
                Some(r) => r,
                None => return vec![],
            };
            let symbols = self.workspace().symbols.query_file(&uri);
            if !symbols.is_empty() {
                return symbols.into_iter().map(StructureItem::from).collect();
            }
            vec![]
        }
    }

    /// Provide workspace symbols.
    fn workspace_symbols<'a>(&'a self, query: String) -> impl Future<Output = Vec<WorkspaceSymbol>> + Send + 'a {
        async move { self.workspace().symbols.query(&query).into_iter().map(|s| WorkspaceSymbol::from(s)).collect() }
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
                            stack.push(entry.to_string());
                        }
                    }
                }
            }
            files
        }
    }

    /// Find definition. Defaults to empty.
    fn definition<'a>(&'a self, uri: &'a str, range: Range<usize>) -> impl Future<Output = Vec<LocationRange>> + Send + 'a {
        let uri = uri.to_string();
        async move {
            let root = match self.get_root(&uri).await {
                Some(r) => r,
                None => return vec![],
            };
            let source = match self.get_source(&uri) {
                Some(s) => s,
                None => return vec![],
            };

            // 1. Identify token at range
            use oak_core::tree::RedTree;
            let node = match root.child_at_offset(range.start) {
                Some(RedTree::Node(n)) => n,
                Some(RedTree::Leaf(l)) => return vec![LocationRange { uri: uri.clone().into(), range: l.span }],
                None => root,
            };

            // 2. If it's a reference, try to resolve it
            let role = node.green.kind.role();
            if role.universal() == oak_core::language::UniversalElementRole::Reference {
                let name = &source.get_text_in(node.span());

                // Try local symbols first (not implemented here, should be done by lang-specific logic)

                // Try global symbols
                if let Some(sym) = self.workspace().symbols.lookup(name) {
                    return vec![LocationRange { uri: sym.uri, range: sym.range }];
                }

                // Try as a module import
                if let Some(resolved_uri) = self.workspace().resolver.resolve(&uri, name) {
                    return vec![LocationRange { uri: resolved_uri.into(), range: (0..0).into() }];
                }
            }

            vec![]
        }
    }

    /// Find references. Defaults to empty.
    fn references<'a>(&'a self, _uri: &'a str, _range: Range<usize>) -> impl Future<Output = Vec<LocationRange>> + Send + 'a {
        async { vec![] }
    }

    /// Find type definition. Defaults to empty.
    fn type_definition<'a>(&'a self, _uri: &'a str, _range: Range<usize>) -> impl Future<Output = Vec<LocationRange>> + Send + 'a {
        async { vec![] }
    }

    /// Find implementation. Defaults to empty.
    fn implementation<'a>(&'a self, _uri: &'a str, _range: Range<usize>) -> impl Future<Output = Vec<LocationRange>> + Send + 'a {
        async { vec![] }
    }

    /// Provide document highlights. Defaults to empty.
    fn document_highlights<'a>(&'a self, _uri: &'a str, _range: Range<usize>) -> impl Future<Output = Vec<DocumentHighlight>> + Send + 'a {
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

    /// Provide semantic tokens for a file. Defaults to None.
    fn semantic_tokens<'a>(&'a self, _uri: &'a str) -> impl Future<Output = Option<SemanticTokens>> + Send + 'a {
        async { None }
    }

    /// Provide semantic tokens for a range. Defaults to None.
    fn semantic_tokens_range<'a>(&'a self, _uri: &'a str, _range: Range<usize>) -> impl Future<Output = Option<SemanticTokens>> + Send + 'a {
        async { None }
    }

    /// Provide selection ranges for a file. Defaults to empty.
    fn selection_ranges<'a>(&'a self, _uri: &'a str, _ranges: Vec<usize>) -> impl Future<Output = Vec<SelectionRange>> + Send + 'a {
        async { vec![] }
    }

    /// Provide signature help at a position. Defaults to None.
    fn signature_help<'a>(&'a self, _uri: &'a str, _position: usize) -> impl Future<Output = Option<SignatureHelp>> + Send + 'a {
        async { None }
    }

    /// Provide inlay hints for a file. Defaults to empty.
    fn inlay_hints<'a>(&'a self, _uri: &'a str) -> impl Future<Output = Vec<InlayHint>> + Send + 'a {
        async { vec![] }
    }

    /// Provide document formatting. Defaults to empty.
    fn formatting<'a>(&'a self, _uri: &'a str) -> impl Future<Output = Vec<TextEdit>> + Send + 'a {
        async { vec![] }
    }

    /// Provide code actions for a file. Defaults to empty.
    fn code_actions<'a>(&'a self, _uri: &'a str, _range: Range<usize>) -> impl Future<Output = Vec<CodeAction>> + Send + 'a {
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
