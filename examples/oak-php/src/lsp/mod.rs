#![doc = include_str!("readme.md")]
#[cfg(feature = "oak-highlight")]
pub mod highlighter;

use crate::{language::PhpLanguage, parser::element_type::PhpElementType};
use core::range::Range;
use oak_core::tree::RedNode;
#[cfg(feature = "lsp")]
use {
    futures::Future,
    oak_hover::{Hover as HoverInfo, HoverProvider},
    oak_lsp::{service::LanguageService, types::Hover},
    oak_vfs::{Vfs, WritableVfs},
};
/// Hover provider implementation for PHP.
#[cfg(feature = "lsp")]
pub struct PhpHoverProvider;
#[cfg(feature = "lsp")]
impl HoverProvider<PhpLanguage> for PhpHoverProvider {
    fn hover(&self, node: &RedNode<'_, PhpLanguage>, _range: Range<usize>) -> Option<HoverInfo> {
        let kind = node.green.kind;
        let contents = match kind {
            PhpElementType::ClassDef => "### PHP Class\nA template for objects, containing properties and methods.",
            PhpElementType::FunctionDef => "### PHP Function\nA block of code that can be repeatedly called.",
            PhpElementType::NamespaceDef => "### PHP Namespace\nA way of encapsulating items to avoid name collisions.",
            _ => return None,
        };
        Some(HoverInfo { contents: contents.to_string(), range: Some(node.span()) })
    }
}
/// PHP language service.
///
/// This struct provides LSP-related functionality for the PHP language.
#[cfg(feature = "lsp")]
pub struct PhpLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: PhpHoverProvider,
}
impl<V: Vfs> PhpLanguageService<V> {
    /// Creates a new `PhpLanguageService` with the given virtual file system.
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: PhpHoverProvider }
    }
}
impl<V: WritableVfs + Send + Sync + 'static> LanguageService for PhpLanguageService<V> {
    type Lang = PhpLanguage;
    type Vfs = V;
    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }
    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }
    fn get_root(&self, uri: &str) -> impl Future<Output = Option<RedNode<'_, PhpLanguage>>> + Send + '_ {
        let source = self.get_source(uri);
        async move {
            let _source = source?;
            // TODO: Implement proper caching and conversion to RedNode
            None
        }
    }
    fn hover(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Option<Hover>> + Send + '_ {
        let uri = uri.to_string();
        async move {
            let hover = self.with_root(&uri, |root| self.hover_provider.hover(&root, range)).await.flatten()?;
            Some(Hover { contents: hover.contents, range: hover.range })
        }
    }
    fn completion(&self, _uri: &str, _offset: usize) -> impl Future<Output = Vec<oak_lsp::types::CompletionItem>> + Send + '_ {
        async move { vec![] }
    }
}
