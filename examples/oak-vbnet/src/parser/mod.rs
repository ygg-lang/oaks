/// Element type module
pub mod element_type;

mod parse_declaration;
mod parse_expression;
mod parse_member;
mod parse_statement;

use crate::{language::VbNetLanguage, lexer::token_type::VbNetTokenType, parser::element_type::VbNetElementType};
use oak_core::{
    OakError,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, VbNetLanguage, S>;

/// VB.NET parser
pub struct VbNetParser<'config> {
    pub(crate) config: &'config VbNetLanguage,
}

impl<'config> VbNetParser<'config> {
    /// Creates a new VB.NET parser
    pub fn new(config: &'config VbNetLanguage) -> Self {
        Self { config }
    }

    pub(crate) fn skip_trivia<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) {
        while let Some(kind) = state.peek_kind() {
            match kind {
                VbNetTokenType::Whitespace | VbNetTokenType::LineComment | VbNetTokenType::BlockComment | VbNetTokenType::Newline => {
                    state.bump();
                }
                _ => break,
            }
        }
    }
}

impl<'config> Parser<VbNetLanguage> for VbNetParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<VbNetLanguage>) -> ParseOutput<'a, VbNetLanguage> {
        let lexer = crate::lexer::VbNetLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let cp = (0, 0);
            while state.not_at_end() {
                self.skip_trivia(state);
                if state.not_at_end() {
                    self.parse_statement(state)?;
                }
            }

            Ok(state.finish_at(cp, VbNetElementType::Root))
        })
    }
}
