#![doc = include_str!("readme.md")]

/// Element types for command-line arguments.
pub mod element_type;

pub use element_type::CmdElementType;

use crate::{
    language::CmdLanguage,
    lexer::{CmdLexer, CmdTokenType},
};
use oak_core::{
    TextEdit,
    parser::{ParseCache, Parser, ParserState},
    source::Source,
};

#[allow(dead_code)]
pub(crate) type State<'a, S> = ParserState<'a, CmdLanguage, S>;

/// Parser for the CMD language.
pub struct CmdParser<'config> {
    pub(crate) config: &'config CmdLanguage,
}

impl<'config> CmdParser<'config> {
    /// Creates a new `CmdParser` with the given configuration.
    pub fn new(config: &'config CmdLanguage) -> Self {
        Self { config }
    }

    fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        self.skip_trivia(state);
        if !state.not_at_end() {
            return Ok(());
        }

        match state.peek_kind() {
            Some(CmdTokenType::Label) => self.parse_label(state),
            Some(CmdTokenType::Keyword) => {
                let text = state.peek_text().map(|s| s.to_uppercase());
                match text.as_deref() {
                    Some("IF") => self.parse_if(state),
                    Some("FOR") => self.parse_for(state),
                    Some("SET") => self.parse_set(state),
                    _ => self.parse_command(state),
                }
            }
            _ => self.parse_command(state),
        }
    }

    fn skip_trivia<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        while state.at(CmdTokenType::Whitespace) || state.at(CmdTokenType::Newline) || state.at(CmdTokenType::Comment) {
            state.bump();
        }
    }

    fn parse_label<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        let cp = state.checkpoint();
        state.expect(CmdTokenType::Label)?;
        state.finish_at(cp, CmdElementType::LabelDefinition);
        Ok(())
    }

    fn parse_if<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        let cp = state.checkpoint();
        state.bump(); // IF
        while state.not_at_end() && !state.at(CmdTokenType::Newline) {
            state.bump();
        }
        state.finish_at(cp, CmdElementType::IfStatement);
        Ok(())
    }

    fn parse_for<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        let cp = state.checkpoint();
        state.bump(); // FOR
        while state.not_at_end() && !state.at(CmdTokenType::Newline) {
            state.bump();
        }
        state.finish_at(cp, CmdElementType::ForStatement);
        Ok(())
    }

    fn parse_set<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        let cp = state.checkpoint();
        state.bump(); // SET
        while state.not_at_end() && !state.at(CmdTokenType::Newline) {
            state.bump();
        }
        state.finish_at(cp, CmdElementType::SetStatement);
        Ok(())
    }

    fn parse_command<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        let cp = state.checkpoint();
        while state.not_at_end() && !state.at(CmdTokenType::Newline) {
            state.bump();
        }
        state.finish_at(cp, CmdElementType::CommandStatement);
        Ok(())
    }
}

impl<'config> Parser<CmdLanguage> for CmdParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<CmdLanguage>) -> oak_core::ParseOutput<'a, CmdLanguage> {
        let lexer = CmdLexer::new(self.config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();
            while state.not_at_end() {
                if self.parse_statement(state).is_err() {
                    break;
                }
            }
            Ok(state.finish_at(checkpoint, CmdElementType::Root))
        })
    }
}
