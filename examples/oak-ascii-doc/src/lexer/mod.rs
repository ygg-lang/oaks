#![doc = include_str!("readme.md")]
/// AsciiDoc token types and role definitions.
pub mod token_type;

pub use token_type::AsciiDocTokenType;

use crate::language::AsciiDocLanguage;
use oak_core::{
    Lexer, OakError,
    lexer::{LexOutput, LexerCache, LexerState},
    source::{Source, TextEdit},
};
use std::sync::LazyLock;

/// State alias for the AsciiDoc lexer.
pub(crate) type State<'a, S> = LexerState<'a, S, AsciiDocLanguage>;

/// Lexer for the AsciiDoc language.
#[derive(Clone)]
pub struct AsciiDocLexer;

impl<'config> Lexer<AsciiDocLanguage> for AsciiDocLexer {
    /// Lexes the source text into tokens.
    fn lex<'a, S: Source + ?Sized>(&self, text: &'a S, _edits: &[oak_core::TextEdit], cache: &'a mut impl LexerCache<AsciiDocLanguage>) -> LexOutput<AsciiDocLanguage> {
        let mut state = State::new(text);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof()
        }
        state.finish_with_cache(result, cache)
    }
}

impl AsciiDocLexer {
    /// Creates a new `AsciiDocLexer` instance.
    pub fn new(_config: &AsciiDocLanguage) -> Self {
        Self
    }

    /// Main lexing logic
    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if let Some(ch) = state.peek() {
                match ch {
                    ':' => {
                        if self.lex_attribute(state) {
                            continue;
                        }
                        self.lex_delimiter(state);
                    }
                    'A'..='Z' => {
                        if self.lex_admonition(state) {
                            continue;
                        }
                        self.lex_text(state);
                    }
                    ' ' | '\t' => {
                        if self.lex_line_break(state) {
                            continue;
                        }
                        self.skip_whitespace(state);
                    }
                    '\n' | '\r' => {
                        self.lex_newline(state);
                    }
                    '=' => {
                        if self.lex_header(state) {
                            continue;
                        }
                        if self.lex_block(state) {
                            continue;
                        }
                        self.lex_text(state);
                    }
                    '*' => {
                        if self.lex_list_item(state) {
                            continue;
                        }
                        if self.lex_block(state) {
                            continue;
                        }
                        if self.lex_bold(state) {
                            continue;
                        }
                        self.lex_text(state);
                    }
                    '_' => {
                        if self.lex_block(state) {
                            continue;
                        }
                        if self.lex_italic(state) {
                            continue;
                        }
                        self.lex_text(state);
                    }
                    '`' => {
                        if self.lex_monospace(state) {
                            continue;
                        }
                        self.lex_text(state);
                    }
                    '-' => {
                        if self.lex_block(state) {
                            continue;
                        }
                        if self.lex_list_item(state) {
                            continue;
                        }
                        self.lex_text(state);
                    }
                    '.' => {
                        if self.lex_block(state) {
                            continue;
                        }
                        if self.lex_list_item(state) {
                            continue;
                        }
                        self.lex_delimiter(state);
                    }
                    '+' => {
                        if self.lex_list_item(state) {
                            continue;
                        }
                        self.lex_text(state);
                    }
                    '/' => {
                        if self.lex_block(state) {
                            continue;
                        }
                        if self.skip_comment(state) {
                            continue;
                        }
                        self.lex_text(state);
                    }
                    '|' => {
                        if self.lex_block(state) {
                            continue;
                        }
                        self.lex_delimiter(state);
                    }
                    'l' | 'h' => {
                        if self.lex_link(state) {
                            continue;
                        }
                        self.lex_text(state);
                    }
                    '[' | ']' | '(' | ')' | ',' => {
                        self.lex_delimiter(state);
                    }
                    '<' => {
                        if self.lex_page_break(state) {
                            continue;
                        }
                        self.lex_text(state);
                    }
                    _ => {
                        self.lex_text(state);
                    }
                }
            }

