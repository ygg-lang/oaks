use crate::{
    ValkyrieLanguage,
    ast::*,
    builder::{ValkyrieBuilder, utils},
    lexer::token_type::ValkyrieTokenType,
};
use oak_core::{OakError, RedNode, Source};

impl<'config> ValkyrieBuilder<'config> {
    pub(crate) fn build_field_expr<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<TermExpression, OakError> {
        let span = node.span();
        let (receiver, field) = utils::build_field_expr(&node, source, |n, s| self.build_expr(*n, s))?;

        Ok(TermExpression::DotCall { receiver, field, span })
    }

    pub(crate) fn build_index<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<TermExpression, OakError> {
        let span = node.span();
        let (receiver, index) = utils::build_index_expr(&node, source, |n, s| self.build_expr(*n, s), "Missing index")?;

        Ok(TermExpression::Index { receiver, index, span })
    }

    /// 构建基数索引表达式。
    ///
    /// 基数索引使用 `⁅ ⁆` 括号，表示从 0 开始的偏移量访问。
    /// 与普通索引 `[ ]`（从 1 开始）不同，基数索引更接近底层指针算术风格。
    pub(crate) fn build_offset<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<TermExpression, OakError> {
        let span = node.span();
        let (receiver, offset) = utils::build_index_expr(&node, source, |n, s| self.build_expr(*n, s), "Missing offset")?;

        Ok(TermExpression::Offset { receiver, offset, span })
    }

    pub(crate) fn build_paren<S: Source + ?Sized>(&self, node: RedNode<ValkyrieLanguage>, source: &S) -> Result<TermExpression, OakError> {
        let span = node.span();
        let expr = utils::get_required_expr(&node, source, |n, s| self.build_expr(*n, s), "Missing parenthesized expression", span.start)?;
        Ok(TermExpression::Paren { expr, span })
    }
}
