use crate::{RustLanguage, lexer::RustLexer};
use oak_core::{
    GreenNode, OakError,
    parser::{Associativity, ParseCache, ParseOutput, Parser, ParserState, Pratt, PrattParser, binary, parse_with_lexer, unary},
    source::{Source, TextEdit},
};

mod element_type;

pub use self::element_type::{RustElement, RustElementType};

/// A parser for the Rust programming language.
#[derive(Clone)]
pub struct RustParser<'config> {
    /// Reference to the Rust language configuration
    #[allow(dead_code)]
    config: &'config RustLanguage,
}

impl<'config> RustParser<'config> {
    pub fn new(config: &'config RustLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Pratt<RustLanguage> for RustParser<'config> {
    fn primary<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, RustLanguage, S>) -> &'a GreenNode<'a, RustLanguage> {
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(crate::lexer::RustTokenType::Identifier) => {
                state.bump();
                state.finish_at(cp, RustElementType::IdentifierExpression.into())
            }
            Some(k) if k.is_literal() => {
                state.bump();
                state.finish_at(cp, RustElementType::LiteralExpression.into())
            }
            Some(crate::lexer::RustTokenType::LeftParen) => {
                state.bump();
                PrattParser::parse(state, 0, self);
                state.expect(crate::lexer::RustTokenType::RightParen).ok();
                state.finish_at(cp, RustElementType::ParenthesizedExpression.into())
            }
            _ => {
                state.bump();
                state.finish_at(cp, RustElementType::Error.into())
            }
        }
    }

    fn prefix<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, RustLanguage, S>) -> &'a GreenNode<'a, RustLanguage> {
        use crate::{lexer::RustTokenType::*, parser::RustElementType::*};
        let kind = match state.peek_kind() {
            Some(k) => k,
            None => return self.primary(state),
        };

        match kind {
            Minus | Bang | Ampersand | Star => unary(state, kind, 13, UnaryExpression.into(), |s, p| PrattParser::parse(s, p, self)),
            _ => self.primary(state),
        }
    }

    fn infix<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, RustLanguage, S>, left: &'a GreenNode<'a, RustLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, RustLanguage>> {
        use crate::{lexer::RustTokenType::*, parser::RustElementType::*};
        let kind = state.peek_kind()?;

        let (prec, assoc) = match kind {
            Eq | PlusEq | MinusEq | StarEq | SlashEq | PercentEq | AndEq | OrEq | CaretEq | ShlEq | ShrEq => (1, Associativity::Right),
            DotDot | DotDotEq => (2, Associativity::Left),
            OrOr => (3, Associativity::Left),
            AndAnd => (4, Associativity::Left),
            EqEq | Ne => (5, Associativity::Left),
            Lt | Le | Gt | Ge => (6, Associativity::Left),
            Pipe => (7, Associativity::Left),
            Caret => (8, Associativity::Left),
            Ampersand => (9, Associativity::Left),
            Shl | Shr => (10, Associativity::Left),
            Plus | Minus => (11, Associativity::Left),
            Star | Slash | Percent => (12, Associativity::Left),
            LeftParen | LeftBracket | Dot => (14, Associativity::Left),
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
                if !state.at(RightParen) {
                    loop {
                        PrattParser::parse(state, 0, self);
                        if !state.eat(Comma) {
                            break;
                        }
                    }
                }
                state.expect(RightParen).ok();
                Some(state.finish_at(cp, CallExpression.into()))
            }
            LeftBracket => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.expect(LeftBracket).ok();
                PrattParser::parse(state, 0, self);
                state.expect(RightBracket).ok();
                Some(state.finish_at(cp, IndexExpression.into()))
            }
            Dot => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.expect(Dot).ok();
                state.expect(crate::lexer::RustTokenType::Identifier).ok();
                Some(state.finish_at(cp, FieldExpression.into()))
            }
            _ => Some(binary(state, left, kind, prec, assoc, BinaryExpression.into(), |s, p| PrattParser::parse(s, p, self))),
        }
    }
}

impl<'config> Parser<RustLanguage> for RustParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<RustLanguage>) -> ParseOutput<'a, RustLanguage> {
        let lexer = RustLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_source_file(state))
    }
}

