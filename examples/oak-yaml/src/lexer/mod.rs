use crate::{kind::YamlSyntaxKind, language::YamlLanguage};
use oak_core::{
    Lexer, LexerState, OakError,
    lexer::{CommentConfig, LexOutput, LexerCache, StringConfig, WhitespaceConfig},
    source::Source,
};

static YAML_WHITESPACE: WhitespaceConfig = WhitespaceConfig { unicode_whitespace: false };

static YAML_COMMENT: CommentConfig = CommentConfig { line_marker: "#", block_start: "", block_end: "", nested_blocks: false };

static YAML_STRING: StringConfig = StringConfig { quotes: &['"'], escape: Some('\\') };

type State<'s, S> = LexerState<'s, S, YamlLanguage>;

#[derive(Clone)]
pub struct YamlLexer {
    _config: YamlLanguage,
}

impl YamlLexer {
    pub fn new(config: &YamlLanguage) -> Self {
        Self { _config: config.clone() }
    }

    fn run<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

            if let Some(ch) = state.peek() {
                match ch {
                    ' ' | '\t' => {
                        self.lex_whitespace(state);
                    }
                    '#' => {
                        self.lex_comment(state);
                    }
                    '\n' | '\r' => {
                        self.lex_newline(state);
                    }
                    '"' => {
                        self.lex_string_literal(state)?;
                    }
                    '0'..='9' | '+' => {
                        if self.lex_number_literal(state)? {
                            continue;
                        }
                        if self.lex_single_char_tokens(state) {
                            continue;
                        }
                    }
                    '-' => {
                        // Could be number, document start (---), or dash
                        if self.lex_number_literal(state)? {
                            continue;
                        }
                        if self.lex_multi_char_operators(state) {
                            continue;
                        }
                        if self.lex_single_char_tokens(state) {
                            continue;
                        }
                    }
                    '.' => {
                        // Could be document end (...)
                        if self.lex_multi_char_operators(state) {
                            continue;
                        }
                        // Fallback to error/unknown if not handled
                        if self.lex_single_char_tokens(state) {
                            continue;
                        }
                        // If we reach here, we have an unexpected character (handled below)
                        state.advance(ch.len_utf8());
                        state.add_token(YamlSyntaxKind::Error, safe_point, state.get_position());
                    }
                    'a'..='z' | 'A'..='Z' | '_' => {
                        self.lex_identifier_or_keyword(state)?;
                    }
                    _ => {
                        if self.lex_single_char_tokens(state) {
                            continue;
                        }

                        // If we reach here, we have an unexpected character
                        state.advance(ch.len_utf8());
                        state.add_token(YamlSyntaxKind::Error, safe_point, state.get_position());
                    }
                }
            }

            state.advance_if_dead_lock(safe_point);
        }

        state.add_eof();
        Ok(())
    }
}

impl Lexer<YamlLanguage> for YamlLexer {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[oak_core::TextEdit], cache: &'a mut impl LexerCache<YamlLanguage>) -> LexOutput<YamlLanguage> {
        let mut state = State::new(source);
        let result = self.run(&mut state);
        state.finish_with_cache(result, cache)
    }
}

impl YamlLexer {
    fn lex_whitespace<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        YAML_WHITESPACE.scan(state, YamlSyntaxKind::Whitespace)
    }

    fn lex_comment<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        YAML_COMMENT.scan(state, YamlSyntaxKind::Comment, YamlSyntaxKind::Comment)
    }

    fn lex_newline<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        if let Some(ch) = state.current() {
            if ch == '\n' {
                let start = state.get_position();
                state.advance(1);
                state.add_token(YamlSyntaxKind::Newline, start, state.get_position());
                return true;
            }
            else if ch == '\r' {
                let start = state.get_position();
                state.advance(1);
                if state.current() == Some('\n') {
                    state.advance(1);
                }
                state.add_token(YamlSyntaxKind::Newline, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_string_literal<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> Result<bool, OakError> {
        Ok(YAML_STRING.scan(state, YamlSyntaxKind::StringLiteral))
    }

    fn lex_number_literal<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> Result<bool, OakError> {
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

    fn lex_identifier_or_keyword<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> Result<bool, OakError> {
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

                let end = state.get_position();
                let text = state.source().get_text_in((start..end).into());
                let kind = self.keyword_kind(text.as_ref()).unwrap_or(YamlSyntaxKind::Identifier);
                state.add_token(kind, start, end);
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

    fn lex_multi_char_operators<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
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

    fn lex_single_char_tokens<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
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
