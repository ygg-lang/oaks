use oak_core::TokenType;
/// Element types for the Ruby language.
pub mod element_type;

use crate::{
    language::RubyLanguage,
    lexer::{RubyLexer, token_type::RubyTokenType},
};
use oak_core::{
    GreenNode, OakError, TextEdit,
    parser::{
        ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer,
        pratt::{Associativity, Pratt, PrattParser, binary, unary},
    },
    source::Source,
};

pub(crate) type State<'a, S> = ParserState<'a, RubyLanguage, S>;

/// A parser for the Ruby language.
pub struct RubyParser<'config> {
    pub(crate) config: &'config RubyLanguage,
}

impl<'config> RubyParser<'config> {
    /// Creates a new `RubyParser` with the given configuration.
    pub fn new(config: &'config RubyLanguage) -> Self {
        Self { config }
    }

    fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::RubyTokenType::*;
        match state.peek_kind() {
            Some(Def) => self.parse_method_def(state)?,
            Some(Class) => self.parse_class_def(state)?,
            Some(Module) => self.parse_module_def(state)?,
            Some(If) => self.parse_if_stmt(state)?,
            Some(Unless) => self.parse_unless_stmt(state)?,
            Some(While) => self.parse_while_stmt(state)?,
            Some(Until) => self.parse_until_stmt(state)?,
            Some(For) => self.parse_for_stmt(state)?,
            Some(Case) => self.parse_case_stmt(state)?,
            Some(Begin) => self.parse_begin_stmt(state)?,
            Some(Return) => self.parse_return_stmt(state)?,
            _ => {
                PrattParser::parse(state, 0, self);
                state.eat(Semicolon);
                state.eat(Newline);
            }
        }
        Ok(())
    }

    fn parse_unless_stmt<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::RubyTokenType::*;
        let cp = state.checkpoint();
        state.bump(); // unless
        PrattParser::parse(state, 0, self);
        self.parse_body(state)?;
        state.finish_at(cp, crate::parser::element_type::RubyElementType::UnlessStatement);
        Ok(())
    }

    fn parse_until_stmt<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::RubyTokenType::*;
        let cp = state.checkpoint();
        state.bump(); // until
        PrattParser::parse(state, 0, self);
        self.parse_body(state)?;
        state.finish_at(cp, crate::parser::element_type::RubyElementType::UntilStatement);
        Ok(())
    }

    fn parse_for_stmt<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::RubyTokenType::*;
        let cp = state.checkpoint();
        state.bump(); // for
        state.expect(Identifier).ok();
        state.expect(In).ok();
        PrattParser::parse(state, 0, self);
        self.parse_body(state)?;
        state.finish_at(cp, crate::parser::element_type::RubyElementType::ForStatement);
        Ok(())
    }

    fn parse_case_stmt<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::RubyTokenType::*;
        let cp = state.checkpoint();
        state.bump(); // case
        if !state.at(When) {
            PrattParser::parse(state, 0, self);
        }

        while state.at(When) {
            let when_cp = state.checkpoint();
            state.bump(); // when
            PrattParser::parse(state, 0, self);
            while state.eat(Comma) {
                PrattParser::parse(state, 0, self);
            }
            state.eat(Then);
            self.parse_case_body(state)?;
            state.finish_at(when_cp, crate::parser::element_type::RubyElementType::WhenClause);
        }

        if state.eat(Else) {
            self.parse_case_body(state)?;
        }

        state.expect(End).ok();
        state.finish_at(cp, crate::parser::element_type::RubyElementType::CaseStatement);
        Ok(())
    }

    fn parse_case_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::RubyTokenType::*;
        while state.not_at_end() && !state.at(End) && !state.at(When) && !state.at(Else) {
            self.parse_statement(state)?
        }
        Ok(())
    }

    fn parse_begin_stmt<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::RubyTokenType::*;
        let cp = state.checkpoint();
        state.bump(); // begin

        while state.not_at_end() && !state.at(End) && !state.at(Rescue) && !state.at(Ensure) && !state.at(Else) {
            self.parse_statement(state)?
        }

        while state.at(Rescue) {
            let rescue_cp = state.checkpoint();
            state.bump(); // rescue
            if !state.at(Then) && !state.at(Newline) && !state.at(Semicolon) {
                PrattParser::parse(state, 0, self); // Exception class
                if state.at(EqualGreater) {
                    state.bump();
                    state.expect(Identifier).ok();
                }
            }
            state.eat(Then);
            while state.not_at_end() && !state.at(End) && !state.at(Rescue) && !state.at(Ensure) && !state.at(Else) {
                self.parse_statement(state)?
            }
            state.finish_at(rescue_cp, crate::parser::element_type::RubyElementType::RescueClause);
        }

        if state.eat(Else) {
            while state.not_at_end() && !state.at(End) && !state.at(Ensure) {
                self.parse_statement(state)?
            }
        }

        if state.at(Ensure) {
            let ensure_cp = state.checkpoint();
            state.bump(); // ensure
            while state.not_at_end() && !state.at(End) {
                self.parse_statement(state)?
            }
            state.finish_at(ensure_cp, crate::parser::element_type::RubyElementType::EnsureClause);
        }

        state.expect(End).ok();
        state.finish_at(cp, crate::parser::element_type::RubyElementType::BeginStatement);
        Ok(())
    }

    fn parse_method_def<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::RubyTokenType::*;
        let cp = state.checkpoint();
        state.bump(); // def
        state.expect(Identifier).ok();
        if state.eat(LeftParen) {
            while state.not_at_end() && !state.at(RightParen) {
                state.advance()
            }
            let _ = state.expect(RightParen);
        }
        self.parse_body(state)?;
        state.finish_at(cp, crate::parser::element_type::RubyElementType::MethodDefinition);
        Ok(())
    }

    fn parse_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::RubyTokenType::*;
        while state.not_at_end() && !state.at(End) && !state.at(Else) && !state.at(Elsif) && !state.at(Rescue) && !state.at(Ensure) {
            self.parse_statement(state)?
        }
        state.eat(End);
        Ok(())
    }

    fn parse_class_def<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::RubyTokenType::*;
        let cp = state.checkpoint();
        state.bump(); // class
        state.expect(Constant).ok();
        self.parse_body(state)?;
        state.finish_at(cp, crate::parser::element_type::RubyElementType::ClassDefinition);
        Ok(())
    }

    fn parse_module_def<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::RubyTokenType::*;
        let cp = state.checkpoint();
        state.bump(); // module
        state.expect(Constant).ok();
        self.parse_body(state)?;
        state.finish_at(cp, crate::parser::element_type::RubyElementType::ModuleDefinition);
        Ok(())
    }

    fn parse_if_stmt<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::RubyTokenType::*;
        let cp = state.checkpoint();
        state.bump(); // if
        PrattParser::parse(state, 0, self);
        self.parse_body(state)?;
        state.finish_at(cp, crate::parser::element_type::RubyElementType::IfStatement);
        Ok(())
    }

    fn parse_while_stmt<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::RubyTokenType::*;
        let cp = state.checkpoint();
        state.bump(); // while
        PrattParser::parse(state, 0, self);
        self.parse_body(state)?;
        state.finish_at(cp, crate::parser::element_type::RubyElementType::WhileStatement);
        Ok(())
    }

    fn parse_return_stmt<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::RubyTokenType::*;
        let cp = state.checkpoint();
        state.bump(); // return
        PrattParser::parse(state, 0, self);
        state.finish_at(cp, crate::parser::element_type::RubyElementType::ReturnStatement);
        Ok(())
    }
}