impl<'config> RustParser<'config> {
    pub(crate) fn parse_source_file<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, RustLanguage, S>) -> Result<&'a GreenNode<'a, RustLanguage>, OakError> {
        let cp = state.checkpoint();
        while state.not_at_end() {
            if state.current().map(|t| t.kind.is_ignored()).unwrap_or(false) {
                state.advance();
                continue;
            }
            self.parse_statement(state)?;
        }
        let root = state.finish_at(cp, RustElementType::SourceFile.into());
        Ok(root)
    }

    fn parse_statement<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, RustLanguage, S>) -> Result<(), OakError> {
        use crate::{lexer::RustTokenType, parser::RustElementType::*};

        let kind = match state.peek_kind() {
            Some(RustTokenType::Fn) => Some(Function),
            Some(RustTokenType::Use) => Some(UseItem),
            Some(RustTokenType::Mod) => Some(ModuleItem),
            Some(RustTokenType::Struct) => Some(StructItem),
            Some(RustTokenType::Enum) => Some(EnumItem),
            Some(RustTokenType::Let) => Some(LetStatement),
            Some(RustTokenType::If) => Some(IfExpression),
            Some(RustTokenType::While) => Some(WhileExpression),
            Some(RustTokenType::Loop) => Some(LoopExpression),
            Some(RustTokenType::For) => Some(ForExpression),
            Some(RustTokenType::Return) => Some(ReturnStatement),
            Some(RustTokenType::LeftBrace) => Some(Block),
            _ => None,
        };

        if let Some(k) = kind {
            state.incremental_node(k.into(), |state| match k {
                Function => self.parse_function_body(state),
                UseItem => self.parse_use_item_body(state),
                ModuleItem => self.parse_mod_item_body(state),
                StructItem => self.parse_struct_item_body(state),
                EnumItem => self.parse_enum_item_body(state),
                LetStatement => self.parse_let_statement_body(state),
                IfExpression => self.parse_if_expression_body(state),
                WhileExpression => self.parse_while_expression_body(state),
                LoopExpression => self.parse_loop_expression_body(state),
                ForExpression => self.parse_for_expression_body(state),
                ReturnStatement => self.parse_return_statement_body(state),
                Block => self.parse_block_body(state),
                _ => unreachable!(),
            })
        }
        else {
            PrattParser::parse(state, 0, self);
            state.eat(RustTokenType::Semicolon);
            Ok(())
        }
    }

    fn parse_function_body<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, RustLanguage, S>) -> Result<(), OakError> {
        self.parse_function(state)
    }

    fn parse_use_item_body<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, RustLanguage, S>) -> Result<(), OakError> {
        self.parse_use_item(state)
    }

    fn parse_mod_item_body<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, RustLanguage, S>) -> Result<(), OakError> {
        self.parse_mod_item(state)
    }

    fn parse_struct_item_body<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, RustLanguage, S>) -> Result<(), OakError> {
        self.parse_struct_item(state)
    }

    fn parse_enum_item_body<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, RustLanguage, S>) -> Result<(), OakError> {
        self.parse_enum_item(state)
    }

    fn parse_let_statement_body<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, RustLanguage, S>) -> Result<(), OakError> {
        self.parse_let_statement(state)
    }

    fn parse_if_expression_body<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, RustLanguage, S>) -> Result<(), OakError> {
        self.parse_if_expression(state)
    }

    fn parse_while_expression_body<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, RustLanguage, S>) -> Result<(), OakError> {
        self.parse_while_expression(state)
    }

    fn parse_loop_expression_body<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, RustLanguage, S>) -> Result<(), OakError> {
        self.parse_loop_expression(state)
    }

    fn parse_for_expression_body<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, RustLanguage, S>) -> Result<(), OakError> {
        self.parse_for_expression(state)
    }

    fn parse_return_statement_body<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, RustLanguage, S>) -> Result<(), OakError> {
        self.parse_return_statement(state)
    }

    fn parse_block_body<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, RustLanguage, S>) -> Result<(), OakError> {
        self.parse_block(state)
    }

    fn parse_function<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, RustLanguage, S>) -> Result<(), OakError> {
        use crate::lexer::RustTokenType;
        let cp = state.checkpoint();
        state.expect(RustTokenType::Fn).ok();
        state.expect(RustTokenType::Identifier).ok();
        self.parse_param_list(state)?;
        if state.eat(RustTokenType::Arrow) {
            while state.not_at_end() && !state.at(RustTokenType::LeftBrace) {
                state.advance();
            }
        }
        self.parse_block(state)?;
        state.finish_at(cp, RustElementType::Function.into());
        Ok(())
    }

    fn parse_param_list<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, RustLanguage, S>) -> Result<(), OakError> {
        use crate::lexer::RustTokenType::*;
        let cp = state.checkpoint();
        state.expect(LeftParen).ok();
        while state.not_at_end() && !state.at(RightParen) {
            state.advance();
        }
        state.expect(RightParen).ok();
        state.finish_at(cp, RustElementType::ParameterList.into());
        Ok(())
    }

    fn parse_block<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, RustLanguage, S>) -> Result<(), OakError> {
        use crate::lexer::RustTokenType::*;
        let cp = state.checkpoint();
        state.expect(LeftBrace).ok();
        while state.not_at_end() && !state.at(RightBrace) {
            self.parse_statement(state)?;
        }
        state.expect(RightBrace).ok();
        state.finish_at(cp, RustElementType::BlockExpression.into());
        Ok(())
    }

    fn parse_use_item<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, RustLanguage, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(crate::lexer::RustTokenType::Use).ok();
        // 简化处理路径
        while !state.at(crate::lexer::RustTokenType::Semicolon) && state.not_at_end() {
            state.bump();
        }
        state.eat(crate::lexer::RustTokenType::Semicolon);
        state.finish_at(cp, RustElementType::UseItem.into());
        Ok(())
    }

    fn parse_mod_item<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, RustLanguage, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // mod
        state.expect(crate::lexer::RustTokenType::Identifier).ok();
        if state.at(crate::lexer::RustTokenType::LeftBrace) {
            self.parse_block(state)?;
        }
        else {
            state.eat(crate::lexer::RustTokenType::Semicolon);
        }
        state.finish_at(cp, RustElementType::ModuleItem.into());
        Ok(())
    }

    fn parse_struct_item<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, RustLanguage, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // struct
        state.expect(crate::lexer::RustTokenType::Identifier).ok();
        while state.not_at_end() && !state.at(crate::lexer::RustTokenType::LeftBrace) && !state.at(crate::lexer::RustTokenType::Semicolon) {
            state.advance();
        }
        if state.at(crate::lexer::RustTokenType::LeftBrace) {
            self.parse_block(state)?;
        }
        else {
            state.eat(crate::lexer::RustTokenType::Semicolon);
        }
        state.finish_at(cp, RustElementType::StructItem.into());
        Ok(())
    }

    fn parse_enum_item<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, RustLanguage, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // enum
        state.expect(crate::lexer::RustTokenType::Identifier).ok();
        self.parse_block(state)?;
        state.finish_at(cp, RustElementType::EnumItem.into());
        Ok(())
    }

    fn parse_let_statement<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, RustLanguage, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // let
        state.expect(crate::lexer::RustTokenType::Identifier).ok();
        if state.eat(crate::lexer::RustTokenType::Eq) {
            PrattParser::parse(state, 0, self);
        }
        state.eat(crate::lexer::RustTokenType::Semicolon);
        state.finish_at(cp, RustElementType::LetStatement.into());
        Ok(())
    }

    fn parse_if_expression<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, RustLanguage, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // if
        PrattParser::parse(state, 0, self);
        self.parse_block(state)?;
        if state.eat(crate::lexer::RustTokenType::Else) {
            if state.at(crate::lexer::RustTokenType::If) {
                self.parse_if_expression(state)?;
            }
            else {
                self.parse_block(state)?;
            }
        }
        state.finish_at(cp, RustElementType::IfExpression.into());
        Ok(())
    }

    fn parse_while_expression<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, RustLanguage, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // while
        PrattParser::parse(state, 0, self);
        self.parse_block(state)?;
        state.finish_at(cp, RustElementType::WhileExpression.into());
        Ok(())
    }

    fn parse_loop_expression<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, RustLanguage, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // loop
        self.parse_block(state)?;
        state.finish_at(cp, RustElementType::LoopExpression.into());
        Ok(())
    }

    fn parse_for_expression<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, RustLanguage, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // for
        state.expect(crate::lexer::RustTokenType::Identifier).ok();
        state.expect(crate::lexer::RustTokenType::In).ok();
        PrattParser::parse(state, 0, self);
        self.parse_block(state)?;
        state.finish_at(cp, RustElementType::ForExpression.into());
        Ok(())
    }

    fn parse_return_statement<'a, S: oak_core::source::Source + ?Sized>(&self, state: &mut ParserState<'a, RustLanguage, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // return
        if !state.at(crate::lexer::RustTokenType::Semicolon) {
            PrattParser::parse(state, 0, self);
        }
        state.eat(crate::lexer::RustTokenType::Semicolon);
        state.finish_at(cp, RustElementType::ReturnStatement.into());
        Ok(())
    }
}
