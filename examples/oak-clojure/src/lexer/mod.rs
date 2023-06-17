use crate::{kind::ClojureSyntaxKind, language::ClojureLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, ClojureLanguage>;

pub struct ClojureLexer<'config> {
    config: &'config ClojureLanguage,
}

impl<'config> ClojureLexer<'config> {
    pub fn new(config: &'config ClojureLanguage) -> Self {
        Self { config }
    }

    /// 跳过空白字符
    fn skip_whitespace(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' || ch == ',' {
                // Clojure 中逗号被视为空
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        if state.get_position() > start_pos {
            state.add_token(ClojureSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(ClojureSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(ClojureSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(';') = state.peek() {
            state.advance(1);

            // 读取到行
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(ClojureSyntaxKind::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符串字面量
    fn lex_string(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('"') = state.peek() {
            state.advance(1);

            let mut escaped = false;
            while let Some(ch) = state.peek() {
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

                if ch == '"' {
                    state.advance(1);
                    break;
                }

                if ch == '\n' || ch == '\r' {
                    break; // 未闭合的字符
                }

                state.advance(ch.len_utf8());
            }

            state.add_token(ClojureSyntaxKind::StringLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符字面
    fn lex_character(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('\\') = state.peek() {
            state.advance(1);

            // 处理特殊字符名称
            if let Some(ch) = state.peek() {
                if ch.is_ascii_alphabetic() {
                    // 可能是命名字符如 \newline, \space, \tab
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_alphabetic() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }
                }
                else {
                    // 单个字符
                    state.advance(ch.len_utf8());
                }
            }

            state.add_token(ClojureSyntaxKind::CharacterLiteral, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理数字字面
    fn lex_number(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() || (ch == '-' && state.peek_next_n(1).map_or(false, |c| c.is_ascii_digit())) {
                if ch == '-' {
                    state.advance(1);
                }

                // 整数部分
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                // 检查小数点
                if let Some('.') = state.peek() {
                    if let Some(next_ch) = state.peek_next_n(1) {
                        if next_ch.is_ascii_digit() {
                            state.advance(1); // 消费小数
                            while let Some(ch) = state.peek() {
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

                // 检查科学记数法
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
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                    }
                }

                // 检查后缀 (M for BigDecimal, N for BigInt)
                if let Some(ch) = state.peek() {
                    if ch == 'M' || ch == 'N' {
                        state.advance(1);
                    }
                }

                state.add_token(ClojureSyntaxKind::NumberLiteral, start_pos, state.get_position());
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

    /// 处理关键
    fn lex_keyword(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(':') = state.peek() {
            state.advance(1);

            // 读取关键字名
            while let Some(ch) = state.peek() {
                if ch.is_ascii_alphanumeric()
                    || ch == '-'
                    || ch == '_'
                    || ch == '?'
                    || ch == '!'
                    || ch == '*'
                    || ch == '+'
                    || ch == '/'
                    || ch == '='
                    || ch == '<'
                    || ch == '>'
                {
                    state.advance(ch.len_utf8());
                }
                else {
                    break;
                }
            }

            if state.get_position() > start_pos + 1 {
                state.add_token(ClojureSyntaxKind::KeywordLiteral, start_pos, state.get_position());
                true
            }
            else {
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// 处理符号
    fn lex_symbol(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic()
                || ch == '-'
                || ch == '_'
                || ch == '?'
                || ch == '!'
                || ch == '*'
                || ch == '+'
                || ch == '/'
                || ch == '='
                || ch == '<'
                || ch == '>'
            {
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric()
                        || ch == '-'
                        || ch == '_'
                        || ch == '?'
                        || ch == '!'
                        || ch == '*'
                        || ch == '+'
                        || ch == '/'
                        || ch == '='
                        || ch == '<'
                        || ch == '>'
                        || ch == '.'
                    {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = source.get_text_in((start_pos..state.get_position()).into()).unwrap_or("");
                let token_kind = match text {
                    "true" | "false" => ClojureSyntaxKind::BooleanLiteral,
                    "nil" => ClojureSyntaxKind::NilLiteral,
                    _ => ClojureSyntaxKind::Symbol,
                };

                state.add_token(token_kind, start_pos, state.get_position());
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

    /// 处理集合分隔
    fn lex_collection_delimiters(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => {
                    state.advance(1);
                    ClojureSyntaxKind::ListStart
                }
                ')' => {
                    state.advance(1);
                    ClojureSyntaxKind::ListEnd
                }
                '[' => {
                    state.advance(1);
                    ClojureSyntaxKind::VectorStart
                }
                ']' => {
                    state.advance(1);
                    ClojureSyntaxKind::VectorEnd
                }
                '{' => {
                    state.advance(1);
                    ClojureSyntaxKind::MapStart
                }
                '}' => {
                    state.advance(1);
                    ClojureSyntaxKind::MapEnd
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

    /// 处理特殊字符和读取器
    fn lex_special_chars(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '\'' => {
                    state.advance(1);
                    ClojureSyntaxKind::Quote
                }
                '~' => {
                    state.advance(1);
                    // 检~@
                    if let Some('@') = state.peek() {
                        state.advance(1);
                        ClojureSyntaxKind::UnquoteSplice
                    }
                    else {
                        ClojureSyntaxKind::Unquote
                    }
                }
                '@' => {
                    state.advance(1);
                    ClojureSyntaxKind::Deref
                }
                '^' => {
                    state.advance(1);
                    ClojureSyntaxKind::Meta
                }
                '#' => {
                    state.advance(1);
                    // 检查各# 开头的形式
                    if let Some(next_ch) = state.peek() {
                        match next_ch {
                            '{' => {
                                state.advance(1);
                                ClojureSyntaxKind::SetStart
                            }
                            '(' => {
                                state.advance(1);
                                ClojureSyntaxKind::AnonFnStart
                            }
                            '"' => {
                                // 正则表达式字面量
                                state.advance(1);
                                let mut escaped = false;
                                while let Some(ch) = state.peek() {
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

                                    if ch == '"' {
                                        state.advance(1);
                                        break;
                                    }

                                    state.advance(ch.len_utf8());
                                }
                                ClojureSyntaxKind::RegexLiteral
                            }
                            _ => ClojureSyntaxKind::Dispatch,
                        }
                    }
                    else {
                        ClojureSyntaxKind::Dispatch
                    }
                }
                '%' => {
                    state.advance(1);
                    // 匿名函数参数
                    if let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            while let Some(ch) = state.peek() {
                                if ch.is_ascii_digit() {
                                    state.advance(1);
                                }
                                else {
                                    break;
                                }
                            }
                        }
                    }
                    ClojureSyntaxKind::AnonFnArg
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
}

impl<'config> Lexer<ClojureLanguage> for ClojureLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<ClojureSyntaxKind> {
        let mut state = LexerState::new(source);

        while state.not_at_end() {
            // 尝试各种词法规则
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

            if self.lex_character(&mut state) {
                continue;
            }

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_keyword(&mut state) {
                continue;
            }

            if self.lex_collection_delimiters(&mut state) {
                continue;
            }

            if self.lex_special_chars(&mut state) {
                continue;
            }

            if self.lex_symbol(&mut state, source) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(ClojureSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(ClojureSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
