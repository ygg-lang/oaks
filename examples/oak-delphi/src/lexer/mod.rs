use crate::{kind::DelphiSyntaxKind, language::DelphiLanguage};
use oak_core::{
    IncrementalCache, Lexer, LexerState, OakError,
    lexer::{CommentLine, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<S: Source> = LexerState<S, DelphiLanguage>;

static DELPHI_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static DELPHI_COMMENT: LazyLock<CommentLine> = LazyLock::new(|| CommentLine { line_markers: &["//"] });
static DELPHI_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['\''], escape: None });

#[derive(Clone)]
pub struct DelphiLexer<'config> {
    config: &'config DelphiLanguage,
}

impl<'config> Lexer<DelphiLanguage> for DelphiLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<DelphiLanguage>,
    ) -> LexOutput<DelphiLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> DelphiLexer<'config> {
    pub fn new(config: &'config DelphiLanguage) -> Self {
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
        state.add_token(DelphiSyntaxKind::Eof, eof_pos, eof_pos);
        Ok(())
    }

    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        match DELPHI_WHITESPACE.scan(state.rest(), state.get_position(), DelphiSyntaxKind::Whitespace) {
            Some(token) => {
                state.advance_with(token);
                true
            }
            None => false,
        }
    }

    fn skip_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // Line comment: // ... until newline
        if rest.starts_with("//") {
            state.advance(2);
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(DelphiSyntaxKind::Comment, start, state.get_position());
            return true;
        }

        // Block comment: { ... } or (* ... *)
        if rest.starts_with("{") {
            state.advance(1);
            while let Some(ch) = state.peek() {
                if ch == '}' {
                    state.advance(1);
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(DelphiSyntaxKind::Comment, start, state.get_position());
            return true;
        }

        if rest.starts_with("(*") {
            state.advance(2);
            while let Some(ch) = state.peek() {
                if ch == '*' && state.peek_next_n(1) == Some(')') {
                    state.advance(2);
                    break;
                }
                state.advance(ch.len_utf8());
            }
            state.add_token(DelphiSyntaxKind::Comment, start, state.get_position());
            return true;
        }

        false
    }

    fn lex_string_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if state.current() != Some('\'') {
            return false;
        }

        state.advance(1); // consume opening quote
        while let Some(ch) = state.peek() {
            if ch == '\'' {
                // Check for escaped quote (double quote)
                if state.peek_next_n(1) == Some('\'') {
                    state.advance(2); // consume both quotes
                    continue;
                }
                else {
                    state.advance(1); // consume closing quote
                    break;
                }
            }
            if ch == '\n' || ch == '\r' {
                break; // unterminated string
            }
            state.advance(ch.len_utf8());
        }

        state.add_token(DelphiSyntaxKind::String, start, state.get_position());
        true
    }

    fn lex_number_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let first = match state.current() {
            Some(c) => c,
            None => return false,
        };

        if !first.is_ascii_digit() && first != '$' {
            return false;
        }

        let mut is_float = false;

        // Hexadecimal number
        if first == '$' {
            state.advance(1);
            while let Some(c) = state.peek() {
                if c.is_ascii_hexdigit() {
                    state.advance(1);
                }
                else {
                    break;
                }
            }
        }
        else {
            // Decimal number
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
                    is_float = true;
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
                        is_float = true;
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
        }

        state.add_token(DelphiSyntaxKind::Number, start, state.get_position());
        true
    }

    fn lex_identifier_or_keyword<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let ch = match state.current() {
            Some(c) => c,
            None => return false,
        };

        if !(ch.is_ascii_alphabetic() || ch == '_') {
            return false;
        }

        state.advance(1);
        while let Some(c) = state.current() {
            if c.is_ascii_alphanumeric() || c == '_' {
                state.advance(1);
            }
            else {
                break;
            }
        }

        let end = state.get_position();
        let text = state.get_text_in((start..end).into());
        let kind = match text.to_lowercase().as_str() {
            "and" => DelphiSyntaxKind::And_,
            "array" => DelphiSyntaxKind::Array,
            "as" => DelphiSyntaxKind::As_,
            "begin" => DelphiSyntaxKind::Begin,
            "case" => DelphiSyntaxKind::Case,
            "class" => DelphiSyntaxKind::Class,
            "const" => DelphiSyntaxKind::Const,
            "div" => DelphiSyntaxKind::Div,
            "do" => DelphiSyntaxKind::Do,
            "downto" => DelphiSyntaxKind::Downto,
            "else" => DelphiSyntaxKind::Else,
            "end" => DelphiSyntaxKind::End,
            "except" => DelphiSyntaxKind::Except,
            "false" => DelphiSyntaxKind::False_,
            "finally" => DelphiSyntaxKind::Finally,
            "for" => DelphiSyntaxKind::For,
            "function" => DelphiSyntaxKind::Function,
            "if" => DelphiSyntaxKind::If,
            "implementation" => DelphiSyntaxKind::Implementation,
            "in" => DelphiSyntaxKind::In_,
            "interface" => DelphiSyntaxKind::Interface,
            "is" => DelphiSyntaxKind::Is_,
            "mod" => DelphiSyntaxKind::Mod,
            "nil" => DelphiSyntaxKind::Nil,
            "not" => DelphiSyntaxKind::Not_,
            "object" => DelphiSyntaxKind::Object,
            "of" => DelphiSyntaxKind::Of,
            "or" => DelphiSyntaxKind::Or_,
            "procedure" => DelphiSyntaxKind::Procedure,
            "program" => DelphiSyntaxKind::Program,
            "record" => DelphiSyntaxKind::Record,
            "repeat" => DelphiSyntaxKind::Repeat,
            "set" => DelphiSyntaxKind::Set,
            "then" => DelphiSyntaxKind::Then,
            "to" => DelphiSyntaxKind::To,
            "true" => DelphiSyntaxKind::True_,
            "try" => DelphiSyntaxKind::Try,
            "type" => DelphiSyntaxKind::Type,
            "unit" => DelphiSyntaxKind::Unit,
            "until" => DelphiSyntaxKind::Until,
            "uses" => DelphiSyntaxKind::Uses,
            "var" => DelphiSyntaxKind::Var,
            "while" => DelphiSyntaxKind::While,
            "with" => DelphiSyntaxKind::With,
            _ => DelphiSyntaxKind::Identifier,
        };

        state.add_token(kind, start, state.get_position());
        true
    }

    fn lex_operators<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // Multi-character operators (longest first)
        let patterns: &[(&str, DelphiSyntaxKind)] = &[
            (":=", DelphiSyntaxKind::Assign),
            ("<=", DelphiSyntaxKind::LessEqual),
            (">=", DelphiSyntaxKind::GreaterEqual),
            ("<>", DelphiSyntaxKind::NotEqual),
            ("..", DelphiSyntaxKind::DotDot),
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
                '+' => Some(DelphiSyntaxKind::Plus),
                '-' => Some(DelphiSyntaxKind::Minus),
                '*' => Some(DelphiSyntaxKind::Star),
                '/' => Some(DelphiSyntaxKind::Slash),
                '=' => Some(DelphiSyntaxKind::Equal),
                '<' => Some(DelphiSyntaxKind::Less),
                '>' => Some(DelphiSyntaxKind::Greater),
                '.' => Some(DelphiSyntaxKind::Dot),
                ':' => Some(DelphiSyntaxKind::Colon),
                '^' => Some(DelphiSyntaxKind::Caret),
                '@' => Some(DelphiSyntaxKind::At),
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
                '(' => DelphiSyntaxKind::LeftParen,
                ')' => DelphiSyntaxKind::RightParen,
                '[' => DelphiSyntaxKind::LeftBracket,
                ']' => DelphiSyntaxKind::RightBracket,
                ',' => DelphiSyntaxKind::Comma,
                ';' => DelphiSyntaxKind::Semicolon,
                _ => return false,
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
