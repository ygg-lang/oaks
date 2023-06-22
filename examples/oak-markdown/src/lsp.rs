use crate::{kind::MarkdownSyntaxKind, language::MarkdownLanguage};
use core::range::Range;
use futures::{Future, FutureExt};
use oak_core::{ElementRole, ElementType, Source, TokenType, tree::RedNode};
use oak_hover::{Hover as HoverInfo, HoverProvider};
use oak_lsp::{service::LanguageService, types::Hover as LspHover};
use oak_vfs::Vfs;

/// Hover provider for Markdown.
pub struct MarkdownHoverProvider;

impl HoverProvider<MarkdownLanguage> for MarkdownHoverProvider {
    fn hover(&self, node: &RedNode<'_, MarkdownLanguage>, _range: Range<usize>) -> Option<HoverInfo> {
        let kind = node.green.kind;

        let contents = match kind {
            MarkdownSyntaxKind::Heading1 => "### Heading 1\nTop-level heading.",
            MarkdownSyntaxKind::Heading2 => "### Heading 2\nSecond-level heading.",
            MarkdownSyntaxKind::Link => "### Markdown Link\nLink to an external resource or internal anchor.",
            MarkdownSyntaxKind::CodeBlock => "### Code Block\nA block of source code.",
            MarkdownSyntaxKind::Strong => "### Strong Emphasis\nBold text.",
            MarkdownSyntaxKind::Emphasis => "### Emphasis\nItalic text.",
            _ => return None,
        };

        Some(HoverInfo { contents: contents.to_string(), range: Some(node.span()) })
    }
}

pub struct MarkdownLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: MarkdownHoverProvider,
}

impl<V: Vfs> MarkdownLanguageService<V> {
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::new(), hover_provider: MarkdownHoverProvider }
    }

    fn collect_definitions(&self, node: &RedNode<MarkdownLanguage>, name: &str, source: &str, uri: &str, definitions: &mut Vec<oak_lsp::LocationRange>) {
        use oak_core::{language::UniversalElementRole, tree::RedTree};

        let role = ElementType::role(&node.green.kind);
        if ElementRole::universal(&role) == UniversalElementRole::Definition {
            for child in node.children() {
                if let RedTree::Leaf(leaf) = child {
                    if TokenType::is_universal(&leaf.kind, oak_core::language::UniversalTokenRole::Name) {
                        if &source[leaf.span.clone()] == name {
                            definitions.push(oak_lsp::LocationRange { uri: uri.into(), range: leaf.span });
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

impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for MarkdownLanguageService<V> {
    type Lang = MarkdownLanguage;
    type Vfs = V;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, _uri: &str) -> impl Future<Output = Option<RedNode<'_, MarkdownLanguage>>> + Send + '_ {
        async move { None }
    }

    fn definition<'a>(&'a self, uri: &'a str, range: Range<usize>) -> impl Future<Output = Vec<oak_lsp::LocationRange>> + Send + 'a {
        let uri = uri.to_string();
        async move {
            let root = self.get_root(&uri).await?;
            let source = self.vfs().get_source(&uri)?;
            let leaf = root.leaf_at_offset(range.start)?;

            if !TokenType::is_universal(&leaf.kind, oak_core::language::UniversalTokenRole::Name) {
                return None;
            }

            let name = source.get_text_in(leaf.span.clone());
            let mut definitions = Vec::new();
            let full_text = source.get_text_in(Range { start: 0, end: source.length() });
            self.collect_definitions(&root, &name, &full_text, &uri, &mut definitions);
            Some(definitions)
        }
        .map(|opt| opt.unwrap_or_default())
    }

    fn references<'a>(&'a self, uri: &'a str, range: Range<usize>) -> impl Future<Output = Vec<oak_lsp::LocationRange>> + Send + 'a {
        let uri = uri.to_string();
        async move {
            let root = self.get_root(&uri).await?;
            let source = self.vfs().get_source(&uri)?;
            let leaf = root.leaf_at_offset(range.start)?;

            if !TokenType::is_universal(&leaf.kind, oak_core::language::UniversalTokenRole::Name) {
                return None;
            }

            let name = source.get_text_in(leaf.span.clone());
            let name = name.to_string();

            let mut all_refs = Vec::new();
            let files = self.list_all_files(&uri).await;

            for file_uri in files {
                if let Some(file_root) = self.get_root(&file_uri).await {
                    if let Some(file_source) = self.vfs().get_source(&file_uri) {
                        let full_text = file_source.get_text_in(Range { start: 0, end: file_source.length() });
                        let refs = oak_navigation::SimpleReferenceFinder::find(&file_root, &name, &full_text, file_uri.clone());
                        all_refs.extend(refs.into_iter().map(|l| oak_lsp::LocationRange { uri: l.uri, range: l.range }));
                    }
                }
            }

            Some(all_refs)
        }
        .map(|opt| opt.unwrap_or_default())
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

    fn hover(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Option<LspHover>> + Send + '_ {
        let uri = uri.to_string();
        async move { self.with_root(&uri, |root| self.hover_provider.hover(&root, range).map(|h| LspHover { contents: h.contents, range: h.range })).await.flatten() }
    }
}
