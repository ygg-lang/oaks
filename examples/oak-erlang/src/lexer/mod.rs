use crate::{kind::ErlangSyntaxKind, language::ErlangLanguage};
use oak_core::{
    IncrementalCache, Lexer,
    lexer::{LexOutput, LexerState},
    source::Source,
};
use std::{collections::HashSet, sync::LazyLock};

/// Erlang 词法分析器
#[derive(Clone)]
pub struct ErlangLexer<'config> {
    config: &'config ErlangLanguage,
}

impl<'config> ErlangLexer<'config> {
    pub fn new(config: &'config ErlangLanguage) -> Self {
        Self { config }
    }

    /// 主要的词法分析运行方法
    pub fn run<S: Source>(&self, state: &mut LexerState<S, ErlangLanguage>) {
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
                    state.add_token(ErlangSyntaxKind::Error, start_pos, state.get_position());
                }
            }
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(ErlangSyntaxKind::Eof, eof_pos, eof_pos);
    }

    /// 跳过空白字符和注释
    fn skip_whitespace_and_comments<S: Source>(&self, state: &mut LexerState<S, ErlangLanguage>) -> bool {
        let mut skipped = false;

        // 跳过空白字符
        while let Some(ch) = state.current() {
            if WHITESPACE.contains(&ch) {
                let start = state.get_position();
                if ch == '\n' {
                    state.advance(1);
                    state.add_token(ErlangSyntaxKind::Newline, start, state.get_position());
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
                    state.add_token(ErlangSyntaxKind::Whitespace, start, state.get_position());
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

                state.add_token(ErlangSyntaxKind::Comment, start, state.get_position());
                skipped = true;
            }
            else {
                break;
            }
        }

        skipped
    }

    /// 词法分析字符串字面量
    fn lex_string_literal<S: Source>(&self, state: &mut LexerState<S, ErlangLanguage>) -> bool {
        if let Some('"') = state.current() {
            let start = state.get_position();
            state.advance(1); // 跳过开始的 '"'

            while let Some(ch) = state.current() {
                if ch == '"' {
                    state.advance(1); // 跳过结束的 '"'
                    state.add_token(ErlangSyntaxKind::String, start, state.get_position());
                    return true;
                }
                else if ch == '\\' {
                    state.advance(1); // 跳过转义字符
                    if let Some(_) = state.current() {
                        state.advance(state.current().unwrap().len_utf8());
                    }
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            // 未闭合的字符串
            state.add_token(ErlangSyntaxKind::Error, start, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 词法分析字符字面量
    fn lex_character_literal<S: Source>(&self, state: &mut LexerState<S, ErlangLanguage>) -> bool {
        if let Some('$') = state.current() {
            let start = state.get_position();
            state.advance(1); // 跳过 '$'

            if let Some(ch) = state.current() {
                if ch == '\\' {
                    state.advance(1); // 跳过转义字符
                    if let Some(_) = state.current() {
                        state.advance(state.current().unwrap().len_utf8());
                    }
                }
                else {
                    state.advance(ch.len_utf8());
                }
                state.add_token(ErlangSyntaxKind::Character, start, state.get_position());
                true
            }
            else {
                state.add_token(ErlangSyntaxKind::Error, start, state.get_position());
                true
            }
        }
        else {
            false
        }
    }

    /// 词法分析数字
    fn lex_number<S: Source>(&self, state: &mut LexerState<S, ErlangLanguage>) -> bool {
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

                state.add_token(ErlangSyntaxKind::Number, start, state.get_position());
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
    fn lex_identifier_atom_or_keyword<S: Source>(&self, state: &mut LexerState<S, ErlangLanguage>) -> bool {
        if let Some(ch) = state.current() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                let start = state.get_position();

                // 读取标识符
                while let Some(ch) = state.current() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = state.get_text_in((start..state.get_position()).into());
                let kind = if KEYWORDS.contains(text) {
                    match text {
                        "after" => ErlangSyntaxKind::After,
                        "and" => ErlangSyntaxKind::And,
                        "andalso" => ErlangSyntaxKind::Andalso,
                        "band" => ErlangSyntaxKind::Band,
                        "begin" => ErlangSyntaxKind::Begin,
                        "bnot" => ErlangSyntaxKind::Bnot,
                        "bor" => ErlangSyntaxKind::Bor,
                        "bsl" => ErlangSyntaxKind::Bsl,
                        "bsr" => ErlangSyntaxKind::Bsr,
                        "bxor" => ErlangSyntaxKind::Bxor,
                        "case" => ErlangSyntaxKind::Case,
                        "catch" => ErlangSyntaxKind::Catch,
                        "cond" => ErlangSyntaxKind::Cond,
                        "div" => ErlangSyntaxKind::Div,
                        "end" => ErlangSyntaxKind::End,
                        "fun" => ErlangSyntaxKind::Fun,
                        "if" => ErlangSyntaxKind::If,
                        "let" => ErlangSyntaxKind::Let,
                        "not" => ErlangSyntaxKind::Not,
                        "of" => ErlangSyntaxKind::Of,
                        "or" => ErlangSyntaxKind::Or,
                        "orelse" => ErlangSyntaxKind::Orelse,
                        "query" => ErlangSyntaxKind::Query,
                        "receive" => ErlangSyntaxKind::Receive,
                        "rem" => ErlangSyntaxKind::Rem,
                        "try" => ErlangSyntaxKind::Try,
                        "when" => ErlangSyntaxKind::When,
                        "xor" => ErlangSyntaxKind::Xor,
                        _ => ErlangSyntaxKind::Identifier,
                    }
                }
                else if ch.is_ascii_uppercase() || ch == '_' {
                    ErlangSyntaxKind::Variable
                }
                else {
                    ErlangSyntaxKind::Identifier
                };

                state.add_token(kind, start, state.get_position());
                true
            }
            else if ch == '\'' {
                // 引用原子
                let start = state.get_position();
                state.advance(1); // 跳过开始的 '\''

                while let Some(ch) = state.current() {
                    if ch == '\'' {
                        state.advance(1); // 跳过结束的 '\''
                        state.add_token(ErlangSyntaxKind::Atom, start, state.get_position());
                        return true;
                    }
                    else if ch == '\\' {
                        state.advance(1); // 跳过转义字符
                        if let Some(_) = state.current() {
                            state.advance(state.current().unwrap().len_utf8());
                        }
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                // 未闭合的原子
                state.add_token(ErlangSyntaxKind::Error, start, state.get_position());
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

    /// 词法分析操作符
    fn lex_operator<S: Source>(&self, state: &mut LexerState<S, ErlangLanguage>) -> bool {
        if let Some(ch) = state.current() {
            let start = state.get_position();

            match ch {
                '+' => {
                    state.advance(1);
                    if let Some('+') = state.current() {
                        state.advance(1);
                        state.add_token(ErlangSyntaxKind::PlusPlus, start, state.get_position());
                    }
                    else {
                        state.add_token(ErlangSyntaxKind::Plus, start, state.get_position());
                    }
                    true
                }
                '-' => {
                    state.advance(1);
                    if let Some('-') = state.current() {
                        state.advance(1);
                        state.add_token(ErlangSyntaxKind::MinusMinus, start, state.get_position());
                    }
                    else if let Some('>') = state.current() {
                        state.advance(1);
                        state.add_token(ErlangSyntaxKind::Arrow, start, state.get_position());
                    }
                    else {
                        state.add_token(ErlangSyntaxKind::Minus, start, state.get_position());
                    }
                    true
                }
                '*' => {
                    state.advance(1);
                    state.add_token(ErlangSyntaxKind::Star, start, state.get_position());
                    true
                }
                '/' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        state.add_token(ErlangSyntaxKind::SlashEqual, start, state.get_position());
                    }
                    else {
                        state.add_token(ErlangSyntaxKind::Slash, start, state.get_position());
                    }
                    true
                }
                '=' => {
                    state.advance(1);
                    match state.current() {
                        Some('=') => {
                            state.advance(1);
                            state.add_token(ErlangSyntaxKind::EqualEqual, start, state.get_position());
                        }
                        Some(':') => {
                            state.advance(1);
                            if let Some('=') = state.current() {
                                state.advance(1);
                                state.add_token(ErlangSyntaxKind::EqualColonEqual, start, state.get_position());
                            }
                            else {
                                // 回退
                                state.set_position(start + 1);
                                state.add_token(ErlangSyntaxKind::Equal, start, state.get_position());
                            }
                        }
                        Some('/') => {
                            state.advance(1);
                            if let Some('=') = state.current() {
                                state.advance(1);
                                state.add_token(ErlangSyntaxKind::EqualSlashEqual, start, state.get_position());
                            }
                            else {
                                // 回退
                                state.set_position(start + 1);
                                state.add_token(ErlangSyntaxKind::Equal, start, state.get_position());
                            }
                        }
                        Some('<') => {
                            state.advance(1);
                            state.add_token(ErlangSyntaxKind::LessEqual, start, state.get_position());
                        }
                        _ => {
                            state.add_token(ErlangSyntaxKind::Equal, start, state.get_position());
                        }
                    }
                    true
                }
                '<' => {
                    state.advance(1);
                    state.add_token(ErlangSyntaxKind::Less, start, state.get_position());
                    true
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.current() {
                        state.advance(1);
                        state.add_token(ErlangSyntaxKind::GreaterEqual, start, state.get_position());
                    }
                    else {
                        state.add_token(ErlangSyntaxKind::Greater, start, state.get_position());
                    }
                    true
                }
                '!' => {
                    state.advance(1);
                    state.add_token(ErlangSyntaxKind::Exclamation, start, state.get_position());
                    true
                }
                '?' => {
                    state.advance(1);
                    state.add_token(ErlangSyntaxKind::Question, start, state.get_position());
                    true
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.current() {
                        state.advance(1);
                        state.add_token(ErlangSyntaxKind::PipePipe, start, state.get_position());
                    }
                    else {
                        state.add_token(ErlangSyntaxKind::Pipe, start, state.get_position());
                    }
                    true
                }
                '#' => {
                    state.advance(1);
                    state.add_token(ErlangSyntaxKind::Hash, start, state.get_position());
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
    fn lex_single_char_token<S: Source>(&self, state: &mut LexerState<S, ErlangLanguage>) -> bool {
        if let Some(ch) = state.current() {
            let start = state.get_position();
            let kind = match ch {
                '(' => Some(ErlangSyntaxKind::LeftParen),
                ')' => Some(ErlangSyntaxKind::RightParen),
                '{' => Some(ErlangSyntaxKind::LeftBrace),
                '}' => Some(ErlangSyntaxKind::RightBrace),
                '[' => Some(ErlangSyntaxKind::LeftBracket),
                ']' => Some(ErlangSyntaxKind::RightBracket),
                ',' => Some(ErlangSyntaxKind::Comma),
                ';' => Some(ErlangSyntaxKind::Semicolon),
                '.' => Some(ErlangSyntaxKind::Dot),
                ':' => Some(ErlangSyntaxKind::Colon),
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

impl<'config> Lexer<ErlangLanguage> for ErlangLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<ErlangLanguage>,
    ) -> LexOutput<ErlangLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);
        self.run(&mut state);
        state.finish(Ok(()))
    }
}

// 静态配置
static WHITESPACE: LazyLock<HashSet<char>> = LazyLock::new(|| [' ', '\t', '\r', '\n'].into_iter().collect());

static KEYWORDS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    [
        "after", "and", "andalso", "band", "begin", "bnot", "bor", "bsl", "bsr", "bxor", "case", "catch", "cond", "div", "end",
        "fun", "if", "let", "not", "of", "or", "orelse", "query", "receive", "rem", "try", "when", "xor",
    ]
    .into_iter()
    .collect()
});
