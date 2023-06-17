use crate::{language::TwigLanguage, syntax::TwigSyntaxKind};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, TwigLanguage>;

pub struct TwigLexer<'config> {
    config: &'config TwigLanguage,
}

impl<'config> TwigLexer<'config> {
    pub fn new(config: &'config TwigLanguage) -> Self {
        Self { config }
    }

    /// 跳过空白字符（不包括换行符）
    fn skip_whitespace(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        if state.get_position() > start_pos {
            state.add_token(TwigSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 跳过换行符
    fn skip_newline(&self, state: &mut State) -> bool {
        if state.peek() == Some('\n') {
            let start_pos = state.get_position();
            state.advance(1);
            state.add_token(TwigSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 解析注释
    fn parse_comment(&self, state: &mut State) -> bool {
        if state.peek() == Some('#') {
            let start_pos = state.get_position();

            // 跳过 #
            state.advance(1);

            // 读取到行尾
            while let Some(ch) = state.peek() {
                if ch == '\n' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(TwigSyntaxKind::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 解析字符串
    fn parse_string(&self, state: &mut State) -> bool {
        let quote_char = match state.peek() {
            Some('"') | Some('\'') => state.peek().unwrap(),
            _ => return false,
        };

        let start_pos = state.get_position();
        state.advance(1); // 跳过开始引号

        while let Some(ch) = state.peek() {
            if ch == quote_char {
                state.advance(1); // 跳过结束引号
                break;
            }
            else if ch == '\\' {
                state.advance(1); // 跳过转义字符
                if state.peek().is_some() {
                    state.advance(state.peek().unwrap().len_utf8());
                }
            }
            else {
                state.advance(ch.len_utf8());
            }
        }

        state.add_token(TwigSyntaxKind::String, start_pos, state.get_position());
        true
    }

    /// 解析数字
    fn parse_number(&self, state: &mut State) -> bool {
        if !state.peek().map_or(false, |ch| ch.is_ascii_digit()) {
            return false;
        }

        let start_pos = state.get_position();

        // 解析整数部分
        while let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);
            }
            else {
                break;
            }
        }

        // 检查小数点
        if state.peek() == Some('.') {
            state.advance(1);

            // 解析小数部分
            while let Some(ch) = state.peek() {
                if ch.is_ascii_digit() {
                    state.advance(1);
                }
                else {
                    break;
                }
            }
        }

        state.add_token(TwigSyntaxKind::Number, start_pos, state.get_position());
        true
    }

    /// 解析标识符
    fn parse_identifier(&self, state: &mut State) -> bool {
        if !state.peek().map_or(false, |ch| ch.is_alphabetic() || ch == '_') {
            return false;
        }

        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        let kind = TwigSyntaxKind::Identifier;
        state.add_token(kind, start_pos, state.get_position());
        true
    }

    /// 判断是否为关键字或标识符
    fn keyword_or_identifier(&self, text: &str) -> TwigSyntaxKind {
        match text {
            "true" | "false" => TwigSyntaxKind::Boolean,
            "null" => TwigSyntaxKind::Null,
            _ => TwigSyntaxKind::Identifier,
        }
    }

    /// 解析单字符标记
    fn parse_single_char(&self, state: &mut State) -> bool {
        if let Some(ch) = state.peek() {
            let start_pos = state.get_position();

            let kind = match ch {
                '{' => TwigSyntaxKind::LeftBrace,
                '}' => TwigSyntaxKind::RightBrace,
                '[' => TwigSyntaxKind::LeftBracket,
                ']' => TwigSyntaxKind::RightBracket,
                '(' => TwigSyntaxKind::LeftParen,
                ')' => TwigSyntaxKind::RightParen,
                '|' => TwigSyntaxKind::Pipe,
                ',' => TwigSyntaxKind::Comma,
                '.' => TwigSyntaxKind::Dot,
                '=' => TwigSyntaxKind::Equal,
                _ => return false,
            };

            state.advance(ch.len_utf8());
            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<TwigLanguage> for TwigLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<TwigSyntaxKind> {
        let mut state = State::new(source);

        while !state.is_at_end() {
            // 跳过空白字符
            if self.skip_whitespace(&mut state) {
                continue;
            }

            // 跳过换行

            if self.skip_newline(&mut state) {
                continue;
            }

            // 解析注释
            if self.parse_comment(&mut state) {
                continue;
            }

            // 解析字符

            if self.parse_string(&mut state) {
                continue;
            }

            // 解析数字
            if self.parse_number(&mut state) {
                continue;
            }

            // 解析标识

            if self.parse_identifier(&mut state) {
                continue;
            }

            // 解析单字符标

            if self.parse_single_char(&mut state) {
                continue;
            }

            // 如果都不匹配，则跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(TwigSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF 标记
        let eof_pos = state.get_position();
        state.add_token(TwigSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
