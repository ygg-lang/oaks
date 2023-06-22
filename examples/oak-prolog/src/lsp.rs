use crate::{kind::PrologSyntaxKind, language::PrologLanguage};
use core::range::Range;
use oak_core::tree::RedNode;
use oak_hover::{Hover, HoverProvider};
use oak_lsp::service::LanguageService;
use oak_vfs::Vfs;
use std::future::Future;

/// Hover provider implementation for Prolog.
pub struct PrologHoverProvider;

impl HoverProvider<PrologLanguage> for PrologHoverProvider {
    fn hover(&self, node: &RedNode<'_, PrologLanguage>, _range: Range<usize>) -> Option<Hover> {
        let kind = node.green.kind;

        let contents = match kind {
            PrologSyntaxKind::Clause => "### Prolog Clause\nA statement that is either a fact or a rule.",
            PrologSyntaxKind::Directive => "### Prolog Directive\nA command to the Prolog system.",
            PrologSyntaxKind::Query => "### Prolog Query\nA goal to be proven by the Prolog system.",
            _ => return None,
        };

        Some(Hover { contents: contents.to_string(), range: Some(node.span()) })
    }
}

/// Language service implementation for Prolog.
pub struct PrologLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: PrologHoverProvider,
}

impl<V: Vfs> PrologLanguageService<V> {
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: PrologHoverProvider }
    }
}

impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for PrologLanguageService<V> {
    type Lang = PrologLanguage;
    type Vfs = V;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, _uri: &str) -> impl Future<Output = Option<RedNode<'_, PrologLanguage>>> + Send + '_ {
        async move { None }
    }

    fn hover(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Option<oak_lsp::Hover>> + Send + '_ {
        let uri = uri.to_string();
        async move {
            let hover = self.with_root(&uri, |root| self.hover_provider.hover(&root, range)).await.flatten()?;
            Some(oak_lsp::Hover { contents: hover.contents, range: hover.range })
        }
    }
}
