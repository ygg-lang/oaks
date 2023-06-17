use crate::{kind::ScalaSyntaxKind, language::ScalaLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, ScalaLanguage>;

pub struct ScalaLexer<'config> {
    config: &'config ScalaLanguage,
}

impl<'config> ScalaLexer<'config> {
    pub fn new(config: &'config ScalaLanguage) -> Self {
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
            state.add_token(ScalaSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(ScalaSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(ScalaSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理行注
    fn lex_line_comment(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            state.advance(1);
            if let Some('/') = state.peek() {
                state.advance(1);

                // 读取到行注释结束
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }

                state.add_token(ScalaSyntaxKind::LineComment, start_pos, state.get_position());
                true
            }
            else {
                // 回退，这不是注释
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// 处理块注
    fn lex_block_comment(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            state.advance(1);
            if let Some('*') = state.peek() {
                state.advance(1);

                let mut depth = 1;
                while depth > 0 && state.not_at_end() {
                    if let Some('/') = state.peek() {
                        state.advance(1);
                        if let Some('*') = state.peek() {
                            state.advance(1);
                            depth += 1;
                        }
                    }
                    else if let Some('*') = state.peek() {
                        state.advance(1);
                        if let Some('/') = state.peek() {
                            state.advance(1);
                            depth -= 1;
                        }
                    }
                    else {
                        let ch = state.peek().unwrap();
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(ScalaSyntaxKind::BlockComment, start_pos, state.get_position());
                true
            }
            else {
                // 回退，这不是注释
                state.set_position(start_pos);
                false
            }
        }
        else {
            false
        }
    }

    /// 处理标识符和关键字
    fn lex_identifier_or_keyword(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' || ch == '$' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' || ch == '$' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                let text = source.get_text_in(core::range::Range { start: start_pos, end: state.get_position() }).unwrap_or("");
                let token_kind = match text {
                    "abstract" => ScalaSyntaxKind::Abstract,
                    "case" => ScalaSyntaxKind::Case,
                    "catch" => ScalaSyntaxKind::Catch,
                    "class" => ScalaSyntaxKind::Class,
                    "def" => ScalaSyntaxKind::Def,
                    "do" => ScalaSyntaxKind::Do,
                    "else" => ScalaSyntaxKind::Else,
                    "extends" => ScalaSyntaxKind::Extends,
                    "false" => ScalaSyntaxKind::False,
                    "final" => ScalaSyntaxKind::Final,
                    "finally" => ScalaSyntaxKind::Finally,
                    "for" => ScalaSyntaxKind::For,
                    "forSome" => ScalaSyntaxKind::ForSome,
                    "if" => ScalaSyntaxKind::If,
                    "implicit" => ScalaSyntaxKind::Implicit,
                    "import" => ScalaSyntaxKind::Import,
                    "lazy" => ScalaSyntaxKind::Lazy,
                    "match" => ScalaSyntaxKind::Match,
                    "new" => ScalaSyntaxKind::New,
                    "null" => ScalaSyntaxKind::Null,
                    "object" => ScalaSyntaxKind::Object,
                    "override" => ScalaSyntaxKind::Override,
                    "package" => ScalaSyntaxKind::Package,
                    "private" => ScalaSyntaxKind::Private,
                    "protected" => ScalaSyntaxKind::Protected,
                    "return" => ScalaSyntaxKind::Return,
                    "sealed" => ScalaSyntaxKind::Sealed,
                    "super" => ScalaSyntaxKind::Super,
                    "this" => ScalaSyntaxKind::This,
                    "throw" => ScalaSyntaxKind::Throw,
                    "trait" => ScalaSyntaxKind::Trait,
                    "try" => ScalaSyntaxKind::Try,
                    "true" => ScalaSyntaxKind::True,
                    "type" => ScalaSyntaxKind::Type,
                    "val" => ScalaSyntaxKind::Val,
                    "var" => ScalaSyntaxKind::Var,
                    "while" => ScalaSyntaxKind::While,
                    "with" => ScalaSyntaxKind::With,
                    "yield" => ScalaSyntaxKind::Yield,
                    _ => ScalaSyntaxKind::Identifier,
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

    /// 处理数字字面
    fn lex_number(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);

                // 读取数字部分
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                let mut is_float = false;

                // 检查小数点
                if let Some('.') = state.peek() {
                    let next_pos = state.get_position() + 1;
                    if let Some(next_ch) = state.peek_next_n(next_pos) {
                        if next_ch.is_ascii_digit() {
                            is_float = true;
                            state.advance(1); // 跳过小数
                            // 读取小数部分
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

                // 检查指数部分
                if let Some(ch) = state.peek() {
                    if ch == 'e' || ch == 'E' {
                        is_float = true;
                        state.advance(1);

                        // 可选的符号
                        if let Some(sign) = state.peek() {
                            if sign == '+' || sign == '-' {
                                state.advance(1);
                            }
                        }

                        // 指数数字
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

                // 检查后缀
                if let Some(ch) = state.peek() {
                    if ch == 'f' || ch == 'F' || ch == 'd' || ch == 'D' {
                        is_float = true;
                        state.advance(1);
                    }
                    else if ch == 'l' || ch == 'L' {
                        state.advance(1);
                    }
                }

                let token_kind = if is_float { ScalaSyntaxKind::FloatLiteral } else { ScalaSyntaxKind::IntegerLiteral };

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

    /// 处理字符串字面量
    fn lex_string(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('"') = state.peek() {
            state.advance(1);

            while let Some(ch) = state.peek() {
                if ch == '"' {
                    state.advance(1);
                    state.add_token(ScalaSyntaxKind::StringLiteral, start_pos, state.get_position());
                    return true;
                }
                else if ch == '\\' {
                    state.advance(1);
                    if let Some(_) = state.peek() {
                        state.advance(1);
                    }
                }
                else if ch == '\n' || ch == '\r' {
                    // 字符串不能跨行（除非是多行字符串                    break;
                }
                else {
                    state.advance(ch.len_utf8());
                }
            }

            // 未找到结束引号，回退
            state.set_position(start_pos);
            false
        }
        else {
            false
        }
    }

    /// 处理字符字面
    fn lex_char(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('\'') = state.peek() {
            state.advance(1);

            if let Some(ch) = state.peek() {
                if ch == '\\' {
                    state.advance(1);
                    if let Some(_) = state.peek() {
                        state.advance(1);
                    }
                }
                else if ch != '\'' && ch != '\n' && ch != '\r' {
                    state.advance(ch.len_utf8());
                }

                if let Some('\'') = state.peek() {
                    state.advance(1);
                    state.add_token(ScalaSyntaxKind::CharLiteral, start_pos, state.get_position());
                    return true;
                }
            }

            // 无效的字符字面量，回退
            state.set_position(start_pos);
            false
        }
        else {
            false
        }
    }

    /// 处理操作
    fn lex_operator(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '+' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        ScalaSyntaxKind::PlusEq
                    }
                    else {
                        ScalaSyntaxKind::Plus
                    }
                }
                '-' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        ScalaSyntaxKind::MinusEq
                    }
                    else {
                        ScalaSyntaxKind::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        ScalaSyntaxKind::StarEq
                    }
                    else {
                        ScalaSyntaxKind::Star
                    }
                }
                '/' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        ScalaSyntaxKind::SlashEq
                    }
                    else {
                        ScalaSyntaxKind::Slash
                    }
                }
                '%' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        ScalaSyntaxKind::PercentEq
                    }
                    else {
                        ScalaSyntaxKind::Percent
                    }
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        ScalaSyntaxKind::EqEq
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        ScalaSyntaxKind::Arrow
                    }
                    else {
                        ScalaSyntaxKind::Eq
                    }
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        ScalaSyntaxKind::Ne
                    }
                    else {
                        ScalaSyntaxKind::Not
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        ScalaSyntaxKind::Le
                    }
                    else if let Some('<') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            ScalaSyntaxKind::LShiftEq
                        }
                        else {
                            ScalaSyntaxKind::LShift
                        }
                    }
                    else if let Some('-') = state.peek() {
                        state.advance(1);
                        ScalaSyntaxKind::LeftArrow
                    }
                    else {
                        ScalaSyntaxKind::Lt
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        ScalaSyntaxKind::Ge
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        if let Some('>') = state.peek() {
                            state.advance(1);
                            if let Some('=') = state.peek() {
                                state.advance(1);
                                ScalaSyntaxKind::URShiftEq
                            }
                            else {
                                ScalaSyntaxKind::URShift
                            }
                        }
                        else if let Some('=') = state.peek() {
                            state.advance(1);
                            ScalaSyntaxKind::RShiftEq
                        }
                        else {
                            ScalaSyntaxKind::RShift
                        }
                    }
                    else {
                        ScalaSyntaxKind::Gt
                    }
                }
                '&' => {
                    state.advance(1);
                    if let Some('&') = state.peek() {
                        state.advance(1);
                        ScalaSyntaxKind::AndAnd
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        ScalaSyntaxKind::AndEq
                    }
                    else {
                        ScalaSyntaxKind::And
                    }
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.peek() {
                        state.advance(1);
                        ScalaSyntaxKind::OrOr
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        ScalaSyntaxKind::OrEq
                    }
                    else {
                        ScalaSyntaxKind::Or
                    }
                }
                '^' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        ScalaSyntaxKind::XorEq
                    }
                    else {
                        ScalaSyntaxKind::Xor
                    }
                }
                '~' => {
                    state.advance(1);
                    ScalaSyntaxKind::Tilde
                }
                ':' => {
                    state.advance(1);
                    if let Some(':') = state.peek() {
                        state.advance(1);
                        ScalaSyntaxKind::ColonColon
                    }
                    else {
                        ScalaSyntaxKind::Colon
                    }
                }
                ';' => {
                    state.advance(1);
                    ScalaSyntaxKind::Semicolon
                }
                '.' => {
                    state.advance(1);
                    ScalaSyntaxKind::Dot
                }
                ',' => {
                    state.advance(1);
                    ScalaSyntaxKind::Comma
                }
                '?' => {
                    state.advance(1);
                    ScalaSyntaxKind::Question
                }
                '@' => {
                    state.advance(1);
                    ScalaSyntaxKind::At
                }
                '#' => {
                    state.advance(1);
                    ScalaSyntaxKind::Hash
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
    fn lex_delimiter(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => ScalaSyntaxKind::LeftParen,
                ')' => ScalaSyntaxKind::RightParen,
                '[' => ScalaSyntaxKind::LeftBracket,
                ']' => ScalaSyntaxKind::RightBracket,
                '{' => ScalaSyntaxKind::LeftBrace,
                '}' => ScalaSyntaxKind::RightBrace,
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

impl<'config> Lexer<ScalaLanguage> for ScalaLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<ScalaSyntaxKind> {
        let mut state = LexerState::new(source);

        while state.not_at_end() {
            // 尝试各种词法规则，按优先级排序
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_line_comment(&mut state) {
                continue;
            }

            if self.lex_block_comment(&mut state) {
                continue;
            }

            if self.lex_string(&mut state) {
                continue;
            }

            if self.lex_char(&mut state) {
                continue;
            }

            if self.lex_number(&mut state) {
                continue;
            }

            if self.lex_identifier_or_keyword(&mut state, source) {
                continue;
            }

            if self.lex_operator(&mut state) {
                continue;
            }

            if self.lex_delimiter(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(ScalaSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(ScalaSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
