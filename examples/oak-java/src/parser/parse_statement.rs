use super::parse_declaration::DeclarationParser;
use crate::{
    language::JavaLanguage,
    lexer::token_type::JavaTokenType,
    parser::{State, element_type::JavaElementType},
};
use oak_core::{
    OakError,
    parser::pratt::{Pratt, PrattParser},
    source::Source,
};

/// Parse a statement
pub(crate) fn parse_statement<'a, S: Source + ?Sized, P: Pratt<JavaLanguage> + DeclarationParser>(parser: &P, state: &mut State<'a, S>) -> Result<(), OakError> {
    use JavaTokenType::*;
    super::parse_expression::skip_trivia(state);
    let cp = state.checkpoint();
    let pk = state.peek_kind();
    match pk {
        Some(Public) | Some(Private) | Some(Protected) | Some(Static) | Some(Final) | Some(Abstract) | Some(Class) | Some(Interface) | Some(Enum) | Some(Struct) | Some(Record) => {
            if let Err(_e) = parser.parse_declaration(state) {
                recover_from_error(state);
            }
        }
        Some(Int) | Some(Boolean) | Some(Void) | Some(Long) | Some(Float) | Some(Double) | Some(Char) | Some(Byte) | Some(Short) => {
            if let Err(_e) = parser.parse_variable_declaration(state) {
                skip_until_semicolon(state);
            }
            state.finish_at(cp, JavaElementType::VariableDeclaration);
        }
        Some(Identifier) => {
            let snapshot = state.checkpoint();
            if parser.parse_type(state).is_ok() {
                super::parse_expression::skip_trivia(state);
                if state.at(Identifier) {
                    state.restore(snapshot);
                    if let Err(_e) = parser.parse_variable_declaration(state) {
                        skip_until_semicolon(state);
                    }
                    state.finish_at(cp, JavaElementType::VariableDeclaration);
                }
                else {
                    state.restore(snapshot);
                    super::parse_expression::parse_expression_statement(parser, state);
                    state.finish_at(cp, JavaElementType::ExpressionStatement);
                }
            }
            else {
                state.restore(snapshot);
                super::parse_expression::parse_expression_statement(parser, state);
                state.finish_at(cp, JavaElementType::ExpressionStatement);
            }
        }
        Some(If) => {
            if let Err(_e) = parse_if_statement(parser, state) {
                recover_from_error(state);
            }
            state.finish_at(cp, JavaElementType::IfStatement);
        }
        Some(While) => {
            if let Err(_e) = parse_while_statement(parser, state) {
                recover_from_error(state);
            }
            state.finish_at(cp, JavaElementType::WhileStatement);
        }
        Some(Do) => {
            if let Err(_e) = parse_do_while_statement(parser, state) {
                recover_from_error(state);
            }
            state.finish_at(cp, JavaElementType::DoWhileStatement);
        }
        Some(For) => {
            if let Err(_e) = parse_for_statement(parser, state) {
                recover_from_error(state);
            }
            state.finish_at(cp, JavaElementType::ForStatement);
        }
        Some(Switch) => {
            if let Err(_e) = parse_switch_statement(parser, state) {
                skip_until_closing_brace(state);
            }
            state.finish_at(cp, JavaElementType::SwitchStatement);
        }
        Some(Return) => {
            if let Err(_e) = parse_return_statement(parser, state) {
                skip_until_semicolon(state);
            }
            state.finish_at(cp, JavaElementType::ReturnStatement);
        }
        Some(Break) => {
            state.bump();
            state.eat(Semicolon);
            state.finish_at(cp, JavaElementType::Break);
        }
        Some(Continue) => {
            state.bump();
            state.eat(Semicolon);
            state.finish_at(cp, JavaElementType::Continue);
        }
        Some(LeftBrace) => {
            if let Err(_e) = parse_block_statement(parser, state) {
                skip_until_closing_brace(state);
            }
        }
        Some(Try) => {
            state.bump();
            if let Err(_e) = parse_block_statement(parser, state) {
                skip_until_catch_or_finally(state);
            }
            super::parse_expression::skip_trivia(state);
            while state.at(Catch) {
                let c_cp = state.checkpoint();
                state.bump();
                super::parse_expression::skip_trivia(state);
                state.expect(LeftParen).ok();
                super::parse_expression::skip_trivia(state);
                let p_cp = state.checkpoint();
                parser.parse_type(state).ok();
                super::parse_expression::skip_trivia(state);
                state.expect(Identifier).ok();
                state.finish_at(p_cp, JavaElementType::Parameter);
                super::parse_expression::skip_trivia(state);
                state.expect(RightParen).ok();
                super::parse_expression::skip_trivia(state);
                if let Err(_e) = parse_block_statement(parser, state) {
                    skip_until_catch_or_finally(state);
                }
                state.finish_at(c_cp, JavaElementType::CatchClause);
                super::parse_expression::skip_trivia(state);
            }
            if state.eat(Finally) {
                super::parse_expression::skip_trivia(state);
                if let Err(_e) = parse_block_statement(parser, state) {
                    skip_until_closing_brace(state);
                }
            }
            state.finish_at(cp, JavaElementType::TryStatement);
        }
        Some(Throw) => {
            state.bump();
            super::parse_expression::skip_trivia(state);
            PrattParser::parse(state, 0, parser);
            super::parse_expression::skip_trivia(state);
            state.eat(Semicolon);
            state.finish_at(cp, JavaElementType::ThrowStatement);
        }
        Some(Package) => {
            if let Err(_e) = parser.parse_package_declaration(state) {
                skip_until_semicolon(state);
            }
        }
        Some(Import) => {
            if let Err(_e) = parser.parse_import_declaration(state) {
                skip_until_semicolon(state);
            }
        }
        _ => {
            super::parse_expression::parse_expression_statement(parser, state);
            state.finish_at(cp, JavaElementType::ExpressionStatement);
        }
    }
    Ok(())
}

