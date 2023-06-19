use crate::{kind::TypstSyntaxKind, language::TypstLanguage};
use oak_core::{
    IncrementalCache, Lexer, LexerState, OakError,
    lexer::{CommentLine, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<S: Source> = LexerState<S, TypstLanguage>;

static TYPST_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static TYPST_COMMENT: LazyLock<CommentLine> = LazyLock::new(|| CommentLine { line_markers: &["//"] });
static TYPST_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });

#[derive(Clone)]
pub struct TypstLexer<'config> {
    config: &'config TypstLanguage,
}

impl<'config> Lexer<TypstLanguage> for TypstLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<TypstLanguage>,
    ) -> LexOutput<TypstLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> TypstLexer<'config> {
    pub fn new(config: &'config TypstLanguage) -> Self {
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
        state.add_token(TypstSyntaxKind::Eof, eof_pos, eof_pos);
        Ok(())
    }

    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        match TYPST_WHITESPACE.scan(state.rest(), state.get_position(), TypstSyntaxKind::Whitespace) {
            Some(token) => {
                state.advance_with(token);
                return true;
            }
            None => {}
        }
        false
    }

    fn skip_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        // 行注释
        if let Some(token) = TYPST_COMMENT.scan(state.rest(), state.get_position(), TypstSyntaxKind::LineComment) {
            state.advance_with(token);
            return true;
        }

        // 块注释
        if state.rest().starts_with("/*") {
            let start = state.get_position();
            let mut pos = 2;
            let text = state.rest();

            while pos < text.len() {
                if text[pos..].starts_with("*/") {
                    pos += 2;
                    break;
                }
                pos += 1;
            }

            state.advance(pos);
            state.add_token(TypstSyntaxKind::BlockComment, start, state.get_position());
            return true;
        }

        false
    }

    fn lex_string_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        match TYPST_STRING.scan(state.rest(), state.get_position(), TypstSyntaxKind::StringLiteral) {
            Some(token) => {
                state.advance_with(token);
                return true;
            }
            None => {}
        }
        false
    }

    fn lex_number_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        let text = state.rest();
        if text.is_empty() || !text.chars().next().unwrap().is_ascii_digit() {
            return false;
        }

        let start = state.get_position();
        let mut pos = 0;
        let chars: Vec<char> = text.chars().collect();

        // 整数部分
        while pos < chars.len() && chars[pos].is_ascii_digit() {
            pos += 1;
        }

        // 小数部分
        if pos < chars.len() && chars[pos] == '.' && pos + 1 < chars.len() && chars[pos + 1].is_ascii_digit() {
            pos += 1; // 跳过 '.'
            while pos < chars.len() && chars[pos].is_ascii_digit() {
                pos += 1;
            }
        }

        // 指数部分
        if pos < chars.len() && (chars[pos] == 'e' || chars[pos] == 'E') {
            pos += 1;
            if pos < chars.len() && (chars[pos] == '+' || chars[pos] == '-') {
                pos += 1;
            }
            while pos < chars.len() && chars[pos].is_ascii_digit() {
                pos += 1;
            }
        }

        if pos > 0 {
            state.advance(pos);
            state.add_token(TypstSyntaxKind::NumericLiteral, start, state.get_position());
            return true;
        }

        false
    }

    fn lex_identifier_or_keyword<S: Source>(&self, state: &mut State<S>) -> bool {
        let text = state.rest();
        if text.is_empty() {
            return false;
        }

        let first_char = text.chars().next().unwrap();
        if !first_char.is_ascii_alphabetic() && first_char != '_' {
            return false;
        }

        let start = state.get_position();
        let mut pos = 0;
        let chars: Vec<char> = text.chars().collect();

        // 第一个字符
        pos += 1;

        // 后续字符
        while pos < chars.len() && (chars[pos].is_ascii_alphanumeric() || chars[pos] == '_') {
            pos += 1;
        }

        if pos > 0 {
            let identifier_text = &text[..pos];
            let kind = self.keyword_or_identifier(identifier_text);
            state.advance(pos);
            state.add_token(kind, start, state.get_position());
            return true;
        }

        false
    }

    fn keyword_or_identifier(&self, text: &str) -> TypstSyntaxKind {
        match text {
            "let" => TypstSyntaxKind::Let,
            "if" => TypstSyntaxKind::If,
            "else" => TypstSyntaxKind::Else,
            "for" => TypstSyntaxKind::For,
            "while" => TypstSyntaxKind::While,
            "break" => TypstSyntaxKind::Break,
            "continue" => TypstSyntaxKind::Continue,
            "return" => TypstSyntaxKind::Return,
            "true" => TypstSyntaxKind::True,
            "false" => TypstSyntaxKind::False,
            "set" => TypstSyntaxKind::Set,
            "show" => TypstSyntaxKind::Show,
            "import" => TypstSyntaxKind::Import,
            "include" => TypstSyntaxKind::Include,
            _ => TypstSyntaxKind::Identifier,
        }
    }

    fn lex_operators<S: Source>(&self, state: &mut State<S>) -> bool {
        let text = state.rest();
        if text.is_empty() {
            return false;
        }

        let start = state.get_position();
        let chars: Vec<char> = text.chars().collect();

        let (kind, len) = match chars[0] {
            '=' => {
                if chars.len() > 1 && chars[1] == '=' {
                    (TypstSyntaxKind::EqualEqual, 2)
                }
                else {
                    (TypstSyntaxKind::Equal, 1)
                }
            }
            '!' => {
                if chars.len() > 1 && chars[1] == '=' {
                    (TypstSyntaxKind::NotEqual, 2)
                }
                else {
                    (TypstSyntaxKind::Not, 1)
                }
            }
            '<' => {
                if chars.len() > 1 && chars[1] == '=' {
                    (TypstSyntaxKind::LessEqual, 2)
                }
                else {
                    (TypstSyntaxKind::Less, 1)
                }
            }
            '>' => {
                if chars.len() > 1 && chars[1] == '=' {
                    (TypstSyntaxKind::GreaterEqual, 2)
                }
                else {
                    (TypstSyntaxKind::Greater, 1)
                }
            }
            '&' => {
                if chars.len() > 1 && chars[1] == '&' {
                    (TypstSyntaxKind::And, 2)
                }
                else {
                    return false;
                }
            }
            '|' => {
                if chars.len() > 1 && chars[1] == '|' {
                    (TypstSyntaxKind::Or, 2)
                }
                else {
                    return false;
                }
            }
            '+' => (TypstSyntaxKind::Plus, 1),
            '-' => (TypstSyntaxKind::Minus, 1),
            '*' => (TypstSyntaxKind::Star, 1),
            '/' => (TypstSyntaxKind::Slash, 1),
            '%' => (TypstSyntaxKind::Percent, 1),
            _ => return false,
        };

        state.advance(len);
        state.add_token(kind, start, state.get_position());
        true
    }

    fn lex_single_char_tokens<S: Source>(&self, state: &mut State<S>) -> bool {
        let text = state.rest();
        if text.is_empty() {
            return false;
        }

        let start = state.get_position();
        let ch = text.chars().next().unwrap();

        let kind = match ch {
            '(' => TypstSyntaxKind::LeftParen,
            ')' => TypstSyntaxKind::RightParen,
            '{' => TypstSyntaxKind::LeftBrace,
            '}' => TypstSyntaxKind::RightBrace,
            '[' => TypstSyntaxKind::LeftBracket,
            ']' => TypstSyntaxKind::RightBracket,
            ';' => TypstSyntaxKind::Semicolon,
            ',' => TypstSyntaxKind::Comma,
            '.' => TypstSyntaxKind::Dot,
            ':' => TypstSyntaxKind::Colon,
            '#' => TypstSyntaxKind::Hash,
            '@' => TypstSyntaxKind::At,
            '$' => TypstSyntaxKind::Dollar,
            '_' => TypstSyntaxKind::Underscore,
            _ => TypstSyntaxKind::Error,
        };

        state.advance(1);
        state.add_token(kind, start, state.get_position());
        true
    }
}
