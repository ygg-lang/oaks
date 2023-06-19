use crate::{TomlSyntaxKind, language::TomlLanguage};
use oak_core::{
    IncrementalCache, Lexer, LexerState, OakError,
    lexer::{CommentLine, LexOutput, StringConfig, WhitespaceConfig},
    source::Source,
};
use std::sync::LazyLock;

type State<S: Source> = LexerState<S, TomlLanguage>;

static TOML_WHITESPACE: LazyLock<WhitespaceConfig> = LazyLock::new(|| WhitespaceConfig { unicode_whitespace: true });
static TOML_COMMENT: LazyLock<CommentLine> = LazyLock::new(|| CommentLine { line_markers: &["#"] });

#[derive(Clone)]
pub struct TomlLexer<'config> {
    config: &'config TomlLanguage,
}

impl<'config> Lexer<TomlLanguage> for TomlLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<TomlLanguage>,
    ) -> LexOutput<TomlLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        let result = self.run(&mut state);
        state.finish(result)
    }
}

impl<'config> TomlLexer<'config> {
    pub fn new(config: &'config TomlLanguage) -> Self {
        Self { config }
    }

    /// 主要的词法分析循环
    fn run<S: Source>(&self, state: &mut State<S>) -> Result<(), OakError> {
        while state.not_at_end() {
            if self.skip_whitespace(state) {
                continue;
            }
            if self.skip_comment(state) {
                continue;
            }
            if self.lex_string(state) {
                continue;
            }
            if self.lex_number(state) {
                continue;
            }
            if self.lex_punctuation(state) {
                continue;
            }
            if self.lex_identifier(state) {
                continue;
            }

            // 如果没有匹配任何模式，跳过当前字符
            state.advance(1);
        }
        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.current() {
            if ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
                state.advance(1);
            }
            else {
                break;
            }
        }

        if state.get_position() > start_pos {
            state.add_token(TomlSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 跳过注释
    fn skip_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        if state.current() == Some('#') {
            let start_pos = state.get_position();
            state.advance(1);

            // 读取到行尾
            while let Some(ch) = state.current() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(1);
            }

            state.add_token(TomlSyntaxKind::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 解析字符串
    fn lex_string<S: Source>(&self, state: &mut State<S>) -> bool {
        match state.current() {
            Some('"') => {
                let start = state.get_position();
                state.advance(1);

                // 简单的字符串解析
                while let Some(ch) = state.current() {
                    if ch == '"' {
                        state.advance(1);
                        break;
                    }
                    if ch == '\\' {
                        state.advance(1); // 跳过转义字符
                        if state.current().is_some() {
                            state.advance(1);
                        }
                    }
                    else {
                        state.advance(1);
                    }
                }

                let end = state.get_position();
                state.add_token(TomlSyntaxKind::BasicString, start, end);
                true
            }
            Some('\'') => {
                let start = state.get_position();
                state.advance(1);

                // 字面字符串解析
                while let Some(ch) = state.current() {
                    if ch == '\'' {
                        state.advance(1);
                        break;
                    }
                    state.advance(1);
                }

                let end = state.get_position();
                state.add_token(TomlSyntaxKind::LiteralString, start, end);
                true
            }
            _ => false,
        }
    }

    /// 解析数字
    fn lex_number<S: Source>(&self, state: &mut State<S>) -> bool {
        if !state.current().map_or(false, |c| c.is_ascii_digit() || c == '-' || c == '+') {
            return false;
        }

        let start = state.get_position();

        // 跳过符号
        if matches!(state.current(), Some('-') | Some('+')) {
            state.advance(1);
        }

        // 解析数字
        while state.current().map_or(false, |c| c.is_ascii_digit()) {
            state.advance(1);
        }

        // 检查是否是浮点数
        let mut is_float = false;
        if state.current() == Some('.') {
            is_float = true;
            state.advance(1);
            while state.current().map_or(false, |c| c.is_ascii_digit()) {
                state.advance(1);
            }
        }

        let end = state.get_position();
        let kind = if is_float { TomlSyntaxKind::Float } else { TomlSyntaxKind::Integer };
        state.add_token(kind, start, end);
        true
    }

    /// 解析标点符号
    fn lex_punctuation<S: Source>(&self, state: &mut State<S>) -> bool {
        let start = state.get_position();

        match state.current() {
            Some('[') => {
                state.advance(1);
                if state.current() == Some('[') {
                    state.advance(1);
                    let end = state.get_position();
                    state.add_token(TomlSyntaxKind::DoubleLeftBracket, start, end);
                }
                else {
                    let end = state.get_position();
                    state.add_token(TomlSyntaxKind::LeftBracket, start, end);
                }
                true
            }
            Some(']') => {
                state.advance(1);
                if state.current() == Some(']') {
                    state.advance(1);
                    let end = state.get_position();
                    state.add_token(TomlSyntaxKind::DoubleRightBracket, start, end);
                }
                else {
                    let end = state.get_position();
                    state.add_token(TomlSyntaxKind::RightBracket, start, end);
                }
                true
            }
            Some('{') => {
                state.advance(1);
                let end = state.get_position();
                state.add_token(TomlSyntaxKind::LeftBrace, start, end);
                true
            }
            Some('}') => {
                state.advance(1);
                let end = state.get_position();
                state.add_token(TomlSyntaxKind::RightBrace, start, end);
                true
            }
            Some(',') => {
                state.advance(1);
                let end = state.get_position();
                state.add_token(TomlSyntaxKind::Comma, start, end);
                true
            }
            Some('.') => {
                state.advance(1);
                let end = state.get_position();
                state.add_token(TomlSyntaxKind::Dot, start, end);
                true
            }
            Some('=') => {
                state.advance(1);
                let end = state.get_position();
                state.add_token(TomlSyntaxKind::Equal, start, end);
                true
            }
            _ => false,
        }
    }

    /// 解析标识符和键
    fn lex_identifier<S: Source>(&self, state: &mut State<S>) -> bool {
        if !state.current().map_or(false, |c| c.is_ascii_alphabetic() || c == '_') {
            return false;
        }

        let start = state.get_position();

        while state.current().map_or(false, |c| c.is_ascii_alphanumeric() || c == '_' || c == '-') {
            state.advance(1);
        }

        let end = state.get_position();

        // 检查是否为关键字
        let text = state.get_text_in((start..end).into());
        let kind = match text {
            "true" | "false" => TomlSyntaxKind::Boolean,
            _ => TomlSyntaxKind::BareKey,
        };

        state.add_token(kind, start, end);
        true
    }
}
