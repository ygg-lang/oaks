use crate::{kind::WolframSyntaxKind, language::WolframLanguage};
use oak_core::{
    Lexer, LexerCache, LexerState, OakError,
    lexer::{CommentConfig, LexOutput, StringConfig, WhitespaceConfig},
    source::{Source, TextEdit},
};
use std::sync::LazyLock;

type State<'a, S> = LexerState<'a, S, WolframLanguage>;

static WL_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static WL_COMMENT: LazyLock<CommentConfig> = LazyLock::new(|| CommentConfig { line_marker: "", block_start: "(*", block_end: "*)", nested_blocks: true });
static WL_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });

#[derive(Clone, Debug)]
pub struct WolframLexer<'config> {
    _config: &'config WolframLanguage,
}

impl<'config> Lexer<WolframLanguage> for WolframLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<WolframLanguage>) -> LexOutput<WolframLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> WolframLexer<'config> {
    pub fn new(config: &'config WolframLanguage) -> Self {
        Self { _config: config }
    }

    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
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

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }

    fn skip_whitespace<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        if WL_WHITESPACE.scan(state, WolframSyntaxKind::Whitespace) {
            return true;
        }

        // Handle newlines separately
        if let Some(ch) = state.peek() {
            if ch == '\n' || ch == '\r' {
                let start = state.get_position();
                state.advance(ch.len_utf8());
                if ch == '\r' && state.peek() == Some('\n') {
                    state.advance(1);
                }
                state.add_token(WolframSyntaxKind::Newline, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn skip_comment<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        WL_COMMENT.scan(state, WolframSyntaxKind::Comment, WolframSyntaxKind::Comment)
    }

    fn lex_string_literal<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        WL_STRING.scan(state, WolframSyntaxKind::String)
    }

    fn lex_number_literal<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let first = match state.peek() {
            Some(c) => c,
            None => return false,
        };

        if !first.is_ascii_digit() {
            return false;
        }

        let mut is_real = false;

        // Integer part
        state.advance(first.len_utf8());
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

    fn lex_identifier_or_keyword<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let ch = match state.peek() {
            Some(c) => c,
            None => return false,
        };

        if !(ch.is_ascii_alphabetic() || ch == '$') {
            return false;
        }

        state.advance(ch.len_utf8());
        while let Some(c) = state.peek() {
            if c.is_ascii_alphanumeric() || c == '$' {
                state.advance(c.len_utf8());
            }
            else {
                break;
            }
        }

        let end = state.get_position();
        let text = state.source().get_text_in((start..end).into());
        let kind = match text.as_ref() {
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

    fn lex_operators<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

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
            if state.starts_with(pat) {
                state.advance(pat.len());
                state.add_token(*kind, start, state.get_position());
                return true;
            }
        }

        // Single-character operators
        if let Some(ch) = state.peek() {
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

    fn lex_single_char_tokens<S: Source + ?Sized>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        if let Some(ch) = state.peek() {
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
