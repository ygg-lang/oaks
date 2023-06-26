//! Parser implementation for the D language.

pub mod element_type;

use crate::{
    language::DLanguage,
    lexer::{DLexer, token_type::DTokenType},
};
use oak_core::{
    GreenNode, TextEdit,
    parser::{Associativity, Parser, ParserState, Pratt, PrattParser},
    source::Source,
};

/// The state of the D parser.
pub(crate) type State<'a, S> = ParserState<'a, DLanguage, S>;

/// A parser for the D language.
pub struct DParser<'config> {
    pub(crate) config: &'config DLanguage,
}

impl<'config> DParser<'config> {
    /// Creates a new D parser.
    pub fn new(config: &'config DLanguage) -> Self {
        Self { config }
    }

    /// Skips trivia tokens (whitespace, comments, newlines).
    fn skip_trivia<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        while let Some(kind) = state.peek_kind() {
            if matches!(kind, DTokenType::Whitespace | DTokenType::Newline | DTokenType::LineComment | DTokenType::BlockComment | DTokenType::NestedComment) {
                state.bump();
            }
            else {
                break;
            }
        }
    }

    /// Parses a module declaration.
    fn parse_module<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        let cp = state.checkpoint();
        state.bump(); // module
        self.skip_trivia(state);
        // Parse module name
        while let Some(kind) = state.peek_kind() {
            if matches!(kind, DTokenType::Identifier | DTokenType::Dot) {
                state.bump();
            }
            else {
                break;
            }
        }
        state.eat(DTokenType::Semicolon);
        state.finish_at(cp, crate::parser::element_type::DElementType::Module);
        Ok(())
    }

    /// Parses an import declaration.
    fn parse_import<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        let cp = state.checkpoint();
        state.bump(); // import
        self.skip_trivia(state);
        // Parse import path
        while let Some(kind) = state.peek_kind() {
            if matches!(kind, DTokenType::Identifier | DTokenType::Dot) {
                state.bump();
            }
            else {
                break;
            }
        }
        state.eat(DTokenType::Semicolon);
        state.finish_at(cp, crate::parser::element_type::DElementType::Import);
        Ok(())
    }

    /// Parses a class definition.
    fn parse_class<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        let cp = state.checkpoint();
        state.bump(); // class
        self.skip_trivia(state);
        // Parse class name
        if state.at(DTokenType::Identifier) {
            state.bump();
        }
        // Parse base classes
        self.skip_trivia(state);
        if state.at(DTokenType::Colon) {
            state.bump();
            while let Some(kind) = state.peek_kind() {
                if !matches!(kind, DTokenType::Semicolon | DTokenType::LeftBrace) {
                    state.bump();
                }
                else {
                    break;
                }
            }
        }
        // Parse class body
        self.skip_trivia(state);
        if state.at(DTokenType::LeftBrace) {
            state.bump();
            while let Some(kind) = state.peek_kind() {
                if !matches!(kind, DTokenType::RightBrace) {
                    self.parse_statement(state)?;
                }
                else {
                    break;
                }
            }
            state.eat(DTokenType::RightBrace);
        }
        state.eat(DTokenType::Semicolon);
        state.finish_at(cp, crate::parser::element_type::DElementType::Class);
        Ok(())
    }

    /// Parses a struct definition.
    fn parse_struct<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        let cp = state.checkpoint();
        state.bump(); // struct
        self.skip_trivia(state);
        // Parse struct name
        if state.at(DTokenType::Identifier) {
            state.bump();
        }
        // Parse struct body
        self.skip_trivia(state);
        if state.at(DTokenType::LeftBrace) {
            state.bump();
            while let Some(kind) = state.peek_kind() {
                if !matches!(kind, DTokenType::RightBrace) {
                    self.parse_statement(state)?;
                }
                else {
                    break;
                }
            }
            state.eat(DTokenType::RightBrace);
        }
        state.eat(DTokenType::Semicolon);
        state.finish_at(cp, crate::parser::element_type::DElementType::Struct);
        Ok(())
    }

    /// Parses an interface definition.
    fn parse_interface<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        let cp = state.checkpoint();
        state.bump(); // interface
        self.skip_trivia(state);
        // Parse interface name
        if state.at(DTokenType::Identifier) {
            state.bump();
        }
        // Parse base interfaces
        self.skip_trivia(state);
        if state.at(DTokenType::Colon) {
            state.bump();
            while let Some(kind) = state.peek_kind() {
                if !matches!(kind, DTokenType::Semicolon | DTokenType::LeftBrace) {
                    state.bump();
                }
                else {
                    break;
                }
            }
        }
        // Parse interface body
        self.skip_trivia(state);
        if state.at(DTokenType::LeftBrace) {
            state.bump();
            while let Some(kind) = state.peek_kind() {
                if !matches!(kind, DTokenType::RightBrace) {
                    self.parse_statement(state)?;
                }
                else {
                    break;
                }
            }
            state.eat(DTokenType::RightBrace);
        }
        state.eat(DTokenType::Semicolon);
        state.finish_at(cp, crate::parser::element_type::DElementType::Interface);
        Ok(())
    }

    /// Parses a function definition.
    fn parse_function<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        let cp = state.checkpoint();
        state.bump(); // function
        self.skip_trivia(state);
        // Parse return type
        while let Some(kind) = state.peek_kind() {
            if !matches!(kind, DTokenType::Identifier) {
                break;
            }
            state.bump();
        }
        // Parse function name
        if state.at(DTokenType::Identifier) {
            state.bump();
        }
        // Parse parameters
        self.skip_trivia(state);
        if state.at(DTokenType::LeftParen) {
            state.bump();
            while let Some(kind) = state.peek_kind() {
                if !matches!(kind, DTokenType::RightParen) {
                    self.skip_trivia(state);
                    // Parse parameter
                    while let Some(kind) = state.peek_kind() {
                        if !matches!(kind, DTokenType::Comma | DTokenType::RightParen) {
                            state.bump();
                        }
                        else {
                            break;
                        }
                    }
                    if state.at(DTokenType::Comma) {
                        state.bump();
                    }
                }
                else {
                    break;
                }
            }
            state.eat(DTokenType::RightParen);
        }
        // Parse function body
        self.skip_trivia(state);
        if state.at(DTokenType::LeftBrace) {
            state.bump();
            while let Some(kind) = state.peek_kind() {
                if !matches!(kind, DTokenType::RightBrace) {
                    self.parse_statement(state)?;
                }
                else {
                    break;
                }
            }
            state.eat(DTokenType::RightBrace);
        }
        state.eat(DTokenType::Semicolon);
        state.finish_at(cp, crate::parser::element_type::DElementType::Function);
        Ok(())
    }

    /// Parses a statement.
    fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        self.skip_trivia(state);
        match state.peek_kind() {
            Some(DTokenType::ModuleKeyword) => self.parse_module(state)?,
            Some(DTokenType::ImportKeyword) => self.parse_import(state)?,
            Some(DTokenType::ClassKeyword) => self.parse_class(state)?,
            Some(DTokenType::StructKeyword) => self.parse_struct(state)?,
            Some(DTokenType::InterfaceKeyword) => self.parse_interface(state)?,
            Some(DTokenType::FunctionKeyword) => self.parse_function(state)?,
            Some(DTokenType::LeftBrace) => {
                // Compound statement
                let cp = state.checkpoint();
                state.bump();
                while let Some(kind) = state.peek_kind() {
                    if !matches!(kind, DTokenType::RightBrace) {
                        self.parse_statement(state)?;
                    }
                    else {
                        break;
                    }
                }
                state.eat(DTokenType::RightBrace);
                state.finish_at(cp, crate::parser::element_type::DElementType::Block);
            }
            Some(DTokenType::IfKeyword) => {
                // If statement
                let cp = state.checkpoint();
                state.bump();
                self.skip_trivia(state);
                if state.at(DTokenType::LeftParen) {
                    state.bump();
                    while let Some(kind) = state.peek_kind() {
                        if !matches!(kind, DTokenType::RightParen) {
                            state.bump();
                        }
                        else {
                            break;
                        }
                    }
                    state.eat(DTokenType::RightParen);
                }
                self.parse_statement(state)?;
                self.skip_trivia(state);
                if state.at(DTokenType::ElseKeyword) {
                    state.bump();
                    self.parse_statement(state)?;
                }
                state.finish_at(cp, crate::parser::element_type::DElementType::IfStatement);
            }
            Some(DTokenType::WhileKeyword) => {
                // While statement
                let cp = state.checkpoint();
                state.bump();
                self.skip_trivia(state);
                if state.at(DTokenType::LeftParen) {
                    state.bump();
                    while let Some(kind) = state.peek_kind() {
                        if !matches!(kind, DTokenType::RightParen) {
                            state.bump();
                        }
                        else {
                            break;
                        }
                    }
                    state.eat(DTokenType::RightParen);
                }
                self.parse_statement(state)?;
                state.finish_at(cp, crate::parser::element_type::DElementType::WhileStatement);
            }
            Some(DTokenType::ForKeyword) => {
                // For statement
                let cp = state.checkpoint();
                state.bump();
                self.skip_trivia(state);
                if state.at(DTokenType::LeftParen) {
                    state.bump();
                    // Initialization
                    while let Some(kind) = state.peek_kind() {
                        if !matches!(kind, DTokenType::Semicolon) {
                            state.bump();
                        }
                        else {
                            break;
                        }
                    }
                    state.eat(DTokenType::Semicolon);
                    // Condition
                    while let Some(kind) = state.peek_kind() {
                        if !matches!(kind, DTokenType::Semicolon) {
                            state.bump();
                        }
                        else {
                            break;
                        }
                    }
                    state.eat(DTokenType::Semicolon);
                    // Increment
                    while let Some(kind) = state.peek_kind() {
                        if !matches!(kind, DTokenType::RightParen) {
                            state.bump();
                        }
                        else {
                            break;
                        }
                    }
                    state.eat(DTokenType::RightParen);
                }
                self.parse_statement(state)?;
                state.finish_at(cp, crate::parser::element_type::DElementType::ForStatement);
            }
            Some(DTokenType::ReturnKeyword) => {
                // Return statement
                let cp = state.checkpoint();
                state.bump();
                self.skip_trivia(state);
                while let Some(kind) = state.peek_kind() {
                    if !matches!(kind, DTokenType::Semicolon) {
                        state.bump();
                    }
                    else {
                        break;
                    }
                }
                state.eat(DTokenType::Semicolon);
                state.finish_at(cp, crate::parser::element_type::DElementType::ReturnStatement);
            }
            _ => {
                // Expression statement
                let cp = state.checkpoint();
                while let Some(kind) = state.peek_kind() {
                    if !matches!(kind, DTokenType::Semicolon) {
                        state.bump();
                    }
                    else {
                        break;
                    }
                }
                state.eat(DTokenType::Semicolon);
                state.finish_at(cp, crate::parser::element_type::DElementType::ExpressionStatement);
            }
        }
        Ok(())
    }
}

