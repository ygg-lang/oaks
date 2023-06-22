use crate::{kind::RubySyntaxKind, language::RubyLanguage};
use core::range::Range;
use futures::Future;
use oak_core::tree::RedNode;
use oak_hover::{Hover, HoverProvider};
use oak_lsp::service::LanguageService;
use oak_vfs::Vfs;

/// Hover provider implementation for Ruby.
pub struct RubyHoverProvider;

impl HoverProvider<RubyLanguage> for RubyHoverProvider {
    fn hover(&self, node: &RedNode<RubyLanguage>, _range: Range<usize>) -> Option<Hover> {
        let kind = node.green.kind;

        // Provide context-aware hover information
        let contents = match kind {
            RubySyntaxKind::MethodDefinition => "### Ruby Method\nDefines a callable block of code.",
            RubySyntaxKind::ClassDefinition => "### Ruby Class\nDefines a blueprint for objects.",
            _ => return None,
        };

        Some(Hover { contents: contents.to_string(), range: Some(node.span()) })
    }
}

/// Language service implementation for Ruby.
pub struct RubyLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: RubyHoverProvider,
}

impl<V: Vfs> RubyLanguageService<V> {
    /// Creates a new `RubyLanguageService`.
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: RubyHoverProvider }
    }
}

impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for RubyLanguageService<V> {
    type Lang = RubyLanguage;
    type Vfs = V;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, uri: &str) -> impl Future<Output = Option<RedNode<'_, RubyLanguage>>> + Send + '_ {
        let source = self.vfs().get_source(uri);
        async move {
            let source = source?;
            let language = RubyLanguage::default();
            let parser = crate::parser::RubyParser::new(&language);
            let lexer = crate::lexer::RubyLexer::new(&language);

            let mut cache = oak_core::parser::session::ParseSession::<RubyLanguage>::default();
            let _output = oak_core::parser::parse(&parser, &lexer, &source, &[], &mut cache);
            None
        }
    }

    fn hover(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Option<oak_lsp::Hover>> + Send + '_ {
        let uri = uri.to_string();
        async move { self.with_root(&uri, |root| self.hover_provider.hover(&root, range).map(|h| oak_lsp::Hover { contents: h.contents, range: h.range })).await.flatten() }
    }
}
