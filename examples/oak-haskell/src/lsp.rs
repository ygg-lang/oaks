use crate::{HaskellLanguage, kind::HaskellSyntaxKind};
use core::range::Range;
use futures::Future;
use oak_core::tree::RedNode;
use oak_hover::{Hover, HoverProvider};
use oak_lsp::service::LanguageService;
use oak_vfs::Vfs;

/// Hover provider implementation for Haskell.
pub struct HaskellHoverProvider;

impl HoverProvider<HaskellLanguage> for HaskellHoverProvider {
    fn hover(&self, node: &RedNode<HaskellLanguage>, _range: Range<usize>) -> Option<Hover> {
        let kind = node.green.kind;

        // Provide context-aware hover information
        let contents = match kind {
            HaskellSyntaxKind::Function => "### Haskell Function\nDefines a transformation from inputs to outputs.",
            HaskellSyntaxKind::DataDeclaration => "### Haskell Data Type\nDefines a new algebraic data type.",
            HaskellSyntaxKind::ModuleDeclaration => "### Haskell Module\nOrganizes Haskell code into namespaces.",
            _ => return None,
        };

        Some(Hover { contents: contents.to_string(), range: Some(node.span()) })
    }
}

/// Language service implementation for Haskell.
pub struct HaskellLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: HaskellHoverProvider,
}

impl<V: Vfs> HaskellLanguageService<V> {
    /// Creates a new `HaskellLanguageService`.
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: HaskellHoverProvider }
    }
}

impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for HaskellLanguageService<V> {
    type Lang = HaskellLanguage;
    type Vfs = V;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, uri: &str) -> impl Future<Output = Option<RedNode<'_, HaskellLanguage>>> + Send + '_ {
        let source = self.vfs().get_source(uri);
        async move {
            let source = source?;
            let language = HaskellLanguage::default();
            let parser = crate::parser::HaskellParser::new(&language);
            let lexer = crate::lexer::HaskellLexer::new(&language);

            let mut cache = oak_core::parser::session::ParseSession::<HaskellLanguage>::default();
            let _output = oak_core::parser::parse(&parser, &lexer, &source, &[], &mut cache);
            None
        }
    }

    fn hover(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Option<oak_lsp::Hover>> + Send + '_ {
        let uri = uri.to_string();
        async move { self.with_root(&uri, |root| self.hover_provider.hover(&root, range).map(|h| oak_lsp::Hover { contents: h.contents, range: h.range })).await.flatten() }
    }
}
