use crate::{kind::NoteSyntaxKind, language::NotedownLanguage};
use oak_core::{Lexer, LexerCache, LexerState, lexer::LexOutput, source::Source};

type State<'a, S> = LexerState<'a, S, NotedownLanguage>;

#[derive(Clone, Debug)]
pub struct NotedownLexer<'config> {
    _config: &'config NotedownLanguage,
}

impl<'config> NotedownLexer<'config> {
    pub fn new(config: &'config NotedownLanguage) -> Self {
        Self { _config: config }
    }
}

impl<'config> Lexer<NotedownLanguage> for NotedownLexer<'config> {
    fn lex<'a, S: Source + ?Sized>(&self, source: &'a S, _edits: &[oak_core::TextEdit], cache: &'a mut impl LexerCache<NotedownLanguage>) -> LexOutput<NotedownLanguage> {
        let mut state = LexerState::new(source);
        let result = self.run(&mut state);
        if result.is_ok() {
            state.add_eof();
        }
        state.finish_with_cache(result, cache)
    }
}

impl<'config> NotedownLexer<'config> {
    fn run<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> Result<(), oak_core::OakError> {
        while state.not_at_end() {
            // 尝试各种词法规则
            if self.skip_whitespace(state) {
                continue;
            }

            if self.lex_newline(state) {
                continue;
            }

            if self.lex_heading(state) {
                continue;
            }

            if self.lex_code_block(state) {
                continue;
            }

            if self.lex_inline_code(state) {
                continue;
            }

            if self.lex_strikethrough(state) {
                continue;
            }

            if self.lex_emphasis(state) {
                continue;
            }

            if self.lex_link_or_image(state) {
                continue;
            }

            if self.lex_task_marker(state) {
                continue;
            }

            if self.lex_list_marker(state) {
                continue;
            }

            if self.lex_blockquote(state) {
                continue;
            }

            if self.lex_horizontal_rule(state) {
                continue;
            }

            if self.lex_special_char(state) {
                continue;
            }

            if self.lex_text(state) {
                continue;
            }

            // 如果没有匹配任何规则，跳过当前字符
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
            }
        }
        Ok(())
    }

    /// 跳过空白字符
    fn skip_whitespace<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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
            state.add_token(NoteSyntaxKind::Whitespace, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理换行
    fn lex_newline<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('\n') = state.peek() {
            state.advance(1);
            state.add_token(NoteSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(NoteSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理标题
    fn lex_heading<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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
                1 => NoteSyntaxKind::Heading1,
                2 => NoteSyntaxKind::Heading2,
                3 => NoteSyntaxKind::Heading3,
                4 => NoteSyntaxKind::Heading4,
                5 => NoteSyntaxKind::Heading5,
                6 => NoteSyntaxKind::Heading6,
                _ => return false,
            };

            state.add_token(heading_kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理行内代码
    fn lex_inline_code<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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
                state.add_token(NoteSyntaxKind::InlineCode, start_pos, state.get_position());
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

    /// 处理代码块
    fn lex_code_block<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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
        state.add_token(NoteSyntaxKind::CodeFence, start_pos, state.get_position());

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
            state.add_token(NoteSyntaxKind::CodeLanguage, lang_start, state.get_position());
        }

        true
    }

    /// 处理加粗和倾斜
    fn lex_emphasis<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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

        let token_kind = if marker_count >= 2 { NoteSyntaxKind::Strong } else { NoteSyntaxKind::Emphasis };

        state.add_token(token_kind, start_pos, state.get_position());
        true
    }

    /// 处理删除
    fn lex_strikethrough<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('~') = state.peek() {
            if let Some('~') = state.get_char_at(start_pos + 1) {
                state.advance(2);
                state.add_token(NoteSyntaxKind::Strikethrough, start_pos, state.get_position());
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

    /// 处理链接和图片
    fn lex_link_or_image<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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

            let token_kind = if is_image { NoteSyntaxKind::Image } else { NoteSyntaxKind::Link };

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
    fn lex_list_marker<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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
                            state.add_token(NoteSyntaxKind::ListMarker, start_pos, state.get_position());
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
                                state.add_token(NoteSyntaxKind::ListMarker, start_pos, state.get_position());
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
    /// 处理任务列表标记
    fn lex_task_marker<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some('[') = state.peek() {
            state.advance(1);
            if let Some(ch) = state.peek() {
                if ch == ' ' || ch == 'x' || ch == 'X' {
                    state.advance(1);
                    if let Some(']') = state.peek() {
                        state.advance(1);
                        state.add_token(NoteSyntaxKind::TaskMarker, start_pos, state.get_position());
                        return true;
                    }
                }
            }
            state.set_position(start_pos);
        }
        false
    }

    /// 处理引用
    fn lex_blockquote<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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
            state.add_token(NoteSyntaxKind::BlockquoteMarker, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理水平分隔线
    fn lex_horizontal_rule<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
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
                    state.add_token(NoteSyntaxKind::HorizontalRule, start_pos, state.get_position());
                    return true;
                }
            }
        }
        false
    }

    /// 处理特殊字符
    fn lex_special_char<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '[' => NoteSyntaxKind::LeftBracket,
                ']' => NoteSyntaxKind::RightBracket,
                '(' => NoteSyntaxKind::LeftParen,
                ')' => NoteSyntaxKind::RightParen,
                '<' => NoteSyntaxKind::LeftAngle,
                '>' => NoteSyntaxKind::RightAngle,
                '*' => NoteSyntaxKind::Asterisk,
                '_' => NoteSyntaxKind::Underscore,
                '`' => NoteSyntaxKind::Backtick,
                '~' => NoteSyntaxKind::Tilde,
                '#' => NoteSyntaxKind::Hash,
                '|' => NoteSyntaxKind::Pipe,
                '-' => NoteSyntaxKind::Dash,
                '+' => NoteSyntaxKind::Plus,
                '.' => NoteSyntaxKind::Dot,
                ':' => NoteSyntaxKind::Colon,
                '!' => NoteSyntaxKind::Exclamation,
                '\\' => NoteSyntaxKind::Escape,
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

    /// 处理普通文本
    fn lex_text<'a, S: Source + ?Sized>(&self, state: &mut State<'a, S>) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            // 遇到特殊字符时停
            match ch {
                ' ' | '\t' | '\n' | '\r' | '#' | '*' | '_' | '`' | '~' | '[' | ']' | '(' | ')' | '<' | '>' | '|' | '-' | '+' | '.' | ':' | '!' | '\\' => break,
                _ => {
                    state.advance(ch.len_utf8());
                }
            }
        }

        if state.get_position() > start_pos {
            state.add_token(NoteSyntaxKind::Text, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}
