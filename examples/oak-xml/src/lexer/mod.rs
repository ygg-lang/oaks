#![doc = include_str!("readme.md")]
pub mod token_type;

use crate::{language::XmlLanguage, lexer::token_type::XmlTokenType};
use oak_core::{
    Lexer, LexerCache, LexerState, OakError, TextEdit,
    lexer::{CommentConfig, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<'a, S> = LexerState<'a, S, XmlLanguage>;

// XML 静态配置
static XML_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });

static XML_COMMENT: LazyLock<CommentConfig> = LazyLock::new(|| CommentConfig { line_marker: "", block_start: "<!--", block_end: "-->", nested_blocks: false });

static XML_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"', '\''], escape: None });

impl<'config> Lexer<XmlLanguage> for XmlLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[oak_core::TextEdit], cache: &'a mut impl LexerCache<XmlLanguage>) -> LexOutput<XmlLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

#[derive(Clone)]
pub struct XmlLexer<'config> {
    _config: &'config XmlLanguage,
}

impl<'config> XmlLexer<'config> {
    pub fn new(config: &'config XmlLanguage) -> Self {
        Self { _config: config }
    }

    /// 主要的词法分析循环
    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if self.skip_whitespace(state) {
                continue;
            }

            if self.lex_comment(state) {
                continue;
            }

            if self.lex_doctype(state) {
                continue;
            }

            if self.lex_cdata(state) {
                continue;
            }

            if self.lex_processing_instruction(state) {
                continue;
            }

            if self.lex_tag_start(state) {
                continue;
            }

            if self.lex_entity_reference(state) {
                continue;
            }

            if self.lex_string_literal(state) {
                continue;
            }

            if self.lex_identifier_or_tag_name(state) {
                continue;
            }

            if self.lex_single_char_tokens(state) {
                continue;
            }

