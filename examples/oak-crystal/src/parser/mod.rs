//! Parser for the Crystal language.

pub mod element_type;
use crate::language::CrystalLanguage;
pub use element_type::CrystalElementType;
use oak_core::{
    TextEdit,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::Source,
};

pub(crate) type State<'a, S> = ParserState<'a, CrystalLanguage, S>;

/// Parser for the Crystal language.
pub struct CrystalParser<'config> {
    pub(crate) config: &'config CrystalLanguage,
}

impl<'config> CrystalParser<'config> {
    /// Creates a new `CrystalParser` with the given configuration.
    pub fn new(config: &'config CrystalLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<CrystalLanguage> for CrystalParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<CrystalLanguage>) -> ParseOutput<'a, CrystalLanguage> {
        let lexer = crate::lexer::CrystalLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let checkpoint = state.checkpoint();

            while state.not_at_end() {
                state.bump()
            }

            Ok(state.finish_at(checkpoint, CrystalElementType::SourceFile))
        })
    }
}
