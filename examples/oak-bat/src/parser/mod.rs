#![doc = include_str!("readme.md")]
pub mod element_type;

pub use element_type::BatElementType;

use crate::{
    language::BatLanguage,
    lexer::{BatLexer, BatTokenType},
};
use oak_core::{
    OakError, TextEdit,
    parser::{ParseCache, Parser, ParserState},
    source::Source,
};

pub(crate) type State<'a, S> = ParserState<'a, BatLanguage, S>;

pub struct BatParser<'config> {
    pub(crate) config: &'config BatLanguage,
}

impl<'config> BatParser<'config> {
    pub fn new(config: &'config BatLanguage) -> Self {
        Self { config }
    }

    fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        self.skip_trivia(state);
        if !state.not_at_end() {
            return Ok(());
        }

        match state.peek_kind() {
            Some(BatTokenType::Label) => self.parse_label(state),
            Some(BatTokenType::Keyword) => {
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
        while state.at(BatTokenType::Whitespace) || state.at(BatTokenType::Newline) || state.at(BatTokenType::Comment) {
            state.bump();
        }
    }

    fn parse_label<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(BatTokenType::Label)?;
        state.finish_at(cp, BatElementType::LabelDefinition);
        Ok(())
    }

    fn parse_if<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // IF
        // Basic IF parsing: consume until newline or block
        while state.not_at_end() && !state.at(BatTokenType::Newline) {
            state.bump();
        }
        state.finish_at(cp, BatElementType::IfStatement);
        Ok(())
    }

    fn parse_for<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // FOR
        while state.not_at_end() && !state.at(BatTokenType::Newline) {
            state.bump();
        }
        state.finish_at(cp, BatElementType::ForStatement);
        Ok(())
    }

    fn parse_set<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // SET
        while state.not_at_end() && !state.at(BatTokenType::Newline) {
            state.bump();
        }
        state.finish_at(cp, BatElementType::SetStatement);
        Ok(())
    }

    fn parse_command<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        while state.not_at_end() && !state.at(BatTokenType::Newline) {
            state.bump();
        }
        state.finish_at(cp, BatElementType::CommandStatement);
        Ok(())
    }
}

impl<'config> Parser<BatLanguage> for BatParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<BatLanguage>) -> oak_core::ParseOutput<'a, BatLanguage> {
        let lexer = BatLexer::new(self.config);
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();
            while state.not_at_end() {
                if self.parse_statement(state).is_err() {
                    break;
                }
            }
            Ok(state.finish_at(checkpoint, BatElementType::Root))
        })
    }
}
