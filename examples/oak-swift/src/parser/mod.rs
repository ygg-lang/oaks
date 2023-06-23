pub mod element_type;

use crate::language::SwiftLanguage;
use oak_core::{
    GreenNode, OakError, TextEdit,
    parser::{
        ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer,
        pratt::{Associativity, Pratt, PrattParser, binary, unary},
    },
    source::Source,
};

pub(crate) type State<'a, S> = ParserState<'a, SwiftLanguage, S>;

pub struct SwiftParser<'config> {
    pub(crate) config: &'config SwiftLanguage,
}

impl<'config> SwiftParser<'config> {
    pub fn new(config: &'config SwiftLanguage) -> Self {
        Self { config }
    }

    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, SwiftLanguage>, OakError> {
        use crate::lexer::SwiftTokenType::*;
        let checkpoint = (0, 0);

        while state.not_at_end() && !state.at(Eof) {
            self.parse_statement(state)?
        }

        state.eat(Eof);

        Ok(state.finish_at(checkpoint, crate::parser::element_type::SwiftElementType::SourceFile))
    }

    fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::SwiftTokenType::*;
        let kind = state.peek_kind();
        // eprintln!("Parsing statement at {:?}, kind: {:?}", state.get_position(), kind);
        match kind {
            Some(Eof) | None => return Ok(()),
            Some(Func) => self.parse_function_declaration(state)?,
            Some(Var) | Some(Let) => self.parse_variable_declaration(state)?,
            Some(If) => self.parse_if_statement(state)?,
            Some(While) => self.parse_while_statement(state)?,
            Some(For) => self.parse_for_statement(state)?,
            Some(Return) => self.parse_return_statement(state)?,
            Some(LeftBrace) => self.parse_block(state)?,
            _ => {
                let cp = state.checkpoint();
                PrattParser::parse(state, 0, self);
                state.eat(Semicolon);
                state.finish_at(cp, crate::parser::element_type::SwiftElementType::ExpressionStatement);
            }
        }
        Ok(())
    }

    fn parse_function_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::SwiftTokenType::*;
        let cp = state.checkpoint();
        state.expect(Func).ok();
        state.expect(Identifier).ok();

        if state.at(LeftParen) {
            let pcp = state.checkpoint();
            state.bump(); // (
            while state.not_at_end() && !state.at(RightParen) {
                let ppcp = state.checkpoint();
                state.expect(Identifier).ok();
                if state.eat(Colon) {
                    state.expect(Identifier).ok();
                }
                state.finish_at(ppcp, crate::parser::element_type::SwiftElementType::Parameter);
                if state.eat(Comma) {
                    continue;
                }
                break;
            }
            state.expect(RightParen).ok();
            state.finish_at(pcp, crate::parser::element_type::SwiftElementType::ParameterList);
        }

        if state.eat(Arrow) {
            state.expect(Identifier).ok();
        }

        self.parse_block(state)?;
        state.finish_at(cp, crate::parser::element_type::SwiftElementType::FunctionDeclaration);
        Ok(())
    }

    fn parse_variable_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::SwiftTokenType::*;
        let cp = state.checkpoint();
        state.bump(); // var/let
        state.expect(Identifier).ok();
        if state.eat(Colon) {
            state.expect(Identifier).ok(); // Type
        }
        if state.eat(Assign) {
            PrattParser::parse(state, 0, self);
        }
        state.eat(Semicolon);
        state.finish_at(cp, crate::parser::element_type::SwiftElementType::VariableDeclaration);
        Ok(())
    }

    fn parse_if_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::SwiftTokenType::*;
        let cp = state.checkpoint();
        state.expect(If).ok();
        PrattParser::parse(state, 0, self);
        self.parse_block(state)?;
        if state.eat(Else) {
            if state.at(If) { self.parse_if_statement(state)? } else { self.parse_block(state)? }
        }
        state.finish_at(cp, crate::parser::element_type::SwiftElementType::IfStatement);
        Ok(())
    }

    fn parse_while_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::SwiftTokenType::*;
        let cp = state.checkpoint();
        state.expect(While).ok();
        PrattParser::parse(state, 0, self);
        self.parse_block(state)?;
        state.finish_at(cp, crate::parser::element_type::SwiftElementType::WhileStatement);
        Ok(())
    }

    fn parse_for_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::SwiftTokenType::*;
        let cp = state.checkpoint();
        state.expect(For).ok();
        state.expect(Identifier).ok();
        state.expect(In).ok();
        PrattParser::parse(state, 0, self);
        self.parse_block(state)?;
        state.finish_at(cp, crate::parser::element_type::SwiftElementType::ForStatement);
        Ok(())
    }

    fn parse_return_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::SwiftTokenType::*;
        let cp = state.checkpoint();
        state.expect(Return).ok();
        if state.not_at_end() && !state.at(Semicolon) && !state.at(RightBrace) {
            PrattParser::parse(state, 0, self);
        }
        state.eat(Semicolon);
        state.finish_at(cp, crate::parser::element_type::SwiftElementType::ReturnStatement);
        Ok(())
    }

    fn parse_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::SwiftTokenType::*;
        let cp = state.checkpoint();
        state.expect(LeftBrace).ok();
        while state.not_at_end() && !state.at(RightBrace) {
            self.parse_statement(state)?;
        }
        state.expect(RightBrace).ok();
        state.finish_at(cp, crate::parser::element_type::SwiftElementType::Block);
        Ok(())
    }
}

