/// Element type definitions for Perl.
pub mod element_type;

use crate::language::PerlLanguage;
use oak_core::{
    OakError,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, PerlLanguage, S>;

/// Parser for the Perl language.
///
/// This parser converts a stream of [`crate::lexer::PerlTokenType`] tokens into a green tree.
pub struct PerlParser<'config> {
    /// The Perl language configuration.
    pub(crate) config: &'config PerlLanguage,
}

impl<'config> PerlParser<'config> {
    /// Creates a new `PerlParser` with the given language configuration.
    pub fn new(config: &'config PerlLanguage) -> Self {
        Self { config }
    }

    /// Parses a single statement.
    pub(crate) fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::PerlTokenType::*;
        self.skip_trivia(state);

        if !state.not_at_end() {
            return Ok(());
        }

        match state.peek_kind() {
            Some(Sub) => self.parse_sub_declaration(state),
            Some(Package) => self.parse_package_declaration(state),
            Some(Use) => self.parse_use_statement(state),
            Some(If) => self.parse_if_statement(state),
            Some(Unless) => self.parse_unless_statement(state),
            Some(While) => self.parse_while_statement(state),
            Some(Until) => self.parse_until_statement(state),
            Some(For) | Some(Foreach) => self.parse_for_statement(state),
            Some(Do) => self.parse_do_statement(state),
            Some(My) | Some(Our) | Some(Local) => self.parse_variable_declaration(state),
            _ => self.parse_expression_statement(state),
        }
    }

    fn skip_trivia<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        use crate::lexer::token_type::PerlTokenType::*;
        while state.at(Whitespace) || state.at(Newline) || state.at(Comment) {
            state.bump();
        }
    }

    fn parse_sub_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::PerlTokenType::*;
        let cp = state.checkpoint();
        state.expect(Sub)?;
        self.skip_trivia(state);
        state.expect(Identifier).ok();
        self.skip_trivia(state);
        if state.at(LeftBrace) {
            self.parse_block(state)?;
        }
        else {
            state.expect(Semicolon).ok();
        }
        state.finish_at(cp, element_type::PerlElementType::SubroutineDeclaration);
        Ok(())
    }

    fn parse_package_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::PerlTokenType::*;
        let cp = state.checkpoint();
        state.expect(Package)?;
        self.skip_trivia(state);
        state.expect(Identifier)?;
        self.skip_trivia(state);
        state.expect(Semicolon)?;
        state.finish_at(cp, element_type::PerlElementType::PackageDeclaration);
        Ok(())
    }

    fn parse_use_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::PerlTokenType::*;
        let cp = state.checkpoint();
        state.expect(Use)?;
        self.skip_trivia(state);
        state.expect(Identifier)?;
        self.skip_trivia(state);
        while state.not_at_end() && !state.at(Semicolon) {
            state.bump();
        }
        state.expect(Semicolon).ok();
        state.finish_at(cp, element_type::PerlElementType::UseStatement);
        Ok(())
    }

    fn parse_if_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::PerlTokenType::*;
        let cp = state.checkpoint();
        state.expect(If)?;
        self.skip_trivia(state);
        state.expect(LeftParen)?;
        while state.not_at_end() && !state.at(RightParen) {
            state.bump();
        }
        state.expect(RightParen).ok();
        self.skip_trivia(state);
        self.parse_block(state)?;

        self.skip_trivia(state);
        while state.at(Elsif) {
            state.bump();
            self.skip_trivia(state);
            state.expect(LeftParen)?;
            while state.not_at_end() && !state.at(RightParen) {
                state.bump();
            }
            state.expect(RightParen).ok();
            self.skip_trivia(state);
            self.parse_block(state)?;
            self.skip_trivia(state);
        }

        if state.at(Else) {
            state.bump();
            self.skip_trivia(state);
            self.parse_block(state)?;
        }

        state.finish_at(cp, element_type::PerlElementType::IfStatement);
        Ok(())
    }

    fn parse_unless_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::PerlTokenType::*;
        let cp = state.checkpoint();
        state.expect(Unless)?;
        self.skip_trivia(state);
        state.expect(LeftParen)?;
        while state.not_at_end() && !state.at(RightParen) {
            state.bump();
        }
        state.expect(RightParen).ok();
        self.skip_trivia(state);
        self.parse_block(state)?;
        state.finish_at(cp, element_type::PerlElementType::UnlessStatement);
        Ok(())
    }

    fn parse_while_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::PerlTokenType::*;
        let cp = state.checkpoint();
        state.expect(While)?;
        self.skip_trivia(state);
        state.expect(LeftParen)?;
        while state.not_at_end() && !state.at(RightParen) {
            state.bump();
        }
        state.expect(RightParen).ok();
        self.skip_trivia(state);
        self.parse_block(state)?;
        state.finish_at(cp, element_type::PerlElementType::WhileStatement);
        Ok(())
    }

    fn parse_until_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::PerlTokenType::*;
        let cp = state.checkpoint();
        state.expect(Until)?;
        self.skip_trivia(state);
        state.expect(LeftParen)?;
        while state.not_at_end() && !state.at(RightParen) {
            state.bump();
        }
        state.expect(RightParen).ok();
        self.skip_trivia(state);
        self.parse_block(state)?;
        state.finish_at(cp, element_type::PerlElementType::UntilStatement);
        Ok(())
    }

    fn parse_for_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::PerlTokenType::*;
        let cp = state.checkpoint();
        if state.at(For) {
            state.bump();
        }
        else {
            state.expect(Foreach)?;
        }
        self.skip_trivia(state);
        if state.at(LeftParen) {
            state.bump();
            while state.not_at_end() && !state.at(RightParen) {
                state.bump();
            }
            state.expect(RightParen).ok();
        }
        self.skip_trivia(state);
        self.parse_block(state)?;
        state.finish_at(cp, element_type::PerlElementType::ForStatement);
        Ok(())
    }

    fn parse_do_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::PerlTokenType::*;
        let cp = state.checkpoint();
        state.expect(Do)?;
        self.skip_trivia(state);
        if state.at(LeftBrace) {
            self.parse_block(state)?;
        }
        else {
            // do sub() or do "file"
            while state.not_at_end() && !state.at(Semicolon) {
                state.bump();
            }
        }
        state.expect(Semicolon).ok();
        state.finish_at(cp, element_type::PerlElementType::DoStatement);
        Ok(())
    }

    fn parse_variable_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::PerlTokenType::*;
        let cp = state.checkpoint();
        state.bump(); // my, our, local
        self.skip_trivia(state);
        while state.not_at_end() && !state.at(Semicolon) {
            state.bump();
        }
        state.expect(Semicolon).ok();
        state.finish_at(cp, element_type::PerlElementType::VariableDeclaration);
        Ok(())
    }

    fn parse_expression_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::PerlTokenType::*;
        let cp = state.checkpoint();
        while state.not_at_end() && !state.at(Semicolon) {
            state.bump();
        }
        state.expect(Semicolon).ok();
        state.finish_at(cp, element_type::PerlElementType::Statement);
        Ok(())
    }

    fn parse_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        use crate::lexer::token_type::PerlTokenType::*;
        let cp = state.checkpoint();
        state.expect(LeftBrace)?;
        while state.not_at_end() && !state.at(RightBrace) {
            self.parse_statement(state)?;
        }
        state.expect(RightBrace).ok();
        state.finish_at(cp, element_type::PerlElementType::Block);
        Ok(())
    }
}

impl<'config> Parser<PerlLanguage> for PerlParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<PerlLanguage>) -> ParseOutput<'a, PerlLanguage> {
        let lexer = crate::lexer::PerlLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();
            while state.not_at_end() {
                if self.parse_statement(state).is_err() {
                    break;
                }
            }
            Ok(state.finish_at(checkpoint, element_type::PerlElementType::Root))
        })
    }
}
