use crate::{kind::YamlSyntaxKind, language::YamlLanguage};
use oak_core::{
    IncrementalCache, Lexer, OakError,
    lexer::{CommentLine, LexOutput, LexerState, StringConfig, WhitespaceConfig},
    source::Source,
};

static YAML_WHITESPACE: WhitespaceConfig = WhitespaceConfig { unicode_whitespace: false };

static YAML_COMMENT: CommentLine = CommentLine { line_markers: &["#"] };

static YAML_STRING: StringConfig = StringConfig { quotes: &['"', '\''], escape: Some('\\') };

#[derive(Clone)]
pub struct YamlLexer {
    config: YamlLanguage,
}

impl YamlLexer {
    pub fn new(config: &YamlLanguage) -> Self {
        Self { config: config.clone() }
    }
}

impl Lexer<YamlLanguage> for YamlLexer {
    fn lex_incremental(
        &self,
        source: impl Source,
        _changed: usize,
        _cache: IncrementalCache<YamlLanguage>,
    ) -> LexOutput<YamlLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl YamlLexer {
    fn run(&self, state: &mut LexerState<impl Source, YamlLanguage>) -> Result<(), OakError> {
        while !state.not_at_end() {
            // Skip whitespace
            if self.lex_whitespace(state) {
                continue;
            }

            // Skip comments
            if self.lex_comment(state) {
                continue;
            }

            // Lex newlines
            if self.lex_newline(state) {
                continue;
            }

            // Lex string literals
            if self.lex_string_literal(state)? {
                continue;
            }

            // Lex number literals
            if self.lex_number_literal(state)? {
                continue;
            }

            // Lex identifiers or keywords
            if self.lex_identifier_or_keyword(state)? {
                continue;
            }

            // Lex multi-character operators
            if self.lex_multi_char_operators(state) {
                continue;
            }

            // Lex single character tokens
            if self.lex_single_char_tokens(state) {
                continue;
            }

            // If we reach here, we have an unexpected character
            let ch = state.peek().unwrap_or('\0');
            state.advance(ch.len_utf8());
            state.add_token(YamlSyntaxKind::Error, state.get_position() - ch.len_utf8(), state.get_position());
        }

        // Add EOF token
        state.add_token(YamlSyntaxKind::Eof, state.get_position(), state.get_position());
        Ok(())
    }

    fn lex_whitespace(&self, state: &mut LexerState<impl Source, YamlLanguage>) -> bool {
        let start = state.get_position();

        if let Some(token) = YAML_WHITESPACE.scan(state.rest(), 0, YamlSyntaxKind::Whitespace) {
            state.advance(token.span.end - token.span.start);
            state.add_token(YamlSyntaxKind::Whitespace, start, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_comment(&self, state: &mut LexerState<impl Source, YamlLanguage>) -> bool {
        let start = state.get_position();

        if let Some(token) = YAML_COMMENT.scan(state.rest(), 0, YamlSyntaxKind::Comment) {
            // For line comments, we need to read until end of line
            let mut length = token.span.end - token.span.start;
            let remaining = &state.rest()[length..];

            // Continue reading until newline
            for ch in remaining.chars() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                length += ch.len_utf8();
            }

            state.advance(length);
            state.add_token(YamlSyntaxKind::Comment, start, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn lex_newline(&self, state: &mut LexerState<impl Source, YamlLanguage>) -> bool {
        if let Some(ch) = state.peek() {
            if ch == '\n' {
                let start = state.get_position();
                state.advance(1);
                state.add_token(YamlSyntaxKind::Newline, start, state.get_position());
                return true;
            }
            else if ch == '\r' {
                let start = state.get_position();
                state.advance(1);
                if state.peek() == Some('\n') {
                    state.advance(1);
                }
                state.add_token(YamlSyntaxKind::Newline, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_string_literal(&self, state: &mut LexerState<impl Source, YamlLanguage>) -> Result<bool, OakError> {
        let start = state.get_position();

        if let Some(token) = YAML_STRING.scan(state.rest(), 0, YamlSyntaxKind::StringLiteral) {
            state.advance(token.span.end - token.span.start);
            state.add_token(YamlSyntaxKind::StringLiteral, start, state.get_position());
            Ok(true)
        }
        else {
            Ok(false)
        }
    }

    fn lex_number_literal(&self, state: &mut LexerState<impl Source, YamlLanguage>) -> Result<bool, OakError> {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() || (ch == '-' || ch == '+') {
                if ch == '-' || ch == '+' {
                    state.advance(1);
                    if !state.peek().map_or(false, |c| c.is_ascii_digit()) {
                        // Not a number, backtrack
                        state.set_position(start);
                        return Ok(false);
                    }
                }

                // Integer part
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // Decimal part
                if state.peek() == Some('.') {
                    state.advance(1);
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() || ch == '_' {
                            state.advance(ch.len_utf8());
                        }
                        else {
                            break;
                        }
                    }
                }

                // Exponent part
                if state.peek() == Some('e') || state.peek() == Some('E') {
                    state.advance(1);
                    if state.peek() == Some('+') || state.peek() == Some('-') {
                        state.advance(1);
                    }
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() || ch == '_' {
                            state.advance(ch.len_utf8());
                        }
                        else {
                            break;
                        }
                    }
                }

                state.add_token(YamlSyntaxKind::NumberLiteral, start, state.get_position());
                Ok(true)
            }
            else {
                Ok(false)
            }
        }
        else {
            Ok(false)
        }
    }

    fn lex_identifier_or_keyword(&self, state: &mut LexerState<impl Source, YamlLanguage>) -> Result<bool, OakError> {
        let start = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' || ch == '-' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = state.get_text_from(start)[..(state.get_position() - start)].to_string();
                let kind = self.keyword_kind(&text).unwrap_or(YamlSyntaxKind::Identifier);
                state.add_token(kind, start, state.get_position());
                Ok(true)
            }
            else {
                Ok(false)
            }
        }
        else {
            Ok(false)
        }
    }

    fn lex_multi_char_operators(&self, state: &mut LexerState<impl Source, YamlLanguage>) -> bool {
        let start = state.get_position();

        // Document start: ---
        if state.peek() == Some('-') && state.peek_next_n(1) == Some('-') && state.peek_next_n(2) == Some('-') {
            state.advance(3);
            state.add_token(YamlSyntaxKind::DocumentStart, start, state.get_position());
            return true;
        }

        // Document end: ...
        if state.peek() == Some('.') && state.peek_next_n(1) == Some('.') && state.peek_next_n(2) == Some('.') {
            state.advance(3);
            state.add_token(YamlSyntaxKind::DocumentEnd, start, state.get_position());
            return true;
        }

        false
    }

    fn lex_single_char_tokens(&self, state: &mut LexerState<impl Source, YamlLanguage>) -> bool {
        if let Some(ch) = state.peek() {
            let start = state.get_position();

            if let Some(kind) = self.single_char_kind(ch) {
                state.advance(ch.len_utf8());
                state.add_token(kind, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn keyword_kind(&self, text: &str) -> Option<YamlSyntaxKind> {
        match text {
            "true" | "True" | "TRUE" | "false" | "False" | "FALSE" => Some(YamlSyntaxKind::BooleanLiteral),
            "null" | "Null" | "NULL" | "~" => Some(YamlSyntaxKind::NullLiteral),
            _ => None,
        }
    }

    fn single_char_kind(&self, ch: char) -> Option<YamlSyntaxKind> {
        match ch {
            ':' => Some(YamlSyntaxKind::Colon),
            '-' => Some(YamlSyntaxKind::Dash),
            '|' => Some(YamlSyntaxKind::Pipe),
            '>' => Some(YamlSyntaxKind::GreaterThan),
            '?' => Some(YamlSyntaxKind::Question),
            '&' => Some(YamlSyntaxKind::Ampersand),
            '*' => Some(YamlSyntaxKind::Asterisk),
            '!' => Some(YamlSyntaxKind::Exclamation),
            '[' => Some(YamlSyntaxKind::LeftBracket),
            ']' => Some(YamlSyntaxKind::RightBracket),
            '{' => Some(YamlSyntaxKind::LeftBrace),
            '}' => Some(YamlSyntaxKind::RightBrace),
            _ => None,
        }
    }
}
