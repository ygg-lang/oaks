#![doc = include_str!("readme.md")]
use crate::YamlLanguage;
use oak_core::tree::RedNode;
/// Highlighter module.
#[cfg(feature = "oak-highlight")]
pub mod highlighter;

#[cfg(feature = "lsp")]
use {oak_lsp::service::LanguageService, oak_vfs::Vfs, std::future::Future};
/// MCP module.
/// Formatter module.
#[cfg(feature = "lsp")]
#[cfg(feature = "oak-pretty-print")]
pub mod formatter;
/// Language service implementation for YAML.
#[cfg(feature = "lsp")]
pub struct YamlLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
}
#[cfg(feature = "lsp")]
impl<V: Vfs> YamlLanguageService<V> {
    /// Creates a new `YamlLanguageService`.
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default() }
    }
}
#[cfg(feature = "lsp")]
impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for YamlLanguageService<V> {
    type Lang = YamlLanguage;
    type Vfs = V;
    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }
    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }
    fn get_root(&self, uri: &str) -> impl Future<Output = Option<RedNode<'_, YamlLanguage>>> + Send + '_ {
        let source = self.vfs().get_source(uri);
        async move {
            let source = source?;
            let language = YamlLanguage::default();
            let parser = crate::parser::YamlParser::new(&language);
            let lexer = crate::lexer::YamlLexer::new(&language);
            let mut cache = oak_core::parser::session::ParseSession::<YamlLanguage>::new(1024);
            let _output = oak_core::parser::parse(&parser, &lexer, &source, &[], &mut cache);
            None
        }
    }
}
