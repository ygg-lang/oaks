use crate::{kind::MatlabSyntaxKind, language::MatlabLanguage};

use oak_core::{IncrementalCache, Lexer, LexerState, lexer::LexOutput, source::Source};

type State<S> = LexerState<S, MatlabLanguage>;

pub struct MatlabLexer<'config> {
    config: &'config MatlabLanguage,
}

impl<'config> MatlabLexer<'config> {
    pub fn new(config: &'config MatlabLanguage) -> Self {
        Self { config }
    }

    /// 跳过空白字符
    fn skip_whitespace<S: Source>(&self, state: &mut State<S>) -> bool {
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
            state.add_token(MatlabSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(MatlabSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(MatlabSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理标识符和关键
    fn lex_identifier<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if !ch.is_ascii_alphabetic() && ch != '_' {
                return false;
            }

            // 收集标识符字
            let mut identifier = String::new();
            while let Some(ch) = state.peek() {
                if ch.is_ascii_alphanumeric() || ch == '_' {
                    identifier.push(ch);
                    state.advance(1);
                }
                else {
                    break;
                }
            }

            // 检查是否是关键
            let token_kind = match identifier.as_str() {
                "function" => MatlabSyntaxKind::Function,
                "end" => MatlabSyntaxKind::End,
                "if" => MatlabSyntaxKind::If,
                "else" => MatlabSyntaxKind::Else,
                "elseif" => MatlabSyntaxKind::Elseif,
                "while" => MatlabSyntaxKind::While,
                "for" => MatlabSyntaxKind::For,
                "break" => MatlabSyntaxKind::Break,
                "continue" => MatlabSyntaxKind::Continue,
                "return" => MatlabSyntaxKind::Return,
                "switch" => MatlabSyntaxKind::Switch,
                "case" => MatlabSyntaxKind::Case,
                "otherwise" => MatlabSyntaxKind::Otherwise,
                "try" => MatlabSyntaxKind::Try,
                "catch" => MatlabSyntaxKind::Catch,
                "global" => MatlabSyntaxKind::Global,
                "persistent" => MatlabSyntaxKind::Persistent,
                "classdef" => MatlabSyntaxKind::Classdef,
                "properties" => MatlabSyntaxKind::Properties,
                "methods" => MatlabSyntaxKind::Methods,
                "events" => MatlabSyntaxKind::Events,
                _ => MatlabSyntaxKind::Identifier,
            };

            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理数字
    fn lex_number<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if !ch.is_ascii_digit() {
                return false;
            }

            // 处理整数部分
            while let Some(ch) = state.peek() {
                if ch.is_ascii_digit() {
                    state.advance(ch.len_utf8());
                }
                else {
                    break;
                }
            }

            // 处理小数
            if let Some('.') = state.peek() {
                if let Some(next_ch) = state.peek_next_n(1) {
                    if next_ch.is_ascii_digit() {
                        state.advance(1); // 跳过小数
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_digit() {
                                state.advance(ch.len_utf8());
                            }
                            else {
                                break;
                            }
                        }
                    }
                }
            }

            // 处理科学记数
            if let Some(ch) = state.peek() {
                if ch == 'e' || ch == 'E' {
                    state.advance(1);
                    if let Some(sign) = state.peek() {
                        if sign == '+' || sign == '-' {
                            state.advance(1);
                        }
                    }
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(ch.len_utf8());
                        }
                        else {
                            break;
                        }
                    }
                }
            }

            state.add_token(MatlabSyntaxKind::Number, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符
    fn lex_string<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(quote) = state.peek() {
            if quote != '\'' && quote != '"' {
                return false;
            }

            state.advance(1); // 跳过开始引
            while let Some(ch) = state.peek() {
                if ch == quote {
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

            let token_kind = if quote == '\'' { MatlabSyntaxKind::Character } else { MatlabSyntaxKind::String };

            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('%') = state.peek() {
            state.advance(1);

            // 检查是否是块注
            if let Some('{') = state.peek() {
                state.advance(1);

                // 查找块注释结
                while let Some(ch) = state.peek() {
                    if ch == '%' {
                        if let Some('}') = state.peek_next_n(1) {
                            state.advance(2); // 跳过 %}
                            break;
                        }
                    }
                    state.advance(ch.len_utf8());
                }

                state.add_token(MatlabSyntaxKind::BlockComment, start_pos, state.get_position());
            }
            else {
                // 行注释，读取到行
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }

                state.add_token(MatlabSyntaxKind::Comment, start_pos, state.get_position());
            }
            true
        }
        else {
            false
        }
    }

    /// 处理运算
    fn lex_operator<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '+' => {
                    state.advance(1);
                    MatlabSyntaxKind::Plus
                }
                '-' => {
                    state.advance(1);
                    MatlabSyntaxKind::Minus
                }
                '*' => {
                    state.advance(1);
                    MatlabSyntaxKind::Times
                }
                '/' => {
                    state.advance(1);
                    MatlabSyntaxKind::Divide
                }
                '^' => {
                    state.advance(1);
                    MatlabSyntaxKind::Power
                }
                '\\' => {
                    state.advance(1);
                    MatlabSyntaxKind::LeftDivide
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        MatlabSyntaxKind::Equal
                    }
                    else {
                        MatlabSyntaxKind::Assign
                    }
                }
                '~' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        MatlabSyntaxKind::NotEqual
                    }
                    else {
                        MatlabSyntaxKind::Not
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        MatlabSyntaxKind::LessEqual
                    }
                    else {
                        MatlabSyntaxKind::Less
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        MatlabSyntaxKind::GreaterEqual
                    }
                    else {
                        MatlabSyntaxKind::Greater
                    }
                }
                '&' => {
                    state.advance(1);
                    if let Some('&') = state.peek() {
                        state.advance(1);
                        MatlabSyntaxKind::AndAnd
                    }
                    else {
                        MatlabSyntaxKind::And
                    }
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.peek() {
                        state.advance(1);
                        MatlabSyntaxKind::OrOr
                    }
                    else {
                        MatlabSyntaxKind::Or
                    }
                }
                '.' => {
                    state.advance(1);
                    if let Some(next_ch) = state.peek() {
                        match next_ch {
                            '*' => {
                                state.advance(1);
                                MatlabSyntaxKind::DotTimes
                            }
                            '/' => {
                                state.advance(1);
                                MatlabSyntaxKind::DotDivide
                            }
                            '^' => {
                                state.advance(1);
                                MatlabSyntaxKind::DotPower
                            }
                            '\\' => {
                                state.advance(1);
                                MatlabSyntaxKind::DotLeftDivide
                            }
                            '\'' => {
                                state.advance(1);
                                MatlabSyntaxKind::DotTranspose
                            }
                            _ => MatlabSyntaxKind::Dot,
                        }
                    }
                    else {
                        MatlabSyntaxKind::Dot
                    }
                }
                '\'' => {
                    state.advance(1);
                    MatlabSyntaxKind::Transpose
                }
                _ => return false,
            };

            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理分隔
    fn lex_delimiter<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => MatlabSyntaxKind::LeftParen,
                ')' => MatlabSyntaxKind::RightParen,
                '[' => MatlabSyntaxKind::LeftBracket,
                ']' => MatlabSyntaxKind::RightBracket,
                '{' => MatlabSyntaxKind::LeftBrace,
                '}' => MatlabSyntaxKind::RightBrace,
                ';' => MatlabSyntaxKind::Semicolon,
                ',' => MatlabSyntaxKind::Comma,
                ':' => MatlabSyntaxKind::Colon,
                '?' => MatlabSyntaxKind::Question,
                '@' => MatlabSyntaxKind::At,
                _ => return false,
            };

            state.advance(ch.len_utf8());
            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<MatlabLanguage> for MatlabLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        _start_offset: usize,
        _cache: IncrementalCache<'_, MatlabLanguage>,
    ) -> LexOutput<MatlabLanguage> {
        self.lex_internal(source)
    }

    fn lex(&self, source: impl Source) -> LexOutput<MatlabLanguage> {
        self.lex_internal(source)
    }
}

impl<'config> MatlabLexer<'config> {
    fn lex_internal<S: Source>(&self, source: S) -> LexOutput<MatlabLanguage> {
        let mut state = LexerState::new(source);

        while state.not_at_end() {
            // 跳过空白字符
            if self.skip_whitespace(&mut state) {
                continue;
            }

            // 处理换行
            if self.lex_newline(&mut state) {
                continue;
            }

            // 处理注释
            if self.lex_comment(&mut state) {
                continue;
            }

            // 处理字符
            if self.lex_string(&mut state) {
                continue;
            }

            // 处理数字
            if self.lex_number(&mut state) {
                continue;
            }

            // 处理标识符和关键
            if self.lex_identifier(&mut state) {
                continue;
            }

            // 处理运算
            if self.lex_operator(&mut state) {
                continue;
            }

            // 处理分隔
            if self.lex_delimiter(&mut state) {
                continue;
            }

            // 如果都不匹配，跳过当前字符并标记为错
            let start_pos = state.get_position();
            if let Some(_ch) = state.peek() {
                state.advance(1);
                state.add_token(MatlabSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        state.finish(Ok(()))
    }
}
