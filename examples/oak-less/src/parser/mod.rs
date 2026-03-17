#![doc = include_str!("readme.md")]
/// Less element types and role definitions.
pub mod element_type;
use crate::{
    language::LessLanguage,
    lexer::{LessLexer, LessTokenType},
};
pub use element_type::LessElementType;
use oak_core::{
    GreenNode, OakError, TextEdit,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::Source,
};

pub(crate) type State<'a, S> = ParserState<'a, LessLanguage, S>;

/// Parser for the Less language.
pub struct LessParser<'config> {
    /// Language configuration.
    pub(crate) config: &'config LessLanguage,
}

impl<'config> LessParser<'config> {
    /// Creates a new `LessParser` with the given language configuration.
    pub fn new(config: &'config LessLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Parser<LessLanguage> for LessParser<'config> {
    /// Parses the Less source code into a green tree.
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<LessLanguage>) -> ParseOutput<'a, LessLanguage> {
        let lexer = LessLexer::new(self.config);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let cp = state.checkpoint();

            while state.not_at_end() {
                if state.at(LessTokenType::AtRule) || state.at(LessTokenType::AtImport) || state.at(LessTokenType::AtMedia) { self.parse_at_rule(state)? } else { self.parse_ruleset(state)? }
            }

            Ok(state.finish_at(cp, LessElementType::SourceFile))
        })
    }
}

impl<'config> LessParser<'config> {
    /// Parses a Less at-rule (e.g., `@import`, `@media`).
    fn parse_at_rule<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.bump(); // Consume the at-keyword

        while state.not_at_end() && !state.at(LessTokenType::Semicolon) && !state.at(LessTokenType::LeftBrace) {
            state.bump()
        }

        if state.at(LessTokenType::LeftBrace) {
            state.expect(LessTokenType::LeftBrace).ok();
            while state.not_at_end() && !state.at(LessTokenType::RightBrace) {
                self.parse_ruleset(state)?
            }
            state.expect(LessTokenType::RightBrace).ok();
        }
        else if state.at(LessTokenType::Semicolon) {
            state.expect(LessTokenType::Semicolon).ok();
        }

        state.finish_at(cp, LessElementType::AtRule);
        Ok(())
    }

    /// Parses a Less rule set (selector + declaration block).
    fn parse_ruleset<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();

        // Parse selector(s)
        self.parse_selectors(state)?;

        // Parse declaration block
        let cp_block = state.checkpoint();
        state.expect(LessTokenType::LeftBrace).ok();
        while state.not_at_end() && !state.at(LessTokenType::RightBrace) {
            self.parse_declaration(state)?;
            if state.at(LessTokenType::Semicolon) {
                state.expect(LessTokenType::Semicolon).ok();
            }
            else if !state.at(LessTokenType::RightBrace) {
                // Potential error, but we try to continue
                break;
            }
        }
        state.expect(LessTokenType::RightBrace).ok();
        state.finish_at(cp_block, LessElementType::DeclarationBlock);

        state.finish_at(cp, LessElementType::RuleSet);
        Ok(())
    }

    /// Parses Less selectors.
    fn parse_selectors<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        while state.not_at_end() && !state.at(LessTokenType::LeftBrace) {
            state.bump()
        }
        state.finish_at(cp, LessElementType::SelectorList);
        Ok(())
    }

    /// Parses a Less declaration (property: value).
    fn parse_declaration<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();

        // Property
        let cp_prop = state.checkpoint();
        while state.not_at_end() && !state.at(LessTokenType::Colon) && !state.at(LessTokenType::Semicolon) && !state.at(LessTokenType::RightBrace) {
            state.bump()
        }
        state.finish_at(cp_prop, LessElementType::Property);

        if state.at(LessTokenType::Colon) {
            state.expect(LessTokenType::Colon).ok();

            // Value
            let cp_val = state.checkpoint();
            while state.not_at_end() && !state.at(LessTokenType::Semicolon) && !state.at(LessTokenType::RightBrace) {
                state.bump()
            }
            state.finish_at(cp_val, LessElementType::Value);
        }

        state.finish_at(cp, LessElementType::Declaration);
        Ok(())
    }
}
