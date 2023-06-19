use crate::{kind::ScalaSyntaxKind, language::ScalaLanguage};
use oak_core::{
    IncrementalCache, Lexer, LexerState, OakError,
    lexer::{CommentLine, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<S: Source> = LexerState<S, ScalaLanguage>;

static SCALA_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static SCALA_COMMENT: LazyLock<CommentLine> = LazyLock::new(|| CommentLine { line_markers: &["//"] });
static SCALA_STRING: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['"'], escape: Some('\\') });
static SCALA_CHAR: LazyLock<StringConfig> = LazyLock::new(|| StringConfig { quotes: &['\''], escape: None });

#[derive(Clone)]
pub struct ScalaLexer<'config> {
    config: &'config ScalaLanguage,
}

impl<'config> Lexer<ScalaLanguage> for ScalaLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<ScalaLanguage>,
    ) -> LexOutput<ScalaLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> ScalaLexer<'config> {
    pub fn new(config: &'config ScalaLanguage) -> Self {
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

            if self.lex_char_literal(state) {
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
                state.add_token(ScalaSyntaxKind::Error, start_pos, state.get_position());
            }

            state.safe_check(safe_point);
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(ScalaSyntaxKind::Eof, eof_pos, eof_pos);
        Ok(())
    }

    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        match SCALA_WHITESPACE.scan(state.rest(), state.get_position(), ScalaSyntaxKind::Whitespace) {
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
            state.add_token(ScalaSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(ScalaSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    fn skip_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        // 行注释
        match SCALA_COMMENT.scan(state.rest(), state.get_position(), ScalaSyntaxKind::LineComment) {
            Some(token) => {
                state.advance_with(token);
                return true;
            }
            None => {}
        }

        // 块注释
        if state.rest().starts_with("/*") {
            let start = state.get_position();
            state.advance(2);

            while state.not_at_end() && !state.rest().starts_with("*/") {
                state.advance(1);
            }

            if state.rest().starts_with("*/") {
                state.advance(2);
            }

            let end = state.get_position();
            state.add_token(ScalaSyntaxKind::BlockComment, start, end);
            return true;
        }

        false
    }

    fn lex_string_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        match SCALA_STRING.scan(state.rest(), state.get_position(), ScalaSyntaxKind::StringLiteral) {
            Some(token) => {
                state.advance_with(token);
                return true;
            }
            None => {}
        }
        false
    }

    fn lex_char_literal<S: Source>(&self, state: &mut State<S>) -> bool {
        match SCALA_CHAR.scan(state.rest(), state.get_position(), ScalaSyntaxKind::CharLiteral) {
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
        if rest.is_empty() || !rest.chars().next().unwrap().is_ascii_digit() {
            return false;
        }

        let start = state.get_position();
        let mut len = 0;

        // 跳过数字
        for ch in rest.chars() {
            if ch.is_ascii_digit() {
                len += ch.len_utf8();
            }
            else if ch == '.' {
                // 浮点数
                len += ch.len_utf8();
                for ch in rest[len..].chars() {
                    if ch.is_ascii_digit() {
                        len += ch.len_utf8();
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

        state.advance(len);
        let end = state.get_position();
        state.add_token(ScalaSyntaxKind::IntegerLiteral, start, end);
        true
    }

    fn lex_identifier_or_keyword<S: Source>(&self, state: &mut State<S>) -> bool {
        let rest = state.rest();
        if rest.is_empty() {
            return false;
        }

        let first_char = rest.chars().next().unwrap();
        if !first_char.is_alphabetic() && first_char != '_' {
            return false;
        }

        let start = state.get_position();
        let mut len = 0;

        for ch in rest.chars() {
            if ch.is_alphanumeric() || ch == '_' {
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
            "abstract" => ScalaSyntaxKind::Abstract,
            "case" => ScalaSyntaxKind::Case,
            "catch" => ScalaSyntaxKind::Catch,
            "class" => ScalaSyntaxKind::Class,
            "def" => ScalaSyntaxKind::Def,
            "do" => ScalaSyntaxKind::Do,
            "else" => ScalaSyntaxKind::Else,
            "extends" => ScalaSyntaxKind::Extends,
            "false" => ScalaSyntaxKind::False,
            "final" => ScalaSyntaxKind::Final,
            "finally" => ScalaSyntaxKind::Finally,
            "for" => ScalaSyntaxKind::For,
            "if" => ScalaSyntaxKind::If,
            "implicit" => ScalaSyntaxKind::Implicit,
            "import" => ScalaSyntaxKind::Import,
            "lazy" => ScalaSyntaxKind::Lazy,
            "match" => ScalaSyntaxKind::Match,
            "new" => ScalaSyntaxKind::New,
            "null" => ScalaSyntaxKind::Null,
            "object" => ScalaSyntaxKind::Object,
            "override" => ScalaSyntaxKind::Override,
            "package" => ScalaSyntaxKind::Package,
            "private" => ScalaSyntaxKind::Private,
            "protected" => ScalaSyntaxKind::Protected,
            "return" => ScalaSyntaxKind::Return,
            "sealed" => ScalaSyntaxKind::Sealed,
            "super" => ScalaSyntaxKind::Super,
            "this" => ScalaSyntaxKind::This,
            "throw" => ScalaSyntaxKind::Throw,
            "trait" => ScalaSyntaxKind::Trait,
            "true" => ScalaSyntaxKind::True,
            "try" => ScalaSyntaxKind::Try,
            "type" => ScalaSyntaxKind::Type,
            "val" => ScalaSyntaxKind::Val,
            "var" => ScalaSyntaxKind::Var,
            "while" => ScalaSyntaxKind::While,
            "with" => ScalaSyntaxKind::With,
            "yield" => ScalaSyntaxKind::Yield,
            _ => ScalaSyntaxKind::Identifier,
        };

        state.add_token(kind, start, end);
        true
    }

    fn lex_operators<S: Source>(&self, state: &mut State<S>) -> bool {
        let rest = state.rest();
        let start = state.get_position();

        // 多字符操作符
        if rest.starts_with("=>") {
            state.advance(2);
            state.add_token(ScalaSyntaxKind::Arrow, start, state.get_position());
            return true;
        }
        if rest.starts_with("<=") {
            state.advance(2);
            state.add_token(ScalaSyntaxKind::LessEqual, start, state.get_position());
            return true;
        }
        if rest.starts_with(">=") {
            state.advance(2);
            state.add_token(ScalaSyntaxKind::GreaterEqual, start, state.get_position());
            return true;
        }
        if rest.starts_with("==") {
            state.advance(2);
            state.add_token(ScalaSyntaxKind::EqualEqual, start, state.get_position());
            return true;
        }
        if rest.starts_with("!=") {
            state.advance(2);
            state.add_token(ScalaSyntaxKind::NotEqual, start, state.get_position());
            return true;
        }

        false
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
            '(' => ScalaSyntaxKind::LeftParen,
            ')' => ScalaSyntaxKind::RightParen,
            '[' => ScalaSyntaxKind::LeftBracket,
            ']' => ScalaSyntaxKind::RightBracket,
            '{' => ScalaSyntaxKind::LeftBrace,
            '}' => ScalaSyntaxKind::RightBrace,
            ',' => ScalaSyntaxKind::Comma,
            ';' => ScalaSyntaxKind::Semicolon,
            ':' => ScalaSyntaxKind::Colon,
            '.' => ScalaSyntaxKind::Dot,
            '+' => ScalaSyntaxKind::Plus,
            '-' => ScalaSyntaxKind::Minus,
            '*' => ScalaSyntaxKind::Star,
            '/' => ScalaSyntaxKind::Slash,
            '%' => ScalaSyntaxKind::Percent,
            '=' => ScalaSyntaxKind::Eq,
            '<' => ScalaSyntaxKind::Lt,
            '>' => ScalaSyntaxKind::Gt,
            '!' => ScalaSyntaxKind::Not,
            '&' => ScalaSyntaxKind::And,
            '|' => ScalaSyntaxKind::Or,
            '^' => ScalaSyntaxKind::Xor,
            '~' => ScalaSyntaxKind::Tilde,
            '?' => ScalaSyntaxKind::Question,
            '@' => ScalaSyntaxKind::At,
            '#' => ScalaSyntaxKind::Hash,
            _ => {
                return false;
            }
        };

        state.add_token(kind, start, end);
        true
    }
}
