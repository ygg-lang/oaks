pub mod element_type;
use crate::{
    language::CssLanguage,
    lexer::{CssLexer, CssTokenType},
};
pub use element_type::CssElementType;
use oak_core::{
    GreenNode, OakError, TextEdit,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::Source,
};

pub(crate) type State<'a, S> = ParserState<'a, CssLanguage, S>;

pub struct CssParser<'config> {
    pub(crate) _config: &'config CssLanguage,
}

impl<'config> CssParser<'config> {
    pub fn new(config: &'config CssLanguage) -> Self {
        Self { _config: config }
    }
}

impl<'config> Parser<CssLanguage> for CssParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<CssLanguage>) -> ParseOutput<'a, CssLanguage> {
        let lexer = CssLexer::new(self._config);
        parse_with_lexer(&lexer, text, edits, cache, |state| self.parse_root_internal(state))
    }
}

impl<'config> CssParser<'config> {
    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<&'a GreenNode<'a, CssLanguage>, OakError> {
        let cp = state.checkpoint();

        while state.not_at_end() {
            if state.at(CssTokenType::AtRule) || state.at(CssTokenType::AtImport) || state.at(CssTokenType::AtMedia) {
                self.parse_at_rule(state)?;
            }
            else {
                self.parse_ruleset(state)?;
            }
        }

        Ok(state.finish_at(cp, CssElementType::SourceFile))
    }

    fn parse_at_rule<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // Consume the at-keyword

        while state.not_at_end() && !state.at(CssTokenType::Semicolon) && !state.at(CssTokenType::LeftBrace) {
            state.bump();
        }

        if state.at(CssTokenType::LeftBrace) {
            state.expect(CssTokenType::LeftBrace).ok();
            while state.not_at_end() && !state.at(CssTokenType::RightBrace) {
                self.parse_ruleset(state)?;
            }
            state.expect(CssTokenType::RightBrace).ok();
        }
        else if state.at(CssTokenType::Semicolon) {
            state.expect(CssTokenType::Semicolon).ok();
        }

        state.finish_at(cp, CssElementType::AtRule);
        Ok(())
    }

    fn parse_ruleset<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();

        // Parse selector(s)
        self.parse_selectors(state)?;

        // Parse declaration block
        let cp_block = state.checkpoint();
        state.expect(CssTokenType::LeftBrace).ok();
        while state.not_at_end() && !state.at(CssTokenType::RightBrace) {
            self.parse_declaration(state)?;
            if state.at(CssTokenType::Semicolon) {
                state.expect(CssTokenType::Semicolon).ok();
            }
            else if !state.at(CssTokenType::RightBrace) {
                // Potential error, but we try to continue
                break;
            }
        }
        state.expect(CssTokenType::RightBrace).ok();
        state.finish_at(cp_block, CssElementType::DeclarationBlock);

        state.finish_at(cp, CssElementType::RuleSet);
        Ok(())
    }

    fn parse_selectors<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        while state.not_at_end() && !state.at(CssTokenType::LeftBrace) {
            state.bump();
        }
        state.finish_at(cp, CssElementType::SelectorList);
        Ok(())
    }

    fn parse_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();

        // Property name
        if state.at(CssTokenType::Identifier) || state.at(CssTokenType::PropertyName) {
            state.bump();
        }
        else {
            return Ok(());
        }

        state.expect(CssTokenType::Colon).ok();

        let cp_value = state.checkpoint();
        while state.not_at_end() && !state.at(CssTokenType::Semicolon) && !state.at(CssTokenType::RightBrace) {
            state.bump();
        }
        state.finish_at(cp_value, CssElementType::Value);

        state.finish_at(cp, CssElementType::Declaration);
        Ok(())
    }
}