/// Parse an if statement
pub(crate) fn parse_if_statement<'a, S: Source + ?Sized, P: Pratt<JavaLanguage> + DeclarationParser>(parser: &P, state: &mut State<'a, S>) -> Result<(), OakError> {
    use JavaTokenType::*;
    state.bump();
    super::parse_expression::skip_trivia(state);
    state.expect(LeftParen).ok();
    super::parse_expression::skip_trivia(state);
    PrattParser::parse(state, 0, parser);
    super::parse_expression::skip_trivia(state);
    state.expect(RightParen).ok();
    super::parse_expression::skip_trivia(state);
    parse_statement(parser, state)?;
    super::parse_expression::skip_trivia(state);
    if state.eat(Else) {
        super::parse_expression::skip_trivia(state);
        parse_statement(parser, state)?;
    }
    Ok(())
}

/// Parse a while statement
pub(crate) fn parse_while_statement<'a, S: Source + ?Sized, P: Pratt<JavaLanguage> + DeclarationParser>(parser: &P, state: &mut State<'a, S>) -> Result<(), OakError> {
    use JavaTokenType::*;
    state.bump();
    super::parse_expression::skip_trivia(state);
    state.expect(LeftParen).ok();
    super::parse_expression::skip_trivia(state);
    PrattParser::parse(state, 0, parser);
    super::parse_expression::skip_trivia(state);
    state.expect(RightParen).ok();
    super::parse_expression::skip_trivia(state);
    parse_statement(parser, state)?;
    Ok(())
}

