use crate::{SassLanguage, kind::SassSyntaxKind};
use core::range::Range;
use futures::Future;
use oak_core::tree::RedNode;
use oak_hover::{Hover, HoverProvider};
use oak_lsp::service::LanguageService;
use oak_vfs::Vfs;

/// Hover provider implementation for Sass.
pub struct SassHoverProvider;

impl HoverProvider<SassLanguage> for SassHoverProvider {
    fn hover(&self, node: &RedNode<SassLanguage>, _range: Range<usize>) -> Option<Hover> {
        let kind = node.green.kind;

        // Provide context-aware hover information
        let contents = match kind {
            SassSyntaxKind::Selector => "### Sass Selector\nSelects HTML elements to style.",
            SassSyntaxKind::Variable => "### Sass Variable\nStores a reusable CSS value.",
            SassSyntaxKind::Mixin => "### Sass Mixin\nDefines a reusable group of CSS declarations.",
            SassSyntaxKind::Function => "### Sass Function\nReturns a value based on arguments.",
            _ => return None,
        };

        Some(Hover { contents: contents.to_string(), range: Some(node.span()) })
    }
}

/// Language service implementation for Sass.
pub struct SassLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: SassHoverProvider,
}

impl<V: Vfs> SassLanguageService<V> {
    /// Creates a new `SassLanguageService`.
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: SassHoverProvider }
    }
}

impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for SassLanguageService<V> {
    type Lang = SassLanguage;
    type Vfs = V;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, uri: &str) -> impl Future<Output = Option<RedNode<'_, SassLanguage>>> + Send + '_ {
        let source = self.vfs().get_source(uri);
        async move {
            let source = source?;
            let language = SassLanguage::default();
            let parser = crate::parser::SassParser::new(&language);
            let lexer = crate::lexer::SassLexer::new(&language);

            // In a real implementation, you would use a cache
            let mut cache = oak_core::parser::session::ParseSession::<SassLanguage>::default();
            let _output = oak_core::parser::parse(&parser, &lexer, &source, &[], &mut cache);
            None
        }
    }

    fn hover(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Option<oak_lsp::Hover>> + Send + '_ {
        let uri = uri.to_string();
        async move { self.with_root(&uri, |root| self.hover_provider.hover(&root, range).map(|h| oak_lsp::Hover { contents: h.contents, range: h.range })).await.flatten() }
    }
}