impl<'config> Pratt<SwiftLanguage> for SwiftParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, SwiftLanguage> {
        use crate::lexer::SwiftTokenType::*;
        state.skip_trivia();
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(Identifier) => {
                state.bump();
                state.finish_at(cp, crate::parser::element_type::SwiftElementType::IdentifierExpression)
            }
            Some(NumberLiteral) | Some(StringLiteral) | Some(CharLiteral) | Some(BooleanLiteral) | Some(Nil) | Some(True) | Some(False) => {
                state.bump();
                state.finish_at(cp, crate::parser::element_type::SwiftElementType::LiteralExpression)
            }
            Some(LeftParen) => {
                state.bump();
                PrattParser::parse(state, 0, self);
                state.expect(RightParen).ok();
                state.finish_at(cp, crate::parser::element_type::SwiftElementType::IdentifierExpression) // Grouping expression
            }
            _ => {
                let kind = state.peek_kind();
                eprintln!("Unsupported primary kind: {:?}", kind);
                state.bump();
                state.finish_at(cp, crate::parser::element_type::SwiftElementType::Error)
            }
        }
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, SwiftLanguage> {
        use crate::lexer::token_type::SwiftTokenType::*;
        let kind = match state.peek_kind() {
            Some(k) => k,
            None => return self.primary(state),
        };

        match kind {
            Plus | Minus | LogicalNot | BitNot => unary(state, kind, 12, UnaryExpression.into(), |s, p| PrattParser::parse(s, p, self)),
            _ => self.primary(state),
        }
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, SwiftLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, SwiftLanguage>> {
        use crate::lexer::token_type::SwiftTokenType::*;
        let kind = state.peek_kind()?;

        let (prec, assoc) = match kind {
            Assign | PlusAssign | MinusAssign | StarAssign | SlashAssign | PercentAssign | AndAssign | OrAssign | XorAssign | LeftShiftAssign | RightShiftAssign => (1, Associativity::Right),
            LogicalOr => (2, Associativity::Left),
            LogicalAnd => (3, Associativity::Left),
            Less | Greater | LessEqual | GreaterEqual | Equal | NotEqual | As | Is => (5, Associativity::Left),
            Range | ClosedRange => (6, Associativity::None),
            Plus | Minus | BitOr | BitXor => (7, Associativity::Left),
            Star | Slash | Percent | BitAnd | LeftShift | RightShift => (8, Associativity::Left),
            Dot | QuestionQuestion => (9, Associativity::Left),
            LeftParen | LeftBracket => (10, Associativity::Left),
            _ => return None,
        };

        if prec < min_precedence {
            return None;
        }

        match kind {
            Dot => {
                let cp = state.checkpoint_before(left);
                state.bump();
                state.expect(Identifier).ok();
                Some(state.finish_at(cp, crate::parser::element_type::SwiftElementType::MemberExpression))
            }
            LeftParen => {
                let cp = state.checkpoint_before(left);
                state.bump();
                while state.not_at_end() && !state.at(RightParen) {
                    PrattParser::parse(state, 0, self);
                    if !state.eat(Comma) {
                        break;
                    }
                }
                state.expect(RightParen).ok();
                Some(state.finish_at(cp, crate::parser::element_type::SwiftElementType::CallExpression))
            }
            LeftBracket => {
                let cp = state.checkpoint_before(left);
                state.bump();
                PrattParser::parse(state, 0, self);
                state.expect(RightBracket).ok();
                Some(state.finish_at(cp, crate::parser::element_type::SwiftElementType::MemberExpression))
            }
            _ => Some(binary(state, left, kind, prec, assoc, BinaryExpression.into(), |s, p| PrattParser::parse(s, p, self))),
        }
    }
}

impl<'config> Parser<SwiftLanguage> for SwiftParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<SwiftLanguage>) -> ParseOutput<'a, SwiftLanguage> {
        let lexer = crate::lexer::SwiftLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
