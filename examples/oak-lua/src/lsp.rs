use crate::language::LuaLanguage;
use core::range::Range;
use futures::Future;
use oak_core::tree::RedNode;
use oak_hover::{Hover, HoverProvider};
use oak_lsp::service::LanguageService;
use oak_vfs::Vfs;

/// Hover provider implementation for Lua.
pub struct LuaHoverProvider;

impl HoverProvider<LuaLanguage> for LuaHoverProvider {
    fn hover(&self, node: &RedNode<LuaLanguage>, _range: Range<usize>) -> Option<Hover> {
        let kind = node.green.kind;

        // Provide context-aware hover information
        // This is a simplified example
        let contents = match format!("{:?}", kind).as_str() {
            "Function" => "### Lua Function\nA block of code that performs a specific task.",
            "LocalDeclaration" => "### Local Variable\nA variable with local scope.",
            _ => return None,
        };

        Some(Hover { contents: contents.to_string(), range: Some(node.span()) })
    }
}

/// Language service implementation for Lua.
pub struct LuaLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: LuaHoverProvider,
}

impl<V: Vfs> LuaLanguageService<V> {
    /// Creates a new `LuaLanguageService`.
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: LuaHoverProvider }
    }
}

impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for LuaLanguageService<V> {
    type Lang = LuaLanguage;
    type Vfs = V;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, uri: &str) -> impl Future<Output = Option<RedNode<'_, LuaLanguage>>> + Send + '_ {
        let source = self.vfs().get_source(uri);
        async move {
            let source = source?;
            let language = LuaLanguage::default();
            let parser = crate::parser::LuaParser::new(&language);
            let lexer = crate::lexer::LuaLexer::new(&language);

            let mut cache = oak_core::parser::session::ParseSession::<LuaLanguage>::default();
            let _output = oak_core::parser::parse(&parser, &lexer, &source, &[], &mut cache);
            None
        }
    }

    fn hover(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Option<oak_lsp::Hover>> + Send + '_ {
        let uri = uri.to_string();
        async move { self.with_root(&uri, |root| self.hover_provider.hover(&root, range).map(|h| oak_lsp::Hover { contents: h.contents, range: h.range })).await.flatten() }
    }
}