/// Parse a do-while statement
pub(crate) fn parse_do_while_statement<'a, S: Source + ?Sized, P: Pratt<JavaLanguage> + DeclarationParser>(parser: &P, state: &mut State<'a, S>) -> Result<(), OakError> {
    use JavaTokenType::*;
    state.bump();
    super::parse_expression::skip_trivia(state);
    parse_statement(parser, state)?;
    super::parse_expression::skip_trivia(state);
    state.expect(While).ok();
    super::parse_expression::skip_trivia(state);
    state.expect(LeftParen).ok();
    super::parse_expression::skip_trivia(state);
    PrattParser::parse(state, 0, parser);
    super::parse_expression::skip_trivia(state);
    state.expect(RightParen).ok();
    super::parse_expression::skip_trivia(state);
    state.eat(Semicolon);
    Ok(())
}

/// Parse a for statement
pub(crate) fn parse_for_statement<'a, S: Source + ?Sized, P: Pratt<JavaLanguage> + DeclarationParser>(parser: &P, state: &mut State<'a, S>) -> Result<(), OakError> {
    use JavaTokenType::*;
    state.bump();
    state.expect(LeftParen).ok();
    super::parse_expression::skip_trivia(state);

    if !state.at(Semicolon) {
        let cp = state.checkpoint();
        let pk = state.peek_kind();
        match pk {
            Some(Int) | Some(Boolean) | Some(Void) | Some(Long) | Some(Float) | Some(Double) | Some(Char) | Some(Byte) | Some(Short) => {
                parser.parse_variable_declaration(state)?;
                state.finish_at(cp, JavaElementType::VariableDeclaration);
            }
            Some(Identifier) => {
                let snapshot = state.checkpoint();
                if parser.parse_type(state).is_ok() && state.at(Identifier) {
                    state.restore(snapshot);
                    parser.parse_variable_declaration(state)?;
                    state.finish_at(cp, JavaElementType::VariableDeclaration);
                }
                else {
                    state.restore(snapshot);
                    PrattParser::parse(state, 0, parser);
                }
            }
            _ => {
                PrattParser::parse(state, 0, parser);
            }
        }
    }
    state.expect(Semicolon).ok();
    super::parse_expression::skip_trivia(state);

    if !state.at(Semicolon) {
        PrattParser::parse(state, 0, parser);
    }
    state.expect(Semicolon).ok();
    super::parse_expression::skip_trivia(state);

    if !state.at(RightParen) {
        PrattParser::parse(state, 0, parser);
    }
    state.expect(RightParen).ok();
    super::parse_expression::skip_trivia(state);

    parse_statement(parser, state)?;
    Ok(())
}

/// Parse a switch statement
pub(crate) fn parse_switch_statement<'a, S: Source + ?Sized, P: Pratt<JavaLanguage> + DeclarationParser>(parser: &P, state: &mut State<'a, S>) -> Result<(), OakError> {
    use JavaTokenType::*;
    state.bump();
    super::parse_expression::skip_trivia(state);
    state.expect(LeftParen).ok();
    super::parse_expression::skip_trivia(state);
    PrattParser::parse(state, 0, parser);
    super::parse_expression::skip_trivia(state);
    state.expect(RightParen).ok();
    super::parse_expression::skip_trivia(state);
    state.expect(LeftBrace).ok();
    super::parse_expression::skip_trivia(state);
    while state.not_at_end() && !state.at(RightBrace) {
        super::parse_expression::skip_trivia(state);
        let cp = state.checkpoint();
        if state.eat(Case) {
            super::parse_expression::skip_trivia(state);
            PrattParser::parse(state, 0, parser);
            super::parse_expression::skip_trivia(state);
            state.expect(Colon).ok();
            super::parse_expression::skip_trivia(state);
            while state.not_at_end() && !state.at(Case) && !state.at(Default) && !state.at(RightBrace) {
                parse_statement(parser, state).ok();
                super::parse_expression::skip_trivia(state);
            }
            state.finish_at(cp, JavaElementType::SwitchCase);
        }
        else if state.eat(Default) {
            super::parse_expression::skip_trivia(state);
            state.expect(Colon).ok();
            super::parse_expression::skip_trivia(state);
            while state.not_at_end() && !state.at(Case) && !state.at(Default) && !state.at(RightBrace) {
                parse_statement(parser, state).ok();
                super::parse_expression::skip_trivia(state)
            }
            state.finish_at(cp, JavaElementType::DefaultCase);
        }
        else {
            state.bump();
            super::parse_expression::skip_trivia(state)
        }
    }
    state.expect(RightBrace).ok();
    Ok(())
}

