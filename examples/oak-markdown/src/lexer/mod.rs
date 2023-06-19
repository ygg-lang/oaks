use crate::{kind::MarkdownSyntaxKind, language::MarkdownLanguage};
use oak_core::{IncrementalCache, Lexer, LexerState, lexer::LexOutput, source::Source};

type State<S> = LexerState<S, MarkdownLanguage>;

#[derive(Clone, Debug)]
pub struct MarkdownLexer<'config> {
    config: &'config MarkdownLanguage,
}

impl<'config> MarkdownLexer<'config> {
    pub fn new(config: &'config MarkdownLanguage) -> Self {
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
            state.add_token(MarkdownSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(MarkdownSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(MarkdownSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理标题
    fn lex_heading<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        // 检查是否在行首
        if start_pos > 0 {
            if let Some(prev_char) = state.get_char_at(start_pos - 1) {
                if prev_char != '\n' && prev_char != '\r' {
                    return false;
                }
            }
        }

        if let Some('#') = state.peek() {
            let mut level = 0;
            let mut pos = start_pos;

            // 计算 # 的数
            while let Some('#') = state.get_char_at(pos) {
                level += 1;
                pos += 1;
                if level > 6 {
                    return false; // 超过6级标题，不是有效标题
                }
            }

            // 检# 后面是否有空
            if let Some(ch) = state.get_char_at(pos) {
                if ch != ' ' && ch != '\t' && ch != '\n' && ch != '\r' {
                    return false;
                }
            }

            state.advance(level);

            let heading_kind = match level {
                1 => MarkdownSyntaxKind::Heading1,
                2 => MarkdownSyntaxKind::Heading2,
                3 => MarkdownSyntaxKind::Heading3,
                4 => MarkdownSyntaxKind::Heading4,
                5 => MarkdownSyntaxKind::Heading5,
                6 => MarkdownSyntaxKind::Heading6,
                _ => return false,
            };

            state.add_token(heading_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理内联代码
    fn lex_inline_code<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('`') = state.peek() {
            state.advance(1);
            let mut found_end = false;

            while let Some(ch) = state.peek() {
                if ch == '`' {
                    state.advance(1);
                    found_end = true;
                    break;
                }
                else if ch == '\n' || ch == '\r' {
                    break; // 内联代码不能跨行
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            if found_end {
                state.add_token(MarkdownSyntaxKind::InlineCode, start_pos, state.get_position());
                true
            }
            else {
                // 回退到开始位
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// 处理代码
    fn lex_code_block<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        // 检查是否在行首
        if start_pos > 0 {
            if let Some(prev_char) = state.get_char_at(start_pos - 1) {
                if prev_char != '\n' && prev_char != '\r' {
                    return false;
                }
            }
        }

        // 检查是否是 ``` ~~~
        let fence_char = if let Some('`') = state.peek() {
            '`'
        }
        else if let Some('~') = state.peek() {
            '~'
        }
        else {
            return false;
        };

        let mut fence_count = 0;
        let mut pos = start_pos;

        // 计算围栏字符数量
        while let Some(ch) = state.get_char_at(pos) {
            if ch == fence_char {
                fence_count += 1;
                pos += 1;
            }
            else {
                break;
            }
        }

        if fence_count < 3 {
            return false; // 至少需个围栏字
        }

        state.advance(fence_count);
        state.add_token(MarkdownSyntaxKind::CodeFence, start_pos, state.get_position());

        // 处理语言标识
        let lang_start = state.get_position();
        while let Some(ch) = state.peek() {
            if ch == '\n' || ch == '\r' {
                break;
            }
            else if ch != ' ' && ch != '\t' {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        if state.get_position() > lang_start {
            state.add_token(MarkdownSyntaxKind::CodeLanguage, lang_start, state.get_position());
        }

        true
    }

    /// 处理强调和加
    fn lex_emphasis<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        let marker_char = if let Some('*') = state.peek() {
            '*'
        }
        else if let Some('_') = state.peek() {
            '_'
        }
        else {
            return false;
        };

        let mut marker_count = 0;
        let mut pos = start_pos;

        // 计算标记字符数量
        while let Some(ch) = state.get_char_at(pos) {
            if ch == marker_char {
                marker_count += 1;
                pos += 1;
            }
            else {
                break;
            }
        }

        if marker_count == 0 {
            return false;
        }

        state.advance(marker_count);

        let token_kind = if marker_count >= 2 { MarkdownSyntaxKind::Strong } else { MarkdownSyntaxKind::Emphasis };

        state.add_token(token_kind, start_pos, state.get_position());
        true
    }

    /// 处理删除
    fn lex_strikethrough<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('~') = state.peek() {
            if let Some('~') = state.get_char_at(start_pos + 1) {
                state.advance(2);
                state.add_token(MarkdownSyntaxKind::Strikethrough, start_pos, state.get_position());
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

    /// 处理链接和图
    fn lex_link_or_image<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        // 检查是否是图片 ![
        let is_image = if let Some('!') = state.peek() {
            state.advance(1);
            true
        }
        else {
            false
        };

        if let Some('[') = state.peek() {
            state.advance(1);

            let token_kind = if is_image { MarkdownSyntaxKind::Image } else { MarkdownSyntaxKind::Link };

            state.add_token(token_kind, start_pos, state.get_position());
            true
        }
        else {
            if is_image {
                // 回退感叹
                state.set_position(start_pos);
            }
            false
        }
    }

    /// 处理列表标记
    fn lex_list_marker<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        // 检查是否在行首或前面只有空
        let mut check_pos = start_pos;
        while check_pos > 0 {
            check_pos -= 1;
            if let Some(ch) = state.get_char_at(check_pos) {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                else if ch != ' ' && ch != '\t' {
                    return false; // 前面有非空白字符
                }
            }
        }

        if let Some(ch) = state.peek() {
            match ch {
                '-' | '*' | '+' => {
                    // 无序列表
                    state.advance(1);
                    if let Some(next_ch) = state.peek() {
                        if next_ch == ' ' || next_ch == '\t' {
                            state.add_token(MarkdownSyntaxKind::ListMarker, start_pos, state.get_position());
                            return true;
                        }
                    }
                    state.set_position(start_pos);
                    false
                }
                '0'..='9' => {
                    // 有序列表
                    while let Some(digit) = state.peek() {
                        if digit.is_ascii_digit() {
                            state.advance(1);
                        }
                        else {
                            break;
                        }
                    }

                    if let Some('.') = state.peek() {
                        state.advance(1);
                        if let Some(next_ch) = state.peek() {
                            if next_ch == ' ' || next_ch == '\t' {
                                state.add_token(MarkdownSyntaxKind::ListMarker, start_pos, state.get_position());
                                return true;
                            }
                        }
                    }

                    state.set_position(start_pos);
                    false
                }
                _ => false,
            }
        }
        else {
            false
        }
    }

    /// 处理任务列表
    fn lex_task_marker<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some('[') = state.peek() {
            state.advance(1);
            if let Some(ch) = state.peek() {
                if ch == ' ' || ch == 'x' || ch == 'X' {
                    state.advance(1);
                    if let Some(']') = state.peek() {
                        state.advance(1);
                        state.add_token(MarkdownSyntaxKind::TaskMarker, start_pos, state.get_position());
                        return true;
                    }
                }
            }
            state.set_position(start_pos);
        }
        false
    }

    /// 处理引用
    fn lex_blockquote<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        // 检查是否在行首或前面只有空
        let mut check_pos = start_pos;
        while check_pos > 0 {
            check_pos -= 1;
            if let Some(ch) = state.get_char_at(check_pos) {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                else if ch != ' ' && ch != '\t' {
                    return false;
                }
            }
        }

        if let Some('>') = state.peek() {
            state.advance(1);
            state.add_token(MarkdownSyntaxKind::BlockquoteMarker, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理水平分隔
    fn lex_horizontal_rule<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        // 检查是否在行首或前面只有空
        let mut check_pos = start_pos;
        while check_pos > 0 {
            check_pos -= 1;
            if let Some(ch) = state.get_char_at(check_pos) {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                else if ch != ' ' && ch != '\t' {
                    return false;
                }
            }
        }

        if let Some(ch) = state.peek() {
            if ch == '-' || ch == '*' || ch == '_' {
                let rule_char = ch;
                let mut count = 0;
                let mut pos = start_pos;

                // 计算连续的分隔符数量
                while let Some(current_ch) = state.get_char_at(pos) {
                    if current_ch == rule_char {
                        count += 1;
                        pos += 1;
                    }
                    else if current_ch == ' ' || current_ch == '\t' {
                        pos += 1; // 允许空格
                    }
                    else {
                        break;
                    }
                }

                if count >= 3 {
                    // 检查到行尾
                    while let Some(current_ch) = state.get_char_at(pos) {
                        if current_ch == '\n' || current_ch == '\r' {
                            break;
                        }
                        else if current_ch == ' ' || current_ch == '\t' {
                            pos += 1;
                        }
                        else {
                            return false; // 行尾有其他字
                        }
                    }

                    state.set_position(pos);
                    state.add_token(MarkdownSyntaxKind::HorizontalRule, start_pos, state.get_position());
                    return true;
                }
            }
        }
        false
    }

    /// 处理特殊字符
    fn lex_special_char<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '[' => MarkdownSyntaxKind::LeftBracket,
                ']' => MarkdownSyntaxKind::RightBracket,
                '(' => MarkdownSyntaxKind::LeftParen,
                ')' => MarkdownSyntaxKind::RightParen,
                '<' => MarkdownSyntaxKind::LeftAngle,
                '>' => MarkdownSyntaxKind::RightAngle,
                '*' => MarkdownSyntaxKind::Asterisk,
                '_' => MarkdownSyntaxKind::Underscore,
                '`' => MarkdownSyntaxKind::Backtick,
                '~' => MarkdownSyntaxKind::Tilde,
                '#' => MarkdownSyntaxKind::Hash,
                '|' => MarkdownSyntaxKind::Pipe,
                '-' => MarkdownSyntaxKind::Dash,
                '+' => MarkdownSyntaxKind::Plus,
                '.' => MarkdownSyntaxKind::Dot,
                ':' => MarkdownSyntaxKind::Colon,
                '!' => MarkdownSyntaxKind::Exclamation,
                '\\' => MarkdownSyntaxKind::Escape,
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

    /// 处理普通文
    fn lex_text<S: Source>(&self, state: &mut State<S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            // 遇到特殊字符时停
            match ch {
                ' ' | '\t' | '\n' | '\r' | '#' | '*' | '_' | '`' | '~' | '[' | ']' | '(' | ')' | '<' | '>' | '|' | '-'
                | '+' | '.' | ':' | '!' | '\\' => break,
                _ => {
                    state.advance(ch.len_utf8());
                }
            }
        }

        if state.get_position() > start_pos {
            state.add_token(MarkdownSyntaxKind::Text, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<MarkdownLanguage> for MarkdownLexer<'config> {
    fn lex_incremental(
        &self,
        source: impl Source,
        changed: usize,
        cache: IncrementalCache<MarkdownLanguage>,
    ) -> LexOutput<MarkdownLanguage> {
        let mut state = LexerState::new_with_cache(source, changed, cache);

        while state.not_at_end() {
            // 尝试各种词法规则
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_heading(&mut state) {
                continue;
            }

            if self.lex_code_block(&mut state) {
                continue;
            }

            if self.lex_inline_code(&mut state) {
                continue;
            }

            if self.lex_strikethrough(&mut state) {
                continue;
            }

            if self.lex_emphasis(&mut state) {
                continue;
            }

            if self.lex_link_or_image(&mut state) {
                continue;
            }

            if self.lex_task_marker(&mut state) {
                continue;
            }

            if self.lex_list_marker(&mut state) {
                continue;
            }

            if self.lex_blockquote(&mut state) {
                continue;
            }

            if self.lex_horizontal_rule(&mut state) {
                continue;
            }

            if self.lex_special_char(&mut state) {
                continue;
            }

            if self.lex_text(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(MarkdownSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF token
        let eof_pos = state.get_position();
        state.add_token(MarkdownSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish(Ok(()))
    }
}

impl<'config> MarkdownLexer<'config> {
    fn lex_internal<S: Source>(&self, source: S) -> LexOutput<MarkdownLanguage> {
        let mut state = State::new(source);

        while state.not_at_end() {
            // 尝试各种词法规则
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_heading(&mut state) {
                continue;
            }

            if self.lex_code_block(&mut state) {
                continue;
            }

            if self.lex_inline_code(&mut state) {
                continue;
            }

            if self.lex_strikethrough(&mut state) {
                continue;
            }

            if self.lex_emphasis(&mut state) {
                continue;
            }

            if self.lex_link_or_image(&mut state) {
                continue;
            }

            if self.lex_task_marker(&mut state) {
                continue;
            }

            if self.lex_list_marker(&mut state) {
                continue;
            }

            if self.lex_blockquote(&mut state) {
                continue;
            }

            if self.lex_horizontal_rule(&mut state) {
                continue;
            }

            if self.lex_special_char(&mut state) {
                continue;
            }

            if self.lex_text(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(MarkdownSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(MarkdownSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish(Ok(()))
    }
}
