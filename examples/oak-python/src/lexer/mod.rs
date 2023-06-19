use crate::{kind::PythonSyntaxKind, language::PythonLanguage};
use oak_core::{IncrementalCache, Lexer, LexerState, lexer::LexOutput, source::Source};

type State<S: Source> = LexerState<S, PythonLanguage>;

#[derive(Clone)]
pub struct PythonLexer<'config> {
    config: &'config PythonLanguage,
}

impl<'config> PythonLexer<'config> {
    pub fn new(config: &'config PythonLanguage) -> Self {
        Self { config }
    }

    /// 跳过空白字符
    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.current() {
            if ch == ' ' || ch == '\t' {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        if state.get_position() > start_pos {
            state.add_token(PythonSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.current() {
            state.advance(1);
            state.add_token(PythonSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.current() {
            state.advance(1);
            if let Some('\n') = state.current() {
                state.advance(1);
            }
            state.add_token(PythonSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some('#') = state.current() {
            let start_pos = state.get_position();
            state.advance(1); // 跳过 '#'

            // 读取到行尾
            while let Some(ch) = state.current() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(PythonSyntaxKind::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符串字面量
    fn lex_string<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        // 检查是否是字符串开始
        let quote_char = match state.current() {
            Some('"') => '"',
            Some('\'') => '\'',
            _ => return false,
        };

        state.advance(1); // 跳过开始引号

        // 检查是否是三引号字符串 - 简化实现，不支持三引号
        let mut escaped = false;
        while let Some(ch) = state.current() {
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

            if ch == quote_char {
                state.advance(1); // 跳过结束引号
                break;
            }
            else if ch == '\n' || ch == '\r' {
                // 单行字符串不能包含换行符
                break;
            }
            else {
                state.advance(ch.len_utf8());
            }
        }

        state.add_token(PythonSyntaxKind::String, start_pos, state.get_position());
        true
    }

    /// 处理数字字面量
    fn lex_number<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if !state.current().map_or(false, |c| c.is_ascii_digit()) {
            return false;
        }

        // 简化实现：只处理基本的十进制数字
        while let Some(ch) = state.current() {
            if ch.is_ascii_digit() || ch == '.' {
                state.advance(1);
            }
            else {
                break;
            }
        }

        state.add_token(PythonSyntaxKind::Number, start_pos, state.get_position());
        true
    }

    /// 处理标识符或关键字
    fn lex_identifier_or_keyword<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        // 检查第一个字符
        if !state.current().map_or(false, |c| c.is_ascii_alphabetic() || c == '_') {
            return false;
        }

        // 读取标识符
        while let Some(ch) = state.current() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        // 检查是否是关键字
        let kind = PythonSyntaxKind::Identifier; // 简化处理，都标记为标识符

        state.add_token(kind, start_pos, state.get_position());
        true
    }

    /// 处理操作符
    fn lex_operator<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        // 简化实现：只处理单字符操作符
        if let Some(ch) = state.current() {
            let kind = match ch {
                '+' => {
                    state.advance(1);
                    PythonSyntaxKind::Plus
                }
                '-' => {
                    state.advance(1);
                    PythonSyntaxKind::Minus
                }
                '*' => {
                    state.advance(1);
                    PythonSyntaxKind::Star
                }
                '/' => {
                    state.advance(1);
                    PythonSyntaxKind::Slash
                }
                '%' => {
                    state.advance(1);
                    PythonSyntaxKind::Percent
                }
                '=' => {
                    state.advance(1);
                    PythonSyntaxKind::Assign
                }
                '<' => {
                    state.advance(1);
                    PythonSyntaxKind::Less
                }
                '>' => {
                    state.advance(1);
                    PythonSyntaxKind::Greater
                }
                '&' => {
                    state.advance(1);
                    PythonSyntaxKind::Ampersand
                }
                '|' => {
                    state.advance(1);
                    PythonSyntaxKind::Pipe
                }
                '^' => {
                    state.advance(1);
                    PythonSyntaxKind::Caret
                }
                '~' => {
                    state.advance(1);
                    PythonSyntaxKind::Tilde
                }
                '@' => {
                    state.advance(1);
                    PythonSyntaxKind::At
                }
                _ => return false,
            };

            state.add_token(kind, start_pos, state.get_position());
            return true;
        }

        false
    }

    /// 处理分隔符
    fn lex_delimiter<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.current() {
            let kind = match ch {
                '(' => PythonSyntaxKind::LeftParen,
                ')' => PythonSyntaxKind::RightParen,
                '[' => PythonSyntaxKind::LeftBracket,
                ']' => PythonSyntaxKind::RightBracket,
                '{' => PythonSyntaxKind::LeftBrace,
                '}' => PythonSyntaxKind::RightBrace,
                ',' => PythonSyntaxKind::Comma,
                ':' => PythonSyntaxKind::Colon,
                ';' => PythonSyntaxKind::Semicolon,
                '.' => PythonSyntaxKind::Dot, // 简化处理，不支持省略号
                _ => return false,
            };

            state.advance(1);
            state.add_token(kind, start_pos, state.get_position());
            return true;
        }

        false
    }

    /// 处理缩进
    fn lex_indent<S: Source>(&self, state: &mut State<S>) -> bool {
        // 简化的缩进处理
        false
    }

    /// 处理其他字符
    fn lex_other<S: Source>(&self, state: &mut State<S>) -> bool {
        if let Some(ch) = state.current() {
            let start_pos = state.get_position();
            state.advance(ch.len_utf8());
            state.add_token(PythonSyntaxKind::Error, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<PythonLanguage> for PythonLexer<'config> {
    fn lex(&self, source: impl Source) -> LexOutput<PythonLanguage> {
        let mut state = LexerState::new(source);

        while state.not_at_end() {
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_comment(&mut state) {
                continue;
            }

            if self.lex_string(&mut state) {
                continue;
            }

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_identifier_or_keyword(&mut state) {
                continue;
            }

            if self.lex_operator(&mut state) {
                continue;
            }

            if self.lex_delimiter(&mut state) {
                continue;
            }

            if self.lex_indent(&mut state) {
                continue;
            }

            if self.lex_other(&mut state) {
                continue;
            }

            // 如果没有匹配任何规则，前进一个字符避免无限循环
            if let Some(ch) = state.current() {
                let start_pos = state.get_position();
                state.advance(ch.len_utf8());
                state.add_token(PythonSyntaxKind::Error, start_pos, state.get_position());
            }
            else {
                break;
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(PythonSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish(Ok(()))
    }

    fn lex_incremental(
        &self,
        source: impl Source,
        _offset: usize,
        _cache: IncrementalCache<'_, PythonLanguage>,
    ) -> LexOutput<PythonLanguage> {
        // 简化实现，直接调用完整的 lex 方法
        self.lex(source)
    }
}
