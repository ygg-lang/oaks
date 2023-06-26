use crate::{
    language::JavaLanguage,
    lexer::token_type::JavaTokenType,
    parser::{State, element_type::JavaElementType},
};
use oak_core::{
    GreenNode, OakError, TokenType,
    parser::pratt::{Associativity, Pratt, PrattParser, binary},
    source::Source,
};

/// Parse an expression statement with error recovery
pub(crate) fn parse_expression_statement<'a, S: Source + ?Sized, P: Pratt<JavaLanguage>>(parser: &P, state: &mut State<'a, S>) {
    use JavaTokenType::*;
    PrattParser::parse(state, 0, parser);
    skip_trivia(state);
    state.eat(Semicolon);
}

/// Parse an anonymous class expression
pub(crate) fn parse_anonymous_class<'a, S: Source + ?Sized, P: Pratt<JavaLanguage> + super::parse_declaration::DeclarationParser>(parser: &P, state: &mut State<'a, S>) -> Result<(), OakError> {
    use JavaTokenType::*;
    let cp = state.checkpoint();
    parser.parse_type(state)?;
    skip_trivia(state);

    if state.at(LeftParen) {
        state.bump();
        skip_trivia(state);
        while state.not_at_end() && !state.at(RightParen) {
            PrattParser::parse(state, 0, parser);
            skip_trivia(state);
            if !state.eat(Comma) {
                break;
            }
            skip_trivia(state);
        }
        state.expect(RightParen).ok();
        skip_trivia(state);
    }

    state.expect(LeftBrace).ok();
    skip_trivia(state);
    while state.not_at_end() && !state.at(RightBrace) {
        super::parse_statement::parse_statement(parser, state).ok();
        skip_trivia(state);
    }
    state.expect(RightBrace).ok();
    state.finish_at(cp, JavaElementType::AnonymousClass);
    Ok(())
}

/// Skip trivia tokens (whitespace, comments)
pub(crate) fn skip_trivia<'a, S: Source + ?Sized>(state: &mut State<'a, S>) {
    while state.not_at_end() {
        let kind = state.peek_kind().unwrap();
        if kind.is_ignored() {
            state.bump();
        }
        else {
            break;
        }
    }
}

/// Pratt parser primary expression handler
pub(crate) fn primary<'a, S: Source + ?Sized, P: Pratt<JavaLanguage> + super::parse_declaration::DeclarationParser>(parser: &P, state: &mut State<'a, S>) -> &'a GreenNode<'a, JavaLanguage> {
    use JavaTokenType::*;
    skip_trivia(state);
    let cp = state.checkpoint();
    match state.peek_kind() {
        Some(Identifier) => {
            state.bump();
            state.finish_at(cp, JavaElementType::Identifier)
        }
        Some(IntegerLiteral) | Some(FloatingPointLiteral) | Some(BooleanLiteral) | Some(CharacterLiteral) | Some(StringLiteral) | Some(NullLiteral) => {
            state.bump();
            state.finish_at(cp, JavaElementType::LiteralExpression)
        }
        Some(LeftParen) => {
            state.bump();
            PrattParser::parse(state, 0, parser);
            state.expect(RightParen).ok();
            state.finish_at(cp, JavaElementType::ParenthesizedExpression)
        }
        Some(New) => {
            state.bump();
            skip_trivia(state);
            parser.parse_type(state).ok();
            skip_trivia(state);
            if state.at(LeftBracket) {
                while state.at(LeftBracket) {
                    state.bump();
                    skip_trivia(state);
                    if !state.at(RightBracket) {
                        PrattParser::parse(state, 0, parser);
                    }
                    skip_trivia(state);
                    state.expect(RightBracket).ok();
                    skip_trivia(state)
                }
                state.finish_at(cp, JavaElementType::ArrayCreation)
            }
            else if state.at(LeftParen) {
                state.bump();
                skip_trivia(state);
                while state.not_at_end() && !state.at(RightParen) {
                    PrattParser::parse(state, 0, parser);
                    skip_trivia(state);
                    if !state.eat(Comma) {
                        break;
                    }
                    skip_trivia(state)
                }
                state.expect(RightParen).ok();
                skip_trivia(state);
                if state.at(LeftBrace) {
                    parse_anonymous_class(parser, state).ok();
                    state.finish_at(cp, JavaElementType::AnonymousClass)
                }
                else {
                    state.finish_at(cp, JavaElementType::MethodCall)
                }
            }
            else if state.at(LeftBrace) {
                parse_anonymous_class(parser, state).ok();
                state.finish_at(cp, JavaElementType::AnonymousClass)
            }
            else {
                state.finish_at(cp, JavaElementType::Error)
            }
        }
        _ => {
            state.bump();
            state.finish_at(cp, JavaElementType::Error)
        }
    }
}

