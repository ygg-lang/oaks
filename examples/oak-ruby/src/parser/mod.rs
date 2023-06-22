use crate::{kind::RubySyntaxKind, language::RubyLanguage, lexer::RubyLexer};
use oak_core::{
    GreenNode, OakError, TextEdit,
    parser::{
        ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer,
        pratt::{Associativity, Pratt, PrattParser, binary, unary},
    },
    source::Source,
};

pub(crate) type State<'a, S> = ParserState<'a, RubyLanguage, S>;

pub struct RubyParser<'config> {
    pub(crate) config: &'config RubyLanguage,
}

impl<'config> RubyParser<'config> {
    pub fn new(config: &'config RubyLanguage) -> Self {
        Self { config }
    }

    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, RubyLanguage>, OakError> {
        let cp = state.checkpoint();
        while state.not_at_end() {
            if state.peek_kind().map(|k| k.is_ignored()).unwrap_or(false) {
                state.bump();
                continue;
            }
            self.parse_statement(state).ok();
        }
        Ok(state.finish_at(cp, RubySyntaxKind::Root))
    }

    fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::RubySyntaxKind::*;
        match state.peek_kind() {
            Some(Def) => self.parse_method_def(state)?,
            Some(Class) => self.parse_class_def(state)?,
            Some(Module) => self.parse_module_def(state)?,
            Some(If) => self.parse_if_stmt(state)?,
            Some(While) => self.parse_while_stmt(state)?,
            Some(Return) => self.parse_return_stmt(state)?,
            _ => {
                PrattParser::parse(state, 0, self);
                state.eat(Semicolon);
                state.eat(Newline);
            }
        }
        Ok(())
    }

    fn parse_method_def<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::RubySyntaxKind::*;
        let cp = state.checkpoint();
        state.bump(); // def
        state.expect(Identifier).ok();
        if state.eat(LeftParen) {
            while state.not_at_end() && !state.at(RightParen) {
                state.advance();
            }
            state.expect(RightParen).ok();
        }
        self.parse_body(state)?;
        state.finish_at(cp, MethodDefinition);
        Ok(())
    }

    fn parse_body<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::RubySyntaxKind::*;
        while state.not_at_end() && !state.at(End) && !state.at(Else) && !state.at(Elsif) && !state.at(Rescue) && !state.at(Ensure) {
            self.parse_statement(state)?;
        }
        state.eat(End);
        Ok(())
    }

    fn parse_class_def<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::RubySyntaxKind::*;
        let cp = state.checkpoint();
        state.bump(); // class
        state.expect(Constant).ok();
        self.parse_body(state)?;
        state.finish_at(cp, ClassDefinition);
        Ok(())
    }

    fn parse_module_def<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::RubySyntaxKind::*;
        let cp = state.checkpoint();
        state.bump(); // module
        state.expect(Constant).ok();
        self.parse_body(state)?;
        state.finish_at(cp, ModuleDefinition);
        Ok(())
    }

    fn parse_if_stmt<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::RubySyntaxKind::*;
        let cp = state.checkpoint();
        state.bump(); // if
        PrattParser::parse(state, 0, self);
        self.parse_body(state)?;
        state.finish_at(cp, IfStatement);
        Ok(())
    }

    fn parse_while_stmt<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::RubySyntaxKind::*;
        let cp = state.checkpoint();
        state.bump(); // while
        PrattParser::parse(state, 0, self);
        self.parse_body(state)?;
        state.finish_at(cp, WhileStatement);
        Ok(())
    }

    fn parse_return_stmt<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::kind::RubySyntaxKind::*;
        let cp = state.checkpoint();
        state.bump(); // return
        PrattParser::parse(state, 0, self);
        state.finish_at(cp, ReturnStatement);
        Ok(())
    }
}

impl<'config> Pratt<RubyLanguage> for RubyParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, RubyLanguage> {
        use crate::kind::RubySyntaxKind::*;
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(Identifier) | Some(Constant) | Some(GlobalVariable) | Some(InstanceVariable) | Some(ClassVariable) => {
                state.bump();
                state.finish_at(cp, Identifier)
            }
            Some(IntegerLiteral) | Some(FloatLiteral) | Some(StringLiteral) | Some(True) | Some(False) | Some(Nil) | Some(Self_) => {
                state.bump();
                state.finish_at(cp, LiteralExpression) // 简化处理
            }
            Some(LeftParen) => {
                state.bump();
                PrattParser::parse(state, 0, self);
                state.expect(RightParen).ok();
                state.finish_at(cp, ParenExpression) // 简化处理
            }
            _ => {
                state.bump();
                state.finish_at(cp, Error)
            }
        }
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, RubyLanguage> {
        use crate::kind::RubySyntaxKind::*;
        match state.peek_kind() {
            Some(kind @ (Plus | Minus | Not | Tilde)) => {
                state.bump();
                unary(state, kind, 13, UnaryExpression, |st, p| PrattParser::parse(st, p, self))
            }
            _ => self.primary(state),
        }
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, RubyLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, RubyLanguage>> {
        use crate::kind::RubySyntaxKind::*;
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

        Some(binary(state, left, kind, prec, assoc, BinaryExpression, |s, p| PrattParser::parse(s, p, self)))
    }
}

impl<'config> Parser<RubyLanguage> for RubyParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<RubyLanguage>) -> ParseOutput<'a, RubyLanguage> {
        let lexer = RubyLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