            state.advance_if_dead_lock(safe_point)
        }

        Ok(())
    }

    /// Handles attributes :name: value
    fn lex_attribute<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        // Must be at the start of the line
        if start > 0 {
            if let Some(prev) = state.source().get_char_at(start - 1) {
                if prev != '\n' && prev != '\r' {
                    return false;
                }
            }
        }

        if state.peek() == Some(':') {
            state.advance(1);
            let mut has_name = false;
            while let Some(ch) = state.peek() {
                if ch == ':' {
                    state.advance(1);
                    has_name = true;
                    break;
                }
                else if ch == '\n' || ch == '\r' || ch == ' ' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            if has_name {
                state.add_token(AsciiDocTokenType::AttributeMarker, start, state.get_position());
                return true;
            }
        }

        state.set_position(start);
        false
    }

    /// Handles admonitions NOTE:, TIP:, etc.
    fn lex_admonition<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        // Must be at the start of the line
        if start > 0 {
            if let Some(prev) = state.source().get_char_at(start - 1) {
                if prev != '\n' && prev != '\r' {
                    return false;
                }
            }
        }

        let markers = ["NOTE:", "TIP:", "IMPORTANT:", "CAUTION:", "WARNING:"];
        for marker in markers {
            if state.starts_with(marker) {
                state.advance(marker.len());
                state.add_token(AsciiDocTokenType::AdmonitionMarker, start, state.get_position());
                return true;
            }
        }

        false
    }

    /// Handles spaces before a newline (Hard Line Break)
    fn lex_line_break<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if state.starts_with(" +") {
            state.advance(2);
            if let Some('\n') | Some('\r') = state.peek() {
                state.add_token(AsciiDocTokenType::LineBreak, start, state.get_position());
                return true;
            }
        }
        state.set_position(start);
        false
    }

    /// Skips whitespace characters
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }
        if state.get_position() > start {
            state.add_token(AsciiDocTokenType::Whitespace, start, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(AsciiDocTokenType::Newline, start, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(AsciiDocTokenType::Newline, start, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if state.starts_with("//") {
            state.advance(2);
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(AsciiDocTokenType::Comment, start, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// Handles headers (= Title, == Subtitle, etc.)
    fn lex_header<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        // Check if at the start of the line
        if start > 0 {
            if let Some(prev) = state.source().get_char_at(start - 1) {
                if prev != '\n' && prev != '\r' {
                    return false;
                }
            }
        }

        let mut level = 0;
        while state.peek() == Some('=') {
            level += 1;
            state.advance(1);
            if level > 6 {
                break;
            }
        }

        if level > 0 {
            // Check for space after marker
            if state.peek() == Some(' ') || state.peek() == Some('\t') {
                let kind = match level {
                    1 => AsciiDocTokenType::Header1,
                    2 => AsciiDocTokenType::Header2,
                    3 => AsciiDocTokenType::Header3,
                    4 => AsciiDocTokenType::Header4,
                    5 => AsciiDocTokenType::Header5,
                    6 => AsciiDocTokenType::Header6,
                    _ => {
                        state.set_position(start);
                        return false;
                    }
                };
                state.add_token(kind, start, state.get_position());
                return true;
            }
        }

        state.set_position(start);
        false
    }

    /// Handles bold text *text* or **text**
    fn lex_bold<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if state.starts_with("**") {
            state.advance(2);
            state.add_token(AsciiDocTokenType::BoldMarker, start, state.get_position());
            return true;
        }
        else if state.starts_with("*") {
            state.advance(1);
            state.add_token(AsciiDocTokenType::BoldMarker, start, state.get_position());
            return true;
        }
        false
    }

    /// Handles italic text _text_ or __text__
    fn lex_italic<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if state.starts_with("__") {
            state.advance(2);
            state.add_token(AsciiDocTokenType::ItalicMarker, start, state.get_position());
            return true;
        }
        else if state.starts_with("_") {
            state.advance(1);
            state.add_token(AsciiDocTokenType::ItalicMarker, start, state.get_position());
            return true;
        }
        false
    }

    /// Handles monospace text `text`
    fn lex_monospace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            if ch == '`' {
                state.advance(1);
                state.add_token(AsciiDocTokenType::MonospaceMarker, start, state.get_position());
                return true;
            }
        }
        false
    }

    /// Handles block markers (----, ====, ****, ____, ...., |===)
    fn lex_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        // Check if at the start of the line
        if start > 0 {
            if let Some(prev) = state.source().get_char_at(start - 1) {
                if prev != '\n' && prev != '\r' {
                    return false;
                }
            }
        }

        if let Some(ch) = state.peek() {
            match ch {
                '-' | '=' | '*' | '_' | '.' | '+' => {
                    let mut count = 0;
                    while state.peek() == Some(ch) {
                        count += 1;
                        state.advance(1);
                    }
                    if count >= 4 {
                        state.add_token(AsciiDocTokenType::CodeBlockMarker, start, state.get_position());
                        return true;
                    }
                }
                '|' => {
                    if state.starts_with("|===") {
                        state.advance(4);
                        while state.peek() == Some('=') {
                            state.advance(1);
                        }
                        state.add_token(AsciiDocTokenType::CodeBlockMarker, start, state.get_position());
                        return true;
                    }
                }
                '/' => {
                    if state.starts_with("////") {
                        state.advance(4);
                        while state.peek() == Some('/') {
                            state.advance(1);
                        }
                        state.add_token(AsciiDocTokenType::Comment, start, state.get_position());
                        return true;
                    }
                }
                _ => {}
            }
        }

        state.set_position(start);
        false
    }

    /// Handles list items *, -, +, . (supports multiple levels)
    fn lex_list_item<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        // Must be at the start of the line
        if start > 0 {
            if let Some(prev) = state.source().get_char_at(start - 1) {
                if prev != '\n' && prev != '\r' {
                    return false;
                }
            }
        }

        if let Some(ch) = state.peek() {
            match ch {
                '*' | '-' | '+' | '.' => {
                    while state.peek() == Some(ch) {
                        state.advance(1);
                    }

                    // Check for space after marker
                    if state.peek() == Some(' ') || state.peek() == Some('\t') {
                        state.add_token(AsciiDocTokenType::ListMarker, start, state.get_position());
                        return true;
                    }
                }
                _ => {}
            }
        }

        state.set_position(start);
        false
    }

    /// Handles links link:url[text] or http(s)://url[text]
    fn lex_link<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if state.starts_with("link:") {
            state.advance(5);
            state.add_token(AsciiDocTokenType::LinkMarker, start, state.get_position());
            return true;
        }
        else if state.starts_with("http://") {
            state.advance(7);
            state.add_token(AsciiDocTokenType::LinkMarker, start, state.get_position());
            return true;
        }
        else if state.starts_with("https://") {
            state.advance(8);
            state.add_token(AsciiDocTokenType::LinkMarker, start, state.get_position());
            return true;
        }
        false
    }

    /// Handles page breaks <<<
    fn lex_page_break<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if state.starts_with("<<<") {
            state.advance(3);
            state.add_token(AsciiDocTokenType::PageBreak, start, state.get_position());
            return true;
        }
        false
    }

    /// Handles delimiters
    fn lex_delimiter<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            let kind = match ch {
                '[' => AsciiDocTokenType::LeftBracket,
                ']' => AsciiDocTokenType::RightBracket,
                '(' => AsciiDocTokenType::LeftParen,
                ')' => AsciiDocTokenType::RightParen,
                ':' => AsciiDocTokenType::Colon,
                ',' => AsciiDocTokenType::Comma,
                '.' => AsciiDocTokenType::Dot,
                '|' => AsciiDocTokenType::TableDelimiter,
                _ => return false,
            };
            state.advance(ch.len_utf8());
            state.add_token(kind, start, state.get_position());
            return true;
        }
        false
    }

    /// Handles plain text
    fn lex_text<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        while let Some(ch) = state.peek() {
            match ch {
                ' ' | '\t' | '\n' | '\r' | '=' | '*' | '_' | '`' | '-' | '+' | '[' | ']' | '(' | ')' | ':' | ',' | '.' | '|' | '/' | '<' => break,
                _ => {
                    state.advance(ch.len_utf8());
                }
            }
        }

        if state.get_position() > start {
            state.add_token(AsciiDocTokenType::Text, start, state.get_position());
            true
        }
        else {
            false
        }
    }
}
