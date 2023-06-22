use crate::{kind::MarkdownSyntaxKind, language::MarkdownLanguage};
use core::range::Range;
use futures::Future;
use oak_core::tree::RedNode;
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

    fn get_root(&self, uri: &str) -> impl Future<Output = Option<RedNode<'_, MarkdownLanguage>>> + Send + '_ {
        let _source = self.get_source(uri);
        async move {
            // NOTE: Markdown parser is not yet implemented in this example.
            // In a real implementation, you would call the parser here.
            None
        }
    }

    fn hover(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Option<LspHover>> + Send + '_ {
        let uri = uri.to_string();
        async move { self.with_root(&uri, |root| self.hover_provider.hover(&root, range).map(|h| LspHover { contents: h.contents, range: h.range })).await.flatten() }
    }
}
