use crate::{lexer::token_type::DejavuTokenType, parser::element_type::DejavuElementType};
use oak_core::{GreenNode, OakError, source::Source};

use super::State;

impl<'config> super::DejavuParser<'config> {
    pub(crate) fn parse_expression_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, _precedence: u8) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        self.parse_primary_expression(state)?;
        // 暂时简化处理，跳过 postfix 和 filter 解析
        Ok(state.finish_at(cp, DejavuElementType::Expression))
    }

    fn parse_primary_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        if state.at(DejavuTokenType::Identifier) {
            let cp = state.checkpoint();
            let token = state.current().unwrap();
            let token_text = state.source.get_text_in(token.span).to_string();

            state.bump(); // 消耗标识符

            // 检查是否是 t 函数调用
            if token_text == "t" && state.at(DejavuTokenType::LeftParen) {
                state.bump(); // 消耗 (
                let t_cp = state.checkpoint();
                self.parse_translate_expression(state)?;
                state.finish_at(t_cp, DejavuElementType::TranslateExpression);
            }
            else {
                // 创建 IdentifierExpression 节点
                state.finish_at(cp, DejavuElementType::IdentifierExpression);
            }
        }
        else if state.at(DejavuTokenType::StringLiteral) || state.at(DejavuTokenType::CharLiteral) {
            let cp = state.checkpoint();
            state.bump();
            state.finish_at(cp, DejavuElementType::LiteralExpression);
        }
        else if state.at(DejavuTokenType::IntegerLiteral) {
            let cp = state.checkpoint();
            state.bump();
            state.finish_at(cp, DejavuElementType::LiteralExpression);
        }
        else if state.at(DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::True)) || state.at(DejavuTokenType::Keyword(crate::lexer::DejavuKeywords::False)) {
            let cp = state.checkpoint();
            state.bump();
            state.finish_at(cp, DejavuElementType::BooleanLiteral);
        }
        else if state.at(DejavuTokenType::LeftParen) {
            let cp = state.checkpoint();
            state.bump();
            self.parse_expression_internal(state, 0)?;
            state.expect(DejavuTokenType::RightParen)?;
            state.finish_at(cp, DejavuElementType::ParenthesizedExpression);
        }
        else {
            state.bump();
        }
        Ok(())
    }

    fn parse_translate_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        // 解析翻译键
        if state.at(DejavuTokenType::StringLiteral) {
            state.bump(); // 消耗字符串字面量
        }

        // 解析参数
        while state.at(DejavuTokenType::Comma) {
            state.bump(); // 消耗 ,

            // 解析参数名
            if state.at(DejavuTokenType::Identifier) {
                state.bump(); // 消耗参数名

                // 解析 = 操作符
                if state.at(DejavuTokenType::Eq) {
                    state.bump(); // 消耗 =

                    // 解析参数值
                    self.parse_expression_internal(state, 0)?;
                }
            }
        }

        state.expect(DejavuTokenType::RightParen)?; // 消耗 )
        Ok(())
    }

    fn parse_filter_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.at(DejavuTokenType::Pipe) {
            let cp = state.checkpoint();
            state.bump(); // 消耗 |> 操作符

            if !state.at(DejavuTokenType::Identifier) {
                state.bump();
                state.finish_at(cp, DejavuElementType::Error);
                return Ok(());
            }

            state.bump(); // 消耗过滤器名称

            // 解析过滤器参数
            if state.at(DejavuTokenType::LeftParen) {
                state.bump();
                while !state.at(DejavuTokenType::RightParen) && state.not_at_end() {
                    self.parse_expression_internal(state, 0)?;
                    if state.at(DejavuTokenType::Comma) {
                        state.bump();
                    }
                    else if !state.at(DejavuTokenType::RightParen) {
                        break;
                    }
                }
                state.expect(DejavuTokenType::RightParen)?;
            }

            state.finish_at(cp, DejavuElementType::FilterExpression);
        }
        Ok(())
    }

    pub(crate) fn parse_template_code<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();

        // 接受 CodeStart 或 TemplateControlStart
        if state.at(DejavuTokenType::CodeStart) {
            state.expect(DejavuTokenType::CodeStart)?;
        }
        else if state.at(DejavuTokenType::TemplateControlStart) {
            state.expect(DejavuTokenType::TemplateControlStart)?;
        }
        else {
            return Err(OakError::custom_error("Expected CodeStart or TemplateControlStart"));
        }

        self.parse_expression_internal(state, 0)?;

        // 接受 CodeEnd 或 TemplateControlEnd
        if state.at(DejavuTokenType::CodeEnd) {
            state.bump();
        }
        else if state.at(DejavuTokenType::TemplateControlEnd) {
            state.bump();
        }

        Ok(state.finish_at(cp, DejavuElementType::Interpolation))
    }

    pub(crate) fn parse_template_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, crate::DejavuLanguage>, OakError> {
        let cp = state.checkpoint();
        state.expect(DejavuTokenType::TemplateCommentStart)?;
        while state.not_at_end() && !state.at(DejavuTokenType::TemplateCommentEnd) {
            state.bump();
        }
        if state.at(DejavuTokenType::TemplateCommentEnd) {
            state.bump();
        }
        Ok(state.finish_at(cp, DejavuElementType::TemplateComment))
    }
}
