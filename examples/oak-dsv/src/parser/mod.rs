#![doc = include_str!("readme.md")]
/// Element types for the DSV language syntax tree.
pub mod element_type;
use crate::{
    language::{Dsv, DsvLanguage},
    lexer::DsvLexer,
};
pub use element_type::DsvElementType;
use oak_core::{
    ParseOutput,
    parser::{ParseCache, Parser, ParserState},
    source::{Source, TextEdit},
};

/// DSV parser state.
pub(crate) type State<'a, const LANG: DsvLanguage, S> = ParserState<'a, Dsv<LANG>, S>;

/// Parser for the DSV language.
pub struct DsvParser<const LANG: DsvLanguage> {
    _phantom: std::marker::PhantomData<Dsv<LANG>>,
}

impl<const LANG: DsvLanguage> Parser<Dsv<LANG>> for DsvParser<LANG> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<Dsv<LANG>>) -> ParseOutput<'a, Dsv<LANG>> {
        let lexer = DsvLexer::<LANG>::new();
        oak_core::parser::parse_with_lexer(&lexer, text, edits, cache, |state| {
            use crate::{lexer::DsvTokenType::*, parser::DsvElementType::*};
            let cp = state.checkpoint();

            while state.not_at_end() && !state.at(Eof) {
                self.parse_record(state)?;
                if state.at(Newline) {
                    state.bump()
                }
            }

            Ok(state.finish_at(cp, SourceFile))
        })
    }
}

impl<const LANG: DsvLanguage> DsvParser<LANG> {
    /// Creates a new `DsvParser`.
    pub fn new() -> Self {
        Self { _phantom: std::marker::PhantomData }
    }

    /// Parses a DSV record.
    fn parse_record<'a, S: oak_core::Source + ?Sized>(&self, state: &mut State<'a, LANG, S>) -> Result<(), oak_core::OakError> {
        use crate::{lexer::DsvTokenType::*, parser::DsvElementType::*};
        let cp = state.checkpoint();

        self.parse_field(state)?;
        while state.at(Separator) {
            state.bump();
            self.parse_field(state)?
        }

        state.finish_at(cp, Record);
        Ok(())
    }

    /// Parses a DSV field.
    fn parse_field<'a, S: oak_core::Source + ?Sized>(&self, state: &mut State<'a, LANG, S>) -> Result<(), oak_core::OakError> {
        let cp = state.checkpoint();
        if state.at(crate::lexer::DsvTokenType::Field) {
            state.bump()
        }
        else {
            // Empty field
        }
        state.finish_at(cp, DsvElementType::Field);
        Ok(())
    }
}
