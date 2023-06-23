#![doc = include_str!("readme.md")]
#[cfg(feature = "oak-highlight")]
pub mod highlighter;

#[cfg(feature = "lsp")]
use {
    oak_hover::{Hover as ProviderHover, HoverProvider},
    oak_lsp::{service::LanguageService, types::Hover as LspHover},
    oak_vfs::Vfs,
};

#[cfg(feature = "oak-pretty-print")]
pub mod formatter;
use crate::{language::TomlLanguage, lexer::token_type::TomlTokenKind};
use core::range::Range;
use oak_core::tree::RedNode;
/// Hover provider for TOML.
#[cfg(feature = "lsp")]
pub struct TomlHoverProvider;
#[cfg(feature = "lsp")]
impl HoverProvider<TomlLanguage> for TomlHoverProvider {
    fn hover(&self, node: &RedNode<'_, TomlLanguage>, _range: Range<usize>) -> Option<ProviderHover> {
        let kind = node.green.kind;
        let contents = match kind {
            TomlTokenKind::BareKey | TomlTokenKind::Identifier => "### TOML Key\nConfiguration key identifier.",
            TomlTokenKind::LeftBracket => "### TOML Table\nDefines a section of configuration.",
            TomlTokenKind::DoubleLeftBracket => "### TOML Array of Tables\nDefines a collection of table sections.",
            _ => return None,
        };
        Some(ProviderHover { contents: contents.to_string(), range: Some(node.span()) })
    }
}
#[cfg(feature = "lsp")]
pub struct TomlLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: TomlHoverProvider,
}
impl<V: Vfs> TomlLanguageService<V> {
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: TomlHoverProvider }
    }
}
impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for TomlLanguageService<V> {
    type Lang = TomlLanguage;
    type Vfs = V;
    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }
    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }
    fn get_root(&self, uri: &str) -> impl futures::Future<Output = Option<RedNode<'_, TomlLanguage>>> + Send + '_ {
        let _source = self.get_source(uri);
        async move {
            // TODO: Implement proper caching of parsed trees in LanguageService
            // For now return None to fix compilation errors
            None
        }
    }
    fn hover(&self, uri: &str, range: Range<usize>) -> impl futures::Future<Output = Option<LspHover>> + Send + '_ {
        let uri = uri.to_string();
        async move {
            self.with_root(&uri, |root| {
                use oak_core::tree::RedTree;
                let node = match root.child_at_offset(range.start) {
                    Some(RedTree::Node(n)) => n,
                    _ => root,
                };
                self.hover_provider.hover(&node, range).map(|h| LspHover { contents: h.contents, range: h.range })
            })
            .await
            .flatten()
        }
    }
}
