use crate::{kind::ScssSyntaxKind, language::ScssLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, ScssLanguage>;

pub struct ScssLexer<'config> {
    config: &'config ScssLanguage,
}

impl<'config> ScssLexer<'config> {
    pub fn new(config: &'config ScssLanguage) -> Self {
        Self { config }
    }

    /// 跳过空白字符
    fn skip_whitespace(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' {
                state.advance(1);
            }
            else {
                break;
            }
        }

        if state.get_position() > start_pos {
            state.add_token(ScssSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(ScssSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(ScssSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理行注释和块注
    fn lex_comment(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            state.advance(1);
            if let Some('/') = state.peek() {
                // 行注
                state.advance(1);
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(1);
                }
                state.add_token(ScssSyntaxKind::LineComment, start_pos, state.get_position());
                true
            }
            else {
                // 回退，不是注
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
                        state.advance(1);
                    }
                }

                state.add_token(ScssSyntaxKind::BlockComment, start_pos, state.get_position());
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

    /// 处理标识符和关键
    fn lex_identifier_or_keyword(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_alphabetic() || ch == '_' {
                state.advance(1);

                while let Some(ch) = state.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                let text = source.get_text_in((start_pos..state.get_position()).into()).unwrap_or("");
                let kind = match text {
                    "abstract" => ScssSyntaxKind::Abstract,
                    "case" => ScssSyntaxKind::Case,
                    "catch" => ScssSyntaxKind::Catch,
                    "class" => ScssSyntaxKind::Class,
                    "def" => ScssSyntaxKind::Def,
                    "do" => ScssSyntaxKind::Do,
                    "else" => ScssSyntaxKind::Else,
                    "extends" => ScssSyntaxKind::Extends,
                    "false" => ScssSyntaxKind::False,
                    "final" => ScssSyntaxKind::Final,
                    "finally" => ScssSyntaxKind::Finally,
                    "for" => ScssSyntaxKind::For,
                    "forSome" => ScssSyntaxKind::ForSome,
                    "if" => ScssSyntaxKind::If,
                    "implicit" => ScssSyntaxKind::Implicit,
                    "import" => ScssSyntaxKind::Import,
                    "lazy" => ScssSyntaxKind::Lazy,
                    "match" => ScssSyntaxKind::Match,
                    "new" => ScssSyntaxKind::New,
                    "null" => ScssSyntaxKind::Null,
                    "object" => ScssSyntaxKind::Object,
                    "override" => ScssSyntaxKind::Override,
                    "package" => ScssSyntaxKind::Package,
                    "private" => ScssSyntaxKind::Private,
                    "protected" => ScssSyntaxKind::Protected,
                    "return" => ScssSyntaxKind::Return,
                    "sealed" => ScssSyntaxKind::Sealed,
                    "super" => ScssSyntaxKind::Super,
                    "this" => ScssSyntaxKind::This,
                    "throw" => ScssSyntaxKind::Throw,
                    "trait" => ScssSyntaxKind::Trait,
                    "try" => ScssSyntaxKind::Try,
                    "true" => ScssSyntaxKind::True,
                    "type" => ScssSyntaxKind::Type,
                    "val" => ScssSyntaxKind::Val,
                    "var" => ScssSyntaxKind::Var,
                    "while" => ScssSyntaxKind::While,
                    "with" => ScssSyntaxKind::With,
                    "yield" => ScssSyntaxKind::Yield,
                    _ => ScssSyntaxKind::Identifier,
                };

                state.add_token(kind, start_pos, state.get_position());
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
    fn lex_number(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                state.advance(1);

                // 处理整数部分
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                // 检查是否是浮点
                let mut is_float = false;
                if let Some('.') = state.peek() {
                    let next_pos = state.get_position() + 1;
                    if let Some(next_ch) = source.get_char_at(next_pos) {
                        if next_ch.is_ascii_digit() {
                            is_float = true;
                            state.advance(1); // 跳过 '.'

                            // 处理小数部分
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
                        is_float = true;
                        state.advance(1);

                        if let Some(ch) = state.peek() {
                            if ch == '+' || ch == '-' {
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

                let kind = if is_float { ScssSyntaxKind::FloatLiteral } else { ScssSyntaxKind::IntegerLiteral };

                state.add_token(kind, start_pos, state.get_position());
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
                    break;
                }
                else if ch == '\\' {
                    state.advance(1);
                    if state.not_at_end() {
                        state.advance(1);
                    }
                }
                else {
                    state.advance(1);
                }
            }

            state.add_token(ScssSyntaxKind::StringLiteral, start_pos, state.get_position());
            true
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
                    if state.not_at_end() {
                        state.advance(1);
                    }
                }
                else {
                    state.advance(1);
                }

                if let Some('\'') = state.peek() {
                    state.advance(1);
                    state.add_token(ScssSyntaxKind::CharLiteral, start_pos, state.get_position());
                    return true;
                }
            }

            // 如果没有正确闭合，回退
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
            let kind = match ch {
                '+' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        ScssSyntaxKind::PlusEq
                    }
                    else {
                        ScssSyntaxKind::Plus
                    }
                }
                '-' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        ScssSyntaxKind::MinusEq
                    }
                    else {
                        ScssSyntaxKind::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        ScssSyntaxKind::StarEq
                    }
                    else {
                        ScssSyntaxKind::Star
                    }
                }
                '/' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        ScssSyntaxKind::SlashEq
                    }
                    else {
                        ScssSyntaxKind::Slash
                    }
                }
                '%' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        ScssSyntaxKind::PercentEq
                    }
                    else {
                        ScssSyntaxKind::Percent
                    }
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        ScssSyntaxKind::EqEq
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        ScssSyntaxKind::Arrow
                    }
                    else {
                        ScssSyntaxKind::Eq
                    }
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        ScssSyntaxKind::Ne
                    }
                    else {
                        ScssSyntaxKind::Not
                    }
                }
                '<' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        ScssSyntaxKind::Le
                    }
                    else if let Some('<') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            ScssSyntaxKind::LShiftEq
                        }
                        else {
                            ScssSyntaxKind::LShift
                        }
                    }
                    else if let Some('-') = state.peek() {
                        state.advance(1);
                        ScssSyntaxKind::LeftArrow
                    }
                    else {
                        ScssSyntaxKind::Lt
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        ScssSyntaxKind::Ge
                    }
                    else if let Some('>') = state.peek() {
                        state.advance(1);
                        if let Some('>') = state.peek() {
                            state.advance(1);
                            if let Some('=') = state.peek() {
                                state.advance(1);
                                ScssSyntaxKind::URShiftEq
                            }
                            else {
                                ScssSyntaxKind::URShift
                            }
                        }
                        else if let Some('=') = state.peek() {
                            state.advance(1);
                            ScssSyntaxKind::RShiftEq
                        }
                        else {
                            ScssSyntaxKind::RShift
                        }
                    }
                    else {
                        ScssSyntaxKind::Gt
                    }
                }
                '&' => {
                    state.advance(1);
                    if let Some('&') = state.peek() {
                        state.advance(1);
                        ScssSyntaxKind::AndAnd
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        ScssSyntaxKind::AndEq
                    }
                    else {
                        ScssSyntaxKind::And
                    }
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.peek() {
                        state.advance(1);
                        ScssSyntaxKind::OrOr
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        ScssSyntaxKind::OrEq
                    }
                    else {
                        ScssSyntaxKind::Or
                    }
                }
                '^' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        ScssSyntaxKind::XorEq
                    }
                    else {
                        ScssSyntaxKind::Xor
                    }
                }
                '~' => {
                    state.advance(1);
                    ScssSyntaxKind::Tilde
                }
                ':' => {
                    state.advance(1);
                    if let Some(':') = state.peek() {
                        state.advance(1);
                        ScssSyntaxKind::ColonColon
                    }
                    else {
                        ScssSyntaxKind::Colon
                    }
                }
                ';' => {
                    state.advance(1);
                    ScssSyntaxKind::Semicolon
                }
                '.' => {
                    state.advance(1);
                    ScssSyntaxKind::Dot
                }
                ',' => {
                    state.advance(1);
                    ScssSyntaxKind::Comma
                }
                '?' => {
                    state.advance(1);
                    ScssSyntaxKind::Question
                }
                '@' => {
                    state.advance(1);
                    ScssSyntaxKind::At
                }
                '#' => {
                    state.advance(1);
                    ScssSyntaxKind::Hash
                }
                _ => return false,
            };

            state.add_token(kind, start_pos, state.get_position());
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
            let kind = match ch {
                '(' => ScssSyntaxKind::LeftParen,
                ')' => ScssSyntaxKind::RightParen,
                '[' => ScssSyntaxKind::LeftBracket,
                ']' => ScssSyntaxKind::RightBracket,
                '{' => ScssSyntaxKind::LeftBrace,
                '}' => ScssSyntaxKind::RightBrace,
                _ => return false,
            };

            state.advance(1);
            state.add_token(kind, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<ScssLanguage> for ScssLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<ScssSyntaxKind> {
        let mut state = LexerState::new(source);

        while state.not_at_end() {
            // 尝试各种词法规则，按优先级排
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_comment(&mut state) {
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

            if self.lex_number(&mut state, source) {
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

            // 如果没有匹配任何规则，添加错误token并前进一个字
            let start_pos = state.get_position();
            state.advance(1);
            state.add_token(ScssSyntaxKind::Error, start_pos, state.get_position());
        }

        // 添加EOF kind
        let eof_pos = state.get_position();
        state.add_token(ScssSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
