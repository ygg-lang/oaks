use crate::{kind::DHallSyntaxKind, language::DHallLanguage};
use oak_core::{
    IncrementalCache, Lexer, LexerState, OakError,
    lexer::{CommentLine, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<S: Source> = LexerState<S, DHallLanguage>;

static DHALL_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static DHALL_COMMENT: LazyLock<CommentLine> = LazyLock::new(|| CommentLine { line_markers: &["--"] });
static DHALL_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });

pub struct DHallLexer<'config> {
    config: &'config DHallLanguage,
}

impl<'config> Lexer<DHallLanguage> for DHallLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        _changed: usize,
        _cache: IncrementalCache<DHallLanguage>,
    ) -> LexOutput<DHallLanguage> {
        let mut state = LexerState::new_with_cache(source, _changed, _cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> DHallLexer<'config> {
    pub fn new(config: &'config DHallLanguage) -> Self {
        Self { config }
    }

    fn run<S: Source>(&self, state: &mut State<S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();
            if self.skip_whitespace(state) {
                continue;
            }

            if self.skip_comment(state) {
                continue;
            }

            if self.lex_string_literal(state) {
                continue;
            }

            if self.lex_number_literal(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_operators(state) {
                continue;
            }

            if self.lex_single_char_tokens(state) {
                continue;
            }

            state.safe_check(safe_point);
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(DHallSyntaxKind::Eof, eof_pos, eof_pos);
        Ok(())
    }

    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        match DHALL_WHITESPACE.scan(state.rest(), state.get_position(), DHallSyntaxKind::Whitespace) {
            Some(token) => {
                state.advance(token.length());
                true
            }
            None => false,
        }
    }

    fn skip_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        match DHALL_COMMENT.scan(state.rest(), state.get_position(), DHallSyntaxKind::Comment) {
            Some(token) => {
                state.advance(token.length());
                true
            }
            None => false,
        }
    }

    fn lex_string_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        match DHALL_STRING.scan(state.rest(), start, DHallSyntaxKind::String) {
            Some(token) => {
                state.advance(token.length());
                state.add_token(token.kind, start, state.get_position());
                true
            }
            None => false,
        }
    }

    fn lex_number_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let first = match state.current() {
            Some(c) => c,
            None => return false,
        };

        if !first.is_ascii_digit() {
            return false;
        }

        state.advance(1);
        while let Some(c) = state.peek() {
            if c.is_ascii_digit() {
                state.advance(1);
            }
            else {
                break;
            }
        }

        // Fractional part
        if state.peek() == Some('.') {
            let next = state.peek_next_n(1);
            if next.map(|c| c.is_ascii_digit()).unwrap_or(false) {
                state.advance(1); // consume '.'
                while let Some(c) = state.peek() {
                    if c.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }
            }
        }

        // Exponent part
        if let Some(c) = state.peek() {
            if c == 'e' || c == 'E' {
                let next = state.peek_next_n(1);
                if next == Some('+') || next == Some('-') || next.map(|d| d.is_ascii_digit()).unwrap_or(false) {
                    state.advance(1);
                    if let Some(sign) = state.peek() {
                        if sign == '+' || sign == '-' {
                            state.advance(1);
                        }
                    }
                    while let Some(d) = state.peek() {
                        if d.is_ascii_digit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }
            }
        }

        state.add_token(DHallSyntaxKind::Number, start, state.get_position());
        true
    }

    fn lex_identifier_or_keyword<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let ch = match state.current() {
            Some(c) => c,
            None => return false,
        };

        if !ch.is_alphabetic() && ch != '_' {
            return false;
        }

        state.advance(ch.len_utf8());
        while let Some(c) = state.peek() {
            if c.is_alphanumeric() || c == '_' {
                state.advance(c.len_utf8());
            }
            else {
                break;
            }
        }

        let end = state.get_position();
        let text = state.get_text_in((start..end).into());
        let kind = match text.to_lowercase().as_str() {
            "after" => DHallSyntaxKind::After,
            "and" => DHallSyntaxKind::And,
            "andalso" => DHallSyntaxKind::Andalso,
            "band" => DHallSyntaxKind::Band,
            "begin" => DHallSyntaxKind::Begin,
            "bnot" => DHallSyntaxKind::Bnot,
            "bor" => DHallSyntaxKind::Bor,
            "bsl" => DHallSyntaxKind::Bsl,
            "bsr" => DHallSyntaxKind::Bsr,
            "bxor" => DHallSyntaxKind::Bxor,
            "case" => DHallSyntaxKind::Case,
            "catch" => DHallSyntaxKind::Catch,
            "cond" => DHallSyntaxKind::Cond,
            "div" => DHallSyntaxKind::Div,
            "end" => DHallSyntaxKind::End,
            "fun" => DHallSyntaxKind::Fun,
            "if" => DHallSyntaxKind::If,
            "let" => DHallSyntaxKind::Let,
            "not" => DHallSyntaxKind::Not,
            "of" => DHallSyntaxKind::Of,
            "or" => DHallSyntaxKind::Or,
            "orelse" => DHallSyntaxKind::Orelse,
            "query" => DHallSyntaxKind::Query,
            "receive" => DHallSyntaxKind::Receive,
            "rem" => DHallSyntaxKind::Rem,
            "try" => DHallSyntaxKind::Try,
            "when" => DHallSyntaxKind::When,
            "xor" => DHallSyntaxKind::Xor,
            _ => DHallSyntaxKind::Identifier,
        };

        state.add_token(kind, start, state.get_position());
        true
    }

    fn lex_operators<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // Multi-character operators (longest first)
        let patterns: &[(&str, DHallSyntaxKind)] = &[
            ("==", DHallSyntaxKind::EqualEqual),
            ("/=", DHallSyntaxKind::SlashEqual),
            ("=:=", DHallSyntaxKind::EqualColonEqual),
            ("=/=", DHallSyntaxKind::EqualSlashEqual),
            ("=<", DHallSyntaxKind::LessEqual),
            (">=", DHallSyntaxKind::GreaterEqual),
            ("++", DHallSyntaxKind::PlusPlus),
            ("--", DHallSyntaxKind::MinusMinus),
            ("->", DHallSyntaxKind::Arrow),
            ("||", DHallSyntaxKind::PipePipe),
        ];

        for (pat, kind) in patterns {
            if rest.starts_with(pat) {
                state.advance(pat.len());
                state.add_token(*kind, start, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_single_char_tokens<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.current() {
            let kind = match ch {
                '+' => Some(DHallSyntaxKind::Plus),
                '-' => Some(DHallSyntaxKind::Minus),
                '*' => Some(DHallSyntaxKind::Star),
                '/' => Some(DHallSyntaxKind::Slash),
                '=' => Some(DHallSyntaxKind::Equal),
                '<' => Some(DHallSyntaxKind::Less),
                '>' => Some(DHallSyntaxKind::Greater),
                '!' => Some(DHallSyntaxKind::Exclamation),
                '?' => Some(DHallSyntaxKind::Question),
                '(' => Some(DHallSyntaxKind::LeftParen),
                ')' => Some(DHallSyntaxKind::RightParen),
                '{' => Some(DHallSyntaxKind::LeftBrace),
                '}' => Some(DHallSyntaxKind::RightBrace),
                '[' => Some(DHallSyntaxKind::LeftBracket),
                ']' => Some(DHallSyntaxKind::RightBracket),
                ',' => Some(DHallSyntaxKind::Comma),
                ';' => Some(DHallSyntaxKind::Semicolon),
                '.' => Some(DHallSyntaxKind::Dot),
                ':' => Some(DHallSyntaxKind::Colon),
                '|' => Some(DHallSyntaxKind::Pipe),
                '#' => Some(DHallSyntaxKind::Hash),
                _ => None,
            };

            if let Some(token_kind) = kind {
                state.advance(ch.len_utf8());
                state.add_token(token_kind, start, state.get_position());
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
}
