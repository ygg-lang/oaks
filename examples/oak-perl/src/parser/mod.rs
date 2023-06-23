pub mod element_type;

use crate::language::PerlLanguage;
use oak_core::{
    OakError,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

mod parse_top_level;

pub(crate) type State<'a, S> = ParserState<'a, PerlLanguage, S>;

/// Parser for the Perl language.
///
/// This parser transforms a stream of tokens into a green tree of [`crate::lexer::PerlTokenType`] nodes.
pub struct PerlParser<'config> {
    pub(crate) config: &'config PerlLanguage,
}

impl<'config> PerlParser<'config> {
    /// Creates a new `PerlParser` with the given language configuration.
    pub fn new(config: &'config PerlLanguage) -> Self {
        Self { config }
    }

    pub(crate) fn parse_statement<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        if state.not_at_end() {
            state.bump();
            Ok(())
        }
        else {
            Err(OakError::unexpected_eof(state.current_offset(), state.source_id()))
        }
    }
}

impl<'config> Parser<PerlLanguage> for PerlParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<PerlLanguage>) -> ParseOutput<'a, PerlLanguage> {
        let lexer = crate::lexer::PerlLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}
