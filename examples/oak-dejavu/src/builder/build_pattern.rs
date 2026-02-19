use crate::{
    DejavuLanguage,
    ast::{IdentifierNode, PatternNode, VariablePatternNode},
    builder::{DejavuBuilder, text},
    lexer::token_type::DejavuTokenType,
    parser::element_type::DejavuElementType,
};
use oak_core::{OakError, RedNode, RedTree, Source};

impl<'config> DejavuBuilder<'config> {
    pub(crate) fn build_pattern<S: Source + ?Sized>(&self, node: RedNode<DejavuLanguage>, source: &S) -> Result<PatternNode, OakError> {
        match node.green.kind {
            DejavuElementType::Pattern => {
                // Traverse children to find specific pattern type or identifier
                for child in node.children() {
                    match child {
                        RedTree::Leaf(t) if t.kind == DejavuTokenType::Identifier => {
                            let t_text = text(source, t.span.clone().into());
                            return Ok(PatternNode::Variable(VariablePatternNode { name: IdentifierNode { name: t_text, span: t.span.clone() }, span: t.span.clone() }));
                        }
                        RedTree::Node(n) => return self.build_pattern(n, source),
                        _ => {}
                    }
                }
                // If no identifier found, it might be other pattern type or error
                Err(source.syntax_error("Invalid pattern".to_string(), node.span().start))
            }
            _ => Err(source.syntax_error(format!("Unexpected node for pattern: {:?}", node.green.kind), node.span().start)),
        }
    }
}
