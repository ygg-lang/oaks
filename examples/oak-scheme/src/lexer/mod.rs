use crate::{kind::SchemeSyntaxKind, language::SchemeLanguage};
use oak_core::{
    Lexer, LexerCache, LexerState, OakError,
    lexer::{CommentConfig, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<'a, S> = LexerState<'a, S, SchemeLanguage>;

static SCHEME_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static SCHEME_COMMENT: LazyLock<CommentConfig> = LazyLock::new(|| CommentConfig { line_marker: ";", block_start: "#|", block_end: "|#", nested_blocks: true });
static SCHEME_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });

#[derive(Clone)]
pub struct SchemeLexer<'config> {
    _config: &'config SchemeLanguage,
}

impl<'config> Lexer<SchemeLanguage> for SchemeLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &S, _edits: &[oak_core::TextEdit], cache: &'a mut impl LexerCache<SchemeLanguage>) -> LexOutput<SchemeLanguage> {
        let mut state: State<'_, S> = LexerState::new(source);
        let result = self.run(&mut state);
        state.finish_with_cache(result, cache)
    }
}

impl<'config> SchemeLexer<'config> {
    pub fn new(config: &'config SchemeLanguage) -> Self {
        Self { _config: config }
    }

    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), OakError> {
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

            state.advance_if_dead_lock(safe_point);
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(SchemeSyntaxKind::Eof, eof_pos, eof_pos);
        Ok(())
    }

    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        SCHEME_WHITESPACE.scan(state, SchemeSyntaxKind::Whitespace)
    }

    /// 处理换行
    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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

    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        SCHEME_COMMENT.scan(state, SchemeSyntaxKind::LineComment, SchemeSyntaxKind::Comment)
    }

    fn lex_string_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        SCHEME_STRING.scan(state, SchemeSyntaxKind::StringLiteral)
    }

    fn lex_number_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let mut len = 0;
        let mut has_digits = false;

        {
            let rest = state.rest();
            if rest.is_empty() {
                return false;
            }

            let first_char = rest.chars().next().unwrap();
            if !first_char.is_ascii_digit() && first_char != '-' && first_char != '+' {
                return false;
            }

            // 处理符号
            if first_char == '-' || first_char == '+' {
                len += first_char.len_utf8();
            }

            // 跳过数字
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
        }

        if has_digits {
            state.advance(len);
            let end = state.get_position();
            state.add_token(SchemeSyntaxKind::NumberLiteral, start, end);
            true
        }
        else {
            false
        }
    }

    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let mut len;

        {
            let rest = state.rest();
            if rest.is_empty() {
                return false;
            }

            let first_char = rest.chars().next().unwrap();
            if !self.is_identifier_start(first_char) {
                return false;
            }

            len = first_char.len_utf8();
            let mut chars = rest.chars().skip(1);

            while let Some(ch) = chars.next() {
                if self.is_identifier_continue(ch) {
                    len += ch.len_utf8();
                }
                else {
                    break;
                }
            }
        }

        let text = state.get_text_in(oak_core::Range { start, end: start + len }).to_string();
        state.advance(len);
        let end = state.get_position();

        let kind = match text.as_str() {
            "define" | "lambda" | "if" | "cond" | "case" | "and" | "or" | "not" | "let" | "let*" | "letrec" | "begin" | "do" | "quote" | "quasiquote" | "unquote" | "unquote-splicing" | "set!" | "delay" | "force" | "#t" | "#f" | "null" | "car" | "cdr"
            | "cons" | "list" | "append" | "length" | "reverse" | "map" | "for-each" | "apply" => SchemeSyntaxKind::Keyword,
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

    fn lex_single_char_tokens<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let ch = match state.peek() {
            Some(ch) => ch,
            None => return false,
        };

        let kind = match ch {
            '(' => Some(SchemeSyntaxKind::LeftParen),
            ')' => Some(SchemeSyntaxKind::RightParen),
            '[' => Some(SchemeSyntaxKind::LeftBracket),
            ']' => Some(SchemeSyntaxKind::RightBracket),
            '{' => Some(SchemeSyntaxKind::LeftBrace),
            '}' => Some(SchemeSyntaxKind::RightBrace),
            '\'' => Some(SchemeSyntaxKind::Quote),
            '`' => Some(SchemeSyntaxKind::Quasiquote),
            ',' => Some(SchemeSyntaxKind::Unquote),
            '.' => Some(SchemeSyntaxKind::Dot),
            '#' => Some(SchemeSyntaxKind::Hash),
            _ => None,
        };

        if let Some(kind) = kind {
            state.advance(ch.len_utf8());
            state.add_token(kind, start, state.get_position());
            true
        }
        else {
            false
        }
    }
}
