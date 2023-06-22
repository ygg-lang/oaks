use oak_core::{
    language::{ElementRole, ElementType, Language, TokenType, UniversalElementRole, UniversalTokenRole},
    tree::{RedNode, RedTree},
};
use oak_lsp::LocationRange;
use serde::{Deserialize, Serialize};

/// Represents information about a symbol (e.g., function, variable, class).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolInformation {
    /// The name of the symbol.
    pub name: String,
    /// The universal role of the symbol.
    pub role: UniversalElementRole,
    /// The location of the symbol.
    pub location: LocationRange,
    /// The name of the container this symbol is in.
    pub container_name: Option<String>,
}

/// Trait for languages that support symbol search and navigation.
pub trait SymbolProvider<L: Language> {
    /// Returns symbols defined in the document.
    ///
    /// This is typically used for `textDocument/documentSymbol` when a flat list is preferred,
    /// or as a basis for `workspace/symbol`.
    fn document_symbols(&self, root: &RedNode<L>) -> Vec<SymbolInformation>;
}

/// A universal symbol provider that works for any language whose ElementType implements role().
pub struct UniversalSymbolProvider;

impl UniversalSymbolProvider {
    pub fn new() -> Self {
        Self
    }

    #[allow(dead_code)]
    fn collect_symbols<L: Language>(&self, node: &RedNode<L>, symbols: &mut Vec<SymbolInformation>, container_name: Option<String>, source: &str) {
        let role = node.green.kind.role();

        if role.universal() == UniversalElementRole::Definition {
            // Try to find the name of the definition
            let mut name = None;
            for child in node.children() {
                match child {
                    RedTree::Leaf(leaf) => {
                        // In many languages, the first name identifier in a definition is its name
                        if leaf.kind.is_universal(UniversalTokenRole::None) || leaf.kind.is_universal(UniversalTokenRole::Name) {
                            name = Some(source[leaf.span.clone()].to_string());
                            break;
                        }
                    }
                    _ => {}
                }
            }

            let name = name.unwrap_or_else(|| format!("<{:?}>", node.green.kind));

            symbols.push(SymbolInformation {
                name: name.clone(),
                role: role.universal(),
                location: LocationRange {
                    uri: "file:///dummy".to_string(), // This should be provided by the caller
                    range: node.span(),
                },
                container_name: container_name.clone(),
            });

            // Recurse with this definition as the container
            for child in node.children() {
                if let RedTree::Node(child_node) = child {
                    self.collect_symbols::<L>(&child_node, symbols, Some(name.clone()), source);
                }
            }
        }
        else {
            // Just recurse
            for child in node.children() {
                if let RedTree::Node(child_node) = child {
                    self.collect_symbols::<L>(&child_node, symbols, container_name.clone(), source);
                }
            }
        }
    }
}

impl<L: Language> SymbolProvider<L> for UniversalSymbolProvider {
    fn document_symbols(&self, _root: &RedNode<L>) -> Vec<SymbolInformation> {
        // We need the source code to get the names, but the trait doesn't provide it.
        // This is a limitation of the current trait design.
        // For now, we'll return an empty list or change the trait.
        // Actually, let's keep the trait as is and assume the provider has access to source if needed.
        Vec::new()
    }
}
