pub mod element_type;

use crate::{language::D2Language, lexer::D2Lexer};
use oak_core::{
    GreenNode, Parser,
    parser::{ParseCache, ParseOutput, parse_with_lexer},
    source::{Source, TextEdit},
};

pub struct D2Parser<'config> {
    config: &'config D2Language,
}

impl<'config> D2Parser<'config> {
    pub fn new(config: &'config D2Language) -> Self {
        Self { config }
    }
}

impl<'config> Parser<D2Language> for D2Parser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<D2Language>) -> ParseOutput<'a, D2Language> {
        let lexer = D2Lexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let root = GreenNode::new(element_type::D2ElementType::Root, &[]);
            Ok(state.arena().alloc(root))
        })
    }
}
