#![doc = include_str!("readme.md")]
/// Element type definition.
pub mod element_type;
pub use element_type::CppElementType;

use crate::{
    language::CppLanguage,
    lexer::{CppLexer, CppTokenType},
};
use oak_core::{
    GreenNode, OakError,
    parser::{Associativity, ParseCache, ParseOutput, Parser, ParserState, Pratt, PrattParser, parse_with_lexer},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, CppLanguage, S>;

/// Parser for the C++ language.
///
/// This parser transforms a stream of tokens into a green tree of C++ syntax nodes,
/// using a combination of top-down recursive descent and Pratt parsing for expressions.
pub struct CppParser<'config> {
    pub(crate) config: &'config CppLanguage,
}

impl<'config> CppParser<'config> {
    /// Creates a new `CppParser` with the given configuration.
    pub fn new(config: &'config CppLanguage) -> Self {
        Self { config }
    }

    /// Parses a single C++ statement.
    ///
    /// This includes keywords, compound statements, preprocessor directives,
    /// and expressions followed by a semicolon.
    fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::CppTokenType::*;
        self.skip_trivia(state);
        match state.peek_kind() {
            Some(LeftBrace) => self.parse_compound_statement(state)?,
            Some(Preprocessor) => {
                // Skip preprocessor directives
                while state.not_at_end() && !state.at(Newline) {
                    state.bump();
                }
            }
            _ => {
                // Skip any tokens until semicolon or brace
                while state.not_at_end() && !state.at(Semicolon) && !state.at(LeftBrace) && !state.at(RightBrace) {
                    state.bump();
                }
                if state.at(Semicolon) {
                    state.bump();
                }
                else if state.at(LeftBrace) {
                    self.parse_compound_statement(state)?;
                }
            }
        }
        Ok(())
    }

    /// Skips trivia tokens (whitespace and comments).
    fn skip_trivia<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        use crate::lexer::CppTokenType::*;
        while let Some(kind) = state.peek_kind() {
            if matches!(kind, Whitespace | Newline | Comment) {
                state.bump();
            }
            else {
                break;
            }
        }
    }

    /// Parses a compound statement (a block of statements enclosed in braces).
    fn parse_compound_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        if !state.eat(CppTokenType::LeftBrace) {
            // Skip until right brace or end of file
            while state.not_at_end() && !state.at(CppTokenType::RightBrace) {
                state.bump();
            }
            if state.at(CppTokenType::RightBrace) {
                state.bump();
            }
            state.finish_at(cp, CppElementType::CompoundStatement);
            return Ok(());
        }

        while state.not_at_end() && !state.at(CppTokenType::RightBrace) {
            self.parse_statement(state)?;
        }

        if !state.eat(CppTokenType::RightBrace) {
            // Skip until end of file or next statement
            while state.not_at_end() && !state.at(CppTokenType::Semicolon) && !state.at(CppTokenType::LeftBrace) {
                state.bump();
            }
        }

        state.finish_at(cp, CppElementType::CompoundStatement);
        Ok(())
    }

    /// Parses an if statement.
    fn parse_if_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // if
        self.skip_trivia(state);
        state.expect(CppTokenType::LeftParen).ok();
        while state.not_at_end() && !state.at(CppTokenType::RightParen) {
            state.bump();
        }
        state.expect(CppTokenType::RightParen).ok();
        self.parse_statement(state)?;
        self.skip_trivia(state);
        if state.at(CppTokenType::Keyword) {
            state.bump();
            self.parse_statement(state)?;
        }
        state.finish_at(cp, CppElementType::IfStatement);
        Ok(())
    }

    /// Parses a while statement.
    fn parse_while_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // while
        self.skip_trivia(state);
        state.expect(CppTokenType::LeftParen).ok();
        let expr = PrattParser::parse(state, 0, self);
        state.push_child(expr);
        state.expect(CppTokenType::RightParen).ok();
        self.parse_statement(state)?;
        state.finish_at(cp, CppElementType::WhileStatement);
        Ok(())
    }

    /// Parses a for statement.
    fn parse_for_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // for
        self.skip_trivia(state);
        state.expect(CppTokenType::LeftParen).ok();

        // Initialize
        self.skip_trivia(state);
        if !state.at(CppTokenType::Semicolon) {
            let expr = PrattParser::parse(state, 0, self);
            state.push_child(expr);
        }
        state.expect(CppTokenType::Semicolon).ok();

        // Condition
        self.skip_trivia(state);
        if !state.at(CppTokenType::Semicolon) {
            let expr = PrattParser::parse(state, 0, self);
            state.push_child(expr);
        }
        state.expect(CppTokenType::Semicolon).ok();

        // Increment
        self.skip_trivia(state);
        if !state.at(CppTokenType::RightParen) {
            let expr = PrattParser::parse(state, 0, self);
            state.push_child(expr);
        }
        state.expect(CppTokenType::RightParen).ok();

        self.parse_statement(state)?;
        state.finish_at(cp, CppElementType::ForStatement);
        Ok(())
    }

    /// Parses a return statement.
    fn parse_return_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // return
        self.skip_trivia(state);
        if !state.at(CppTokenType::Semicolon) {
            let expr = PrattParser::parse(state, 0, self);
            state.push_child(expr);
        }
        state.eat(CppTokenType::Semicolon);
        state.finish_at(cp, CppElementType::ReturnStatement);
        Ok(())
    }

    /// Parses a declaration statement.
    fn parse_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();

        // Parse type specifiers and qualifiers
        while state.not_at_end() && !state.at(CppTokenType::Semicolon) && !state.at(CppTokenType::LeftParen) {
            state.bump();
        }

        // Check if it's a function declaration
        if state.at(CppTokenType::LeftParen) {
            // Function declaration
            self.parse_function_definition(state)?;
        }
        else {
            // Variable declaration
            while state.not_at_end() && !state.at(CppTokenType::Semicolon) {
                state.bump();
            }
            state.eat(CppTokenType::Semicolon);
            state.finish_at(cp, CppElementType::DeclarationStatement);
        }

        Ok(())
    }

    /// Parses a function definition.
    fn parse_function_definition<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();

        // Parse function name
        if state.at(CppTokenType::Identifier) {
            state.bump();
        }

        // Parse parameters
        self.skip_trivia(state);
        state.expect(CppTokenType::LeftParen).ok();
        while state.not_at_end() && !state.at(CppTokenType::RightParen) {
            self.skip_trivia(state);
            // Parse parameter
            while state.not_at_end() && !state.at(CppTokenType::Comma) && !state.at(CppTokenType::RightParen) {
                state.bump();
            }
            if state.at(CppTokenType::Comma) {
                state.bump();
            }
        }
        state.expect(CppTokenType::RightParen).ok();

        // Parse function body
        self.skip_trivia(state);
        if state.at(CppTokenType::LeftBrace) {
            self.parse_compound_statement(state)?;
        }

        state.finish_at(cp, CppElementType::FunctionDefinition);
        Ok(())
    }

    /// Parses a class definition.
    fn parse_class_definition<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // class/struct/enum

        // Parse class name
        self.skip_trivia(state);
        if state.at(CppTokenType::Identifier) {
            state.bump();
        }

        // Parse template parameters (if any)
        self.skip_trivia(state);
        if state.at(CppTokenType::Less) {
            while state.not_at_end() && !state.at(CppTokenType::Greater) {
                state.bump();
            }
            state.eat(CppTokenType::Greater);
        }

        // Parse base classes (if any)
        self.skip_trivia(state);
        if state.at(CppTokenType::Colon) {
            state.bump();
            while state.not_at_end() && !state.at(CppTokenType::LeftBrace) {
                state.bump();
            }
        }

        // Parse class body
        self.skip_trivia(state);
        if state.at(CppTokenType::LeftBrace) {
            state.bump();
            while state.not_at_end() && !state.at(CppTokenType::RightBrace) {
                // Parse class members
                self.skip_trivia(state);
                if state.at(CppTokenType::Keyword) {
                    state.bump();
                    if state.at(CppTokenType::Colon) {
                        state.bump();
                    }
                }
                // Parse member declaration
                while state.not_at_end() && !state.at(CppTokenType::Semicolon) && !state.at(CppTokenType::RightBrace) {
                    state.bump();
                }
                state.eat(CppTokenType::Semicolon);
            }
            state.expect(CppTokenType::RightBrace).ok();
        }

        state.eat(CppTokenType::Semicolon);
        state.finish_at(cp, CppElementType::ClassDefinition);
        Ok(())
    }

    /// Parses a namespace definition.
    fn parse_namespace_definition<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // namespace

        // Parse namespace name
        self.skip_trivia(state);
        if state.at(CppTokenType::Identifier) {
            state.bump();
            // Handle nested namespaces (::)
            while state.at(CppTokenType::Scope) {
                state.bump();
                if state.at(CppTokenType::Identifier) {
                    state.bump();
                }
            }
        }

        // Parse namespace body
        self.skip_trivia(state);
        if state.at(CppTokenType::LeftBrace) {
            state.bump();
            while state.not_at_end() && !state.at(CppTokenType::RightBrace) {
                self.parse_statement(state)?;
            }
            state.expect(CppTokenType::RightBrace).ok();
        }

        state.finish_at(cp, CppElementType::NamespaceDefinition);
        Ok(())
    }
}

