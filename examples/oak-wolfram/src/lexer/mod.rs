use crate::{kind::WolframSyntaxKind, language::WolframLanguage};
use oak_core::{
    IncrementalCache, Lexer, LexerState, OakError,
    lexer::{CommentLine, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<S: Source> = LexerState<S, WolframLanguage>;

static WL_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static WL_COMMENT: LazyLock<CommentLine> = LazyLock::new(|| CommentLine { line_markers: &[] }); // Wolfram uses block comments
static WL_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });

#[derive(Clone)]
pub struct WolframLexer<'config> {
    config: &'config WolframLanguage,
}

impl<'config> Lexer<WolframLanguage> for WolframLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<WolframLanguage>,
    ) -> LexOutput<WolframLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> WolframLexer<'config> {
    pub fn new(config: &'config WolframLanguage) -> Self {
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

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(WolframSyntaxKind::Eof, eof_pos, eof_pos);
        Ok(())
    }

    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        match WL_WHITESPACE.scan(state.rest(), state.get_position(), WolframSyntaxKind::Whitespace) {
            Some(token) => {
                state.advance_with(token);
                return true;
            }
            None => {}
        }

        // Handle newlines separately
        if let Some(ch) = state.current() {
            if ch == '\n' || ch == '\r' {
                let start = state.get_position();
                state.advance(1);
                if ch == '\r' && state.current() == Some('\n') {
                    state.advance(1);
                }
                state.add_token(WolframSyntaxKind::Newline, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn skip_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // Wolfram block comment: (* ... *) with nesting support
        if rest.starts_with("(*") {
            state.advance(2);
            let mut depth = 1usize;
            while let Some(ch) = state.peek() {
                if ch == '(' && state.peek_next_n(1) == Some('*') {
                    state.advance(2);
                    depth += 1;
                    continue;
                }
                if ch == '*' && state.peek_next_n(1) == Some(')') {
                    state.advance(2);
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                    continue;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(WolframSyntaxKind::Comment, start, state.get_position());
            return true;
        }
        false
    }

    fn lex_string_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        // Normal string: "..."
        if state.current() == Some('"') {
            state.advance(1);
            let mut escaped = false;
            while let Some(ch) = state.peek() {
                if ch == '"' && !escaped {
                    state.advance(1); // consume closing quote
                    break;
                }
                state.advance(ch.len_utf8());
                if escaped {
                    escaped = false;
                    continue;
                }
                if ch == '\\' {
                    escaped = true;
                    continue;
                }
                if ch == '\n' || ch == '\r' {
                    break;
                }
            }
            state.add_token(WolframSyntaxKind::String, start, state.get_position());
            return true;
        }
        false
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

        let mut is_real = false;

        // Integer part
        state.advance(1);
        while let Some(c) = state.peek() {
            if c.is_ascii_digit() {
                state.advance(1);
            }
            else {
                break;
            }
        }

        // Decimal part
        if state.peek() == Some('.') {
            let next = state.peek_next_n(1);
            if next.map(|c| c.is_ascii_digit()).unwrap_or(false) {
                is_real = true;
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

        // Scientific notation
        if let Some(c) = state.peek() {
            if c == 'e' || c == 'E' {
                let next = state.peek_next_n(1);
                if next == Some('+') || next == Some('-') || next.map(|d| d.is_ascii_digit()).unwrap_or(false) {
                    is_real = true;
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

        let end = state.get_position();
        state.add_token(if is_real { WolframSyntaxKind::Real } else { WolframSyntaxKind::Integer }, start, end);
        true
    }

    fn lex_identifier_or_keyword<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let ch = match state.current() {
            Some(c) => c,
            None => return false,
        };

        if !(ch.is_ascii_alphabetic() || ch == '$') {
            return false;
        }

        state.advance(1);
        while let Some(c) = state.current() {
            if c.is_ascii_alphanumeric() || c == '$' {
                state.advance(1);
            }
            else {
                break;
            }
        }

        let end = state.get_position();
        let text = state.get_text_in((start..end).into());
        let kind = match text {
            "If" => WolframSyntaxKind::If,
            "Then" => WolframSyntaxKind::Then,
            "Else" => WolframSyntaxKind::Else,
            "While" => WolframSyntaxKind::While,
            "For" => WolframSyntaxKind::For,
            "Do" => WolframSyntaxKind::Do,
            "Function" => WolframSyntaxKind::Function,
            "Module" => WolframSyntaxKind::Module,
            "Block" => WolframSyntaxKind::Block,
            "With" => WolframSyntaxKind::With,
            "Table" => WolframSyntaxKind::Table,
            "Map" => WolframSyntaxKind::Map,
            "Apply" => WolframSyntaxKind::Apply,
            "Select" => WolframSyntaxKind::Select,
            "Cases" => WolframSyntaxKind::Cases,
            "Rule" => WolframSyntaxKind::Rule,
            "RuleDelayed" => WolframSyntaxKind::RuleDelayed,
            "Set" => WolframSyntaxKind::Set,
            "SetDelayed" => WolframSyntaxKind::SetDelayed,
            "Unset" => WolframSyntaxKind::Unset,
            "Clear" => WolframSyntaxKind::Clear,
            "ClearAll" => WolframSyntaxKind::ClearAll,
            "Return" => WolframSyntaxKind::Return,
            "Break" => WolframSyntaxKind::Break,
            "Continue" => WolframSyntaxKind::Continue,
            "True" => WolframSyntaxKind::True,
            "False" => WolframSyntaxKind::False,
            "Null" => WolframSyntaxKind::Null,
            "Export" => WolframSyntaxKind::Export,
            "Import" => WolframSyntaxKind::Import,
            _ => WolframSyntaxKind::Identifier,
        };
        state.add_token(kind, start, state.get_position());
        true
    }

    fn lex_operators<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // Multi-character operators (prefer longest matches first)
        let patterns: &[(&str, WolframSyntaxKind)] = &[
            ("===", WolframSyntaxKind::Equal),    // SameQ
            ("=!=", WolframSyntaxKind::NotEqual), // UnsameQ
            ("->", WolframSyntaxKind::Arrow),
            ("=>", WolframSyntaxKind::DoubleArrow),
            ("==", WolframSyntaxKind::Equal),
            ("!=", WolframSyntaxKind::NotEqual),
            ("<=", WolframSyntaxKind::LessEqual),
            (">=", WolframSyntaxKind::GreaterEqual),
            ("&&", WolframSyntaxKind::And),
            ("||", WolframSyntaxKind::Or),
            ("+=", WolframSyntaxKind::AddTo),
            ("-=", WolframSyntaxKind::SubtractFrom),
            ("*=", WolframSyntaxKind::TimesBy),
            ("/=", WolframSyntaxKind::DivideBy),
            ("___", WolframSyntaxKind::TripleUnderscore),
            ("__", WolframSyntaxKind::DoubleUnderscore),
            ("##", WolframSyntaxKind::SlotSequence),
        ];

        for (pat, kind) in patterns {
            if rest.starts_with(pat) {
                state.advance(pat.len());
                state.add_token(*kind, start, state.get_position());
                return true;
            }
        }

        // Single-character operators
        if let Some(ch) = state.current() {
            let kind = match ch {
                '+' => Some(WolframSyntaxKind::Plus),
                '-' => Some(WolframSyntaxKind::Minus),
                '*' => Some(WolframSyntaxKind::Times),
                '/' => Some(WolframSyntaxKind::Divide),
                '^' => Some(WolframSyntaxKind::Power),
                '=' => Some(WolframSyntaxKind::Assign),
                '<' => Some(WolframSyntaxKind::Less),
                '>' => Some(WolframSyntaxKind::Greater),
                '!' => Some(WolframSyntaxKind::Not),
                '?' => Some(WolframSyntaxKind::Question),
                '_' => Some(WolframSyntaxKind::Underscore),
                '#' => Some(WolframSyntaxKind::Slot),
                '.' => Some(WolframSyntaxKind::Dot),
                ':' => Some(WolframSyntaxKind::Colon),
                _ => None,
            };
            if let Some(k) = kind {
                state.advance(ch.len_utf8());
                state.add_token(k, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_single_char_tokens<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.current() {
            let kind = match ch {
                '(' => WolframSyntaxKind::LeftParen,
                ')' => WolframSyntaxKind::RightParen,
                '[' => WolframSyntaxKind::LeftBracket,
                ']' => WolframSyntaxKind::RightBracket,
                '{' => WolframSyntaxKind::LeftBrace,
                '}' => WolframSyntaxKind::RightBrace,
                ',' => WolframSyntaxKind::Comma,
                ';' => WolframSyntaxKind::Semicolon,
                _ => {
                    // Unknown character, treat as error
                    state.advance(ch.len_utf8());
                    state.add_token(WolframSyntaxKind::Error, start, state.get_position());
                    return true;
                }
            };
            state.advance(ch.len_utf8());
            state.add_token(kind, start, state.get_position());
            true
        }
        else {
            false
        }
    }
}
