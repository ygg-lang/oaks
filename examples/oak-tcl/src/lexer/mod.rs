use crate::{kind::TclSyntaxKind, language::TclLanguage};
use oak_core::{
    Lexer, LexerState, OakError,
    lexer::{CommentConfig, LexOutput, LexerCache, StringConfig, WhitespaceConfig},
    source::Source,
};

type State<'s, S> = LexerState<'s, S, TclLanguage>;

static TCL_WHITESPACE: WhitespaceConfig = WhitespaceConfig { unicode_whitespace: true };
static TCL_COMMENT: CommentConfig = CommentConfig { line_marker: "#", block_start: "", block_end: "", nested_blocks: false };
static TCL_STRING: StringConfig = StringConfig { quotes: &['"'], escape: Some('\\') };

#[derive(Clone)]
pub struct TclLexer<'config> {
    _config: &'config TclLanguage,
}

impl<'config> TclLexer<'config> {
    pub fn new(config: &'config TclLanguage) -> Self {
        Self { _config: config }
    }

    fn run<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> Result<(), OakError> {
        while state.not_at_end() {
            let safe_point = state.get_position();

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

            state.advance_if_dead_lock(safe_point);
        }

        state.add_eof();
        Ok(())
    }
}

impl<'config> Lexer<TclLanguage> for TclLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[oak_core::TextEdit], cache: &'a mut impl LexerCache<TclLanguage>) -> LexOutput<TclLanguage> {
        let mut state = State::new(source);
        let result = self.run(&mut state);
        state.finish_with_cache(result, cache)
    }
}

impl<'config> TclLexer<'config> {
    fn skip_whitespace<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        TCL_WHITESPACE.scan(state, TclSyntaxKind::Whitespace)
    }

    fn lex_newline<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        if let Some(ch) = state.current() {
            if ch == '\n' {
                let start = state.get_position();
                state.advance(1);
                state.add_token(TclSyntaxKind::Newline, start, state.get_position());
                return true;
            }
            else if ch == '\r' {
                let start = state.get_position();
                state.advance(1);
                if state.current() == Some('\n') {
                    state.advance(1);
                }
                state.add_token(TclSyntaxKind::Newline, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn skip_comment<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        TCL_COMMENT.scan(state, TclSyntaxKind::Comment, TclSyntaxKind::Comment)
    }

    fn lex_string_literal<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        TCL_STRING.scan(state, TclSyntaxKind::StringLiteral)
    }

    fn lex_brace_string<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
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

    fn lex_numeric_literal<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
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

    fn lex_identifier_or_keyword<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
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
        let text = state.source().get_text_in(oak_core::Range { start, end });
        let kind = match text.as_ref() {
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

    fn lex_operators<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
        let start = state.get_position();

        // 多字符操作符
        let patterns: &[(&str, TclSyntaxKind)] =
            &[("==", TclSyntaxKind::Equal), ("!=", TclSyntaxKind::NotEqual), ("<=", TclSyntaxKind::LessEqual), (">=", TclSyntaxKind::GreaterEqual), ("&&", TclSyntaxKind::AmpersandAmpersand), ("||", TclSyntaxKind::PipePipe)];

        for (pat, kind) in patterns {
            let mut matches = true;
            for (i, c) in pat.chars().enumerate() {
                if state.peek_next_n(i) != Some(c) {
                    matches = false;
                    break;
                }
            }

            if matches {
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

    fn lex_single_char_tokens<'s, S: Source + ?Sized>(&self, state: &mut State<'s, S>) -> bool {
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
