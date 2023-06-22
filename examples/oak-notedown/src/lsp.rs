use crate::{kind::NoteSyntaxKind, language::NotedownLanguage};
use core::range::Range;
use oak_core::tree::RedNode;
use oak_hover::HoverProvider;
use oak_lsp::service::LanguageService;
use oak_vfs::Vfs;

/// Hover provider implementation for Notedown.
pub struct NoteHoverProvider;

impl HoverProvider<NotedownLanguage> for NoteHoverProvider {
    fn hover(&self, node: &RedNode<NotedownLanguage>, _range: Range<usize>) -> Option<oak_hover::Hover> {
        let kind = node.green.kind;

        let contents = match kind {
            NoteSyntaxKind::Heading1 => "### Heading 1\nTop-level heading.",
            NoteSyntaxKind::Heading2 => "### Heading 2\nSecond-level heading.",
            NoteSyntaxKind::CodeBlock => "### Code Block\nSyntax-highlighted code block.",
            NoteSyntaxKind::Table => "### Table\nMarkdown table structure.",
            _ => return None,
        };

        Some(oak_hover::Hover { contents: contents.to_string(), range: Some(node.span()) })
    }
}

/// Language service implementation for Notedown.
pub struct NoteLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: NoteHoverProvider,
}

impl<V: Vfs> NoteLanguageService<V> {
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: NoteHoverProvider }
    }
}

impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for NoteLanguageService<V> {
    type Lang = NotedownLanguage;
    type Vfs = V;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, _uri: &str) -> impl futures::Future<Output = Option<RedNode<'_, NotedownLanguage>>> + Send + '_ {
        async move { None }
    }

    fn hover(&self, uri: &str, range: Range<usize>) -> impl futures::Future<Output = Option<oak_lsp::types::Hover>> + Send + '_ {
        let uri = uri.to_string();
        async move { self.with_root(&uri, |root| self.hover_provider.hover(&root, range).map(|h| oak_lsp::types::Hover { contents: h.contents, range: h.range })).await.flatten() }
    }
}
