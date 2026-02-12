#![doc = include_str!("readme.md")]
//! Raku language service implementation

pub mod formatter;
#[cfg(feature = "oak-highlight")]
pub mod highlighter;

use crate::{RakuLanguage, parser::element_type::RakuElementType};
use core::range::Range;
use oak_core::tree::RedNode;
#[cfg(feature = "lsp")]
use {
    futures::Future,
    oak_hover::{Hover, HoverProvider},
    oak_lsp::service::LanguageService,
    oak_vfs::Vfs,
};

/// Raku hover provider implementation.
#[cfg(feature = "lsp")]
pub struct RakuHoverProvider;

#[cfg(feature = "lsp")]
impl HoverProvider<RakuLanguage> for RakuHoverProvider {
    fn hover(&self, node: &RedNode<RakuLanguage>, _range: Range<usize>) -> Option<Hover> {
        let kind = node.green.kind;
        let contents = match kind {
            RakuElementType::Statement => "### Raku Statement\nA single execution unit in Raku.",
            RakuElementType::Expression => "### Raku Expression\nA piece of code that evaluates to a value.",
            _ => return None,
        };

        Some(Hover { contents: contents.to_string(), range: Some(node.span()) })
    }
}

/// Raku language service implementation.
#[cfg(feature = "lsp")]
pub struct RakuLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: RakuHoverProvider,
}

#[cfg(feature = "lsp")]
impl<V: Vfs> RakuLanguageService<V> {
    /// Creates a new `RakuLanguageService` with the given VFS.
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: RakuHoverProvider }
    }
}

#[cfg(feature = "lsp")]
impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for RakuLanguageService<V> {
    type Lang = RakuLanguage;
    type Vfs = V;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, _uri: &str) -> impl Future<Output = Option<RedNode<'_, RakuLanguage>>> + Send + '_ {
        async move { None }
    }

    fn hover(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Option<oak_lsp::Hover>> + Send + '_ {
        let uri = uri.to_string();
        async move { self.with_root(&uri, |root| self.hover_provider.hover(&root, range).map(|h| oak_lsp::Hover { contents: h.contents, range: h.range })).await.flatten() }
    }
}
