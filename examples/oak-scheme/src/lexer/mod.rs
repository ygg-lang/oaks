use crate::{kind::SchemeSyntaxKind, language::SchemeLanguage};
use oak_core::{
    IncrementalCache, Lexer, LexerState, OakError,
    lexer::{CommentLine, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<S: Source> = LexerState<S, SchemeLanguage>;

static SCHEME_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static SCHEME_COMMENT: LazyLock<CommentLine> = LazyLock::new(|| CommentLine { line_markers: &[";"] });
static SCHEME_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });

#[derive(Clone)]
pub struct SchemeLexer<'config> {
    config: &'config SchemeLanguage,
}

impl<'config> Lexer<SchemeLanguage> for SchemeLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<SchemeLanguage>,
    ) -> LexOutput<SchemeLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> SchemeLexer<'config> {
    pub fn new(config: &'config SchemeLanguage) -> Self {
        Self { config }
    }

    fn run<S: Source>(&self, state: &mut State<S>) -> Result<(), OakError> {
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

            if self.lex_number_literal(state) {
                continue;
            }

            if self.lex_identifier_or_keyword(state) {
                continue;
            }

            if self.lex_single_char_tokens(state) {
                continue;
            }

            // 错误处理：如果没有匹配任何规则，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(SchemeSyntaxKind::Error, start_pos, state.get_position());
            }

            state.safe_check(safe_point);
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(SchemeSyntaxKind::Eof, eof_pos, eof_pos);
        Ok(())
    }

    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        match SCHEME_WHITESPACE.scan(state.rest(), state.get_position(), SchemeSyntaxKind::Whitespace) {
            Some(token) => {
                state.advance_with(token);
                return true;
            }
            None => {}
        }
        false
    }

    /// 处理换行
    fn lex_newline<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(SchemeSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(SchemeSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn skip_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        match SCHEME_COMMENT.scan(state.rest(), state.get_position(), SchemeSyntaxKind::Comment) {
            Some(token) => {
                state.advance_with(token);
                return true;
            }
            None => {}
        }
        false
    }

    fn lex_string_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        match SCHEME_STRING.scan(state.rest(), state.get_position(), SchemeSyntaxKind::StringLiteral) {
            Some(token) => {
                state.advance_with(token);
                return true;
            }
            None => {}
        }
        false
    }

    fn lex_number_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let rest = state.rest();
        if rest.is_empty() {
            return false;
        }

        let first_char = rest.chars().next().unwrap();
        if !first_char.is_ascii_digit() && first_char != '-' && first_char != '+' {
            return false;
        }

        let start = state.get_position();
        let mut len = 0;

        // 处理符号
        if first_char == '-' || first_char == '+' {
            len += first_char.len_utf8();
        }

        // 跳过数字
        let mut has_digits = false;
        let mut chars = rest.chars().skip(if first_char == '-' || first_char == '+' { 1 } else { 0 });

        while let Some(ch) = chars.next() {
            if ch.is_ascii_digit() {
                len += ch.len_utf8();
                has_digits = true;
            }
            else if ch == '.' {
                // 浮点数
                len += ch.len_utf8();
                while let Some(ch) = chars.next() {
                    if ch.is_ascii_digit() {
                        len += ch.len_utf8();
                        has_digits = true;
                    }
                    else {
                        break;
                    }
                }
                break;
            }
            else {
                break;
            }
        }

        if has_digits {
            state.advance(len);
        }

        if !has_digits {
            // 重置位置，这不是一个数字
            return false;
        }

        let end = state.get_position();
        state.add_token(SchemeSyntaxKind::NumberLiteral, start, end);
        true
    }

    fn lex_identifier_or_keyword<S: Source>(&self, state: &mut State<S>) -> bool {
        let rest = state.rest();
        if rest.is_empty() {
            return false;
        }

        let first_char = rest.chars().next().unwrap();
        if !self.is_identifier_start(first_char) {
            return false;
        }

        let start = state.get_position();
        let mut len = first_char.len_utf8();
        let mut chars = rest.chars().skip(1);

        while let Some(ch) = chars.next() {
            if self.is_identifier_continue(ch) {
                len += ch.len_utf8();
            }
            else {
                break;
            }
        }

        let text = rest[..len].to_string();
        state.advance(len);
        let end = state.get_position();

        let kind = match text.as_str() {
            "define" | "lambda" | "if" | "cond" | "case" | "and" | "or" | "not" | "let" | "let*" | "letrec" | "begin"
            | "do" | "quote" | "quasiquote" | "unquote" | "unquote-splicing" | "set!" | "delay" | "force" | "#t" | "#f"
            | "null" | "car" | "cdr" | "cons" | "list" | "append" | "length" | "reverse" | "map" | "for-each" | "apply" => {
                SchemeSyntaxKind::Keyword
            }
            _ => SchemeSyntaxKind::Identifier,
        };

        state.add_token(kind, start, end);
        true
    }

    fn is_identifier_start(&self, ch: char) -> bool {
        ch.is_alphabetic() || "!$%&*+-./:<=>?@^_~".contains(ch)
    }

    fn is_identifier_continue(&self, ch: char) -> bool {
        self.is_identifier_start(ch) || ch.is_ascii_digit()
    }

    fn lex_single_char_tokens<S: Source>(&self, state: &mut State<S>) -> bool {
        let rest = state.rest();
        if rest.is_empty() {
            return false;
        }

        let ch = rest.chars().next().unwrap();
        let start = state.get_position();
        state.advance(ch.len_utf8());
        let end = state.get_position();

        let kind = match ch {
            '(' => SchemeSyntaxKind::LeftParen,
            ')' => SchemeSyntaxKind::RightParen,
            '[' => SchemeSyntaxKind::LeftBracket,
            ']' => SchemeSyntaxKind::RightBracket,
            '{' => SchemeSyntaxKind::LeftBrace,
            '}' => SchemeSyntaxKind::RightBrace,
            '\'' => SchemeSyntaxKind::Quote,
            '`' => SchemeSyntaxKind::Quasiquote,
            ',' => SchemeSyntaxKind::Unquote,
            '.' => SchemeSyntaxKind::Dot,
            '#' => SchemeSyntaxKind::Hash,
            _ => {
                return false;
            }
        };

        state.add_token(kind, start, end);
        true
    }
}