/// Pratt parser prefix expression handler
pub(crate) fn prefix<'a, S: Source + ?Sized, P: Pratt<JavaLanguage> + super::parse_declaration::DeclarationParser>(parser: &P, state: &mut State<'a, S>) -> &'a GreenNode<'a, JavaLanguage> {
    use JavaTokenType::*;
    skip_trivia(state);
    let cp = state.checkpoint();
    match state.peek_kind() {
        Some(Bang) | Some(Tilde) | Some(Plus) | Some(Minus) | Some(PlusPlus) | Some(MinusMinus) => {
            state.bump();
            PrattParser::parse(state, 14, parser);
            state.finish_at(cp, JavaElementType::UnaryExpression)
        }
        Some(LeftParen) => {
            let snapshot = state.checkpoint();
            state.bump();
            skip_trivia(state);
            if parser.parse_type(state).is_ok() {
                skip_trivia(state);
                if state.eat(RightParen) {
                    skip_trivia(state);
                    PrattParser::parse(state, 13, parser);
                    state.finish_at(cp, JavaElementType::CastExpression)
                }
                else {
                    state.restore(snapshot);
                    primary(parser, state)
                }
            }
            else {
                state.restore(snapshot);
                primary(parser, state)
            }
        }
        _ => primary(parser, state),
    }
}

/// Pratt parser infix expression handler
pub(crate) fn infix<'a, S: Source + ?Sized, P: Pratt<JavaLanguage> + super::parse_declaration::DeclarationParser>(parser: &P, state: &mut State<'a, S>, left: &'a GreenNode<'a, JavaLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, JavaLanguage>> {
    use JavaTokenType::*;
    skip_trivia(state);
    let kind = state.peek_kind()?;

    let (prec, assoc) = match kind {
        Assign | PlusEquals | MinusEquals | AsteriskEquals | SlashEquals | PercentEquals | LeftShiftEquals | RightShiftEquals | UnsignedRightShiftEquals | AmpersandEquals | PipeEquals | CaretEquals => (1, Associativity::Right),
        Question => (2, Associativity::Right),
        PipePipe => (3, Associativity::Left),
        AmpersandAmpersand => (4, Associativity::Left),
        Pipe => (5, Associativity::Left),
        Caret => (6, Associativity::Left),
        Ampersand => (7, Associativity::Left),
        Equals | BangEquals => (8, Associativity::Left),
        LessThan | GreaterThan | LessThanEquals | GreaterThanEquals | Instanceof => (9, Associativity::Left),
        LeftShift | RightShift | UnsignedRightShift => (10, Associativity::Left),
        Plus | Minus => (11, Associativity::Left),
        Asterisk | Slash | Percent => (12, Associativity::Left),
        Arrow => (13, Associativity::Right),
        PlusPlus | MinusMinus => (15, Associativity::Left),
        LeftParen | Dot | LeftBracket => (16, Associativity::Left),
        _ => return None,
    };

    if prec < min_precedence {
        return None;
    }

    match kind {
        PlusPlus | MinusMinus => {
            let cp = state.checkpoint_before(left);
            state.bump();
            Some(state.finish_at(cp, JavaElementType::PostfixExpression))
        }
        Assign | PlusEquals | MinusEquals | AsteriskEquals | SlashEquals | PercentEquals | LeftShiftEquals | RightShiftEquals | UnsignedRightShiftEquals | AmpersandEquals | PipeEquals | CaretEquals => {
            Some(binary(state, left, kind, prec, assoc, JavaElementType::AssignmentExpression.into(), |s, p| PrattParser::parse(s, p, parser)))
        }
        Question => {
            let cp = state.checkpoint_before(left);
            state.bump();
            skip_trivia(state);
            PrattParser::parse(state, 0, parser);
            skip_trivia(state);
            state.expect(Colon).ok();
            skip_trivia(state);
            PrattParser::parse(state, prec, parser);
            Some(state.finish_at(cp, JavaElementType::TernaryExpression))
        }
        LeftParen => {
            let cp = state.checkpoint_before(left);
            state.expect(LeftParen).ok();
            skip_trivia(state);
            while state.not_at_end() && !state.at(RightParen) {
                PrattParser::parse(state, 0, parser);
                skip_trivia(state);
                if state.eat(Comma) {
                    skip_trivia(state);
                    continue;
                }
            }
            state.expect(RightParen).ok();
            Some(state.finish_at(cp, JavaElementType::MethodCall))
        }
        Dot => {
            let cp = state.checkpoint_before(left);
            state.expect(Dot).ok();
            skip_trivia(state);
            state.expect(Identifier).ok();
            Some(state.finish_at(cp, JavaElementType::MemberSelect))
        }
        LeftBracket => {
            let cp = state.checkpoint_before(left);
            state.expect(LeftBracket).ok();
            skip_trivia(state);
            PrattParser::parse(state, 0, parser);
            skip_trivia(state);
            state.expect(RightBracket).ok();
            Some(state.finish_at(cp, JavaElementType::ArrayAccess))
        }
        Arrow => {
            let cp = state.checkpoint_before(left);
            state.bump();
            skip_trivia(state);
            if state.at(LeftBrace) {
                super::parse_statement::parse_block_statement(parser, state).ok()?;
            }
            else {
                PrattParser::parse(state, 0, parser);
            }
            Some(state.finish_at(cp, JavaElementType::LambdaExpression))
        }
        _ => Some(binary(state, left, kind, prec, assoc, JavaElementType::BinaryExpression.into(), |s, p| PrattParser::parse(s, p, parser))),
    }
}
