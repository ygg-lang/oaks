use crate::{
    ValkyrieLanguage,
    lexer::{keywords::ValkyrieKeywords, token_type::ValkyrieTokenType},
    parser::element_type::ValkyrieElementType,
};
use oak_core::parser::ParserState;

type State<'a, S> = ParserState<'a, ValkyrieLanguage, S>;

/// 解析类型
pub(crate) fn parse_type<S: oak_core::Source + ?Sized>(state: &mut State<'_, S>) -> Result<(), oak_core::OakError> {
    let cp = state.sink.checkpoint();
    if state.at(ValkyrieTokenType::Question) {
        state.bump();
    }
    if state.at(ValkyrieTokenType::Identifier) {
        state.bump();
    }
    if state.at(ValkyrieTokenType::LeftBracket) {
        parse_generic_argument_list(state)?;
    }
    if state.at(ValkyrieTokenType::LeftParen) {
        state.bump();
        while state.not_at_end() && !state.at(ValkyrieTokenType::RightParen) {
            parse_type(state)?;
            if state.at(ValkyrieTokenType::Comma) {
                state.bump();
            }
        }
        if state.at(ValkyrieTokenType::RightParen) {
            state.bump();
        }
    }
    if state.at(ValkyrieTokenType::Arrow) {
        state.bump();
        parse_type(state)?;
    }
    state.sink.finish_node(cp, ValkyrieElementType::Type);
    Ok(())
}

/// 解析泛型参数列表
pub(crate) fn parse_generic_parameter_list<S: oak_core::Source + ?Sized>(state: &mut State<'_, S>) -> Result<(), oak_core::OakError> {
    let cp = state.sink.checkpoint();
    state.bump();
    while state.not_at_end() && !state.at(ValkyrieTokenType::RightBracket) {
        let gcp = state.sink.checkpoint();
        if state.at(ValkyrieTokenType::Keyword(ValkyrieKeywords::Type)) {
            state.bump();
        }
        if state.at(ValkyrieTokenType::Identifier) {
            state.bump();
        }
        if state.at(ValkyrieTokenType::Colon) {
            state.bump();
            parse_type(state)?;
        }
        if state.at(ValkyrieTokenType::Eq) {
            state.bump();
            parse_type(state)?;
        }
        state.sink.finish_node(gcp, ValkyrieElementType::GenericParameter);
        if state.at(ValkyrieTokenType::Comma) {
            state.bump();
        }
    }
    if state.at(ValkyrieTokenType::RightBracket) {
        state.bump();
    }
    state.sink.finish_node(cp, ValkyrieElementType::GenericParameterList);
    Ok(())
}

/// 解析泛型参数列表
pub(crate) fn parse_generic_argument_list<S: oak_core::Source + ?Sized>(state: &mut State<'_, S>) -> Result<(), oak_core::OakError> {
    let cp = state.sink.checkpoint();
    state.bump();
    while state.not_at_end() && !state.at(ValkyrieTokenType::RightBracket) {
        parse_type(state)?;
        if state.at(ValkyrieTokenType::Comma) {
            state.bump();
        }
    }
    if state.at(ValkyrieTokenType::RightBracket) {
        state.bump();
    }
    state.sink.finish_node(cp, ValkyrieElementType::GenericArgumentList);
    Ok(())
}

/// 解析参数列表
pub(crate) fn parse_parameter_list<S: oak_core::Source + ?Sized>(state: &mut State<'_, S>) -> Result<(), oak_core::OakError> {
    let cp = state.sink.checkpoint();
    state.bump();
    while state.not_at_end() && !state.at(ValkyrieTokenType::RightParen) {
        let pcp = state.sink.checkpoint();
        if state.at(ValkyrieTokenType::Keyword(ValkyrieKeywords::Mut)) {
            state.bump();
        }
        if state.at(ValkyrieTokenType::Identifier) {
            state.bump();
        }
        if state.at(ValkyrieTokenType::Colon) {
            state.bump();
            parse_type(state)?;
        }
        if state.at(ValkyrieTokenType::Eq) {
            state.bump();
            parse_expression(state)?;
        }
        state.sink.finish_node(pcp, ValkyrieElementType::Parameter);
        if state.at(ValkyrieTokenType::Comma) {
            state.bump();
        }
    }
    if state.at(ValkyrieTokenType::RightParen) {
        state.bump();
    }
    state.sink.finish_node(cp, ValkyrieElementType::ParameterList);
    Ok(())
}

// 以下函数在其他模块中定义
pub(crate) fn parse_expression<S: oak_core::Source + ?Sized>(state: &mut State<'_, S>) -> Result<(), oak_core::OakError> {
    crate::parser::parse_expressions::parse_expression(state)
}
