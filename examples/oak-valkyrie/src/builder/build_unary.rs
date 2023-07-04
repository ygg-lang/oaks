use crate::{
    ValkyrieLanguage,
    ast::{term_nodes::TermUnaryNode as Unary, *},
    builder::{ValkyrieBuilder, utils},
    lexer::token_type::ValkyrieTokenType,
};
use oak_core::{OakError, RedNode, Source};

impl<'config> ValkyrieBuilder<'config> {
    pub(crate) fn build_unary<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<TermExpression, OakError> {
        let span = node.span();
        let (op, expr) = utils::build_unary_expr(&node, source, |n, s| self.build_expr(*n, s), ValkyrieTokenType::Bang)?;

        Ok(TermExpression::Unary(Box::new(Unary { operator: op, base: *expr, span })))
    }
}
