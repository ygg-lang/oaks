use crate::{OrgModeLanguage, kind::OrgModeSyntaxKind};
use core::range::Range;
use oak_core::tree::RedNode;
use oak_hover::{Hover as ProviderHover, HoverProvider};
use oak_lsp::{service::LanguageService, types::Hover as LspHover};
use oak_vfs::Vfs;

/// Hover provider implementation for Org-mode.
pub struct OrgModeHoverProvider;

impl HoverProvider<OrgModeLanguage> for OrgModeHoverProvider {
    fn hover(&self, node: &RedNode<OrgModeLanguage>, _range: Range<usize>) -> Option<ProviderHover> {
        let kind = node.green.kind;

        let contents = match kind {
            OrgModeSyntaxKind::Heading => "### Org Heading\nA hierarchical heading in the document.",
            OrgModeSyntaxKind::Todo => "### TODO Keyword\nAn actionable item.",
            OrgModeSyntaxKind::Block => "### Org Block\nA special block (e.g., code, quote).",
            _ => return None,
        };

        Some(ProviderHover { contents: contents.to_string(), range: Some(node.span()) })
    }
}

/// Language service implementation for Org-mode.
pub struct OrgModeLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: OrgModeHoverProvider,
}

impl<V: Vfs> OrgModeLanguageService<V> {
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: OrgModeHoverProvider }
    }
}

impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for OrgModeLanguageService<V> {
    type Lang = OrgModeLanguage;
    type Vfs = V;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, _uri: &str) -> impl std::future::Future<Output = Option<RedNode<'_, OrgModeLanguage>>> + Send + '_ {
        async move { None }
    }

    fn hover(&self, uri: &str, range: Range<usize>) -> impl std::future::Future<Output = Option<LspHover>> + Send + '_ {
        let uri = uri.to_string();
        async move { self.with_root(&uri, |root| self.hover_provider.hover(&root, range).map(|h| LspHover { contents: h.contents, range: h.range })).await.flatten() }
    }
}
