use crate::{
    ValkyrieLanguage,
    ast::*,
    builder::{ValkyrieBuilder, utils},
    lexer::token_type::ValkyrieTokenType,
};
use oak_core::{OakError, RedNode, Source};

impl<'config> ValkyrieBuilder<'config> {
    pub(crate) fn build_call<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<TermExpression, OakError> {
        let span = node.span();
        let (callee, args) = utils::build_call_expr(&node, source, |n, s| self.build_expr(*n, s))?;

        Ok(TermExpression::ApplyCall { callee, args, span })
    }
}
