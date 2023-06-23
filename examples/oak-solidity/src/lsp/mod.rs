#![doc = include_str!("readme.md")]
#[cfg(feature = "oak-highlight")]
pub mod highlighter;

#[cfg(feature = "lsp")]
use {
    futures::Future,
    oak_hover::{Hover, HoverProvider},
    oak_lsp::service::LanguageService,
    oak_vfs::Vfs,
};
#[cfg(feature = "oak-pretty-print")]
pub mod formatter;
use crate::language::SolidityLanguage;
use core::range::Range;
use oak_core::tree::RedNode;
#[cfg(feature = "lsp")]
pub struct SolidityHoverProvider;
#[cfg(feature = "lsp")]
impl HoverProvider<SolidityLanguage> for SolidityHoverProvider {
    fn hover(&self, node: &RedNode<SolidityLanguage>, _range: Range<usize>) -> Option<Hover> {
        let _kind = node.green.kind;
        // Solidity specific kinds would go here
        None
    }
}
#[cfg(feature = "lsp")]
pub struct SolidityLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: SolidityHoverProvider,
}
impl<V: Vfs> SolidityLanguageService<V> {
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: SolidityHoverProvider }
    }
}
impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for SolidityLanguageService<V> {
    type Lang = SolidityLanguage;
    type Vfs = V;
    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }
    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }
    fn get_root(&self, uri: &str) -> impl Future<Output = Option<RedNode<'_, SolidityLanguage>>> + Send + '_ {
        let source = self.vfs().get_source(uri);
        async move {
            let source = source?;
            let language = SolidityLanguage::default();
            let parser = crate::parser::SolidityParser::new(&language);
            let lexer = crate::lexer::SolidityLexer::new(&language);
            let mut cache = oak_core::parser::session::ParseSession::<SolidityLanguage>::default();
            let _output = oak_core::parser::parse(&parser, &lexer, &source, &[], &mut cache);
            None
        }
    }
    fn hover(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Option<oak_lsp::Hover>> + Send + '_ {
        let uri = uri.to_string();
        async move { self.with_root(&uri, |root| self.hover_provider.hover(&root, range).map(|h| oak_lsp::Hover { contents: h.contents, range: h.range })).await.flatten() }
    }
}
