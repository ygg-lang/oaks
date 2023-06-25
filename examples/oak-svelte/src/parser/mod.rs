//! Svelte parser implementation.

pub mod element_type;
use crate::{language::SvelteLanguage, lexer::token_type::SvelteTokenType};
pub use element_type::SvelteElementType;
use oak_core::{
    errors::OakError,
    parser::{ParseCache, ParseOutput, Parser, ParserState, parse_with_lexer},
    source::{Source, TextEdit},
};

pub(crate) type State<'a, S> = ParserState<'a, SvelteLanguage, S>;

/// Svelte parser.
pub struct SvelteParser<'config> {
    language: &'config SvelteLanguage,
}

#[allow(dead_code)]
impl<'config> SvelteParser<'config> {
    /// Creates a new `SvelteParser`.
    pub fn new(language: &'config SvelteLanguage) -> Self {
        Self { language }
    }

    fn parse_node<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        match state.peek_kind() {
            Some(SvelteTokenType::Lt) => self.parse_element(state),
            Some(SvelteTokenType::LeftBrace) => self.parse_expression(state),
            Some(SvelteTokenType::HashBrace) => self.parse_block(state),
            Some(SvelteTokenType::Comment) => {
                let cp = state.checkpoint();
                state.bump();
                state.finish_at(cp, SvelteElementType::CommentNode);
                Ok(())
            }
            _ => {
                let cp = state.checkpoint();
                state.advance();
                state.finish_at(cp, SvelteElementType::TextNode);
                Ok(())
            }
        }
    }

    fn parse_element<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();

        // Tag (Opening)
        let tag_cp = state.checkpoint();
        state.expect(SvelteTokenType::Lt)?;
        state.expect(SvelteTokenType::Identifier).ok();

        while state.not_at_end() && !matches!(state.peek_kind(), Some(SvelteTokenType::Gt) | Some(SvelteTokenType::SlashGt)) {
            if state.at(SvelteTokenType::Identifier) || state.at(SvelteTokenType::LeftBrace) {
                self.parse_attribute(state)?;
            }
            else {
                state.advance();
            }
        }

        let is_self_closing = state.eat(SvelteTokenType::SlashGt);
        if !is_self_closing {
            state.expect(SvelteTokenType::Gt).ok();
        }
        state.finish_at(tag_cp, SvelteElementType::Tag);

        if !is_self_closing {
            // Content
            while state.not_at_end() && !state.at(SvelteTokenType::LtSlash) {
                self.parse_node(state)?;
            }

            // Closing Tag
            if state.at(SvelteTokenType::LtSlash) {
                let close_cp = state.checkpoint();
                state.bump(); // </
                state.eat(SvelteTokenType::Identifier);
                state.expect(SvelteTokenType::Gt).ok();
                state.finish_at(close_cp, SvelteElementType::CloseTag);
            }
        }

        state.finish_at(cp, SvelteElementType::Element);
        Ok(())
    }

    fn parse_attribute<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();

        if state.at(SvelteTokenType::LeftBrace) {
            // Shorthand: {name}
            self.parse_expression(state)?;
        }
        else {
            state.expect(SvelteTokenType::Identifier)?;

            if state.eat(SvelteTokenType::Colon) {
                state.expect(SvelteTokenType::Identifier).ok();
            }

            if state.eat(SvelteTokenType::Eq) {
                if state.at(SvelteTokenType::StringLiteral) {
                    state.bump();
                }
                else if state.at(SvelteTokenType::LeftBrace) {
                    self.parse_expression(state)?;
                }
            }
        }
        state.finish_at(cp, SvelteElementType::Attribute);
        Ok(())
    }

    fn parse_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();
        state.expect(SvelteTokenType::LeftBrace)?;
        while state.not_at_end() && !state.at(SvelteTokenType::RightBrace) {
            state.advance();
        }
        state.expect(SvelteTokenType::RightBrace).ok();
        state.finish_at(cp, SvelteElementType::Expression);
        Ok(())
    }

    fn parse_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();

        // Block Header
        let header_cp = state.checkpoint();
        state.expect(SvelteTokenType::HashBrace)?;
        state.expect(SvelteTokenType::Identifier).ok();

        while state.not_at_end() && !state.at(SvelteTokenType::RightBrace) {
            state.advance();
        }
        state.expect(SvelteTokenType::RightBrace).ok();
        state.finish_at(header_cp, SvelteElementType::BlockHeader);

        // Block Content
        let content_cp = state.checkpoint();
        while state.not_at_end() && !state.at(SvelteTokenType::SlashBrace) && !state.at(SvelteTokenType::ColonBrace) {
            self.parse_node(state)?;
        }
        state.finish_at(content_cp, SvelteElementType::BlockContent);

        // Block Branches
        while state.at(SvelteTokenType::ColonBrace) {
            self.parse_branch(state)?;
        }

        // Block Footer
        if state.at(SvelteTokenType::SlashBrace) {
            let footer_cp = state.checkpoint();
            state.bump();
            state.expect(SvelteTokenType::Identifier).ok();
            state.expect(SvelteTokenType::RightBrace).ok();
            state.finish_at(footer_cp, SvelteElementType::BlockFooter);
        }

        state.finish_at(cp, SvelteElementType::Block);
        Ok(())
    }

    fn parse_branch<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        let cp = state.checkpoint();

        // Branch Header
        let header_cp = state.checkpoint();
        state.expect(SvelteTokenType::ColonBrace)?;
        state.expect(SvelteTokenType::Identifier).ok();
        while state.not_at_end() && !state.at(SvelteTokenType::RightBrace) {
            state.advance();
        }
        state.expect(SvelteTokenType::RightBrace).ok();
        state.finish_at(header_cp, SvelteElementType::BlockHeader);

        // Branch Content
        let content_cp = state.checkpoint();
        while state.not_at_end() && !state.at(SvelteTokenType::SlashBrace) && !state.at(SvelteTokenType::ColonBrace) {
            self.parse_node(state)?;
        }
        state.finish_at(content_cp, SvelteElementType::BlockContent);

        state.finish_at(cp, SvelteElementType::BlockBranch);
        Ok(())
    }
}

impl<'config> Parser<SvelteLanguage> for SvelteParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<SvelteLanguage>) -> ParseOutput<'a, SvelteLanguage> {
        let lexer = crate::lexer::SvelteLexer::new(self.language);
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let cp = state.checkpoint();

            while state.not_at_end() {
                self.parse_node(state)?;
            }

            Ok(state.finish_at(cp, SvelteElementType::Root))
        })
    }
}
