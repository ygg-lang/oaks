use crate::{kind::IniSyntaxKind, language::IniLanguage, lexer::IniLexer};
use oak_core::{
    GreenNode, OakError, TextEdit,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::Source,
};

mod parse;

pub(crate) type State<'a, S> = ParserState<'a, IniLanguage, S>;

pub struct IniParser<'config> {
    pub(crate) config: &'config IniLanguage,
}

impl<'config> IniParser<'config> {
    pub fn new(config: &'config IniLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<IniLanguage> for IniParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<IniLanguage>) -> ParseOutput<'a, IniLanguage> {
        let lexer = IniLexer::new(&self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}

impl<'config> IniParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, IniLanguage>, OakError> {
        let checkpoint = state.checkpoint();
        while state.not_at_end() {
            self.skip_trivia(state);
            if !state.not_at_end() {
                break;
            }

            if state.at(IniSyntaxKind::LeftBracket) || state.at(IniSyntaxKind::DoubleLeftBracket) {
                self.parse_table(state)?;
            }
            else {
                self.parse_key_value(state)?;
            }
        }

        Ok(state.finish_at(checkpoint, IniSyntaxKind::Root.into()))
    }
}