impl<'config> Parser<DLanguage> for DParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl oak_core::ParseCache<DLanguage>) -> oak_core::ParseOutput<'a, DLanguage> {
        let lexer = DLexer::new(self.config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                self.parse_statement(state)?;
            }

            Ok(state.finish_at(checkpoint, crate::parser::element_type::DElementType::Root))
        })
    }
}

impl<'config> Pratt<DLanguage> for DParser<'config> {
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, DLanguage> {
        self.skip_trivia(state);
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(DTokenType::Identifier) => {
                state.bump();
                state.finish_at(cp, crate::parser::element_type::DElementType::Identifier)
            }
            Some(DTokenType::IntegerLiteral) | Some(DTokenType::FloatLiteral) | Some(DTokenType::StringLiteral) | Some(DTokenType::CharLiteral) => {
                state.bump();
                state.finish_at(cp, crate::parser::element_type::DElementType::Literal)
            }
            Some(DTokenType::LeftParen) => {
                state.bump();
                let expr = PrattParser::parse(state, 0, self);
                state.push_child(expr);
                self.skip_trivia(state);
                state.eat(DTokenType::RightParen);
                state.finish_at(cp, crate::parser::element_type::DElementType::ParenthesizedExpression)
            }
            _ => {
                state.bump();
                state.finish_at(cp, crate::parser::element_type::DElementType::Error)
            }
        }
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, DLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, DLanguage>> {
        self.skip_trivia(state);
        let kind = state.peek_kind()?;

        let (prec, assoc) = match kind {
            DTokenType::Assign => (1, Associativity::Right),
            DTokenType::PlusAssign | DTokenType::MinusAssign | DTokenType::MultiplyAssign | DTokenType::DivideAssign | DTokenType::ModuloAssign => (1, Associativity::Right),
            DTokenType::LogicalOr => (2, Associativity::Left),
            DTokenType::LogicalAnd => (3, Associativity::Left),
            DTokenType::Equal | DTokenType::NotEqual | DTokenType::Less | DTokenType::LessEqual | DTokenType::Greater | DTokenType::GreaterEqual => (4, Associativity::Left),
            DTokenType::Plus | DTokenType::Minus => (5, Associativity::Left),
            DTokenType::Multiply | DTokenType::Divide | DTokenType::Modulo => (6, Associativity::Left),
            DTokenType::Dot => (7, Associativity::Left),
            _ => return None,
        };

        if prec < min_precedence {
            return None;
        }

        let cp = state.checkpoint();
        state.push_child(left);
        state.bump();
        self.skip_trivia(state);
        let right = PrattParser::parse(state, prec + (assoc as u8), self);
        state.push_child(right);
        Some(state.finish_at(cp, crate::parser::element_type::DElementType::BinaryExpression))
    }
}
