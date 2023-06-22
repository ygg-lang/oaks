#![feature(new_range_api)]
use oak_core::{
    Arc, Range,
    language::{ElementRole, ElementType, Language, TokenType, UniversalElementRole, UniversalTokenRole},
    source::Source,
    tree::{RedNode, RedTree},
};
use serde::{Deserialize, Serialize};

/// Represents information about a symbol (e.g., function, variable, class).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolInformation {
    /// The name of the symbol.
    pub name: String,
    /// The universal role of the symbol.
    pub role: UniversalElementRole,
    /// The URI of the resource.
    #[serde(with = "oak_core::serde_arc_str")]
    pub uri: Arc<str>,
    /// The byte range within the resource.
    #[serde(with = "oak_core::serde_range")]
    pub range: Range<usize>,
    /// The name of the container this symbol is in.
    pub container_name: Option<String>,
}

/// Trait for languages that support symbol search and navigation.
pub trait SymbolProvider<L: Language> {
    /// Returns symbols defined in the document.
    fn document_symbols<S: Source + ?Sized>(&self, uri: &str, root: &RedNode<L>, source: &S) -> Vec<SymbolInformation>;
}

/// A universal symbol provider that works for any language whose ElementType implements role().
pub struct UniversalSymbolProvider;

impl UniversalSymbolProvider {
    pub fn new() -> Self {
        Self
    }

    fn collect_symbols<L: Language, S: Source + ?Sized>(&self, uri: &str, node: &RedNode<L>, symbols: &mut Vec<SymbolInformation>, container_name: Option<String>, source: &S) {
        let role = node.green.kind.role();

        if role.universal() == UniversalElementRole::Definition {
            // Try to find the name of the definition
            let mut name = None;
            for child in node.children() {
                match child {
                    RedTree::Leaf(leaf) => {
                        // In many languages, the first name identifier in a definition is its name
                        if leaf.kind.is_universal(UniversalTokenRole::Name) || leaf.kind.is_universal(UniversalTokenRole::None) {
                            name = Some(source.get_text_in(leaf.span).to_string());
                            break;
                        }
                    }
                    _ => {}
                }
            }

            let name = name.unwrap_or_else(|| format!("<{:?}>", node.green.kind));

            symbols.push(SymbolInformation { name: name.clone(), role: role.universal(), uri: uri.to_string().into(), range: node.span(), container_name: container_name.clone() });

            // Recurse with this definition as the container
            for child in node.children() {
                if let RedTree::Node(child_node) = child {
                    self.collect_symbols(uri, &child_node, symbols, Some(name.clone()), source);
                }
            }
        }
        else {
            // Just recurse
            for child in node.children() {
                if let RedTree::Node(child_node) = child {
                    self.collect_symbols(uri, &child_node, symbols, container_name.clone(), source);
                }
            }
        }
    }
}

impl<L: Language> SymbolProvider<L> for UniversalSymbolProvider {
    fn document_symbols<S: Source + ?Sized>(&self, uri: &str, root: &RedNode<L>, source: &S) -> Vec<SymbolInformation> {
        let mut symbols = Vec::new();
        self.collect_symbols(uri, root, &mut symbols, None, source);
        symbols
    }
}
