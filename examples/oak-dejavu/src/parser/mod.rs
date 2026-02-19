use crate::{DejavuLanguage, DejavuLexer, lexer::token_type::DejavuTokenType};
use oak_core::{GreenNode, Language, OakError, Parser, Source, TextEdit, parser::ParseCache};

pub use element_type::DejavuElementType;

/// Dejavu parser.
pub struct DejavuParser {
    /// Language configuration.
    pub language: DejavuLanguage,
}

pub(crate) type State<'a, S> = oak_core::parser::ParserState<'a, DejavuLanguage, S>;

impl DejavuParser {
    /// Create a new Dejavu parser.
    pub fn new(language: &DejavuLanguage) -> Self {
        Self { language: language.clone() }
    }

    pub(crate) fn skip_trivia<'a, S: oak_core::Source + ?Sized>(&self, state: &mut oak_core::parser::ParserState<'a, DejavuLanguage, S>) {
        state.skip_trivia();
    }
}

impl Parser<DejavuLanguage> for DejavuParser {
    fn parse<'a, S: oak_core::Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<DejavuLanguage>) -> oak_core::parser::ParseOutput<'a, DejavuLanguage> {
        oak_core::parser::parse_with_lexer(&DejavuLexer::new(&self.language), text, edits, cache, |state| {
            // TODO: Implement actual parsing logic
            // For now, just create a root node
            let checkpoint = state.sink.checkpoint();
            while state.not_at_end() {
                state.advance();
            }
            let root = state.sink.finish_node(checkpoint, DejavuElementType::Root);
            Ok(root)
        })
    }
}

/// Element type definitions.
pub mod element_type;
/// Control flow parsing utilities.
pub mod parse_control_flow;
/// Expression parsing utilities.
pub mod parse_expr;
/// Type parsing utilities.
pub mod parse_types;
