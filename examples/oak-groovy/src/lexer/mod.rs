use crate::{kind::GroovySyntaxKind, language::GroovyLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, GroovyLanguage>;

pub struct GroovyLexer<'config> {
    config: &'config GroovyLanguage,
}

impl<'config> GroovyLexer<'config> {
    pub fn new(config: &'config GroovyLanguage) -> Self {
        Self { config }
    }

    /// 跳过空白字符
    fn skip_whitespace(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            if ch == ' ' || ch == '\t' || ch == '\r' {
                state.advance(ch.len_utf8());
            }
            else {
                break;
            }
        }

        if state.get_position() > start_pos {
            state.add_token(GroovySyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(GroovySyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some('/') = state.peek() {
            if let Some('/') = source.get_char_at(start_pos + 1) {
                // 单行注释
                state.advance(2);
                while let Some(ch) = state.peek() {
                    if ch == '\n' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }
                state.add_token(GroovySyntaxKind::Comment, start_pos, state.get_position());
                true
            }
            else if let Some('*') = source.get_char_at(start_pos + 1) {
                // 多行注释
                state.advance(2);
                while let Some(ch) = state.peek() {
                    if ch == '*' {
                        if let Some('/') = source.get_char_at(state.get_position() + 1) {
                            state.advance(2);
                            break;
                        }
                    }
                    state.advance(ch.len_utf8());
                }
                state.add_token(GroovySyntaxKind::Comment, start_pos, state.get_position());
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
                        let token_kind =
                            if quote == '"' { GroovySyntaxKind::StringLiteral } else { GroovySyntaxKind::CharLiteral };
                        state.add_token(token_kind, start_pos, state.get_position());
                        return true;
                    }
                    else if ch == '\n' && quote == '\'' {
                        break; // 单引号字符串不能跨行
                    }
                    state.advance(ch.len_utf8());
                }

                // 未闭合的字符
                state.add_token(GroovySyntaxKind::Error, start_pos, state.get_position());
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
                let mut has_dot = false;
                let mut has_exp = false;

                // 处理十六进制、八进制、二进制
                if ch == '0' {
                    state.advance(1);
                    if let Some(next_ch) = state.peek() {
                        match next_ch {
                            'x' | 'X' => {
                                // 十六进制
                                state.advance(1);
                                while let Some(hex_ch) = state.peek() {
                                    if hex_ch.is_ascii_hexdigit() {
                                        state.advance(1);
                                    }
                                    else {
                                        break;
                                    }
                                }
                                state.add_token(GroovySyntaxKind::IntLiteral, start_pos, state.get_position());
                                return true;
                            }
                            'b' | 'B' => {
                                // 二进
                                state.advance(1);
                                while let Some(bin_ch) = state.peek() {
                                    if bin_ch == '0' || bin_ch == '1' {
                                        state.advance(1);
                                    }
                                    else {
                                        break;
                                    }
                                }
                                state.add_token(GroovySyntaxKind::IntLiteral, start_pos, state.get_position());
                                return true;
                            }
                            '0'..='7' => {
                                // 八进
                                while let Some(oct_ch) = state.peek() {
                                    if oct_ch >= '0' && oct_ch <= '7' {
                                        state.advance(1);
                                    }
                                    else {
                                        break;
                                    }
                                }
                                state.add_token(GroovySyntaxKind::IntLiteral, start_pos, state.get_position());
                                return true;
                            }
                            _ => {}
                        }
                    }
                }

                // 处理十进制数
                while let Some(digit_ch) = state.peek() {
                    if digit_ch.is_ascii_digit() {
                        state.advance(1);
                    }
                    else if digit_ch == '.' && !has_dot && !has_exp {
                        has_dot = true;
                        state.advance(1);
                    }
                    else if (digit_ch == 'e' || digit_ch == 'E') && !has_exp {
                        has_exp = true;
                        state.advance(1);
                        if let Some(sign_ch) = state.peek() {
                            if sign_ch == '+' || sign_ch == '-' {
                                state.advance(1);
                            }
                        }
                    }
                    else if digit_ch == 'f' || digit_ch == 'F' || digit_ch == 'd' || digit_ch == 'D' {
                        // Groovy 浮点数后缀
                        state.advance(1);
                        has_dot = true; // 标记为浮点数
                        break;
                    }
                    else if digit_ch == 'l' || digit_ch == 'L' || digit_ch == 'g' || digit_ch == 'G' {
                        // Groovy 整数后缀
                        state.advance(1);
                        break;
                    }
                    else {
                        break;
                    }
                }

                let token_kind = if has_dot || has_exp { GroovySyntaxKind::FloatLiteral } else { GroovySyntaxKind::IntLiteral };
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

    /// 处理标识符和关键
    fn lex_identifier(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' || ch == '$' {
                state.advance(ch.len_utf8());

                while let Some(id_ch) = state.peek() {
                    if id_ch.is_ascii_alphanumeric() || id_ch == '_' || id_ch == '$' {
                        state.advance(id_ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 检查是否是关键
                let text = source.get_text_in((start_pos..state.get_position()).into()).unwrap_or("");
                let token_kind = match text {
                    "abstract" => GroovySyntaxKind::AbstractKeyword,
                    "as" => GroovySyntaxKind::AsKeyword,
                    "assert" => GroovySyntaxKind::AssertKeyword,
                    "break" => GroovySyntaxKind::BreakKeyword,
                    "case" => GroovySyntaxKind::CaseKeyword,
                    "catch" => GroovySyntaxKind::CatchKeyword,
                    "class" => GroovySyntaxKind::ClassKeyword,
                    "const" => GroovySyntaxKind::ConstKeyword,
                    "continue" => GroovySyntaxKind::ContinueKeyword,
                    "def" => GroovySyntaxKind::DefKeyword,
                    "default" => GroovySyntaxKind::DefaultKeyword,
                    "do" => GroovySyntaxKind::DoKeyword,
                    "else" => GroovySyntaxKind::ElseKeyword,
                    "enum" => GroovySyntaxKind::EnumKeyword,
                    "extends" => GroovySyntaxKind::ExtendsKeyword,
                    "final" => GroovySyntaxKind::FinalKeyword,
                    "finally" => GroovySyntaxKind::FinallyKeyword,
                    "for" => GroovySyntaxKind::ForKeyword,
                    "goto" => GroovySyntaxKind::GotoKeyword,
                    "if" => GroovySyntaxKind::IfKeyword,
                    "implements" => GroovySyntaxKind::ImplementsKeyword,
                    "import" => GroovySyntaxKind::ImportKeyword,
                    "in" => GroovySyntaxKind::InKeyword,
                    "instanceof" => GroovySyntaxKind::InstanceofKeyword,
                    "interface" => GroovySyntaxKind::InterfaceKeyword,
                    "native" => GroovySyntaxKind::NativeKeyword,
                    "new" => GroovySyntaxKind::NewKeyword,
                    "package" => GroovySyntaxKind::PackageKeyword,
                    "private" => GroovySyntaxKind::PrivateKeyword,
                    "protected" => GroovySyntaxKind::ProtectedKeyword,
                    "public" => GroovySyntaxKind::PublicKeyword,
                    "return" => GroovySyntaxKind::ReturnKeyword,
                    "static" => GroovySyntaxKind::StaticKeyword,
                    "strictfp" => GroovySyntaxKind::StrictfpKeyword,
                    "super" => GroovySyntaxKind::SuperKeyword,
                    "switch" => GroovySyntaxKind::SwitchKeyword,
                    "synchronized" => GroovySyntaxKind::SynchronizedKeyword,
                    "this" => GroovySyntaxKind::ThisKeyword,
                    "throw" => GroovySyntaxKind::ThrowKeyword,
                    "throws" => GroovySyntaxKind::ThrowsKeyword,
                    "trait" => GroovySyntaxKind::TraitKeyword,
                    "transient" => GroovySyntaxKind::TransientKeyword,
                    "try" => GroovySyntaxKind::TryKeyword,
                    "void" => GroovySyntaxKind::VoidKeyword,
                    "volatile" => GroovySyntaxKind::VolatileKeyword,
                    "while" => GroovySyntaxKind::WhileKeyword,
                    "true" | "false" => GroovySyntaxKind::BooleanLiteral,
                    "null" => GroovySyntaxKind::NullLiteral,
                    _ => GroovySyntaxKind::Identifier,
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

    /// 处理操作符和标点符号
    fn lex_operator(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '+' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        GroovySyntaxKind::PlusAssign
                    }
                    else if let Some('+') = state.peek() {
                        state.advance(1);
                        GroovySyntaxKind::Increment
                    }
                    else {
                        GroovySyntaxKind::Plus
                    }
                }
                '-' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        GroovySyntaxKind::MinusAssign
                    }
                    else if let Some('-') = state.peek() {
                        state.advance(1);
                        GroovySyntaxKind::Decrement
                    }
                    else {
                        GroovySyntaxKind::Minus
                    }
                }
                '*' => {
                    state.advance(1);
                    if let Some('*') = state.peek() {
                        state.advance(1);
                        if let Some('=') = state.peek() {
                            state.advance(1);
                            GroovySyntaxKind::PowerAssign
                        }
                        else {
                            GroovySyntaxKind::Power
                        }
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        GroovySyntaxKind::StarAssign
                    }
                    else {
                        GroovySyntaxKind::Star
                    }
                }
                '/' => {
                    // 注释已经在前面处理了
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        GroovySyntaxKind::SlashAssign
                    }
                    else {
                        GroovySyntaxKind::Slash
                    }
                }
                '%' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        GroovySyntaxKind::PercentAssign
                    }
                    else {
                        GroovySyntaxKind::Percent
                    }
                }
                '&' => {
                    state.advance(1);
                    if let Some('&') = state.peek() {
                        state.advance(1);
                        GroovySyntaxKind::LogicalAnd
                    }
                    else {
                        GroovySyntaxKind::BitAnd
                    }
                }
                '|' => {
                    state.advance(1);
                    if let Some('|') = state.peek() {
                        state.advance(1);
                        GroovySyntaxKind::LogicalOr
                    }
                    else {
                        GroovySyntaxKind::BitOr
                    }
                }
                '^' => {
                    state.advance(1);
                    GroovySyntaxKind::BitXor
                }
                '~' => {
                    state.advance(1);
                    GroovySyntaxKind::BitNot
                }
                '<' => {
                    state.advance(1);
                    if let Some('<') = state.peek() {
                        state.advance(1);
                        GroovySyntaxKind::LeftShift
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        if let Some('>') = state.peek() {
                            state.advance(1);
                            GroovySyntaxKind::Spaceship
                        }
                        else {
                            GroovySyntaxKind::LessEqual
                        }
                    }
                    else {
                        GroovySyntaxKind::Less
                    }
                }
                '>' => {
                    state.advance(1);
                    if let Some('>') = state.peek() {
                        state.advance(1);
                        if let Some('>') = state.peek() {
                            state.advance(1);
                            GroovySyntaxKind::UnsignedRightShift
                        }
                        else {
                            GroovySyntaxKind::RightShift
                        }
                    }
                    else if let Some('=') = state.peek() {
                        state.advance(1);
                        GroovySyntaxKind::GreaterEqual
                    }
                    else {
                        GroovySyntaxKind::Greater
                    }
                }
                '=' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        GroovySyntaxKind::Equal
                    }
                    else {
                        GroovySyntaxKind::Assign
                    }
                }
                '!' => {
                    state.advance(1);
                    if let Some('=') = state.peek() {
                        state.advance(1);
                        GroovySyntaxKind::NotEqual
                    }
                    else {
                        GroovySyntaxKind::LogicalNot
                    }
                }
                '?' => {
                    state.advance(1);
                    if let Some(':') = state.peek() {
                        state.advance(1);
                        GroovySyntaxKind::Elvis
                    }
                    else if let Some('.') = state.peek() {
                        state.advance(1);
                        GroovySyntaxKind::SafeNavigation
                    }
                    else {
                        GroovySyntaxKind::Question
                    }
                }
                ':' => {
                    state.advance(1);
                    GroovySyntaxKind::Colon
                }
                '.' => {
                    state.advance(1);
                    GroovySyntaxKind::Period
                }
                '(' => {
                    state.advance(1);
                    GroovySyntaxKind::LeftParen
                }
                ')' => {
                    state.advance(1);
                    GroovySyntaxKind::RightParen
                }
                '[' => {
                    state.advance(1);
                    GroovySyntaxKind::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    GroovySyntaxKind::RightBracket
                }
                '{' => {
                    state.advance(1);
                    GroovySyntaxKind::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    GroovySyntaxKind::RightBrace
                }
                ',' => {
                    state.advance(1);
                    GroovySyntaxKind::Comma
                }
                ';' => {
                    state.advance(1);
                    GroovySyntaxKind::Semicolon
                }
                '@' => {
                    state.advance(1);
                    GroovySyntaxKind::At
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

impl<'config> Lexer<GroovyLanguage> for GroovyLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<GroovySyntaxKind> {
        let mut state = State::new(source);

        while state.not_at_end() {
            // 尝试各种词法规则
            if self.skip_whitespace(&mut state) {
                continue;
            }

            if self.lex_newline(&mut state) {
                continue;
            }

            if self.lex_comment(&mut state, source) {
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

            if self.lex_operator(&mut state, source) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(GroovySyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(GroovySyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