/// Parse a block statement
pub(crate) fn parse_block_statement<'a, S: Source + ?Sized, P: Pratt<JavaLanguage> + DeclarationParser>(parser: &P, state: &mut State<'a, S>) -> Result<(), OakError> {
    let cp = state.checkpoint();
    state.expect(JavaTokenType::LeftBrace).ok();
    while state.not_at_end() && !state.at(JavaTokenType::RightBrace) {
        super::parse_expression::skip_trivia(state);
        if state.at(JavaTokenType::RightBrace) {
            break;
        }
        parse_statement(parser, state).ok();
        super::parse_expression::skip_trivia(state)
    }
    state.expect(JavaTokenType::RightBrace).ok();
    state.finish_at(cp, JavaElementType::BlockStatement);
    Ok(())
}

/// Parse a return statement
pub(crate) fn parse_return_statement<'a, S: Source + ?Sized, P: Pratt<JavaLanguage> + DeclarationParser>(parser: &P, state: &mut State<'a, S>) -> Result<(), OakError> {
    state.bump();
    if !state.at(JavaTokenType::Semicolon) && !state.at(JavaTokenType::RightBrace) {
        PrattParser::parse(state, 0, parser);
    }
    state.eat(JavaTokenType::Semicolon);
    Ok(())
}

/// Skip until semicolon for error recovery
pub(crate) fn skip_until_semicolon<'a, S: Source + ?Sized>(state: &mut State<'a, S>) {
    use JavaTokenType::*;
    while state.not_at_end() && !state.at(Semicolon) && !state.at(LeftBrace) && !state.at(RightBrace) {
        state.bump();
        super::parse_expression::skip_trivia(state);
    }
    state.eat(Semicolon);
    super::parse_expression::skip_trivia(state);
}

/// Skip until closing brace for error recovery
pub(crate) fn skip_until_closing_brace<'a, S: Source + ?Sized>(state: &mut State<'a, S>) {
    use JavaTokenType::*;
    let mut brace_count = 1;
    while state.not_at_end() && brace_count > 0 {
        match state.peek_kind() {
            Some(LeftBrace) => brace_count += 1,
            Some(RightBrace) => brace_count -= 1,
            _ => {}
        }
        state.bump();
        super::parse_expression::skip_trivia(state);
    }
}

/// Skip until catch or finally for error recovery
pub(crate) fn skip_until_catch_or_finally<'a, S: Source + ?Sized>(state: &mut State<'a, S>) {
    use JavaTokenType::*;
    while state.not_at_end() && !state.at(Catch) && !state.at(Finally) && !state.at(RightBrace) {
        state.bump();
        super::parse_expression::skip_trivia(state);
    }
}

/// General error recovery: skip until next statement
pub(crate) fn recover_from_error<'a, S: Source + ?Sized>(state: &mut State<'a, S>) {
    use JavaTokenType::*;
    while state.not_at_end() {
        match state.peek_kind() {
            Some(Semicolon) | Some(LeftBrace) | Some(RightBrace) | Some(If) | Some(While) | Some(Do) | Some(For) | Some(Switch) | Some(Return) | Some(Break) | Some(Continue) | Some(Try) | Some(Throw) | Some(Package) | Some(Import) | Some(Public)
            | Some(Private) | Some(Protected) | Some(Static) | Some(Final) | Some(Abstract) | Some(Class) | Some(Interface) | Some(Enum) | Some(Struct) | Some(Record) => {
                break;
            }
            _ => {
                state.bump();
                super::parse_expression::skip_trivia(state);
            }
        }
    }
}
