use crate::{kind::ScssSyntaxKind, language::ScssLanguage};
use oak_core::{
    Lexer, LexerState, OakError, TextEdit,
    lexer::{CommentConfig, LexOutput, LexerCache, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<'a, S> = LexerState<'a, S, ScssLanguage>;

static SCSS_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static SCSS_COMMENT: LazyLock<CommentConfig> = LazyLock::new(|| CommentConfig { line_marker: "//", block_start: "/*", block_end: "*/", nested_blocks: true });
static SCSS_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });

#[derive(Debug, Clone)]
pub struct ScssLexer<'config> {
    _config: &'config ScssLanguage,
}

impl<'config> Lexer<ScssLanguage> for ScssLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, text: &S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<ScssLanguage>) -> LexOutput<ScssLanguage> {
        let mut state = LexerState::new(text);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> ScssLexer<'config> {
    pub fn new(config: &'config ScssLanguage) -> Self {
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

            if self.lex_operators(state) {
                continue;
            }

            if self.lex_single_char_tokens(state) {
                continue;
            }

            // 错误处理：如果没有匹配任何规则，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(ScssSyntaxKind::Error, start_pos, state.get_position());
            }

            state.advance_if_dead_lock(safe_point);
        }

        Ok(())
    }

    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        SCSS_WHITESPACE.scan(state, ScssSyntaxKind::Whitespace)
    }

    /// 处理换行
    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(ScssSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(ScssSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn skip_comment<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        SCSS_COMMENT.scan(state, ScssSyntaxKind::Comment, ScssSyntaxKind::Comment)
    }

    fn lex_string_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        SCSS_STRING.scan(state, ScssSyntaxKind::StringLiteral)
    }

    fn lex_number_literal<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();

        if let Some(first_char) = state.peek() {
            if first_char.is_ascii_digit() {
                state.advance(first_char.len_utf8());

                // Continue with digits
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // Handle decimal point
                if state.peek() == Some('.') && state.peek_next_n(1).map_or(false, |c| c.is_ascii_digit()) {
                    state.advance(1); // consume '.'
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(ch.len_utf8());
                        }
                        else {
                            break;
                        }
                    }
                }

                state.add_token(ScssSyntaxKind::IntegerLiteral, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_identifier_or_keyword<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let text = state.source().get_text_from(start);

        if let Some(first_char) = text.chars().next() {
            if first_char.is_alphabetic() || first_char == '_' || first_char == '$' {
                let mut len = first_char.len_utf8();

                let mut chars = text.chars().skip(1);
                while let Some(ch) = chars.next() {
                    if ch.is_alphanumeric() || ch == '_' || ch == '-' {
                        len += ch.len_utf8();
                    }
                    else {
                        break;
                    }
                }

                let word = &text[..len];
                let kind = self.keyword_kind(word).unwrap_or(ScssSyntaxKind::Identifier);
                state.advance(len);
                state.add_token(kind, start, state.get_position());
                return true;
            }
        }
        false
    }

    fn lex_operators<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let text = state.source().get_text_from(start);

        // Two-character operators
        if text.len() >= 2 {
            let two_char = &text[..2];
            if let Some(kind) = self.operator_kind(two_char) {
                state.advance(2);
                state.add_token(kind, start, state.get_position());
                return true;
            }
        }

        // Single-character operators
        if let Some(first_char) = text.chars().next() {
            let single_char = &text[..first_char.len_utf8()];
            if let Some(kind) = self.operator_kind(single_char) {
                state.advance(first_char.len_utf8());
                state.add_token(kind, start, state.get_position());
                return true;
            }
        }

        false
    }

    fn lex_single_char_tokens<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start = state.get_position();
        let text = state.source().get_text_from(start);

        if let Some(first_char) = text.chars().next() {
            let single_char = &text[..first_char.len_utf8()];
            if let Some(kind) = self.single_char_kind(single_char) {
                state.advance(first_char.len_utf8());
                state.add_token(kind, start, state.get_position());
                return true;
            }
        }

        false
    }

    fn keyword_kind(&self, text: &str) -> Option<ScssSyntaxKind> {
        match text {
            "import" => Some(ScssSyntaxKind::Import),
            "include" => Some(ScssSyntaxKind::Include),
            "mixin" => Some(ScssSyntaxKind::Mixin),
            "function" => Some(ScssSyntaxKind::Function),
            "return" => Some(ScssSyntaxKind::Return),
            "if" => Some(ScssSyntaxKind::If),
            "else" => Some(ScssSyntaxKind::Else),
            "for" => Some(ScssSyntaxKind::For),
            "while" => Some(ScssSyntaxKind::While),
            "each" => Some(ScssSyntaxKind::Each),
            "in" => Some(ScssSyntaxKind::In),
            "true" => Some(ScssSyntaxKind::True),
            "false" => Some(ScssSyntaxKind::False),
            "null" => Some(ScssSyntaxKind::Null),
            _ => None,
        }
    }

    fn operator_kind(&self, text: &str) -> Option<ScssSyntaxKind> {
        match text {
            "==" => Some(ScssSyntaxKind::EqEq),
            "!=" => Some(ScssSyntaxKind::Ne),
            "<=" => Some(ScssSyntaxKind::Le),
            ">=" => Some(ScssSyntaxKind::Ge),
            "&&" => Some(ScssSyntaxKind::AndAnd),
            "||" => Some(ScssSyntaxKind::OrOr),
            "=" => Some(ScssSyntaxKind::Eq),
            "<" => Some(ScssSyntaxKind::Lt),
            ">" => Some(ScssSyntaxKind::Gt),
            "&" => Some(ScssSyntaxKind::And),
            "|" => Some(ScssSyntaxKind::Or),
            "^" => Some(ScssSyntaxKind::Xor),
            "+" => Some(ScssSyntaxKind::Plus),
            "-" => Some(ScssSyntaxKind::Minus),
            "*" => Some(ScssSyntaxKind::Star),
            "/" => Some(ScssSyntaxKind::Slash),
            "%" => Some(ScssSyntaxKind::Percent),
            "!" => Some(ScssSyntaxKind::Bang),
            _ => None,
        }
    }

    fn single_char_kind(&self, text: &str) -> Option<ScssSyntaxKind> {
        match text {
            "(" => Some(ScssSyntaxKind::LeftParen),
            ")" => Some(ScssSyntaxKind::RightParen),
            "{" => Some(ScssSyntaxKind::LeftBrace),
            "}" => Some(ScssSyntaxKind::RightBrace),
            "[" => Some(ScssSyntaxKind::LeftBracket),
            "]" => Some(ScssSyntaxKind::RightBracket),
            ";" => Some(ScssSyntaxKind::Semicolon),
            ":" => Some(ScssSyntaxKind::Colon),
            "," => Some(ScssSyntaxKind::Comma),
            "." => Some(ScssSyntaxKind::Dot),
            "#" => Some(ScssSyntaxKind::Hash),
            "@" => Some(ScssSyntaxKind::At),
            "$" => Some(ScssSyntaxKind::Dollar),
            _ => None,
        }
    }
}
