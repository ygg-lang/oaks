use crate::{PerlLanguage, kind::PerlSyntaxKind};
use core::range::Range;
use oak_core::tree::RedNode;
use oak_hover::{Hover as ProviderHover, HoverProvider};
use oak_lsp::{service::LanguageService, types::Hover as LspHover};
use oak_vfs::Vfs;

/// Hover provider implementation for Perl.
pub struct PerlHoverProvider;

impl HoverProvider<PerlLanguage> for PerlHoverProvider {
    fn hover(&self, node: &RedNode<PerlLanguage>, _range: Range<usize>) -> Option<ProviderHover> {
        let kind = node.green.kind;

        let contents = match kind {
            PerlSyntaxKind::SubroutineDeclaration => "### Perl Subroutine\nA named block of code that can be called.",
            PerlSyntaxKind::PackageDeclaration => "### Perl Package\nDefines a namespace for the code.",
            PerlSyntaxKind::VariableDeclaration => "### Perl Variable\nA variable declared with `my`, `our`, or `local`.",
            PerlSyntaxKind::IfStatement => "### If Statement\nConditional execution block.",
            _ => return None,
        };

        Some(ProviderHover { contents: contents.to_string(), range: Some(node.span()) })
    }
}

/// Language service implementation for Perl.
pub struct PerlLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: PerlHoverProvider,
}

impl<V: Vfs> PerlLanguageService<V> {
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: PerlHoverProvider }
    }
}

impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for PerlLanguageService<V> {
    type Lang = PerlLanguage;
    type Vfs = V;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, uri: &str) -> impl futures::Future<Output = Option<RedNode<'_, PerlLanguage>>> + Send + '_ {
        let _source = self.get_source(uri);
        async move {
            // TODO: Implement proper caching of parsed trees in LanguageService
            None
        }
    }

    fn hover(&self, uri: &str, range: Range<usize>) -> impl futures::Future<Output = Option<LspHover>> + Send + '_ {
        let uri = uri.to_string();
        async move { self.with_root(&uri, |root| self.hover_provider.hover(&root, range).map(|h| LspHover { contents: h.contents, range: h.range })).await.flatten() }
    }
}
