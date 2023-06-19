use crate::{kind::TclSyntaxKind, language::TclLanguage};
use oak_core::{
    IncrementalCache, Lexer, LexerState, OakError,
    lexer::{CommentLine, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<S: Source> = LexerState<S, TclLanguage>;

static TCL_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static TCL_COMMENT: LazyLock<CommentLine> = LazyLock::new(|| CommentLine { line_markers: &["#"] });
static TCL_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });

#[derive(Clone)]
pub struct TclLexer<'config> {
    config: &'config TclLanguage,
}

impl<'config> Lexer<TclLanguage> for TclLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<TclLanguage>,
    ) -> LexOutput<TclLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> TclLexer<'config> {
    pub fn new(config: &'config TclLanguage) -> Self {
        Self { config }
    }

    fn run<S: Source>(&self, state: &mut State<S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let _safe_point = state.get_position();

            if self.skip_whitespace(state) {
                continue;
            }

            if self.lex_newline(state) {
                continue;
            }

            if self.skip_comment(state) {
                continue;
            }

            if self.lex_string_literal(state) {
                continue;
            }

            if self.lex_brace_string(state) {
                continue;
            }

            if self.lex_numeric_literal(state) {
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

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            if let Some(ch) = state.current() {
                state.advance(ch.len_utf8());
            }
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(TclSyntaxKind::Eof, eof_pos, eof_pos);
        Ok(())
    }

    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        match TCL_WHITESPACE.scan(state.rest(), state.get_position(), TclSyntaxKind::Whitespace) {
            Some(token) => {
                state.advance_with(token);
                true
            }
            None => false,
        }
    }

    fn lex_newline<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if let Some(ch) = state.current() {
            if ch == '\n' {
                state.advance(1);
                state.add_token(TclSyntaxKind::Newline, start, state.get_position());
                return true;
            }
            else if ch == '\r' {
                state.advance(1);
                if state.peek() == Some('\n') {
                    state.advance(1);
                }
                state.add_token(TclSyntaxKind::Newline, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn skip_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        match TCL_COMMENT.scan(state.rest(), state.get_position(), TclSyntaxKind::Comment) {
            Some(token) => {
                state.advance_with(token);
                true
            }
            None => false,
        }
    }

    fn lex_string_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        if state.current() != Some('"') {
            return false;
        }

        let start = state.get_position();
        state.advance(1); // consume opening quote

        let mut escaped = false;
        while let Some(ch) = state.peek() {
            if escaped {
                escaped = false;
                state.advance(ch.len_utf8());
                continue;
            }

            if ch == '\\' {
                escaped = true;
                state.advance(1);
                continue;
            }

            if ch == '"' {
                state.advance(1); // consume closing quote
                break;
            }

            state.advance(ch.len_utf8());
        }

        state.add_token(TclSyntaxKind::StringLiteral, start, state.get_position());
        true
    }

    fn lex_brace_string<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        if state.current() != Some('{') {
            return false;
        }

        state.advance(1);
        let mut brace_count = 1;

        while let Some(ch) = state.peek() {
            if ch == '{' {
                brace_count += 1;
            }
            else if ch == '}' {
                brace_count -= 1;
                if brace_count == 0 {
                    state.advance(1);
                    break;
                }
            }
            state.advance(ch.len_utf8());
        }

        state.add_token(TclSyntaxKind::StringLiteral, start, state.get_position());
        true
    }

    fn lex_numeric_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let first = match state.current() {
            Some(c) => c,
            None => return false,
        };

        if !first.is_ascii_digit() && !(first == '-' && state.peek().map_or(false, |c| c.is_ascii_digit())) {
            return false;
        }

        if first == '-' {
            state.advance(1);
        }

        // 整数部分
        while let Some(c) = state.current() {
            if c.is_ascii_digit() {
                state.advance(1);
            }
            else {
                break;
            }
        }

        // 小数部分
        if state.current() == Some('.') && state.peek().map_or(false, |c| c.is_ascii_digit()) {
            state.advance(1); // consume '.'
            while let Some(c) = state.current() {
                if c.is_ascii_digit() {
                    state.advance(1);
                }
                else {
                    break;
                }
            }
        }

        // 科学计数法
        if let Some(c) = state.current() {
            if c == 'e' || c == 'E' {
                let next = state.peek();
                if next == Some('+') || next == Some('-') || next.map_or(false, |d| d.is_ascii_digit()) {
                    state.advance(1);
                    if let Some(sign) = state.current() {
                        if sign == '+' || sign == '-' {
                            state.advance(1);
                        }
                    }
                    while let Some(d) = state.current() {
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

        state.add_token(TclSyntaxKind::Number, start, state.get_position());
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

        state.advance(ch.len_utf8());
        while let Some(c) = state.current() {
            if c.is_ascii_alphanumeric() || c == '_' {
                state.advance(c.len_utf8());
            }
            else {
                break;
            }
        }

        let end = state.get_position();
        let text = state.get_text_in((start..end).into());
        let kind = match text {
            "if" => TclSyntaxKind::If,
            "else" => TclSyntaxKind::Else,
            "elseif" => TclSyntaxKind::ElseIf,
            "for" => TclSyntaxKind::For,
            "while" => TclSyntaxKind::While,
            "foreach" => TclSyntaxKind::ForEach,
            "proc" => TclSyntaxKind::Proc,
            "return" => TclSyntaxKind::Return,
            "break" => TclSyntaxKind::Break,
            "continue" => TclSyntaxKind::Continue,
            "set" => TclSyntaxKind::Set,
            "unset" => TclSyntaxKind::Unset,
            "global" => TclSyntaxKind::Global,
            "upvar" => TclSyntaxKind::Upvar,
            "variable" => TclSyntaxKind::Variable,
            _ => TclSyntaxKind::Identifier,
        };

        state.add_token(kind, start, state.get_position());
        true
    }

    fn lex_operators<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();
        let rest = state.rest();

        // 多字符操作符
        let patterns: &[(&str, TclSyntaxKind)] = &[
            ("==", TclSyntaxKind::Equal),
            ("!=", TclSyntaxKind::NotEqual),
            ("<=", TclSyntaxKind::LessEqual),
            (">=", TclSyntaxKind::GreaterEqual),
            ("&&", TclSyntaxKind::AmpersandAmpersand),
            ("||", TclSyntaxKind::PipePipe),
        ];

        for (pat, kind) in patterns {
            if rest.starts_with(pat) {
                state.advance(pat.len());
                state.add_token(*kind, start, state.get_position());
                return true;
            }
        }

        // 单字符操作符
        if let Some(ch) = state.current() {
            let kind = match ch {
                '+' => Some(TclSyntaxKind::Plus),
                '-' => Some(TclSyntaxKind::Minus),
                '*' => Some(TclSyntaxKind::Star),
                '/' => Some(TclSyntaxKind::Slash),
                '%' => Some(TclSyntaxKind::Percent),
                '<' => Some(TclSyntaxKind::Less),
                '>' => Some(TclSyntaxKind::Greater),
                '!' => Some(TclSyntaxKind::Exclamation),
                '&' => Some(TclSyntaxKind::Ampersand),
                '|' => Some(TclSyntaxKind::Pipe),
                '=' => Some(TclSyntaxKind::Equal),
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
                '(' => TclSyntaxKind::LeftParen,
                ')' => TclSyntaxKind::RightParen,
                '[' => TclSyntaxKind::LeftBracket,
                ']' => TclSyntaxKind::RightBracket,
                '{' => TclSyntaxKind::LeftBrace,
                '}' => TclSyntaxKind::RightBrace,
                ';' => TclSyntaxKind::Semicolon,
                ',' => TclSyntaxKind::Comma,
                '$' => TclSyntaxKind::Dollar,
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
