use crate::{MatlabLanguage, kind::MatlabSyntaxKind};
use core::range::Range;
use oak_core::tree::RedNode;
use oak_hover::HoverProvider;
use oak_lsp::service::LanguageService;
use oak_vfs::Vfs;
use std::future::Future;

pub use oak_hover::Hover;

/// Hover provider implementation for MATLAB.
pub struct MatlabHoverProvider;

impl HoverProvider<MatlabLanguage> for MatlabHoverProvider {
    fn hover(&self, node: &RedNode<MatlabLanguage>, _range: Range<usize>) -> Option<oak_hover::Hover> {
        let kind = node.green.kind;

        let contents = match kind {
            MatlabSyntaxKind::FunctionDef => "### MATLAB Function\nDefines a reusable function block.",
            MatlabSyntaxKind::ClassDef => "### MATLAB Class\nDefines a class for object-oriented programming.",
            MatlabSyntaxKind::Script => "### MATLAB Script\nA file containing a sequence of MATLAB commands.",
            _ => return None,
        };

        Some(oak_hover::Hover { contents: contents.to_string(), range: Some(node.span()) })
    }
}

/// Language service implementation for MATLAB.
pub struct MatlabLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: MatlabHoverProvider,
}

impl<V: Vfs> MatlabLanguageService<V> {
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: MatlabHoverProvider }
    }
}

impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for MatlabLanguageService<V> {
    type Lang = MatlabLanguage;
    type Vfs = V;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, uri: &str) -> impl Future<Output = Option<RedNode<'_, MatlabLanguage>>> + Send + '_ {
        let source = self.get_source(uri);
        async move {
            let source = source?;
            let language = MatlabLanguage::default();
            let lexer = crate::lexer::MatlabLexer::new(&language);
            let parser = crate::parser::MatlabParser::new(&language);

            let mut cache = Box::new(oak_core::parser::ParseSession::<MatlabLanguage>::default());
            let cache_ptr: *mut oak_core::parser::ParseSession<MatlabLanguage> = &mut *cache;

            let output = oak_core::parser::parse(&parser, &lexer, &source, &[], unsafe { &mut *cache_ptr }).result.ok()?;

            let _leaked_cache = Box::leak(cache);
            let green_static: &'static oak_core::GreenNode<MatlabLanguage> = unsafe { std::mem::transmute(output) };

            Some(RedNode::new(green_static, 0))
        }
    }

    fn hover(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Option<oak_lsp::types::Hover>> + Send + '_ {
        let uri = uri.to_string();
        async move {
            let hover = self.with_root(&uri, |root| self.hover_provider.hover(&root, range)).await.flatten()?;
            Some(oak_lsp::types::Hover { contents: hover.contents, range: hover.range })
        }
    }
}
