use crate::{
    MatlabLanguage,
    ast::MatlabRoot,
    builder::MatlabBuilder,
    lexer::token_type::MatlabTokenType,
};
use oak_core::{GreenNode, OakError, RedNode, RedTree, Source, TokenType};

impl<'config> MatlabBuilder<'config> {
    /// Build owned [`MatlabRoot`] from a green root.
    pub fn build_root<S: Source + ?Sized>(&self, green_tree: &GreenNode<MatlabLanguage>, source: &S) -> Result<MatlabRoot, OakError> {
        let red_root = RedNode::new(green_tree, 0);
        let mut items = Vec::new();
        for child in red_root.children() {
            match child {
                RedTree::Node(n) => items.push(self.build_stmt(n, source)?),
                RedTree::Leaf(t) => {
                    if t.kind().is_ignored()
                        || matches!(t.kind(), MatlabTokenType::Eof | MatlabTokenType::Semicolon | MatlabTokenType::Comma)
                    {
                        continue;
                    }
                    return Err(source.syntax_error(format!("Unexpected token in MATLAB root: {:?}", t.kind()), t.span.start));
                }
            }
        }
        Ok(MatlabRoot { items, span: red_root.span() })
    }
}
