#![doc = include_str!("readme.md")]
pub mod token_type;
pub use token_type::ErlangTokenType;

use crate::language::ErlangLanguage;
use oak_core::{
    errors::OakError,
    lexer::{LexOutput, Lexer, LexerCache, LexerState},
    source::{Source, TextEdit},
};
use std::{collections::HashSet, sync::LazyLock};

/// Erlang 词法分析器
#[derive(Clone)]
pub struct ErlangLexer<'config> {
    _config: &'config ErlangLanguage,
}

impl<'config> Lexer<ErlangLanguage> for ErlangLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, text: &'a S, _edits: &[TextEdit], cache: &'a mut impl LexerCache<ErlangLanguage>) -> LexOutput<ErlangLanguage> {
        let mut state = LexerState::new(text);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> ErlangLexer<'config> {
    pub fn new(config: &'config ErlangLanguage) -> Self {
        Self { _config: config }
    }

    /// 主要的词法分析运行方法
    pub fn run<'a, S: Source + ?Sized>(&self, state: &mut LexerState<'a, S, ErlangLanguage>) -> Result<(), OakError> {
        while state.not_at_end() {
            // 安全检查，防止无限循环
            let start_pos = state.get_position();

            // 跳过空白字符和注释
            if self.skip_whitespace_and_comments(state) {
                continue;
            }

            // 词法分析各种 token
            if self.lex_string_literal(state) {
                continue;
            }

            if self.lex_character_literal(state) {
                continue;
            }

            if self.lex_number(state) {
                continue;
            }

            if self.lex_identifier_atom_or_keyword(state) {
                continue;
            }

            if self.lex_operator(state) {
                continue;
            }

            if self.lex_single_char_token(state) {
                continue;
            }

            // 安全检查
            if state.get_position() == start_pos {
                // 如果位置没有前进，跳过一个字符以避免无限循环
                if let Some(ch) = state.current() {
                    state.advance(ch.len_utf8());
                    let end = state.get_position();
                    state.add_token(ErlangTokenType::Error, start_pos, end);
                }
            }
        }
        Ok(())
    }

    /// 跳过空白字符和注释
    fn skip_whitespace_and_comments<S: Source + ?Sized>(&self, state: &mut LexerState<S, ErlangLanguage>) -> bool {
        let mut skipped = false;

        // 跳过空白字符
        while let Some(ch) = state.current() {
            if WHITESPACE.contains(&ch) {
                let start = state.get_position();
                if ch == '\n' {
                    state.advance(1);
                    state.add_token(ErlangTokenType::Newline, start, state.get_position());
                }
                else {
                    // 跳过连续的空白字符
                    while let Some(ch) = state.current() {
                        if WHITESPACE.contains(&ch) && ch != '\n' {
                            state.advance(ch.len_utf8());
                        }
                        else {
                            break;
                        }
                    }
                    state.add_token(ErlangTokenType::Whitespace, start, state.get_position());
                }
                skipped = true;
            }
            else if ch == '%' {
                // 行注释
                let start = state.get_position();
                state.advance(1); // 跳过 '%'

                // 读取到行尾
                while let Some(ch) = state.current() {
                    if ch == '\n' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }

                state.add_token(ErlangTokenType::Comment, start, state.get_position());
                skipped = true;
            }
            else {
                break;
            }
        }

        skipped
    }

    /// 词法分析字符串字面量
    fn lex_string_literal<S: Source + ?Sized>(&self, state: &mut LexerState<S, ErlangLanguage>) -> bool {
        if let Some('"') = state.current() {
            let start = state.get_position();
            state.advance(1); // 跳过开始的 '"'

            while let Some(ch) = state.current() {
                if ch == '"' {
                    state.advance(1); // 跳过结束的 '"'
                    let end = state.get_position();
                    state.add_token(ErlangTokenType::String, start, end);
                    return true;
                }
                else if ch == '\\' {
                    state.advance(1); // 跳过转义字符
                    if let Some(ch) = state.current() {
                        state.advance(ch.len_utf8());
                    }
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            // 未闭合的字符串
            let end = state.get_position();
            state.add_token(ErlangTokenType::String, start, end);
            true
        }
        else {
            false
        }
    }

    /// 词法分析字符字面量
    fn lex_character_literal<S: Source + ?Sized>(&self, state: &mut LexerState<S, ErlangLanguage>) -> bool {
        if let Some('$') = state.current() {
            let start = state.get_position();
            state.advance(1); // 跳过 '$'

            if let Some(ch) = state.current() {
                if ch == '\\' {
                    state.advance(1);
                    // 简单的转义或八进制转义
                    if let Some(next) = state.current() {
                        if next.is_ascii_digit() {
                            // 八进制
                            let mut count = 0;
                            while let Some(ch) = state.current() {
                                if ch.is_ascii_digit() && count < 3 {
                                    state.advance(1);
                                    count += 1;
                                }
                                else {
                                    break;
                                }
                            }
                        }
                        else {
                            state.advance(next.len_utf8());
                        }
                    }
                }
                else {
                    state.advance(ch.len_utf8());
                }
                state.add_token(ErlangTokenType::Character, start, state.get_position());
                return true;
            }
            else {
                // 只有 $ 没有字符
                state.add_token(ErlangTokenType::Error, start, state.get_position());
                return true;
            }
        }
        else {
            false
        }
    }

    /// 词法分析数字
    fn lex_number<S: Source + ?Sized>(&self, state: &mut LexerState<S, ErlangLanguage>) -> bool {
        if let Some(ch) = state.current() {
            if ch.is_ascii_digit() {
                let start = state.get_position();

                // 读取整数部分
                while let Some(ch) = state.current() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                // 检查小数点
                if let Some('.') = state.current() {
                    if let Some(next_ch) = state.peek() {
                        if next_ch.is_ascii_digit() {
                            state.advance(1); // 跳过 '.'

                            // 读取小数部分
                            while let Some(ch) = state.current() {
                                if ch.is_ascii_digit() {
                                    state.advance(1);
                                }
                                else {
                                    break;
                                }
                            }
                        }
                    }
                }

                // 检查科学计数法
                if let Some(ch) = state.current() {
                    if ch == 'e' || ch == 'E' {
                        state.advance(1);

                        // 可选的符号
                        if let Some(ch) = state.current() {
                            if ch == '+' || ch == '-' {
                                state.advance(1);
                            }
                        }

                        // 指数部分
                        while let Some(ch) = state.current() {
                            if ch.is_ascii_digit() {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                    }
                }

                state.add_token(ErlangTokenType::Number, start, state.get_position());
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    /// 词法分析标识符、原子或关键字
    fn lex_identifier_atom_or_keyword<S: Source + ?Sized>(&self, state: &mut LexerState<S, ErlangLanguage>) -> bool {
        if let Some(ch) = state.current() {
            let start = state.get_position();

            // 变量 (大写字母或下划线开头)
            if ch.is_ascii_uppercase() || ch == '_' {
                state.advance(1);
                while let Some(ch) = state.current() {
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '@' {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }
                state.add_token(ErlangTokenType::Variable, start, state.get_position());
                return true;
            }

            // 原子 (小写字母开头)
            if ch.is_ascii_lowercase() {
                state.advance(1);
                while let Some(ch) = state.current() {
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '@' {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }
                let end = state.get_position();
                let text = state.source().get_text_in(oak_core::Range { start, end });

                // 检查是否是关键字
                if KEYWORDS.contains(text.as_ref()) {
                    let kind = match text.as_ref() {
                        "after" => ErlangTokenType::After,
                        "and" => ErlangTokenType::And,
                        "andalso" => ErlangTokenType::Andalso,
                        "band" => ErlangTokenType::Band,
                        "begin" => ErlangTokenType::Begin,
                        "bnot" => ErlangTokenType::Bnot,
                        "bor" => ErlangTokenType::Bor,
                        "bsl" => ErlangTokenType::Bsl,
                        "bsr" => ErlangTokenType::Bsr,
                        "bxor" => ErlangTokenType::Bxor,
                        "case" => ErlangTokenType::Case,
                        "catch" => ErlangTokenType::Catch,
                        "cond" => ErlangTokenType::Cond,
                        "div" => ErlangTokenType::Div,
                        "end" => ErlangTokenType::End,
                        "fun" => ErlangTokenType::Fun,
                        "if" => ErlangTokenType::If,
                        "let" => ErlangTokenType::Let,
                        "not" => ErlangTokenType::Not,
                        "of" => ErlangTokenType::Of,
                        "or" => ErlangTokenType::Or,
                        "orelse" => ErlangTokenType::Orelse,
                        "query" => ErlangTokenType::Query,
                        "receive" => ErlangTokenType::Receive,
                        "rem" => ErlangTokenType::Rem,
                        "try" => ErlangTokenType::Try,
                        "when" => ErlangTokenType::When,
                        "xor" => ErlangTokenType::Xor,
                        _ => ErlangTokenType::Atom,
                    };
                    state.add_token(kind, start, end);
                }
                else {
                    state.add_token(ErlangTokenType::Atom, start, end);
                }
                return true;
            }

            // 引用原子 ('atom')
            if ch == '\'' {
                state.advance(1);
                while let Some(ch) = state.current() {
                    if ch == '\'' {
                        state.advance(1);
                        state.add_token(ErlangTokenType::Atom, start, state.get_position());
                        return true;
                    }
                    else if ch == '\\' {
                        state.advance(1);
                        if let Some(next) = state.current() {
                            state.advance(next.len_utf8());
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }
                state.add_token(ErlangTokenType::Atom, start, state.get_position());
                return true;
            }
        }
        false
    }

    /// 词法分析操作符
    fn lex_operator<S: Source + ?Sized>(&self, state: &mut LexerState<S, ErlangLanguage>) -> bool {
        if let Some(ch) = state.current() {
            let start = state.get_position();

            match ch {
                '+' => {
                    state.advance(1);
                    if let Some('+') = state.current() {
                        state.advance(1);
                        state.add_token(ErlangTokenType::PlusPlus, start, state.get_position());
                    }
                    else {
                        state.add_token(ErlangTokenType::Plus, start, state.get_position());
                    }
                    true
                }
                '-' => {
                    state.advance(1);
                    if let Some('-') = state.current() {
                        state.advance(1);
                        state.add_token(ErlangTokenType::MinusMinus, start, state.get_position());
                    }
                    else if let Some('>') = state.current() {
                        state.advance(1);
                        state.add_token(ErlangTokenType::Arrow, start, state.get_position());
                    }
                    else {
                        state.add_token(ErlangTokenType::Minus, start, state.get_position());
                    }
                    true
                }
                '*' => {
                    state.advance(1);
                    state.add_token(ErlangTokenType::Star, start, state.get_position());
                    true
                }
                '/' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        state.add_token(ErlangTokenType::SlashEqual, start, state.get_position());
                    }
                    else {
                        state.add_token(ErlangTokenType::Slash, start, state.get_position());
                    }
                    true
                }
                '=' => {
                    state.advance(1);
                    match state.current() {
                        Some('=') => {
                            state.advance(1);
                            state.add_token(ErlangTokenType::EqualEqual, start, state.get_position());
                        }
                        Some(':') => {
                            state.advance(1);
                            if let Some('=') = state.current() {
                                state.advance(1);
                                state.add_token(ErlangTokenType::EqualColonEqual, start, state.get_position());
                            }
                            else {
                                // 回退
                                state.set_position(start + 1);
                                state.add_token(ErlangTokenType::Equal, start, state.get_position());
                            }
                        }
                        Some('/') => {
                            state.advance(1);
                            if let Some('=') = state.current() {
                                state.advance(1);
                                state.add_token(ErlangTokenType::EqualSlashEqual, start, state.get_position());
                            }
                            else {
                                // 回退
                                state.set_position(start + 1);
                                state.add_token(ErlangTokenType::Equal, start, state.get_position());
                            }
                        }
                        Some('<') => {
                            state.advance(1);
                            state.add_token(ErlangTokenType::LessEqual, start, state.get_position());
                        }
                        _ => {
                            state.add_token(ErlangTokenType::Equal, start, state.get_position());
                        }
                    }
                    true
                }
                '<' => {
                    state.advance(1);
                    state.add_token(ErlangTokenType::Less, start, state.get_position());
                    true
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        state.add_token(ErlangTokenType::GreaterEqual, start, state.get_position());
                    }
                    else {
                        state.add_token(ErlangTokenType::Greater, start, state.get_position());
                    }
                    true
                }
                '!' => {
                    state.advance(1);
                    state.add_token(ErlangTokenType::Exclamation, start, state.get_position());
                    true
                }
                '?' => {
                    state.advance(1);
                    state.add_token(ErlangTokenType::Question, start, state.get_position());
                    true
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.current() {
                        state.advance(1);
                        state.add_token(ErlangTokenType::PipePipe, start, state.get_position());
                    }
                    else {
                        state.add_token(ErlangTokenType::Pipe, start, state.get_position());
                    }
                    true
                }
                '#' => {
                    state.advance(1);
                    state.add_token(ErlangTokenType::Hash, start, state.get_position());
                    true
                }
                _ => false,
            }
        }
        else {
            false
        }
    }

    /// 词法分析单字符 token
    fn lex_single_char_token<S: Source + ?Sized>(&self, state: &mut LexerState<S, ErlangLanguage>) -> bool {
        if let Some(ch) = state.current() {
            let start = state.get_position();
            let kind = match ch {
                '(' => Some(ErlangTokenType::LeftParen),
                ')' => Some(ErlangTokenType::RightParen),
                '{' => Some(ErlangTokenType::LeftBrace),
                '}' => Some(ErlangTokenType::RightBrace),
                '[' => Some(ErlangTokenType::LeftBracket),
                ']' => Some(ErlangTokenType::RightBracket),
                ',' => Some(ErlangTokenType::Comma),
                ';' => Some(ErlangTokenType::Semicolon),
                '.' => Some(ErlangTokenType::Dot),
                ':' => Some(ErlangTokenType::Colon),
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
        else {
            false
        }
    }
}

// 静态配置
static WHITESPACE: LazyLock<HashSet<char>> = LazyLock::new(|| [' ', '\t', '\r', '\n'].into_iter().collect());

static KEYWORDS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    ["after", "and", "andalso", "band", "begin", "bnot", "bor", "bsl", "bsr", "bxor", "case", "catch", "cond", "div", "end", "fun", "if", "let", "not", "of", "or", "orelse", "query", "receive", "rem", "try", "when", "xor"].into_iter().collect()
});
