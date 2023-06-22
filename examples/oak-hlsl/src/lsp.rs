use crate::{HlslLanguage, kind::HlslSyntaxKind};
use core::range::Range;
use futures::Future;
use oak_core::tree::RedNode;
use oak_hover::{Hover, HoverProvider};
use oak_lsp::service::LanguageService;
use oak_vfs::Vfs;

/// Hover provider implementation for HLSL.
pub struct HlslHoverProvider;

impl HoverProvider<HlslLanguage> for HlslHoverProvider {
    fn hover(&self, node: &RedNode<HlslLanguage>, _range: Range<usize>) -> Option<Hover> {
        let kind = node.green.kind;

        // Provide context-aware hover information
        let contents = match kind {
            HlslSyntaxKind::Struct => "### HLSL Struct\nDefines a custom data structure.",
            HlslSyntaxKind::Cbuffer => "### HLSL Constant Buffer\nDefines a buffer for shader constants.",
            HlslSyntaxKind::Technique => "### HLSL Technique\nDefines a rendering technique.",
            HlslSyntaxKind::Pass => "### HLSL Pass\nDefines a rendering pass within a technique.",
            _ => return None,
        };

        Some(Hover { contents: contents.to_string(), range: Some(node.span()) })
    }
}

/// Language service implementation for HLSL.
pub struct HlslLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: HlslHoverProvider,
}

impl<V: Vfs> HlslLanguageService<V> {
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: HlslHoverProvider }
    }
}

impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for HlslLanguageService<V> {
    type Lang = HlslLanguage;
    type Vfs = V;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, uri: &str) -> impl Future<Output = Option<RedNode<'_, HlslLanguage>>> + Send + '_ {
        let _uri = uri.to_string();
        async move {
            // Placeholder implementation
            None
        }
    }

    fn hover(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Option<oak_lsp::Hover>> + Send + '_ {
        let uri = uri.to_string();
        async move { self.with_root(&uri, |root| self.hover_provider.hover(&root, range).map(|h| oak_lsp::Hover { contents: h.contents, range: h.range })).await.flatten() }
    }
}
