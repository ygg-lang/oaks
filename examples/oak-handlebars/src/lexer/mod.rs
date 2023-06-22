use crate::{kind::HandlebarsSyntaxKind, language::HandlebarsLanguage};
use oak_core::{
    Lexer, LexerCache, LexerState, OakError, Range,
    lexer::{LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<'a, S> = LexerState<'a, S, HandlebarsLanguage>;

// Scanner configurations
static HB_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static HB_STRING_DOUBLE: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });
static HB_STRING_SINGLE: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['\''], escape: Some('\\') });

#[derive(Clone)]
pub struct HandlebarsLexer<'config> {
    _config: &'config HandlebarsLanguage,
}

impl<'config> Lexer<HandlebarsLanguage> for HandlebarsLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[oak_core::TextEdit], cache: &'a mut impl LexerCache<HandlebarsLanguage>) -> LexOutput<HandlebarsLanguage> {
        let mut state: State<'_, S> = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> HandlebarsLexer<'config> {
    pub fn new(config: &'config HandlebarsLanguage) -> Self {
        Self { _config: config }
    }

    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if self.skip_whitespace(state) {
                continue;
            }

            if self.skip_newline(state) {
                continue;
            }

            if self.lex_comment(state) {
                continue;
            }

            if self.lex_handlebars_expression(state) {
                continue;
            }

            if self.lex_string_literal(state) {
                continue;
            }

            if self.lex_number_literal(state) {
                continue;
            }

            if self.lex_identifier(state) {
                continue;
            }

            if self.lex_single_char_tokens(state) {
                continue;
            }

            if self.lex_content(state) {
                continue;
            }

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }

    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        HB_WHITESPACE.scan(state, HandlebarsSyntaxKind::Whitespace)
    }

    fn skip_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if state.current() == Some('\n') || state.current() == Some('\r') {
            let start = state.get_position();
            state.advance(1);
            if state.current() == Some('\n') && state.peek() == Some('\r') {
                state.advance(1);
            }
            let end = state.get_position();
            state.add_token(HandlebarsSyntaxKind::Newline, start, end);
            true
        }
        else {
            false
        }
    }

    fn lex_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        if state.consume_if_starts_with("{{!--") {
            // Find the end of the comment
            while state.not_at_end() {
                if state.starts_with("--}}") {
                    state.advance(4); // Skip "--}}"
                    break;
                }
                state.advance(1);
            }

            let end = state.get_position();
            state.add_token(HandlebarsSyntaxKind::Comment, start, end);
            true
        }
        else if state.consume_if_starts_with("{{!") {
            // Find the end of the comment
            while state.not_at_end() {
                if state.starts_with("}}") {
                    state.advance(2); // Skip "}}"
                    break;
                }
                state.advance(1);
            }

            let end = state.get_position();
            state.add_token(HandlebarsSyntaxKind::Comment, start, end);
            true
        }
        else {
            false
        }
    }

    fn lex_handlebars_expression<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if state.consume_if_starts_with("{{{{/") {
            let end = state.get_position();
            state.add_token(HandlebarsSyntaxKind::OpenEndRawBlock, start, end);
            true
        }
        else if state.consume_if_starts_with("{{{{") {
            let end = state.get_position();
            state.add_token(HandlebarsSyntaxKind::OpenRawBlock, start, end);
            true
        }
        else if state.consume_if_starts_with("}}}}") {
            let end = state.get_position();
            state.add_token(HandlebarsSyntaxKind::CloseRawBlock, start, end);
            true
        }
        else if state.consume_if_starts_with("{{{") {
            let end = state.get_position();
            state.add_token(HandlebarsSyntaxKind::OpenUnescaped, start, end);
            true
        }
        else if state.consume_if_starts_with("{{#") {
            let end = state.get_position();
            state.add_token(HandlebarsSyntaxKind::OpenBlock, start, end);
            true
        }
        else if state.consume_if_starts_with("{{^") {
            let end = state.get_position();
            state.add_token(HandlebarsSyntaxKind::OpenInverseBlock, start, end);
            true
        }
        else if state.consume_if_starts_with("{{/") {
            let end = state.get_position();
            state.add_token(HandlebarsSyntaxKind::CloseBlock, start, end);
            true
        }
        else if state.consume_if_starts_with("{{>") {
            let end = state.get_position();
            state.add_token(HandlebarsSyntaxKind::OpenPartial, start, end);
            true
        }
        else if state.consume_if_starts_with("{{") {
            let end = state.get_position();
            state.add_token(HandlebarsSyntaxKind::Open, start, end);
            true
        }
        else if state.consume_if_starts_with("}}}") {
            let end = state.get_position();
            state.add_token(HandlebarsSyntaxKind::CloseUnescaped, start, end);
            true
        }
        else if state.consume_if_starts_with("}}") {
            let end = state.get_position();
            state.add_token(HandlebarsSyntaxKind::Close, start, end);
            true
        }
        else {
            false
        }
    }

    fn lex_string_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let config = if state.current() == Some('"') {
            &*HB_STRING_DOUBLE
        }
        else if state.current() == Some('\'') {
            &*HB_STRING_SINGLE
        }
        else {
            return false;
        };

        config.scan(state, HandlebarsSyntaxKind::StringLiteral)
    }

    fn lex_number_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if let Some(c) = state.current() {
            if c.is_ascii_digit() {
                let start = state.get_position();
                while let Some(c) = state.current() {
                    if c.is_ascii_digit() || c == '.' {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }
                let end = state.get_position();
                state.add_token(HandlebarsSyntaxKind::NumberLiteral, start, end);
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    fn lex_identifier<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if let Some(c) = state.current() {
            if c.is_alphabetic() || c == '_' || c == '@' {
                let start = state.get_position();
                while let Some(c) = state.current() {
                    if c.is_alphanumeric() || c == '_' || c == '-' || c == '.' {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }
                let end = state.get_position();
                let text = state.get_text_in(Range { start, end });
                let kind = match text.as_ref() {
                    "else" => HandlebarsSyntaxKind::Else,
                    "true" | "false" => HandlebarsSyntaxKind::BooleanLiteral,
                    _ => HandlebarsSyntaxKind::Identifier,
                };
                state.add_token(kind, start, end);
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    fn lex_single_char_tokens<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        if let Some(c) = state.current() {
            let start = state.get_position();
            let kind = match c {
                '(' => HandlebarsSyntaxKind::LeftParen,
                ')' => HandlebarsSyntaxKind::RightParen,
                '[' => HandlebarsSyntaxKind::LeftBracket,
                ']' => HandlebarsSyntaxKind::RightBracket,
                '=' => HandlebarsSyntaxKind::Equal,
                '|' => HandlebarsSyntaxKind::Pipe,
                '#' => HandlebarsSyntaxKind::Hash,
                '.' => HandlebarsSyntaxKind::Dot,
                '/' => HandlebarsSyntaxKind::Slash,
                '@' => HandlebarsSyntaxKind::At,
                '^' => HandlebarsSyntaxKind::Caret,
                _ => return false,
            };
            state.advance(1);
            let end = state.get_position();
            state.add_token(kind, start, end);
            true
        }
        else {
            false
        }
    }

    fn lex_content<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let mut count = 0;

        while let Some(c) = state.current() {
            if c == '{' && state.peek() == Some('{') {
                break;
            }
            state.advance(1);
            count += 1;
        }

        if count > 0 {
            let end = state.get_position();
            state.add_token(HandlebarsSyntaxKind::Content, start, end);
            true
        }
        else {
            false
        }
    }
}
