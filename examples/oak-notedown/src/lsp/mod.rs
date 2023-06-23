#![doc = include_str!("readme.md")]
#[cfg(feature = "oak-highlight")]
pub mod highlighter;

use crate::{language::NotedownLanguage, lexer::token_type::NoteTokenType, parser::element_type::NoteElementType};
use core::range::Range;
use oak_core::tree::RedNode;
#[cfg(feature = "lsp")]
use {futures::Future, oak_hover::HoverProvider, oak_lsp::service::LanguageService, oak_lsp::types::Hover as LspHover, oak_vfs::Vfs};
/// Hover provider implementation for Notedown.
#[cfg(feature = "lsp")]
pub struct NoteHoverProvider;
#[cfg(feature = "lsp")]
impl HoverProvider<NotedownLanguage> for NoteHoverProvider {
    fn hover(&self, node: &RedNode<NotedownLanguage>, _range: Range<usize>) -> Option<oak_hover::Hover> {
        let kind = node.green.kind;
        let contents = match kind {
            NoteElementType::Token(t) if t == NoteTokenType::Heading1 => "### Heading 1\nTop-level heading.",
            NoteElementType::Token(t) if t == NoteTokenType::Heading2 => "### Heading 2\nSecond-level heading.",
            NoteElementType::Token(t) if t == NoteTokenType::CodeBlock => "### Code Block\nSyntax-highlighted code block.",
            NoteElementType::Token(t) if t == NoteTokenType::Table => "### Table\nMarkdown table structure.",
            _ => return None,
        };
        Some(oak_hover::Hover { contents: contents.to_string(), range: Some(node.span()) })
    }
}
/// Language service implementation for Notedown.
#[cfg(feature = "lsp")]
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
