use crate::{WolframLanguage, lexer::token_type::WolframTokenType, parser::element_type::WolframElementType};
use oak_core::{RedNode, RedTree, TokenType};

/// Whether a red tree child is trivia / ignored.
pub(crate) fn should_skip(node: &RedTree<'_, WolframLanguage>) -> bool {
    match node {
        RedTree::Leaf(t) => t.kind().is_ignored(),
        RedTree::Node(n) => matches!(n.element_type(), WolframElementType::Error) && n.green.children.is_empty(),
    }
}

/// True if the token can appear as a Wolfram symbol head / name.
pub(crate) fn is_symbol_like(kind: WolframTokenType) -> bool {
    matches!(
        kind,
        WolframTokenType::Identifier
            | WolframTokenType::Slot
            | WolframTokenType::SlotSequence
            | WolframTokenType::If
            | WolframTokenType::Then
            | WolframTokenType::Else
            | WolframTokenType::While
            | WolframTokenType::For
            | WolframTokenType::Do
            | WolframTokenType::Function
            | WolframTokenType::Module
            | WolframTokenType::Block
            | WolframTokenType::With
            | WolframTokenType::Table
            | WolframTokenType::Map
            | WolframTokenType::Apply
            | WolframTokenType::Select
            | WolframTokenType::Cases
            | WolframTokenType::Rule
            | WolframTokenType::RuleDelayed
            | WolframTokenType::Set
            | WolframTokenType::SetDelayed
            | WolframTokenType::Unset
            | WolframTokenType::Clear
            | WolframTokenType::ClearAll
            | WolframTokenType::Return
            | WolframTokenType::Break
            | WolframTokenType::Continue
            | WolframTokenType::True
            | WolframTokenType::False
            | WolframTokenType::Null
            | WolframTokenType::Export
            | WolframTokenType::Import
    )
}

/// Collect non-trivia child expression nodes.
pub(crate) fn child_expr_nodes<'a>(node: RedNode<'a, WolframLanguage>) -> impl Iterator<Item = RedNode<'a, WolframLanguage>> {
    node.children().filter_map(move |child| {
        if should_skip(&child) {
            return None;
        }
        match child {
            RedTree::Node(n) if n.element_type() != WolframElementType::Arguments => Some(n),
            _ => None,
        }
    })
}