            if self.lex_text(state) {
                continue;
            }

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }

    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        XML_WHITESPACE.scan(state, XmlTokenType::Whitespace)
    }

    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        XML_COMMENT.scan(state, XmlTokenType::Comment, XmlTokenType::Comment)
    }

    fn lex_doctype<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('<') = state.peek() {
            if let Some('!') = state.peek_next_n(1) {
                // Check for DOCTYPE keyword
                let doctype_keyword = "DOCTYPE";
                let mut matches = true;
                for (i, expected_ch) in doctype_keyword.chars().enumerate() {
                    if let Some(actual_ch) = state.peek_next_n(2 + i) {
                        if actual_ch.to_ascii_uppercase() != expected_ch {
                            matches = false;
                            break;
                        }
                    }
                    else {
                        matches = false;
                        break;
                    }
                }

                if matches {
                    state.advance(2 + doctype_keyword.len()); // Skip <!DOCTYPE

                    let mut bracket_depth = 0;
                    // Find DOCTYPE end
                    while state.not_at_end() {
                        match state.peek() {
                            Some('[') => {
                                bracket_depth += 1;
                                state.advance(1);
                            }
                            Some(']') => {
                                bracket_depth -= 1;
                                state.advance(1);
                            }
                            Some('>') => {
                                if bracket_depth == 0 {
                                    state.advance(1); // Skip >
                                    state.add_token(XmlTokenType::DoctypeDeclaration, start_pos, state.get_position());
                                    return true;
                                }
                                else {
                                    state.advance(1);
                                }
                            }
                            Some(ch) => {
                                state.advance(ch.len_utf8());
                            }
                            None => break,
                        }
                    }

                    // Unclosed DOCTYPE
                    state.add_token(XmlTokenType::Error, start_pos, state.get_position());
                    return true;
                }
            }
        }

        false
    }

    fn lex_cdata<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('<') = state.peek() {
            if let Some('!') = state.peek_next_n(1) {
                if let Some('[') = state.peek_next_n(2) {
                    // Check CDATA start tag
                    let cdata_start = "CDATA[";
                    let mut matches = true;
                    for (i, expected_ch) in cdata_start.chars().enumerate() {
                        if let Some(actual_ch) = state.peek_next_n(3 + i) {
                            if actual_ch != expected_ch {
                                matches = false;
                                break;
                            }
                        }
                        else {
                            matches = false;
                            break;
                        }
                    }

                    if matches {
                        state.advance(3 + cdata_start.len()); // Skip <![CDATA[

                        // Find CDATA end ]]>
                        while state.not_at_end() {
                            if let Some(']') = state.peek() {
                                if let Some(']') = state.peek_next_n(1) {
                                    if let Some('>') = state.peek_next_n(2) {
                                        state.advance(3); // Skip ]]>
                                        state.add_token(XmlTokenType::CData, start_pos, state.get_position());
                                        return true;
                                    }
                                }
                            }
                            if let Some(ch) = state.peek() {
                                state.advance(ch.len_utf8());
                            }
                            else {
                                break;
                            }
                        }

                        // Unclosed CDATA
                        state.add_token(XmlTokenType::Error, start_pos, state.get_position());
                        return true;
                    }
                }
            }
        }

        false
    }

    fn lex_processing_instruction<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('<') = state.peek() {
            if let Some('?') = state.peek_next_n(1) {
                state.advance(2); // Skip <?

                // Find processing instruction end ?>
                while state.not_at_end() {
                    if let Some('?') = state.peek() {
                        if let Some('>') = state.peek_next_n(1) {
                            state.advance(2); // Skip ?>
                            state.add_token(XmlTokenType::ProcessingInstruction, start_pos, state.get_position());
                            return true;
                        }
                    }
                    if let Some(ch) = state.peek() {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // Unclosed processing instruction
                state.add_token(XmlTokenType::Error, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_tag_start<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        match state.peek() {
            Some('<') => {
                state.advance(1);
                if state.peek() == Some('/') {
                    state.advance(1);
                    state.add_token(XmlTokenType::LeftAngleSlash, start_pos, state.get_position());
                }
                else {
                    state.add_token(XmlTokenType::LeftAngle, start_pos, state.get_position());
                }
                true
            }
            Some('/') => {
                if state.peek_next_n(1) == Some('>') {
                    state.advance(2);
                    state.add_token(XmlTokenType::SlashRightAngle, start_pos, state.get_position());
                    true
                }
                else {
                    false
                }
            }
            Some('>') => {
                state.advance(1);
                state.add_token(XmlTokenType::RightAngle, start_pos, state.get_position());
                true
            }
            Some('=') => {
                state.advance(1);
                state.add_token(XmlTokenType::Equals, start_pos, state.get_position());
                true
            }
            _ => false,
        }
    }

    fn lex_entity_reference<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if state.peek() == Some('&') {
            state.advance(1);

            // Check for character reference &#...;
            if state.peek() == Some('#') {
                state.advance(1);
                let mut has_digits = false;

                // Hexadecimal character reference &#x...;
                if state.peek() == Some('x') {
                    state.advance(1);
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_hexdigit() {
                            state.advance(1);
                            has_digits = true;
                        }
                        else {
                            break;
                        }
                    }
                }
                else {
                    // Decimal character reference &#...;
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(1);
                            has_digits = true;
                        }
                        else {
                            break;
                        }
                    }
                }

                if has_digits && state.peek() == Some(';') {
                    state.advance(1);
                    state.add_token(XmlTokenType::CharacterReference, start_pos, state.get_position());
                    return true;
                }
            }
            else {
                // Named entity reference &name;
                let mut has_name = false;
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() {
                        state.advance(1);
                        has_name = true;
                    }
                    else {
                        break;
                    }
                }

                if has_name && state.peek() == Some(';') {
                    state.advance(1);
                    state.add_token(XmlTokenType::EntityReference, start_pos, state.get_position());
                    return true;
                }
            }

            // Invalid entity reference
            state.add_token(XmlTokenType::Error, start_pos, state.get_position());
            return true;
        }

        false
    }

    fn lex_string_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        XML_STRING.scan(state, XmlTokenType::StringLiteral)
    }

    fn lex_identifier_or_tag_name<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' || ch == ':' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' || ch == '.' || ch == ':' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                state.add_token(XmlTokenType::Identifier, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_single_char_tokens<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        match state.peek() {
            Some('"') => {
                state.advance(1);
                state.add_token(XmlTokenType::Quote, start_pos, state.get_position());
                true
            }
            Some('\'') => {
                state.advance(1);
                state.add_token(XmlTokenType::SingleQuote, start_pos, state.get_position());
                true
            }
            Some('!') => {
                state.advance(1);
                state.add_token(XmlTokenType::Exclamation, start_pos, state.get_position());
                true
            }
            Some('?') => {
                state.advance(1);
                state.add_token(XmlTokenType::Question, start_pos, state.get_position());
                true
            }
            Some('&') => {
                state.advance(1);
                state.add_token(XmlTokenType::Ampersand, start_pos, state.get_position());
                true
            }
            Some(';') => {
                state.advance(1);
                state.add_token(XmlTokenType::Semicolon, start_pos, state.get_position());
                true
            }
            _ => false,
        }
    }

    fn lex_text<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            // Stop at special characters
            match ch {
                ' ' | '\t' | '\n' | '\r' | '<' | '>' | '=' | '"' | '\'' | '!' | '?' | '&' | ';' => break,
                _ => {
                    state.advance(ch.len_utf8());
                }
            }
        }

        if state.get_position() > start_pos {
            state.add_token(XmlTokenType::Text, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}
