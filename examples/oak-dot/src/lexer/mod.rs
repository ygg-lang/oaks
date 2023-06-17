use crate::{kind::DotSyntaxKind, language::DotLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, DotLanguage>;

pub struct DotLexer<'config> {
    config: &'config DotLanguage,
}

impl<'config> DotLexer<'config> {
    pub fn new(config: &'config DotLanguage) -> Self {
        Self { config }
    }

    /// 跳过空白字符
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
            state.add_token(DotSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(DotSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(DotSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            match state.peek_next_n(1) {
                Some('/') => {
                    // 单行注释
                    state.advance(1);
                    state.advance(1);

                    while let Some(ch) = state.peek() {
                        if ch == '\n' || ch == '\r' {
                            break;
                        }
                        state.advance(ch.len_utf8());
                    }

                    state.add_token(DotSyntaxKind::Comment, start_pos, state.get_position());
                    true
                }
                Some('*') => {
                    // 多行注释
                    state.advance(1);
                    state.advance(1);

                    while let Some(ch) = state.peek() {
                        if ch == '*' {
                            if state.peek_next_n(1) == Some('/') {
                                state.advance(1);
                                state.advance(1);
                                break;
                            }
                        }
                        state.advance(ch.len_utf8());
                    }

                    state.add_token(DotSyntaxKind::Comment, start_pos, state.get_position());
                    true
                }
                _ => false,
            }
        }
        else if let Some('#') = state.peek() {
            // # 风格注释
            state.advance(1);

            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(DotSyntaxKind::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理标识符和关键
    fn lex_identifier_or_keyword(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let end_pos = state.get_position();
                let text = source.get_text_in(core::range::Range { start: start_pos, end: end_pos }).unwrap_or("");

                let token_kind = match text.to_lowercase().as_str() {
                    "graph" => DotSyntaxKind::Graph,
                    "digraph" => DotSyntaxKind::Digraph,
                    "subgraph" => DotSyntaxKind::Subgraph,
                    "node" => DotSyntaxKind::Node,
                    "edge" => DotSyntaxKind::Edge,
                    "strict" => DotSyntaxKind::Strict,
                    _ => DotSyntaxKind::Identifier,
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

    /// 处理数字
    fn lex_number(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let is_negative = ch == '-';
            let mut has_digit = false;

            if is_negative {
                // 检查负号后面是否有数字
                let next_pos = state.get_position() + 1;
                if next_pos < source.len() {
                    let next_ch = source.get_char_at(next_pos);
                    if next_ch.map_or(false, |c| c.is_ascii_digit()) {
                        state.advance(1); // 跳过负号
                    }
                    else {
                        return false;
                    }
                }
                else {
                    return false;
                }
            }

            if let Some(ch) = state.peek() {
                if ch.is_ascii_digit() {
                    has_digit = true;
                    state.advance(ch.len_utf8());

                    // 处理整数部分
                    while let Some(ch) = state.peek() {
                        if ch.is_ascii_digit() {
                            state.advance(ch.len_utf8());
                        }
                        else {
                            break;
                        }
                    }

                    // 处理小数部分
                    if let Some('.') = state.peek() {
                        let dot_pos = state.get_position();
                        state.advance(1);

                        if let Some(ch) = state.peek() {
                            if ch.is_ascii_digit() {
                                while let Some(ch) = state.peek() {
                                    if ch.is_ascii_digit() {
                                        state.advance(ch.len_utf8());
                                    }
                                    else {
                                        break;
                                    }
                                }
                            }
                            else {
                                // 回退点号
                                state.set_position(dot_pos);
                            }
                        }
                        else {
                            // 回退点号
                            state.set_position(dot_pos);
                        }
                    }
                }
            }

            if has_digit || (is_negative && state.get_position() > start_pos + 1) {
                state.add_token(DotSyntaxKind::Number, start_pos, state.get_position());
                true
            }
            else {
                // 回退到开始位                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// 处理字符
    fn lex_string(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('"') = state.peek() {
            state.advance(1);

            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1);
                    state.add_token(DotSyntaxKind::String, start_pos, state.get_position());
                    return true;
                }
                else if ch == '\\' {
                    state.advance(1);
                    if state.peek().is_some() {
                        state.advance(1);
                    }
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            // 未闭合的字符            state.add_token(DotSyntaxKind::Error, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理操作
    fn lex_operator(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            match ch {
                '-' => {
                    let next_pos = state.get_position() + 1;
                    if next_pos < source.len() {
                        let next_ch = source.get_char_at(next_pos);
                        match next_ch {
                            Some('>') => {
                                state.advance(1);
                                state.advance(1);
                                state.add_token(DotSyntaxKind::Arrow, start_pos, state.get_position());
                                true
                            }
                            Some('-') => {
                                state.advance(1);
                                state.advance(1);
                                state.add_token(DotSyntaxKind::Line, start_pos, state.get_position());
                                true
                            }
                            _ => false,
                        }
                    }
                    else {
                        false
                    }
                }
                '=' => {
                    state.advance(1);
                    state.add_token(DotSyntaxKind::Equal, start_pos, state.get_position());
                    true
                }
                ';' => {
                    state.advance(1);
                    state.add_token(DotSyntaxKind::Semicolon, start_pos, state.get_position());
                    true
                }
                ',' => {
                    state.advance(1);
                    state.add_token(DotSyntaxKind::Comma, start_pos, state.get_position());
                    true
                }
                _ => false,
            }
        }
        else {
            false
        }
    }

    /// 处理分隔
    fn lex_delimiter(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '{' => DotSyntaxKind::LeftBrace,
                '}' => DotSyntaxKind::RightBrace,
                '[' => DotSyntaxKind::LeftBracket,
                ']' => DotSyntaxKind::RightBracket,
                '(' => DotSyntaxKind::LeftParen,
                ')' => DotSyntaxKind::RightParen,
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

impl<'config> Lexer<DotLanguage> for DotLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<DotSyntaxKind> {
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

            if self.lex_identifier_or_keyword(&mut state, source) {
                continue;
            }

            if self.lex_number(&mut state, source) {
                continue;
            }

            if self.lex_string(&mut state) {
                continue;
            }

            if self.lex_operator(&mut state, source) {
                continue;
            }

            if self.lex_delimiter(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(DotSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(DotSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
