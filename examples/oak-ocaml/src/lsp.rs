use crate::{OCamlLanguage, kind::OCamlSyntaxKind};
use core::range::Range;
use oak_core::tree::RedNode;
use oak_hover::HoverProvider;
use oak_lsp::{service::LanguageService, types::Hover};
use oak_vfs::Vfs;

/// Hover provider implementation for OCaml.
pub struct OCamlHoverProvider;

impl HoverProvider<OCamlLanguage> for OCamlHoverProvider {
    fn hover(&self, node: &RedNode<OCamlLanguage>, _range: Range<usize>) -> Option<oak_hover::Hover> {
        let kind = node.green.kind;

        let contents = match kind {
            OCamlSyntaxKind::LetBinding => "### OCaml Let Binding\nDefines a value or function binding.",
            OCamlSyntaxKind::ModuleDef => "### OCaml Module\nDefines an OCaml module.",
            OCamlSyntaxKind::TypeDefinition => "### OCaml Type\nDefines a new type.",
            OCamlSyntaxKind::MatchExpr => "### OCaml Match\nPattern matching expression.",
            _ => return None,
        };

        Some(oak_hover::Hover { contents: contents.to_string(), range: Some(node.span()) })
    }
}

/// Language service implementation for OCaml.
pub struct OCamlLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: OCamlHoverProvider,
}

impl<V: Vfs> OCamlLanguageService<V> {
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: OCamlHoverProvider }
    }
}

impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for OCamlLanguageService<V> {
    type Lang = OCamlLanguage;
    type Vfs = V;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, _uri: &str) -> impl std::future::Future<Output = Option<RedNode<'_, OCamlLanguage>>> + Send + '_ {
        async move { None }
    }

    fn hover(&self, uri: &str, range: Range<usize>) -> impl std::future::Future<Output = Option<Hover>> + Send + '_ {
        let uri = uri.to_string();
        async move { self.with_root(&uri, |root| self.hover_provider.hover(&root, range).map(|h| Hover { contents: h.contents, range: h.range })).await.flatten() }
    }
}
