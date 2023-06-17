use crate::{kind::YamlSyntaxKind, language::YamlLanguage};
use oak_core::{
    SourceText,
    lexer::{LexOutput, Lexer, LexerState},
};

type State<'config> = LexerState<'config, YamlSyntaxKind>;

pub struct YamlLexer<'config> {
    config: &'config YamlLanguage,
}

impl<'config> YamlLexer<'config> {
    pub fn new(config: &'config YamlLanguage) -> Self {
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
            state.add_token(YamlSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(YamlSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(YamlSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理 YAML 注释 # ...
    fn lex_comment(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('#') = state.peek() {
            state.advance(1);

            // 读取到行

            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(YamlSyntaxKind::Comment, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理文档分隔符
    fn lex_document_separator(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('-') = state.peek() {
            if let Some('-') = state.peek_at(1) {
                if let Some('-') = state.peek_at(2) {
                    state.advance(3);
                    state.add_token(YamlSyntaxKind::DocumentStart, start_pos, state.get_position());
                    return true;
                }
            }
        }
        else if let Some('.') = state.peek() {
            if let Some('.') = state.peek_at(1) {
                if let Some('.') = state.peek_at(2) {
                    state.advance(3);
                    state.add_token(YamlSyntaxKind::DocumentEnd, start_pos, state.get_position());
                    return true;
                }
            }
        }

        false
    }

    /// 处理 YAML 指令 %YAML, %TAG
    fn lex_directive(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('%') = state.peek() {
            state.advance(1);

            // 读取指令名称
            while let Some(ch) = state.peek() {
                if ch.is_ascii_alphabetic() || ch.is_ascii_digit() {
                    state.advance(ch.len_utf8());
                }
                else {
                    break;
                }
            }

            // 读取指令参数直到行尾
            while let Some(ch) = state.peek() {
                if ch == '\n' || ch == '\r' {
                    break;
                }
                state.advance(ch.len_utf8());
            }

            state.add_token(YamlSyntaxKind::Directive, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理字符串字面量（单引号和双引号）
    fn lex_string(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(quote_char) = state.peek() {
            if quote_char == '"' || quote_char == '\'' {
                state.advance(1); // 跳过开始引

                while let Some(ch) = state.peek() {
                    if ch == quote_char {
                        state.advance(1); // 跳过结束引号
                        state.add_token(YamlSyntaxKind::StringLiteral, start_pos, state.get_position());
                        return true;
                    }
                    else if ch == '\\' && quote_char == '"' {
                        // 处理转义字符（仅在双引号字符串中

                        state.advance(1);
                        if let Some(_) = state.peek() {
                            state.advance(1);
                        }
                    }
                    else if ch == '\n' || ch == '\r' {
                        // YAML 字符串可以跨

                        state.advance(ch.len_utf8());
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                // 未闭合的字符

                state.add_token(YamlSyntaxKind::Error, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理数字字面

    fn lex_number(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() || (ch == '-' && state.peek_at(1).map_or(false, |c| c.is_ascii_digit())) {
                // 处理负号
                if ch == '-' {
                    state.advance(1);
                }

                // 读取整数部分
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                // 处理小数点
                if let Some('.') = state.peek() {
                    if let Some(next_ch) = state.peek_at(1) {
                        if next_ch.is_ascii_digit() {
                            state.advance(1); // 跳过小数

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

                // 处理科学计数

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

                state.add_token(YamlSyntaxKind::NumberLiteral, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理布尔值和 null
    fn lex_boolean_null(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        // 检查布尔值和 null 关键

        let keywords = [
            ("true", YamlSyntaxKind::BooleanLiteral),
            ("false", YamlSyntaxKind::BooleanLiteral),
            ("True", YamlSyntaxKind::BooleanLiteral),
            ("False", YamlSyntaxKind::BooleanLiteral),
            ("TRUE", YamlSyntaxKind::BooleanLiteral),
            ("FALSE", YamlSyntaxKind::BooleanLiteral),
            ("yes", YamlSyntaxKind::BooleanLiteral),
            ("no", YamlSyntaxKind::BooleanLiteral),
            ("Yes", YamlSyntaxKind::BooleanLiteral),
            ("No", YamlSyntaxKind::BooleanLiteral),
            ("YES", YamlSyntaxKind::BooleanLiteral),
            ("NO", YamlSyntaxKind::BooleanLiteral),
            ("on", YamlSyntaxKind::BooleanLiteral),
            ("off", YamlSyntaxKind::BooleanLiteral),
            ("On", YamlSyntaxKind::BooleanLiteral),
            ("Off", YamlSyntaxKind::BooleanLiteral),
            ("ON", YamlSyntaxKind::BooleanLiteral),
            ("OFF", YamlSyntaxKind::BooleanLiteral),
            ("null", YamlSyntaxKind::NullLiteral),
            ("Null", YamlSyntaxKind::NullLiteral),
            ("NULL", YamlSyntaxKind::NullLiteral),
            ("~", YamlSyntaxKind::NullLiteral),
        ];

        for (keyword, token_kind) in &keywords {
            if self.match_keyword(state, keyword) {
                state.advance(keyword.len());
                state.add_token(*token_kind, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 检查是否匹配关键字
    fn match_keyword(&self, state: &State, keyword: &str) -> bool {
        for (i, expected_ch) in keyword.chars().enumerate() {
            if let Some(actual_ch) = state.peek_at(i) {
                if actual_ch != expected_ch {
                    return false;
                }
            }
            else {
                return false;
            }
        }

        // 确保关键字后面不是字母数字字

        if let Some(next_ch) = state.peek_at(keyword.len()) {
            if next_ch.is_ascii_alphanumeric() || next_ch == '_' {
                return false;
            }
        }

        true
    }

    /// 处理标签 !tag
    fn lex_tag(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('!') = state.peek() {
            state.advance(1);

            // 读取标签名称
            while let Some(ch) = state.peek() {
                if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' || ch == '.' || ch == '/' {
                    state.advance(ch.len_utf8());
                }
                else {
                    break;
                }
            }

            state.add_token(YamlSyntaxKind::Tag, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理锚点 &anchor
    fn lex_anchor(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('&') = state.peek() {
            state.advance(1);

            // 读取锚点名称
            while let Some(ch) = state.peek() {
                if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                    state.advance(ch.len_utf8());
                }
                else {
                    break;
                }
            }

            state.add_token(YamlSyntaxKind::Anchor, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理别名 *alias
    fn lex_alias(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('*') = state.peek() {
            state.advance(1);

            // 读取别名名称
            while let Some(ch) = state.peek() {
                if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                    state.advance(ch.len_utf8());
                }
                else {
                    break;
                }
            }

            state.add_token(YamlSyntaxKind::Alias, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理块标量指示符 | >
    fn lex_block_scalar(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch == '|' || ch == '>' {
                state.advance(1);

                // 处理块标量修饰符（如 |-, |+, >-, >+

                if let Some(modifier) = state.peek() {
                    if modifier == '-' || modifier == '+' {
                        state.advance(1);
                    }
                }

                let token_kind = if ch == '|' { YamlSyntaxKind::LiteralScalar } else { YamlSyntaxKind::FoldedScalar };

                state.add_token(token_kind, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理标识

    fn lex_identifier(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                state.add_token(YamlSyntaxKind::Identifier, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理标点符号和操作符
    fn lex_punctuation(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '-' => {
                    // 检查是否是块序列条

                    if let Some(next_ch) = state.peek_at(1) {
                        if next_ch == ' ' || next_ch == '\t' || next_ch == '\n' || next_ch == '\r' {
                            state.advance(1);
                            YamlSyntaxKind::BlockSequenceEntry
                        }
                        else {
                            return false; // 让其他处理器处理
                        }
                    }
                    else {
                        state.advance(1);
                        YamlSyntaxKind::BlockSequenceEntry
                    }
                }
                '[' => {
                    state.advance(1);
                    YamlSyntaxKind::FlowSequenceStart
                }
                ']' => {
                    state.advance(1);
                    YamlSyntaxKind::FlowSequenceEnd
                }
                '{' => {
                    state.advance(1);
                    YamlSyntaxKind::FlowMappingStart
                }
                '}' => {
                    state.advance(1);
                    YamlSyntaxKind::FlowMappingEnd
                }
                '?' => {
                    state.advance(1);
                    YamlSyntaxKind::KeyIndicator
                }
                ':' => {
                    state.advance(1);
                    YamlSyntaxKind::ValueIndicator
                }
                ',' => {
                    state.advance(1);
                    YamlSyntaxKind::Comma
                }
                '|' => {
                    state.advance(1);
                    YamlSyntaxKind::Pipe
                }
                '>' => {
                    state.advance(1);
                    YamlSyntaxKind::GreaterThan
                }
                '\'' => {
                    state.advance(1);
                    YamlSyntaxKind::SingleQuote
                }
                '"' => {
                    state.advance(1);
                    YamlSyntaxKind::DoubleQuote
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

    /// 处理普通文本内

    fn lex_text(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            // 遇到特殊字符时停

            match ch {
                ' ' | '\t' | '\n' | '\r' | '#' | '-' | '[' | ']' | '{' | '}' | '?' | ':' | ',' | '|' | '>' | '\'' | '"'
                | '!' | '&' | '*' | '%' => break,
                _ => {
                    state.advance(ch.len_utf8());
                }
            }
        }

        if state.get_position() > start_pos {
            state.add_token(YamlSyntaxKind::Text, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<YamlLanguage> for YamlLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<YamlSyntaxKind> {
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

            if self.lex_document_separator(&mut state) {
                continue;
            }

            if self.lex_directive(&mut state) {
                continue;
            }

            if self.lex_string(&mut state) {
                continue;
            }

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_boolean_null(&mut state) {
                continue;
            }

            if self.lex_tag(&mut state) {
                continue;
            }

            if self.lex_anchor(&mut state) {
                continue;
            }

            if self.lex_alias(&mut state) {
                continue;
            }

            if self.lex_block_scalar(&mut state) {
                continue;
            }

            if self.lex_identifier(&mut state) {
                continue;
            }

            if self.lex_punctuation(&mut state) {
                continue;
            }

            if self.lex_text(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(YamlSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(YamlSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
