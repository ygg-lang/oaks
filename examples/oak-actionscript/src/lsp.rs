use crate::{ActionScriptLanguage, parser::ActionScriptElementType};
use core::range::Range;
use futures::Future;
use oak_core::tree::RedNode;
use oak_hover::{Hover, HoverProvider};
use oak_lsp::service::LanguageService;
use oak_vfs::Vfs;

/// Hover provider implementation for ActionScript.
pub struct ActionScriptHoverProvider;

impl HoverProvider<ActionScriptLanguage> for ActionScriptHoverProvider {
    fn hover(&self, node: &RedNode<ActionScriptLanguage>, _range: Range<usize>) -> Option<Hover> {
        let kind = node.green.kind;

        let contents = match kind {
            ActionScriptElementType::Class => "### ActionScript Class\nDefines a blueprint for objects.",
            ActionScriptElementType::Interface => "### ActionScript Interface\nDefines a contract for classes.",
            ActionScriptElementType::Function => "### ActionScript Function\nDefines a block of code that performs a task.",
            _ => return None,
        };

        Some(Hover { contents: contents.to_string(), range: Some(node.span()) })
    }
}

/// Language service implementation for ActionScript.
pub struct ActionScriptLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: ActionScriptHoverProvider,
}

impl<V: Vfs> ActionScriptLanguageService<V> {
    /// Creates a new `ActionScriptLanguageService`.
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: ActionScriptHoverProvider }
    }
}

impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for ActionScriptLanguageService<V> {
    type Lang = ActionScriptLanguage;
    type Vfs = V;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, uri: &str) -> impl Future<Output = Option<RedNode<'_, ActionScriptLanguage>>> + Send + '_ {
        let source = self.vfs().get_source(uri);
        async move {
            let _source = source?;
            // Implementation similar to Rust
            None
        }
    }

    fn hover(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Option<oak_lsp::Hover>> + Send + '_ {
        let uri = uri.to_string();
        async move { self.with_root(&uri, |root| self.hover_provider.hover(&root, range).map(|h| oak_lsp::Hover { contents: h.contents, range: h.range })).await.flatten() }
    }
}