impl<'config> Parser<CppLanguage> for CppParser<'config> {
    /// Parses the entire C++ source file.
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<CppLanguage>) -> ParseOutput<'a, CppLanguage> {
        let lexer = CppLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let cp = state.checkpoint();
            while state.not_at_end() {
                self.parse_statement(state)?
            }
            Ok(state.finish_at(cp, CppElementType::SourceFile))
        })
    }
}

impl<'config> Pratt<CppLanguage> for CppParser<'config> {
    /// Parses a primary expression (e.g., identifiers, literals, parenthesized expressions).
    fn primary<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, CppLanguage> {
        use crate::lexer::CppTokenType::*;
        self.skip_trivia(state);
        let cp = state.checkpoint();
        match state.peek_kind() {
            Some(Identifier) => {
                state.bump();
                state.finish_at(cp, CppElementType::Token(Identifier))
            }
            Some(IntegerLiteral) | Some(FloatLiteral) | Some(CharacterLiteral) | Some(StringLiteral) | Some(BooleanLiteral) => {
                state.bump();
                state.finish_at(cp, CppElementType::ExpressionStatement)
            }
            Some(LeftParen) => {
                state.bump();
                let expr = PrattParser::parse(state, 0, self);
                state.push_child(expr);
                self.skip_trivia(state);
                state.expect(RightParen).ok();
                state.finish_at(cp, CppElementType::ExpressionStatement)
            }
            _ => {
                state.bump();
                state.finish_at(cp, CppElementType::Error)
            }
        }
    }

