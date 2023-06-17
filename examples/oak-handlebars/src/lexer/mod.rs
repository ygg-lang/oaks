use crate::{kind::HandlebarsSyntaxKind, language::HandlebarsLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, HandlebarsLanguage>;

pub struct HandlebarsLexer<'config> {
    config: &'config HandlebarsLanguage,
}

impl<'config> HandlebarsLexer<'config> {
    pub fn new(config: &'config HandlebarsLanguage) -> Self {
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
            state.add_token(HandlebarsSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(HandlebarsSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(HandlebarsSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理 Handlebars 表达式开始标记
    fn lex_handlebars_open(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some('{') = state.peek() {
            // 检查 {{{{ (raw block)
            if source.get_char_at(start_pos + 1) == Some('{')
                && source.get_char_at(start_pos + 2) == Some('{')
                && source.get_char_at(start_pos + 3) == Some('{')
            {
                // 检查是否是 {{{{/ (end raw block)
                if source.get_char_at(start_pos + 4) == Some('/') {
                    state.advance(5);
                    state.add_token(HandlebarsSyntaxKind::OpenEndRawBlock, start_pos, state.get_position());
                }
                else {
                    state.advance(4);
                    state.add_token(HandlebarsSyntaxKind::OpenRawBlock, start_pos, state.get_position());
                }
                return true;
            }
            // 检查 {{{ (unescaped)
            else if source.get_char_at(start_pos + 1) == Some('{') && source.get_char_at(start_pos + 2) == Some('{') {
                state.advance(3);
                state.add_token(HandlebarsSyntaxKind::OpenUnescaped, start_pos, state.get_position());
                return true;
            }
            // 检查 {{ 系列
            else if source.get_char_at(start_pos + 1) == Some('{') {
                // 检查特殊的开始标记
                match source.get_char_at(start_pos + 2) {
                    Some('#') => {
                        state.advance(3);
                        state.add_token(HandlebarsSyntaxKind::OpenBlock, start_pos, state.get_position());
                        return true;
                    }
                    Some('/') => {
                        state.advance(3);
                        state.add_token(HandlebarsSyntaxKind::CloseBlock, start_pos, state.get_position());
                        return true;
                    }
                    Some('>') => {
                        state.advance(3);
                        state.add_token(HandlebarsSyntaxKind::OpenPartial, start_pos, state.get_position());
                        return true;
                    }
                    Some('!') => {
                        // 检查是否是 {{!-- (comment block)
                        if source.get_char_at(start_pos + 3) == Some('-') && source.get_char_at(start_pos + 4) == Some('-') {
                            state.advance(5);
                            state.add_token(HandlebarsSyntaxKind::OpenCommentBlock, start_pos, state.get_position());
                        }
                        else {
                            state.advance(3);
                            state.add_token(HandlebarsSyntaxKind::OpenComment, start_pos, state.get_position());
                        }
                        return true;
                    }
                    _ => {
                        // 普通的 {{
                        state.advance(2);
                        state.add_token(HandlebarsSyntaxKind::Open, start_pos, state.get_position());
                        return true;
                    }
                }
            }
        }

        false
    }

    /// 处理 Handlebars 表达式结束标记
    fn lex_handlebars_close(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some('}') = state.peek() {
            // 检查 }}}} (raw block close)
            if source.get_char_at(start_pos + 1) == Some('}')
                && source.get_char_at(start_pos + 2) == Some('}')
                && source.get_char_at(start_pos + 3) == Some('}')
            {
                state.advance(4);
                state.add_token(HandlebarsSyntaxKind::CloseRawBlock, start_pos, state.get_position());
                return true;
            }
            // 检查 }}} (unescaped close)
            else if source.get_char_at(start_pos + 1) == Some('}') && source.get_char_at(start_pos + 2) == Some('}') {
                state.advance(3);
                state.add_token(HandlebarsSyntaxKind::CloseUnescaped, start_pos, state.get_position());
                return true;
            }
            // 检查 }} (normal close)
            else if source.get_char_at(start_pos + 1) == Some('}') {
                state.advance(2);
                state.add_token(HandlebarsSyntaxKind::Close, start_pos, state.get_position());
                return true;
            }
        }

        // 检查 --}} (comment block close)
        if let Some('-') = state.peek() {
            if source.get_char_at(start_pos + 1) == Some('-')
                && source.get_char_at(start_pos + 2) == Some('}')
                && source.get_char_at(start_pos + 3) == Some('}')
            {
                state.advance(4);
                state.add_token(HandlebarsSyntaxKind::CloseCommentBlock, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理注释内容
    fn lex_comment(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        // 这个方法处理注释内容，通常在 OpenComment 或 OpenCommentBlock 之后调用
        // 对于简单的 {{! comment }}，读取到 }} 为止
        // 对于 {{!-- comment --}}，读取到 --}} 为止

        // 这里简化处理，实际应该根据上下文来决定如何结束注释
        while let Some(ch) = state.peek() {
            if ch == '}' && source.get_char_at(state.get_position() + 1) == Some('}') {
                break;
            }
            if ch == '-'
                && source.get_char_at(state.get_position() + 1) == Some('-')
                && source.get_char_at(state.get_position() + 2) == Some('}')
                && source.get_char_at(state.get_position() + 3) == Some('}')
            {
                break;
            }
            state.advance(ch.len_utf8());
        }

        if state.get_position() > start_pos {
            state.add_token(HandlebarsSyntaxKind::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符串字面量
    fn lex_string(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(quote) = state.peek() {
            if quote == '"' || quote == '\'' {
                state.advance(1);
                let mut escaped = false;

                while let Some(ch) = state.peek() {
                    if escaped {
                        escaped = false;
                    }
                    else if ch == '\\' {
                        escaped = true;
                    }
                    else if ch == quote {
                        state.advance(1);
                        break;
                    }
                    state.advance(ch.len_utf8());
                }

                state.add_token(HandlebarsSyntaxKind::StringLiteral, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理数字字面量
    fn lex_number(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() || ch == '.' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                state.add_token(HandlebarsSyntaxKind::NumberLiteral, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理标识符和关键字
    fn lex_identifier(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' || ch == '@' {
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' || ch == '.' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 检查是否是布尔字面量
                let text = source.get_text_range(start_pos, state.get_position());
                let token_kind = match text {
                    "true" | "false" => HandlebarsSyntaxKind::BooleanLiteral,
                    _ => HandlebarsSyntaxKind::Identifier,
                };

                state.add_token(token_kind, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理操作符和分隔符
    fn lex_operator(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '.' => HandlebarsSyntaxKind::Dot,
                '/' => HandlebarsSyntaxKind::Slash,
                '#' => HandlebarsSyntaxKind::Hash,
                '@' => HandlebarsSyntaxKind::At,
                '|' => HandlebarsSyntaxKind::Pipe,
                '=' => HandlebarsSyntaxKind::Equal,
                '(' => HandlebarsSyntaxKind::LeftParen,
                ')' => HandlebarsSyntaxKind::RightParen,
                '[' => HandlebarsSyntaxKind::LeftBracket,
                ']' => HandlebarsSyntaxKind::RightBracket,
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

    /// 处理内容（HTML/文本）
    fn lex_content(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            // 遇到 { 时停止，可能是 Handlebars 表达式的开始
            if ch == '{' {
                break;
            }
            // 跳过换行和空白，它们由专门的方法处理
            if ch == '\n' || ch == '\r' || ch == ' ' || ch == '\t' {
                break;
            }
            state.advance(ch.len_utf8());
        }

        if state.get_position() > start_pos {
            state.add_token(HandlebarsSyntaxKind::Content, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<HandlebarsLanguage> for HandlebarsLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<HandlebarsSyntaxKind> {
        let mut state = State::new(source);

        while state.not_at_end() {
            // 尝试各种词法规则
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_handlebars_open(&mut state, source) {
                continue;
            }

            if self.lex_handlebars_close(&mut state, source) {
                continue;
            }

            if self.lex_string(&mut state) {
                continue;
            }

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_identifier(&mut state, source) {
                continue;
            }

            if self.lex_operator(&mut state) {
                continue;
            }

            if self.lex_content(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(HandlebarsSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(HandlebarsSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
