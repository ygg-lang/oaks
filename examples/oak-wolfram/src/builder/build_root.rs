use crate::{WolframLanguage, ast::WolframRoot, builder::WolframBuilder, lexer::token_type::WolframTokenType};
use oak_core::{GreenNode, OakError, RedNode, RedTree, Source, TokenType};

impl<'config> WolframBuilder<'config> {
    /// Build owned [`WolframRoot`] from a green root.
    pub fn build_root<S: Source + ?Sized>(&self, green_tree: &GreenNode<WolframLanguage>, source: &S) -> Result<WolframRoot, OakError> {
        let red_root = RedNode::new(green_tree, 0);
        let mut expressions = Vec::new();
        for child in red_root.children() {
            match child {
                RedTree::Node(n) => {
                    expressions.push(self.build_expr(n, source)?);
                }
                RedTree::Leaf(t) => {
                    if t.kind().is_ignored() || t.kind() == WolframTokenType::Eof || t.kind() == WolframTokenType::Semicolon {
                        continue;
                    }
                    return Err(source.syntax_error(format!("Unexpected token in Wolfram root: {:?}", t.kind()), t.span.start));
                }
            }
        }
        Ok(WolframRoot { expressions, span: red_root.span() })
    }
}