    fn prefix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> &'a GreenNode<'a, CppLanguage> {
        self.primary(state)
    }

    fn infix<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>, left: &'a GreenNode<'a, CppLanguage>, min_precedence: u8) -> Option<&'a GreenNode<'a, CppLanguage>> {
        use crate::lexer::CppTokenType::*;
        self.skip_trivia(state);
        let kind = state.peek_kind()?;

        let (prec, assoc) = match kind {
            Assign | PlusAssign | MinusAssign | StarAssign | SlashAssign | PercentAssign | AndAssign | OrAssign | XorAssign | LeftShiftAssign | RightShiftAssign => (1, Associativity::Right),
            LogicalOr => (2, Associativity::Left),
            LogicalAnd => (3, Associativity::Left),
            Equal | NotEqual | Less | Greater | LessEqual | GreaterEqual => (4, Associativity::Left),
            Plus | Minus => (10, Associativity::Left),
            Star | Slash | Percent => (11, Associativity::Left),
            LeftParen | LeftBracket | Dot | Arrow => (15, Associativity::Left),
            Scope => (16, Associativity::Left),
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
                    self.skip_trivia(state);
                    let expr = PrattParser::parse(state, 0, self);
                    state.push_child(expr);
                    self.skip_trivia(state);
                    if !state.eat(Comma) {
                        break;
                    }
                }
                state.expect(RightParen).ok();
                Some(state.finish_at(cp, CppElementType::FunctionCall))
            }
            _ => {
                let cp = state.checkpoint();
                state.push_child(left);
                state.bump();
                self.skip_trivia(state);
                let right = PrattParser::parse(state, prec + (assoc as u8), self);
                state.push_child(right);
                Some(state.finish_at(cp, CppElementType::ExpressionStatement))
            }
        }
    }
}
