#![doc = include_str!("readme.md")]
#[cfg(feature = "oak-highlight")]
pub mod highlighter;

use crate::{ObjectiveCLanguage, parser::element_type::ObjectiveCElementType};
use core::range::Range;
use oak_core::tree::RedNode;
#[cfg(feature = "lsp")]
use {
    oak_hover::HoverProvider,
    oak_lsp::{service::LanguageService, types::Hover},
    oak_vfs::Vfs,
    std::future::Future,
};
/// Hover provider implementation for Objective-C.
#[cfg(feature = "lsp")]
pub struct ObjectiveCHoverProvider;
#[cfg(feature = "lsp")]
impl HoverProvider<ObjectiveCLanguage> for ObjectiveCHoverProvider {
    fn hover(&self, node: &RedNode<'_, ObjectiveCLanguage>, _range: Range<usize>) -> Option<oak_hover::Hover> {
        let kind = node.green.kind;
        let contents = match kind {
            ObjectiveCElementType::InterfaceDeclaration => "### Objective-C @interface\nDefines the interface for a class.",
            ObjectiveCElementType::ImplementationDeclaration => "### Objective-C @implementation\nProvides the implementation for a class.",
            ObjectiveCElementType::ProtocolDeclaration => "### Objective-C @protocol\nDefines a set of methods that a class can implement.",
            ObjectiveCElementType::PropertyDeclaration => "### Objective-C @property\nDeclares a property for a class.",
            _ => return None,
        };
        Some(oak_hover::Hover { contents: contents.to_string(), range: Some(node.span()) })
    }
}
/// Language service implementation for Objective-C.
#[cfg(feature = "lsp")]
pub struct ObjectiveCLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: ObjectiveCHoverProvider,
}
impl<V: Vfs> ObjectiveCLanguageService<V> {
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: ObjectiveCHoverProvider }
    }
}
impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for ObjectiveCLanguageService<V> {
    type Lang = ObjectiveCLanguage;
    type Vfs = V;
    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }
    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }
    fn get_root(&self, uri: &str) -> impl Future<Output = Option<RedNode<'_, ObjectiveCLanguage>>> + Send + '_ {
        let source = self.get_source(uri);
        async move {
            let _source = source?;
            // In a real implementation, you would parse and cache the root properly
            None
        }
    }
    fn hover(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Option<Hover>> + Send + '_ {
        let uri = uri.to_string();
        async move {
            let hover = self.with_root(&uri, |root| self.hover_provider.hover(&root, range)).await.flatten()?;
            Some(Hover { contents: hover.contents, range: hover.range })
        }
    }
}
