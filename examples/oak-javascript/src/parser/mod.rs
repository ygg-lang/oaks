//! JavaScript parser implementation.

pub mod element_type;

use crate::{language::JavaScriptLanguage, parser::element_type::JavaScriptElementType};
use oak_core::{
    GreenNode, OakError, ParseCache, TextEdit,
    parser::{Associativity, Parser, ParserState, Pratt, PrattParser, binary},
    source::Source,
};

pub(crate) type State<'a, S> = ParserState<'a, JavaScriptLanguage, S>;

/// JavaScript parser.
pub struct JavaScriptParser<'config> {
    /// Configuration for the JavaScript language.
    pub(crate) config: &'config JavaScriptLanguage,
}

impl<'config> JavaScriptParser<'config> {
    /// Creates a new JavaScript parser.
    pub fn new(config: &'config JavaScriptLanguage) -> Self {
        Self { config }
    }

    fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::JavaScriptTokenType::*;
        match state.peek_kind() {
            Some(Function) => self.parse_function_declaration(state)?,
            Some(Var) | Some(Let) | Some(Const) => self.parse_variable_declaration(state)?,
            Some(If) => self.parse_if_statement(state)?,
            Some(While) => self.parse_while_statement(state)?,
            Some(For) => self.parse_for_statement(state)?,
            Some(Return) => self.parse_return_statement(state)?,
            Some(LeftBrace) => self.parse_block_statement(state)?,
            _ => {
                PrattParser::parse(state, 0, self);
                state.eat(Semicolon);
            }
        }
        Ok(())
    }

    fn parse_function_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::JavaScriptTokenType::*;
        let cp = state.checkpoint();
        state.expect(Function).ok();
        state.eat(IdentifierName);
        state.expect(LeftParen).ok();
        while state.not_at_end() && !state.at(RightParen) {
            state.advance()
        }
        state.expect(RightParen).ok();
        self.parse_block_statement(state)?;
        state.finish_at(cp, crate::parser::element_type::JavaScriptElementType::FunctionDeclaration);
        Ok(())
    }

    fn parse_variable_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::JavaScriptTokenType::*;
        let cp = state.checkpoint();
        state.bump(); // var/let/const
        state.expect(IdentifierName).ok();
        if state.eat(Equal) {
            PrattParser::parse(state, 0, self);
        }
        state.eat(Semicolon);
        state.finish_at(cp, crate::parser::element_type::JavaScriptElementType::VariableDeclaration);
        Ok(())
    }

    fn parse_if_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::JavaScriptTokenType::*;
        let cp = state.checkpoint();
        state.expect(If).ok();
        state.expect(LeftParen).ok();
        PrattParser::parse(state, 0, self);
        state.expect(RightParen).ok();
        self.parse_statement(state)?;
        if state.eat(Else) {
            self.parse_statement(state)?
        }
        state.finish_at(cp, crate::parser::element_type::JavaScriptElementType::IfStatement);
        Ok(())
    }

    fn parse_while_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::JavaScriptTokenType::*;
        let cp = state.checkpoint();
        state.expect(While).ok();
        state.expect(LeftParen).ok();
        PrattParser::parse(state, 0, self);
        state.expect(RightParen).ok();
        self.parse_statement(state)?;
        state.finish_at(cp, crate::parser::element_type::JavaScriptElementType::WhileStatement);
        Ok(())
    }

    fn parse_for_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::JavaScriptTokenType::*;
        let cp = state.checkpoint();
        state.expect(For).ok();
        state.expect(LeftParen).ok();
        // Simplified handling
        while state.not_at_end() && !state.at(RightParen) {
            state.advance()
        }
        state.expect(RightParen).ok();
        self.parse_statement(state)?;
        state.finish_at(cp, crate::parser::element_type::JavaScriptElementType::ForStatement);
        Ok(())
    }

    fn parse_return_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::JavaScriptTokenType::*;
        let cp = state.checkpoint();
        state.expect(Return).ok();
        if !state.at(Semicolon) {
            PrattParser::parse(state, 0, self);
        }
        state.eat(Semicolon);
        state.finish_at(cp, crate::parser::element_type::JavaScriptElementType::ReturnStatement);
        Ok(())
    }

    fn parse_block_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::JavaScriptTokenType::*;
        let cp = state.checkpoint();
        state.expect(LeftBrace).ok();
        while state.not_at_end() && !state.at(RightBrace) {
            self.parse_statement(state)?
        }
        state.expect(RightBrace).ok();
        state.finish_at(cp, crate::parser::element_type::JavaScriptElementType::BlockStatement);
        Ok(())
    }
}

