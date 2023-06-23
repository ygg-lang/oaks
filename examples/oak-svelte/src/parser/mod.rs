#![doc = include_str!("readme.md")]
use oak_core::{
    GreenNode, OakError,
    parser::{ParseCache, ParseOutput, Parser, state::ParserState},
    source::{Source, TextEdit},
};

/// Element types for Svelte.
pub mod element_type;

use crate::{
    lexer::token_type::{SvelteLanguage, SvelteTokenType},
    parser::element_type::SvelteElementType,
};

pub(crate) type SvelteParserState<'a, S> = ParserState<'a, SvelteLanguage, S>;

/// Svelte Parser
pub struct SvelteParser<'config> {
    _config: &'config SvelteLanguage,
}

impl<'config> SvelteParser<'config> {
    /// Create a new Svelte parser
    pub fn new(config: &'config SvelteLanguage) -> Self {
        Self { _config: config }
    }

    pub(crate) fn parse_root_internal<'a, S: Source + ?Sized>(&self, state: &mut SvelteParserState<'a, S>) -> Result<&'a GreenNode<'a, SvelteLanguage>, OakError> {
        let cp = state.checkpoint();

        while state.not_at_end() {
            self.parse_node(state);
        }

        Ok(state.finish_at(cp, SvelteElementType::Root))
    }

    fn parse_node<'a, S: Source + ?Sized>(&self, state: &mut SvelteParserState<'a, S>) {
        if state.at(SvelteTokenType::TagOpen) {
            self.parse_element(state);
        }
        else if state.at(SvelteTokenType::OpenBrace) {
            let next = state.peek_kind_at(1);
            if next == Some(SvelteTokenType::Hash) {
                self.parse_control_block(state);
            }
            else if next == Some(SvelteTokenType::Slash) || next == Some(SvelteTokenType::Colon) {
                // These should normally be handled by parse_control_block,
                // but if we find them at top level, parse them as errors or individual blocks.
                self.parse_control_block(state);
            }
            else if next == Some(SvelteTokenType::At) {
                self.parse_special_tag(state);
            }
            else {
                self.parse_interpolation(state);
            }
        }
        else if state.at(SvelteTokenType::Comment) {
            let cp = state.checkpoint();
            state.bump();
            state.finish_at(cp, SvelteElementType::Comment);
        }
        else {
            self.parse_text(state);
        }
    }

    fn parse_element<'a, S: Source + ?Sized>(&self, state: &mut SvelteParserState<'a, S>) {
        let cp = state.checkpoint();

        // Start Tag
        let start_cp = state.checkpoint();
        state.expect(SvelteTokenType::TagOpen).ok();
        state.expect(SvelteTokenType::Identifier).ok();

        while state.not_at_end() && !state.at(SvelteTokenType::TagClose) && !state.at(SvelteTokenType::TagSelfClose) {
            if state.at(SvelteTokenType::Whitespace) {
                state.bump();
                continue;
            }
            self.parse_attribute(state);
        }

        let mut is_self_closing = false;
        if state.at(SvelteTokenType::TagSelfClose) {
            state.bump();
            is_self_closing = true;
        }
        else {
            state.expect(SvelteTokenType::TagClose).ok();
        }
        state.finish_at(start_cp, SvelteElementType::StartTag);

        if !is_self_closing {
            while state.not_at_end() && !state.at(SvelteTokenType::TagEndOpen) {
                self.parse_node(state);
            }

            if state.at(SvelteTokenType::TagEndOpen) {
                let end_cp = state.checkpoint();
                state.bump(); // </
                state.expect(SvelteTokenType::Identifier).ok();
                state.expect(SvelteTokenType::TagClose).ok();
                state.finish_at(end_cp, SvelteElementType::EndTag);
            }
        }

        state.finish_at(cp, SvelteElementType::Element);
    }

    fn parse_attribute<'a, S: Source + ?Sized>(&self, state: &mut SvelteParserState<'a, S>) {
        let cp = state.checkpoint();

        if state.at(SvelteTokenType::Identifier) {
            let text = state.peek_text().map(|c| c.to_string()).unwrap_or_default();
            let is_directive = text.contains(':') || text.starts_with('|');

            state.bump(); // name

            if state.eat(SvelteTokenType::Eq) {
                if state.at(SvelteTokenType::StringLiteral) {
                    state.bump();
                }
                else if state.at(SvelteTokenType::OpenBrace) {
                    self.parse_interpolation(state);
                }
            }

            let element_type = if is_directive { SvelteElementType::Directive } else { SvelteElementType::Attribute };
            state.finish_at(cp, element_type);
        }
        else {
            state.bump();
            state.finish_at(cp, SvelteElementType::Error);
        }
    }

    fn parse_control_block<'a, S: Source + ?Sized>(&self, state: &mut SvelteParserState<'a, S>) {
        let cp = state.checkpoint();
        state.expect(SvelteTokenType::OpenBrace).ok();

        let mut is_start_block = false;
        let mut is_snippet = false;

        if state.at(SvelteTokenType::Hash) {
            state.bump();
            is_start_block = true;
            if let Some(text) = state.peek_text() {
                if text == "snippet" {
                    is_snippet = true;
                }
            }
            state.expect(SvelteTokenType::Identifier).ok();
        }
        else if state.at(SvelteTokenType::Slash) || state.at(SvelteTokenType::Colon) {
            state.bump();
            state.expect(SvelteTokenType::Identifier).ok();
        }

        while state.not_at_end() && !state.at(SvelteTokenType::CloseBrace) {
            state.bump();
        }
        state.expect(SvelteTokenType::CloseBrace).ok();

        if is_start_block {
            // Parse body until end tag or middle tag
            while state.not_at_end() {
                if state.at(SvelteTokenType::OpenBrace) {
                    let next = state.peek_kind_at(1);
                    if next == Some(SvelteTokenType::Slash) || next == Some(SvelteTokenType::Colon) {
                        break;
                    }
                }
                self.parse_node(state);
            }

            // If it's a middle block, we stop here and let the parent handle it?
            // Actually, Svelte blocks are usually nested.
            // {#if} ... {:else} ... {/if}
            // We can treat the whole thing as one ControlBlock.

            while state.at(SvelteTokenType::OpenBrace) && state.peek_kind_at(1) == Some(SvelteTokenType::Colon) {
                self.parse_middle_block(state);

                while state.not_at_end() {
                    if state.at(SvelteTokenType::OpenBrace) {
                        let next = state.peek_kind_at(1);
                        if next == Some(SvelteTokenType::Slash) || next == Some(SvelteTokenType::Colon) {
                            break;
                        }
                    }
                    self.parse_node(state);
                }
            }

            if state.at(SvelteTokenType::OpenBrace) && state.peek_kind_at(1) == Some(SvelteTokenType::Slash) {
                self.parse_end_block(state);
            }
        }

        let element_type = if is_snippet { SvelteElementType::Snippet } else { SvelteElementType::ControlBlock };
        state.finish_at(cp, element_type);
    }

    fn parse_middle_block<'a, S: Source + ?Sized>(&self, state: &mut SvelteParserState<'a, S>) {
        let cp = state.checkpoint();
        state.expect(SvelteTokenType::OpenBrace).ok();
        state.expect(SvelteTokenType::Colon).ok();
        state.expect(SvelteTokenType::Identifier).ok();
        while state.not_at_end() && !state.at(SvelteTokenType::CloseBrace) {
            state.bump();
        }
        state.expect(SvelteTokenType::CloseBrace).ok();
        state.finish_at(cp, SvelteElementType::MiddleBlock);
    }

    fn parse_end_block<'a, S: Source + ?Sized>(&self, state: &mut SvelteParserState<'a, S>) {
        let cp = state.checkpoint();
        state.expect(SvelteTokenType::OpenBrace).ok();
        state.expect(SvelteTokenType::Slash).ok();
        state.expect(SvelteTokenType::Identifier).ok();
        while state.not_at_end() && !state.at(SvelteTokenType::CloseBrace) {
            state.bump();
        }
        state.expect(SvelteTokenType::CloseBrace).ok();
        state.finish_at(cp, SvelteElementType::EndTag);
    }

    fn parse_special_tag<'a, S: Source + ?Sized>(&self, state: &mut SvelteParserState<'a, S>) {
        let cp = state.checkpoint();
        state.expect(SvelteTokenType::OpenBrace).ok();
        state.expect(SvelteTokenType::At).ok();
        while state.not_at_end() && !state.at(SvelteTokenType::CloseBrace) {
            state.bump();
        }
        state.expect(SvelteTokenType::CloseBrace).ok();
        state.finish_at(cp, SvelteElementType::ControlBlock);
    }

    fn parse_interpolation<'a, S: Source + ?Sized>(&self, state: &mut SvelteParserState<'a, S>) {
        let cp = state.checkpoint();
        state.expect(SvelteTokenType::OpenBrace).ok();
        while state.not_at_end() && !state.at(SvelteTokenType::CloseBrace) {
            state.bump();
        }
        state.expect(SvelteTokenType::CloseBrace).ok();
        state.finish_at(cp, SvelteElementType::Interpolation);
    }

    fn parse_text<'a, S: Source + ?Sized>(&self, state: &mut SvelteParserState<'a, S>) {
        let cp = state.checkpoint();
        while state.not_at_end() && !state.at(SvelteTokenType::TagOpen) && !state.at(SvelteTokenType::OpenBrace) && !state.at(SvelteTokenType::Comment) {
            state.bump();
        }
        if state.tokens.index() > cp.0 {
            state.finish_at(cp, SvelteElementType::Text);
        }
    }
}

impl<'config> Parser<SvelteLanguage> for SvelteParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, source: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<SvelteLanguage>) -> ParseOutput<'a, SvelteLanguage> {
        let lexer = crate::lexer::SvelteLexer::new(self._config);
        oak_core::parser::parse_with_lexer(&lexer, source, edits, cache, |state| self.parse_root_internal(state))
    }
}
