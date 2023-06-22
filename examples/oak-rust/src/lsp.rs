use crate::{RustLanguage, parser::RustElementType};
use core::range::Range;
use dashmap::DashMap;
use futures::{Future, FutureExt};
use oak_core::{
    GreenNode, Source,
    language::{ElementType, TokenType},
    parser::{ParseCache, Parser, session::ParseSession},
    tree::RedNode,
};
use oak_hover::{Hover, HoverProvider};
use oak_lsp::service::LanguageService;
use oak_vfs::Vfs;

/// Hover provider implementation for Rust.
pub struct RustHoverProvider;

impl HoverProvider<RustLanguage> for RustHoverProvider {
    fn hover(&self, node: &RedNode<RustLanguage>, _range: Range<usize>) -> Option<Hover> {
        let kind = node.green.kind;

        // Provide context-aware hover information
        let contents = match kind {
            RustElementType::Function => "### Rust Function\nDefines a callable block of code.",
            RustElementType::StructItem => "### Rust Struct\nDefines a custom data type.",
            RustElementType::ModuleItem => "### Rust Module\nOrganizes code into namespaces.",
            RustElementType::Trait => "### Rust Trait\nDefines a shared behavior.",
            _ => return None,
        };

        Some(Hover { contents: contents.to_string(), range: Some(node.span()) })
    }
}

/// Language service implementation for Rust.
pub struct RustLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: RustHoverProvider,
    sessions: DashMap<String, Box<ParseSession<RustLanguage>>>,
}

impl<V: Vfs> RustLanguageService<V> {
    /// Creates a new `RustLanguageService`.
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: RustHoverProvider, sessions: DashMap::new() }
    }

    fn collect_definitions<S: Source + ?Sized>(&self, node: &RedNode<RustLanguage>, name: &str, source: &S, uri: &str, definitions: &mut Vec<oak_lsp::LocationRange>) {
        use oak_core::{
            language::{ElementRole, UniversalElementRole},
            tree::RedTree,
        };

        let role = node.green.kind.role();
        if role.universal() == UniversalElementRole::Definition {
            for child in node.children() {
                if let RedTree::Leaf(leaf) = child {
                    if leaf.kind.is_universal(oak_core::language::UniversalTokenRole::Name) {
                        let leaf_name = source.get_text_in(leaf.span.clone());
                        if leaf_name.as_ref() == name {
                            definitions.push(oak_lsp::LocationRange { uri: uri.to_string().into(), range: leaf.span });
                            return;
                        }
                    }
                }
            }
        }

        for child in node.children() {
            if let RedTree::Node(child_node) = child {
                self.collect_definitions(&child_node, name, source, uri, definitions);
            }
        }
    }
}

impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for RustLanguageService<V> {
    type Lang = RustLanguage;
    type Vfs = V;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, uri: &str) -> impl Future<Output = Option<RedNode<'_, RustLanguage>>> + Send + '_ {
        let uri = uri.to_string();
        async move {
            let source = self.vfs().get_source(&uri)?;
            let mut session = self.sessions.entry(uri.clone()).or_insert_with(|| Box::new(ParseSession::<RustLanguage>::default()));

            let language = RustLanguage::default();
            let parser = crate::parser::RustParser::new(&language);

            let session_guard = session.value_mut();
            let session_ptr: *mut ParseSession<RustLanguage> = session_guard.as_mut();

            let output = parser.parse(&source, &[], session_guard.as_mut());

            // Commit the generation so it can be retrieved via last_root
            let root_green = output.result.ok()?;
            unsafe {
                (*session_ptr).commit_generation(root_green);
            }

            // Safety: The root is stored in the ParseSession, which is inside a Box in the DashMap.
            // Since the DashMap is owned by self, and the Box provides stable addressing,
            // we can safely extend the lifetime to the lifetime of self.
            let root_ptr = unsafe {
                let ptr = (*session_ptr).old_tree().or_else(|| Some(root_green))?;
                std::mem::transmute::<&GreenNode<RustLanguage>, &GreenNode<RustLanguage>>(ptr)
            };

            Some(RedNode::new(root_ptr, 0))
        }
    }

    fn definition<'a>(&'a self, uri: &'a str, range: Range<usize>) -> impl Future<Output = Vec<oak_lsp::LocationRange>> + Send + 'a {
        let uri = uri.to_string();
        async move {
            let root = self.get_root(&uri).await?;
            let source = self.vfs().get_source(&uri)?;
            let leaf = root.leaf_at_offset(range.start)?;

            if !leaf.kind.is_universal(oak_core::language::UniversalTokenRole::Name) {
                return None;
            }

            use oak_core::Source;
            let name = source.get_text_in(leaf.span.clone());
            let name = name.as_ref();

            // Search for definitions in all files in the workspace
            let mut all_definitions = Vec::new();
            let files = self.list_all_files(&uri).await;

            for file_uri in files {
                if let Some(file_root) = self.get_root(&file_uri).await {
                    if let Some(file_source) = self.vfs().get_source(&file_uri) {
                        self.collect_definitions(&file_root, name, &file_source, &file_uri, &mut all_definitions);
                    }
                }
            }

            Some(all_definitions)
        }
        .then(|opt| async { opt.unwrap_or_default() })
    }

    fn references<'a>(&'a self, uri: &'a str, range: Range<usize>) -> impl Future<Output = Vec<oak_lsp::LocationRange>> + Send + 'a {
        let uri = uri.to_string();
        async move {
            let root = self.get_root(&uri).await?;
            let source = self.vfs().get_source(&uri)?;
            let leaf = root.leaf_at_offset(range.start)?;

            if !leaf.kind.is_universal(oak_core::language::UniversalTokenRole::Name) {
                return None;
            }

            use oak_core::Source;
            let name = source.get_text_in(leaf.span.clone());
            let name = name.to_string();

            // Search in all files in the workspace
            // Note: In a real LSP, we would use an index for performance
            let mut all_refs = Vec::new();

            // Get all workspace folders or just list files from root
            // For this example, we'll just search the current file and any other files we can find
            let files = self.list_all_files(&uri).await; // This is a bit hacky as it uses current file as root

            for file_uri in files {
                if let Some(file_root) = self.get_root(&file_uri).await {
                    if let Some(file_source) = self.vfs().get_source(&file_uri) {
                        let source_ref: &dyn oak_core::Source = &file_source;
                        // For SimpleReferenceFinder::find, we need the text source content as a string slice
                        // But SimpleReferenceFinder might expect different arguments based on the error
                        // "expected `&str`, found `&<V as Vfs>::Source`"
                        // This implies SimpleReferenceFinder::find expects `&str` for source.
                        // We need to get the full text from the source.
                        let source_len = source_ref.length();
                        let full_text = source_ref.get_text_in(oak_core::Range { start: 0, end: source_len });
                        let full_text_str = full_text.as_ref();

                        let refs = oak_navigation::SimpleReferenceFinder::find(&file_root, &name, full_text_str, file_uri.clone());
                        all_refs.extend(refs.into_iter().map(|l| oak_lsp::LocationRange { uri: l.uri, range: l.range }));
                    }
                }
            }

            Some(all_refs)
        }
        .then(|opt| async { opt.unwrap_or_default() })
    }

    fn rename<'a>(&'a self, uri: &'a str, range: Range<usize>, new_name: String) -> impl Future<Output = Option<oak_lsp::WorkspaceEdit>> + Send + 'a {
        let uri = uri.to_string();
        async move {
            let refs = self.references(&uri, range).await;
            if refs.is_empty() {
                return None;
            }

            let mut changes = std::collections::HashMap::new();
            for r in refs {
                changes.entry(r.uri.to_string()).or_insert_with(Vec::new).push(oak_lsp::TextEdit { range: r.range, new_text: new_name.clone() });
            }

            Some(oak_lsp::WorkspaceEdit { changes })
        }
    }

    fn hover(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Option<oak_lsp::Hover>> + Send + '_ {
        let uri = uri.to_string();
        async move {
            self.with_root(&uri, |root| {
                // In a real implementation, you would find the specific node at offset
                // For this example, we just check the root or simple children
                self.hover_provider.hover(&root, range).map(|h| oak_lsp::Hover { contents: h.contents, range: h.range })
            })
            .await
            .flatten()
        }
    }
}