impl<'config> Pratt<RubyLanguage> for RubyParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, RubyLanguage> {
        use crate::lexer::token_type::RubyTokenType::*;
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(Identifier) | Some(Constant) | Some(GlobalVariable) | Some(InstanceVariable) | Some(ClassVariable) => {
                state.bump();
                state.finish_at(cp, crate::parser::element_type::RubyElementType::Identifier)
            }
            Some(IntegerLiteral) | Some(FloatLiteral) | Some(StringLiteral) | Some(True) | Some(False) | Some(Nil) | Some(Self_) => {
                state.bump();
                state.finish_at(cp, crate::parser::element_type::RubyElementType::LiteralExpression) // Simplified handling
            }
            Some(LeftParen) => {
                state.bump();
                PrattParser::parse(state, 0, self);
                state.expect(RightParen).ok();
                state.finish_at(cp, crate::parser::element_type::RubyElementType::ParenExpression) // Simplified handling
            }
            _ => {
                state.bump();
                state.finish_at(cp, crate::parser::element_type::RubyElementType::Error)
            }
        }
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, RubyLanguage> {
        use crate::lexer::token_type::RubyTokenType::*;
        match state.peek_kind() {
            Some(kind @ (Plus | Minus | Not | Tilde)) => {
                state.bump();
                unary(state, kind, 13, crate::parser::element_type::RubyElementType::UnaryExpression, |st, p| PrattParser::parse(st, p, self))
            }
            _ => self.primary(state),
        }
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, RubyLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, RubyLanguage>> {
        use crate::lexer::token_type::RubyTokenType::*;
        let kind = state.peek_kind()?;

        let (prec, assoc) = match kind {
            Power => (30, Associativity::Right),
            Multiply | Divide | Modulo => (20, Associativity::Left),
            Plus | Minus => (10, Associativity::Left),
            EqualEqual | NotEqual | Less | Greater | LessEqual | GreaterEqual => (5, Associativity::Left),
            AndAnd => (2, Associativity::Left),
            OrOr => (1, Associativity::Left),
            _ => return None,
        };

        if prec < min_precedence {
            return None;
        }

        Some(binary(state, left, kind, prec, assoc, crate::parser::element_type::RubyElementType::BinaryExpression, |s, p| PrattParser::parse(s, p, self)))
    }
}

impl<'config> Parser<RubyLanguage> for RubyParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<RubyLanguage>) -> ParseOutput<'a, RubyLanguage> {
        let lexer = RubyLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let cp = state.checkpoint();
            while state.not_at_end() {
                if state.peek_kind().map(|k| k.is_ignored()).unwrap_or(false) {
                    state.bump();
                    continue;
                }
                let _ = self.parse_statement(state);
            }
            Ok(state.finish_at(cp, crate::parser::element_type::RubyElementType::Root))
        })
    }
}
