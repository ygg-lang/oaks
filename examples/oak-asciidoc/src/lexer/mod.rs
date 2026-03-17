#![doc = include_str!("readme.md")]
/// Token types for the AsciiDoc language.
pub mod token_type;

mod block;
mod directive;
mod inline;

use crate::{language::AsciidocLanguage, lexer::token_type::AsciidocTokenType};
use oak_core::{Lexer, LexerCache, LexerState, TextEdit, errors::OakError, lexer::LexOutput, source::Source};

pub(crate) type State<'a, S> = LexerState<'a, S, AsciidocLanguage>;

/// Lexer for AsciiDoc language.
#[derive(Clone, Debug)]
pub struct AsciidocLexer<'config> {
    pub(crate) config: &'config AsciidocLanguage,
}

impl<'config> AsciidocLexer<'config> {
    /// Creates a new AsciidocLexer with the given configuration.
    pub fn new(config: &'config AsciidocLanguage) -> Self {
        Self { config }
    }

    fn run<S: Source + ?Sized>(&self, state: &mut State<S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if let Some(ch) = state.peek() {
                match ch {
                    ' ' | '\t' => {
                        self.skip_whitespace(state);
                    }
                    '\n' | '\r' => {
                        self.lex_newline(state);
                    }
                    '#' => {
                        if self.lex_heading(state) {
                            continue;
                        }
                        self.lex_text(state);
                    }
                    '=' => {
                        if self.lex_horizontal_rule(state) {
                            continue;
                        }
                        self.lex_text(state);
                    }
                    '-' => {
                        if self.lex_list_item_marker(state) {
                            continue;
                        }
                        self.lex_text(state);
                    }
                    '+' => {
                        if self.lex_block_delimiter(state) {
                            continue;
                        }
                        self.lex_text(state);
                    }
                    '|' => {
                        if self.lex_table_separator(state) {
                            continue;
                        }
                        if self.lex_table(state) {
                            continue;
                        }
                        self.lex_text(state);
                    }
                    '`' => {
                        if self.lex_code_block(state) {
                            continue;
                        }
                        if self.lex_monospace(state) {
                            continue;
                        }
                        self.lex_text(state);
                    }
                    '*' => {
                        if self.lex_emphasis(state) {
                            continue;
                        }
                        self.lex_text(state);
                    }
                    '[' => {
                        if self.lex_attribute_or_macro(state) {
                            continue;
                        }
                        self.lex_text(state);
                    }
                    ']' => {
                        self.lex_text(state);
                    }
                    '(' => {
                        self.lex_text(state);
                    }
                    ')' => {
                        self.lex_text(state);
                    }
                    '!' => {
                        if self.lex_image(state) {
                            continue;
                        }
                        self.lex_text(state);
                    }
                    '~' => {
                        if self.lex_footnote(state) {
                            continue;
                        }
                        self.lex_text(state);
                    }
                    '^' => {
                        if self.lex_cross_reference(state) {
                            continue;
                        }
                        self.lex_text(state);
                    }
                    '{' => {
                        if self.lex_macro(state) {
                            continue;
                        }
                        self.lex_text(state);
                    }
                    '.' => {
                        if self.lex_comment(state) {
                            continue;
                        }
                        if self.lex_table_caption(state) {
                            continue;
                        }
                        self.lex_text(state);
                    }
                    ':' => {
                        if self.lex_document_attribute(state) {
                            continue;
                        }
                        self.lex_text(state);
                    }
                    'i' => {
                        if self.lex_include_directive(state) {
                            continue;
                        }
                        if self.lex_conditional_directive(state) {
                            continue;
                        }
                        self.lex_text(state);
                    }
                    '\\' => {
                        self.lex_escape(state);
                    }
                    _ => {
                        self.lex_text(state);
                    }
                }
            }

            if state.get_position() == safe_point {
                let start_pos = state.get_position();
                if let Some(ch) = state.peek() {
                    state.advance(ch.len_utf8());
                    state.add_token(AsciidocTokenType::Error, start_pos, state.get_position());
                }
            }
        }
        Ok(())
    }

    /// Skips whitespace.
    fn skip_whitespace<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        if state.get_position() > start_pos {
            state.add_token(AsciidocTokenType::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles newlines.
    fn lex_newline<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(AsciidocTokenType::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(AsciidocTokenType::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles comments.
    fn lex_comment<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if state.peek() == Some('.') {
            if state.source().get_char_at(start_pos + 1) == Some('/') {
                state.advance(2);
                while state.not_at_end() {
                    if let Some(ch) = state.peek() {
                        if ch == '\n' || ch == '\r' {
                            break;
                        }
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }
                state.add_token(AsciidocTokenType::Comment, start_pos, state.get_position());
                return true;
            }
        }
        false
    }

    /// Handles headings.
    fn lex_heading<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        let mut count = 0;
        while state.peek() == Some('#') {
            state.advance(1);
            count += 1;
        }

        if count >= 1 && count <= 6 {
            if let Some(ch) = state.peek() {
                if ch == ' ' || ch == '\t' {
                    state.add_token(AsciidocTokenType::Heading, start_pos, state.get_position());
                    return true;
                }
            }
        }

        state.set_position(start_pos);
        false
    }

    /// Handles horizontal rules.
    fn lex_horizontal_rule<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        let mut count = 0;
        while state.peek() == Some('=') {
            state.advance(1);
            count += 1;
        }

        if count >= 3 {
            state.add_token(AsciidocTokenType::HorizontalRule, start_pos, state.get_position());
            return true;
        }

        state.set_position(start_pos);
        false
    }

    /// Handles list item markers.
    fn lex_list_item_marker<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if state.peek() == Some('-') {
            state.advance(1);
            if let Some(ch) = state.peek() {
                if ch == ' ' || ch == '\t' {
                    state.add_token(AsciidocTokenType::ListItemMarker, start_pos, state.get_position());
                    return true;
                }
            }
            state.set_position(start_pos);
        }
        false
    }

    /// Handles attributes and macros.
    fn lex_attribute_or_macro<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if state.peek() == Some('[') {
            state.advance(1);
            while state.not_at_end() {
                if let Some(ch) = state.peek() {
                    if ch == ']' {
                        state.advance(1);
                        if self.config.allow_attributes {
                            state.add_token(AsciidocTokenType::Attribute, start_pos, state.get_position());
                        }
                        else if self.config.allow_macros {
                            state.add_token(AsciidocTokenType::Macro, start_pos, state.get_position());
                        }
                        return true;
                    }
                    else if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }
                else {
                    break;
                }
            }
            state.set_position(start_pos);
        }
        false
    }

    /// Handles images.
    fn lex_image<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if state.peek() == Some('!') {
            if state.source().get_char_at(start_pos + 1) == Some('[') {
                state.advance(2);
                while state.not_at_end() {
                    if let Some(ch) = state.peek() {
                        if ch == ']' {
                            state.advance(1);
                            if state.peek() == Some('(') {
                                state.advance(1);
                                while state.not_at_end() {
                                    if let Some(ch) = state.peek() {
                                        if ch == ')' {
                                            state.advance(1);
                                            state.add_token(AsciidocTokenType::Image, start_pos, state.get_position());
                                            return true;
                                        }
                                        state.advance(ch.len_utf8());
                                    }
                                    else {
                                        break;
                                    }
                                }
                            }
                            break;
                        }
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }
            }
            state.set_position(start_pos);
        }
        false
    }

    /// Handles escape sequences.
    fn lex_escape<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        if state.peek() == Some('\\') {
            state.advance(1);
            if state.not_at_end() {
                state.advance(1);
            }
            self.lex_text(state);
            true
        }
        else {
            false
        }
    }

    /// Lexes plain text.
    fn lex_text<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        while state.not_at_end() {
            if let Some(ch) = state.peek() {
                match ch {
                    ' ' | '\t' | '\n' | '\r' | '#' | '=' | '-' | '+' | '|' | '`' | '*' | '[' | ']' | '(' | ')' | '!' | '~' | '^' | '.' | ':' | '{' | '\\' => break,
                    _ => {
                        state.advance(ch.len_utf8());
                    }
                }
            }
            else {
                break;
            }
        }

        if state.get_position() > start_pos {
            state.add_token(AsciidocTokenType::Text, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles table captions.
    fn lex_table_caption<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if state.peek() == Some('.') {
            if let Some(next_ch) = state.source().get_char_at(start_pos + 1) {
                if next_ch == ' ' || next_ch == '\t' {
                    state.advance(1);

                    while state.not_at_end() {
                        if let Some(ch) = state.peek() {
                            if ch == ' ' || ch == '\t' {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                        else {
                            break;
                        }
                    }

                    while state.not_at_end() {
                        if let Some(ch) = state.peek() {
                            if ch == '\n' || ch == '\r' {
                                break;
                            }
                            state.advance(ch.len_utf8());
                        }
                        else {
                            break;
                        }
                    }

                    state.add_token(AsciidocTokenType::TableCaption, start_pos, state.get_position());
                    return true;
                }
            }
        }
        false
    }
}

impl<'config> Lexer<AsciidocLanguage> for AsciidocLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, text: &'a S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<AsciidocLanguage>) -> LexOutput<AsciidocLanguage> {
        let mut state = State::new(text);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> AsciidocLexer<'config> {
    /// Runs the lexer on the given source and returns the output.
    pub fn lex_internal<'a, S: Source + ?Sized>(&self, source: &'a S) -> LexOutput<AsciidocLanguage> {
        let mut state = State::new(source);
        let result = self.run(&mut state);
        state.finish(result)
    }
}
