use oak_core::{
    parser::{ParseCache, ParseOutput, Parser as CoreParser, parse_with_lexer},
    source::{Source, TextEdit},
    tree::GreenNode,
};

use crate::{language::RacketLanguage, lexer::Lexer};

mod element_type;
pub use element_type::ElementType;

/// Parser for Racket source code.
pub struct Parser;

impl CoreParser<RacketLanguage> for Parser {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<RacketLanguage>) -> ParseOutput<'a, RacketLanguage> {
        parse_with_lexer(&Lexer, text, edits, cache, |state| {
            let root = state.arena().alloc(GreenNode::new(ElementType::Expression, &[]));
            Ok(root)
        })
    }
}
