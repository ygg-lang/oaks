use crate::{JsonLanguage, kind::JsonSyntaxKind};
use core::range::Range;
use dashmap::DashMap;
use futures::Future;
use oak_core::{ParseCache, TokenType, parser::session::ParseSession, source::Source, tree::RedNode};
use oak_hover::{Hover, HoverProvider};
use oak_lsp::service::LanguageService;
use oak_vfs::Vfs;

/// Hover provider implementation for JSON.
pub struct JsonHoverProvider;

impl HoverProvider<JsonLanguage> for JsonHoverProvider {
    fn hover(&self, node: &RedNode<JsonLanguage>, _range: Range<usize>) -> Option<Hover> {
        let kind = node.green.kind;

        // Provide context-aware hover information
        let contents = match kind {
            JsonSyntaxKind::Object => "### JSON Object\nA collection of key-value pairs.",
            JsonSyntaxKind::Array => "### JSON Array\nAn ordered list of values.",
            JsonSyntaxKind::ObjectEntry => "### JSON Property\nA key-value pair in an object.",
            JsonSyntaxKind::StringLiteral => "### JSON String\nA sequence of Unicode characters.",
            JsonSyntaxKind::NumberLiteral => "### JSON Number\nA numeric value.",
            JsonSyntaxKind::BooleanLiteral => "### JSON Boolean\nA true or false value.",
            JsonSyntaxKind::NullLiteral => "### JSON Null\nRepresents the intentional absence of any value.",
            _ => return None,
        };

        Some(Hover { contents: contents.to_string(), range: Some(node.span()) })
    }
}

/// Language service implementation for JSON.
pub struct JsonLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: JsonHoverProvider,
    sessions: DashMap<String, Box<ParseSession<JsonLanguage>>>,
}

impl<V: Vfs> JsonLanguageService<V> {
    /// Creates a new `JsonLanguageService`.
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: JsonHoverProvider, sessions: DashMap::new() }
    }

    fn collect_definitions(&self, node: &RedNode<JsonLanguage>, name: &str, source: &V::Source, uri: &str, definitions: &mut Vec<oak_lsp::LocationRange>) {
        use oak_core::{
            language::{ElementType, UniversalElementRole, UniversalTokenRole},
            tree::RedTree,
        };

        // In JSON, every ObjectEntry can be considered a definition if its key matches
        if ElementType::is_universal(&node.green.kind, UniversalElementRole::Statement) && node.green.kind == JsonSyntaxKind::ObjectEntry {
            for child in node.children() {
                if let RedTree::Leaf(leaf) = child {
                    // Keys are Name (if BareKey) or Literal (if StringLiteral)
                    // But for navigation, we treat the key as the identifier
                    if TokenType::is_universal(&leaf.kind, UniversalTokenRole::Name) || leaf.kind == JsonSyntaxKind::StringLiteral {
                        let text = source.get_text_in(leaf.span.clone());
                        // Strip quotes for string literals
                        let key_name = if leaf.kind == JsonSyntaxKind::StringLiteral { text.trim_matches('"') } else { &text };

                        if key_name == name {
                            definitions.push(oak_lsp::LocationRange { uri: uri.to_string(), range: leaf.span });
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

impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for JsonLanguageService<V> {
    type Lang = JsonLanguage;
    type Vfs = V;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, uri: &str) -> impl Future<Output = Option<RedNode<'_, JsonLanguage>>> + Send + '_ {
        let uri = uri.to_string();
        async move {
            let source = self.vfs().get_source(&uri)?;
            let mut session_entry = self.sessions.entry(uri.clone()).or_insert_with(|| Box::new(ParseSession::<JsonLanguage>::default()));
            let session = session_entry.as_mut();

            let language = JsonLanguage::default();
            let parser = crate::parser::JsonParser::new(&language);
            let lexer = crate::lexer::JsonLexer::new(&language);

            let tree = {
                let output = oak_core::parser::parse(&parser, &lexer, &source, &[], session);
                let tree_ref = output.result.as_ref().ok()?;
                // Safety: The tree is allocated in session's arena.
                // We cast away local lifetimes to return a reference that can be returned from this block.
                // The tree actually lives as long as the session in self.sessions.
                unsafe { &*(*tree_ref as *const oak_core::GreenNode<JsonLanguage> as *const oak_core::GreenNode<'static, JsonLanguage>) }
            };
            session.commit_generation(tree);

            Some(RedNode::new(tree, 0))
        }
    }

    fn definition<'a>(&'a self, uri: &'a str, range: Range<usize>) -> impl Future<Output = Vec<oak_lsp::LocationRange>> + Send + 'a {
        let uri = uri.to_string();
        async move {
            let Some(root) = self.get_root(&uri).await
            else {
                return vec![];
            };
            let Some(source) = self.vfs().get_source(&uri)
            else {
                return vec![];
            };
            let Some(leaf) = root.leaf_at_offset(range.start)
            else {
                return vec![];
            };

            let text = source.get_text_in(leaf.span.clone());
            let name = text.trim_matches('"');

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

            all_definitions
        }
    }

    fn references<'a>(&'a self, uri: &'a str, range: Range<usize>) -> impl Future<Output = Vec<oak_lsp::LocationRange>> + Send + 'a {
        let uri = uri.to_string();
        async move {
            let Some(root) = self.get_root(&uri).await
            else {
                return vec![];
            };
            let Some(source) = self.vfs().get_source(&uri)
            else {
                return vec![];
            };
            let Some(leaf) = root.leaf_at_offset(range.start)
            else {
                return vec![];
            };

            let text = source.get_text_in(leaf.span.clone());
            let name = text.trim_matches('"');

            // Search for references in all files in the workspace
            let mut all_refs = Vec::new();
            let files = self.list_all_files(&uri).await;

            for file_uri in files {
                if let Some(file_root) = self.get_root(&file_uri).await {
                    if let Some(file_source) = self.vfs().get_source(&file_uri) {
                        // In JSON, we use collect_definitions as a proxy for finding key references
                        self.collect_definitions(&file_root, name, &file_source, &file_uri, &mut all_refs);
                    }
                }
            }

            all_refs
        }
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
                // Ensure new name is quoted if the original was quoted
                let Some(source) = self.vfs().get_source(&r.uri)
                else {
                    continue;
                };
                let old_text = source.get_text_in(r.range.clone());
                let formatted_new_name = if old_text.starts_with('"') { format!("\"{}\"", new_name) } else { new_name.clone() };

                changes.entry(r.uri.clone()).or_insert_with(Vec::new).push(oak_lsp::TextEdit { range: r.range, new_text: formatted_new_name });
            }

            Some(oak_lsp::WorkspaceEdit { changes })
        }
    }

    fn hover(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Option<oak_lsp::Hover>> + Send + '_ {
        let uri = uri.to_string();
        async move { self.with_root(&uri, |root| self.hover_provider.hover(&root, range).map(|h| oak_lsp::Hover { contents: h.contents, range: h.range })).await.flatten() }
    }
}
