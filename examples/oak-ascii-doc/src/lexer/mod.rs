use crate::{kind::AsciiDocSyntaxKind, language::AsciiDocLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, AsciiDocLanguage>;

pub struct AsciiDocLexer<'config> {
    config: &'config AsciiDocLanguage,
}

impl<'config> AsciiDocLexer<'config> {
    pub fn new(config: &'config AsciiDocLanguage) -> Self {
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
            state.add_token(AsciiDocSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(AsciiDocSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(AsciiDocSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理标题
    fn lex_header(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('=') = state.peek() {
            let mut level = 0;

            // 计算标题级别
            while let Some('=') = state.peek() {
                state.advance(1);
                level += 1;
                if level > 6 {
                    break;
                }
            }

            // 检查后面是否有空格
            if let Some(' ') = state.peek() {
                state.advance(1);

                // 读取标题文本
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }

                let token_kind = match level {
                    1 => AsciiDocSyntaxKind::Header1,
                    2 => AsciiDocSyntaxKind::Header2,
                    3 => AsciiDocSyntaxKind::Header3,
                    4 => AsciiDocSyntaxKind::Header4,
                    5 => AsciiDocSyntaxKind::Header5,
                    _ => AsciiDocSyntaxKind::Header6,
                };

                state.add_token(token_kind, start_pos, state.get_position());
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

    /// 处理粗体文本
    fn lex_bold(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('*') = state.peek() {
            if let Some('*') = state.peek_next_n(1) {
                state.advance(2);

                // 查找结束标记
                while let Some(ch) = state.peek() {
                    if ch == '*' {
                        if let Some('*') = state.peek_next_n(1) {
                            state.advance(2);
                            break;
                        }
                    }
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }

                state.add_token(AsciiDocSyntaxKind::Bold, start_pos, state.get_position());
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

    /// 处理斜体文本
    fn lex_italic(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('_') = state.peek() {
            state.advance(1);

            // 查找结束标记
            while let Some(ch) = state.peek() {
                if ch == '_' {
                    state.advance(1);
                    break;
                }
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(AsciiDocSyntaxKind::Italic, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理等宽字体
    fn lex_monospace(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('`') = state.peek() {
            state.advance(1);

            // 查找结束标记
            while let Some(ch) = state.peek() {
                if ch == '`' {
                    state.advance(1);
                    break;
                }
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(AsciiDocSyntaxKind::Monospace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理代码
    fn lex_code_block(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('-') = state.peek() {
            let mut dash_count = 0;
            let temp_pos = state.get_position();

            // 检查是否有至少4个连续的破折
            while let Some('-') = state.peek() {
                state.advance(1);
                dash_count += 1;
            }

            if dash_count >= 4 {
                // 跳过到行
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }

                // 读取代码块内容直到结束标
                while state.not_at_end() {
                    if let Some('-') = state.peek() {
                        let block_end_pos = state.get_position();
                        let mut end_dash_count = 0;

                        while let Some('-') = state.peek() {
                            state.advance(1);
                            end_dash_count += 1;
                        }

                        if end_dash_count >= 4 {
                            break;
                        }
                        else {
                            state.set_position(block_end_pos);
                            if let Some(ch) = state.peek() {
                                state.advance(ch.len_utf8());
                            }
                        }
                    }
                    else if let Some(ch) = state.peek() {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(AsciiDocSyntaxKind::CodeBlock, start_pos, state.get_position());
                true
            }
            else {
                state.set_position(temp_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// 处理链接
    fn lex_link(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('h') = state.peek() {
            // 检查是否是 http:// https://
            let current_pos = state.get_position();
            let mut is_link = false;

            if state.peek_next_n(1) == Some('t') && state.peek_next_n(2) == Some('t') && state.peek_next_n(3) == Some('p') {
                if state.peek_next_n(4) == Some(':') && state.peek_next_n(5) == Some('/') && state.peek_next_n(6) == Some('/') {
                    state.advance(7);
                    is_link = true;
                }
                else if state.peek_next_n(4) == Some('s')
                    && state.peek_next_n(5) == Some(':')
                    && state.peek_next_n(6) == Some('/')
                    && state.peek_next_n(7) == Some('/')
                {
                    state.advance(8);
                    is_link = true;
                }
            }

            if is_link {
                // 读取URL的其余部
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_whitespace() || ch == '[' || ch == ']' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }

                state.add_token(AsciiDocSyntaxKind::Link, start_pos, state.get_position());
                true
            }
            else {
                state.set_position(current_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// 处理列表
    fn lex_list_item(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('*') = state.peek() {
            state.advance(1);
            if let Some(' ') = state.peek() {
                state.advance(1);

                // 读取列表项内
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }

                state.add_token(AsciiDocSyntaxKind::UnorderedListItem, start_pos, state.get_position());
                true
            }
            else {
                state.set_position(start_pos);
                false
            }
        }
        else if let Some('.') = state.peek() {
            state.advance(1);
            if let Some(' ') = state.peek() {
                state.advance(1);

                // 读取列表项内
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }

                state.add_token(AsciiDocSyntaxKind::OrderedListItem, start_pos, state.get_position());
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

    /// 处理注释
    fn lex_comment(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            if let Some('/') = state.peek_next_n(1) {
                state.advance(2);

                // 读取到行
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }

                state.add_token(AsciiDocSyntaxKind::Comment, start_pos, state.get_position());
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

    /// 处理分隔
    fn lex_delimiter(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch == '[' || ch == ']' || ch == '(' || ch == ')' || ch == '{' || ch == '}' || ch == '<' || ch == '>' {
                state.advance(1);
                state.add_token(AsciiDocSyntaxKind::Delimiter, start_pos, state.get_position());
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

    /// 处理普通文
    fn lex_text(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if !ch.is_ascii_whitespace()
                && !['=', '*', '_', '`', '-', '[', ']', '(', ')', '{', '}', '<', '>', '/', '.'].contains(&ch)
            {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if !ch.is_ascii_whitespace()
                        && !['=', '*', '_', '`', '-', '[', ']', '(', ')', '{', '}', '<', '>', '/', '.'].contains(&ch)
                    {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                state.add_token(AsciiDocSyntaxKind::Text, start_pos, state.get_position());
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

impl<'config> Lexer<AsciiDocLanguage> for AsciiDocLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<AsciiDocSyntaxKind> {
        let mut state = LexerState::new(source);

        while state.not_at_end() {
            // 尝试各种词法规则
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_header(&mut state) {
                continue;
            }

            if self.lex_code_block(&mut state) {
                continue;
            }

            if self.lex_comment(&mut state) {
                continue;
            }

            if self.lex_bold(&mut state) {
                continue;
            }

            if self.lex_italic(&mut state) {
                continue;
            }

            if self.lex_monospace(&mut state) {
                continue;
            }

            if self.lex_link(&mut state) {
                continue;
            }

            if self.lex_list_item(&mut state) {
                continue;
            }

            if self.lex_delimiter(&mut state) {
                continue;
            }

            if self.lex_text(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(AsciiDocSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(AsciiDocSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