impl<'config> Pratt<JavaScriptLanguage> for JavaScriptParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, JavaScriptLanguage> {
        use crate::lexer::token_type::JavaScriptTokenType::*;
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(IdentifierName) => {
                state.bump();
                state.finish_at(cp, crate::parser::element_type::JavaScriptElementType::Identifier)
            }
            Some(NumericLiteral) | Some(StringLiteral) | Some(True) | Some(False) | Some(Null) => {
                state.bump();
                state.finish_at(cp, crate::parser::element_type::JavaScriptElementType::Literal)
            }
            Some(LeftParen) => {
                state.bump();
                PrattParser::parse(state, 0, self);
                state.expect(RightParen).ok();
                state.finish_at(cp, crate::parser::element_type::JavaScriptElementType::Expression)
            }
            _ => {
                state.bump();
                state.finish_at(cp, crate::parser::element_type::JavaScriptElementType::Error)
            }
        }
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, JavaScriptLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, JavaScriptLanguage>> {
        use crate::lexer::token_type::JavaScriptTokenType::*;
        let kind = state.peek_kind()?;

        let (prec, assoc) = match kind {
            Equal
            | PlusEqual
            | MinusEqual
            | StarEqual
            | SlashEqual
            | PercentEqual
            | StarStarEqual
            | LeftShiftEqual
            | RightShiftEqual
            | UnsignedRightShiftEqual
            | AmpersandEqual
            | PipeEqual
            | CaretEqual
            | AmpersandAmpersandEqual
            | PipePipeEqual
            | QuestionQuestionEqual => (1, Associativity::Right),
            PipePipe => (2, Associativity::Left),
            AmpersandAmpersand => (3, Associativity::Left),
            EqualEqual | NotEqual | EqualEqualEqual | NotEqualEqual => (4, Associativity::Left),
            Plus | Minus => (10, Associativity::Left),
            Star | Slash | Percent => (11, Associativity::Left),
            LeftParen | Dot => (15, Associativity::Left),
            _ => return None,
        };

        if prec < min_precedence {
            return None;
        }

        match kind {
            LeftParen => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.expect(LeftParen).ok();
                while state.not_at_end() && !state.at(RightParen) {
                    state.advance()
                }
                state.expect(RightParen).ok();
                Some(state.finish_at(cp, crate::parser::element_type::JavaScriptElementType::CallExpression))
            }
            Dot => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.expect(Dot).ok();
                state.expect(IdentifierName).ok();
                Some(state.finish_at(cp, crate::parser::element_type::JavaScriptElementType::MemberExpression))
            }
            _ => {
                let result_kind = match kind {
                    Equal
                    | PlusEqual
                    | MinusEqual
                    | StarEqual
                    | SlashEqual
                    | PercentEqual
                    | StarStarEqual
                    | LeftShiftEqual
                    | RightShiftEqual
                    | UnsignedRightShiftEqual
                    | AmpersandEqual
                    | PipeEqual
                    | CaretEqual
                    | AmpersandAmpersandEqual
                    | PipePipeEqual
                    | QuestionQuestionEqual => JavaScriptElementType::AssignmentExpression,
                    PipePipe | AmpersandAmpersand => JavaScriptElementType::LogicalExpression,
                    _ => JavaScriptElementType::BinaryExpression,
                };
                Some(binary(state, left, kind, prec, assoc, result_kind.into(), |s, p| PrattParser::parse(s, p, self)))
            }
        }
    }
}

impl<'config> JavaScriptParser<'config> {
    fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, JavaScriptLanguage>, OakError> {
        let cp = state.checkpoint();
        while state.not_at_end() {
            self.parse_statement(state).ok();
        }
        Ok(state.finish_at(cp, crate::parser::element_type::JavaScriptElementType::Root))
    }
}

impl<'config> Parser<JavaScriptLanguage> for JavaScriptParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<JavaScriptLanguage>) -> oak_core::parser::ParseOutput<'a, JavaScriptLanguage> {
        let lexer = crate::lexer::JavaScriptLexer::new(&self.config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
