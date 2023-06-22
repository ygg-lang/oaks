use crate::{kind::PowerShellSyntaxKind, language::PowerShellLanguage};
use core::range::Range;
use oak_core::tree::RedNode;
use oak_hover::{Hover as ProviderHover, HoverProvider};
use oak_lsp::{service::LanguageService, types::Hover as LspHover};
use oak_vfs::Vfs;

/// Hover provider implementation for PowerShell.
pub struct PowerShellHoverProvider;

impl HoverProvider<PowerShellLanguage> for PowerShellHoverProvider {
    fn hover(&self, node: &RedNode<PowerShellLanguage>, _range: Range<usize>) -> Option<ProviderHover> {
        let kind = node.green.kind;

        let contents = match kind {
            PowerShellSyntaxKind::FunctionDef => "### PowerShell Function\nA named block of code that performs a specific task.",
            PowerShellSyntaxKind::ClassDef => "### PowerShell Class\nA blueprint for creating objects in PowerShell.",
            PowerShellSyntaxKind::IfStatement => "### PowerShell If Statement\nA conditional statement that executes a block of code if a condition is true.",
            _ => return None,
        };

        Some(ProviderHover { contents: contents.to_string(), range: Some(node.span()) })
    }
}

/// Language service implementation for PowerShell.
pub struct PowerShellLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: PowerShellHoverProvider,
}

impl<V: Vfs> PowerShellLanguageService<V> {
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: PowerShellHoverProvider }
    }
}

impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for PowerShellLanguageService<V> {
    type Lang = PowerShellLanguage;
    type Vfs = V;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, uri: &str) -> impl futures::Future<Output = Option<RedNode<'_, PowerShellLanguage>>> + Send + '_ {
        let _source = self.get_source(uri);
        async move {
            // TODO: Implement proper caching of parsed trees in LanguageService
            None
        }
    }

    fn hover(&self, uri: &str, range: Range<usize>) -> impl futures::Future<Output = Option<LspHover>> + Send + '_ {
        let uri = uri.to_string();
        async move { self.with_root(&uri, |root| self.hover_provider.hover(&root, range).map(|h| LspHover { contents: h.contents, range: h.range })).await.flatten() }
    }
}
