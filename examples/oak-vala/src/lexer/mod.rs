use crate::{kind::ValaSyntaxKind, language::ValaLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, ValaLanguage>;

pub struct ValaLexer<'config> {
    config: &'config ValaLanguage,
}

impl<'config> ValaLexer<'config> {
    pub fn new(config: &'config ValaLanguage) -> Self {
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
            state.add_token(ValaSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(ValaSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(ValaSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }

    /// 处理注释
    fn lex_comment(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        // 行注
        //
        if let Some('/') = state.peek() {
            if let Some('/') = source.get_char_at(start_pos + 1) {
                state.advance(2);
                while let Some(ch) = state.peek() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    state.advance(ch.len_utf8());
                }
                state.add_token(ValaSyntaxKind::LineComment, start_pos, state.get_position());
                return true;
            }
        }

        // 块注
        // ...
        if let Some('/') = state.peek() {
            if let Some('*') = source.get_char_at(start_pos + 1) {
                state.advance(2);
                let mut depth = 1;

                while depth > 0 && state.not_at_end() {
                    if let Some('/') = state.peek() {
                        if let Some('*') = source.get_char_at(state.get_position() + 1) {
                            state.advance(2);
                            depth += 1;
                            continue;
                        }
                    }
                    if let Some('*') = state.peek() {
                        if let Some('/') = source.get_char_at(state.get_position() + 1) {
                            state.advance(2);
                            depth -= 1;
                            continue;
                        }
                    }
                    if let Some(ch) = state.peek() {
                        state.advance(ch.len_utf8());
                    }
                }

                state.add_token(ValaSyntaxKind::BlockComment, start_pos, state.get_position());
                return true;
            }
        }

        false
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
                        state.advance(ch.len_utf8());
                    }
                    else if ch == '\\' {
                        escaped = true;
                        state.advance(1);
                    }
                    else if ch == quote {
                        state.advance(1);
                        let token_kind = if quote == '"' { ValaSyntaxKind::StringLiteral } else { ValaSyntaxKind::CharLiteral };
                        state.add_token(token_kind, start_pos, state.get_position());
                        return true;
                    }
                    else if ch == '\n' || ch == '\r' {
                        break; // 字符串不能跨
                    }
                    else {
                        state.advance(ch.len_utf8());
                    }
                }

                // 未闭合的字符

                state.add_token(ValaSyntaxKind::Error, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理数字字面

    fn lex_number(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                // 整数部分
                while let Some(digit) = state.peek() {
                    if digit.is_ascii_digit() {
                        state.advance(1);
                    }
                    else {
                        break;
                    }
                }

                // 检查是否是浮点

                if let Some('.') = state.peek() {
                    if let Some(next_ch) = source.get_char_at(state.get_position() + 1) {
                        if next_ch.is_ascii_digit() {
                            state.advance(1); // 跳过 '.'
                            while let Some(digit) = state.peek() {
                                if digit.is_ascii_digit() {
                                    state.advance(1);
                                }
                                else {
                                    break;
                                }
                            }

                            // 检查科学计数法
                            if let Some(e) = state.peek() {
                                if e == 'e' || e == 'E' {
                                    state.advance(1);
                                    if let Some(sign) = state.peek() {
                                        if sign == '+' || sign == '-' {
                                            state.advance(1);
                                        }
                                    }
                                    while let Some(digit) = state.peek() {
                                        if digit.is_ascii_digit() {
                                            state.advance(1);
                                        }
                                        else {
                                            break;
                                        }
                                    }
                                }
                            }

                            state.add_token(ValaSyntaxKind::FloatLiteral, start_pos, state.get_position());
                            return true;
                        }
                    }
                }

                state.add_token(ValaSyntaxKind::IntegerLiteral, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理标识符和关键

    fn lex_ident_or_keyword(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                state.advance(ch.len_utf8());

                while let Some(ch) = state.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        state.advance(ch.len_utf8());
                    }
                    else {
                        break;
                    }
                }

                // 检查是否是关键

                let text = source.get_text_in((start_pos..state.get_position()).into());
                let token_kind = match text {
                    Some(text_str) => match text_str {
                        "abstract" => ValaSyntaxKind::AbstractKw,
                        "as" => ValaSyntaxKind::AsKw,
                        "base" => ValaSyntaxKind::BaseKw,
                        "break" => ValaSyntaxKind::BreakKw,
                        "case" => ValaSyntaxKind::CaseKw,
                        "catch" => ValaSyntaxKind::CatchKw,
                        "class" => ValaSyntaxKind::ClassKw,
                        "const" => ValaSyntaxKind::ConstKw,
                        "construct" => ValaSyntaxKind::ConstructKw,
                        "continue" => ValaSyntaxKind::ContinueKw,
                        "default" => ValaSyntaxKind::DefaultKw,
                        "delegate" => ValaSyntaxKind::DelegateKw,
                        "delete" => ValaSyntaxKind::DeleteKw,
                        "do" => ValaSyntaxKind::DoKw,
                        "else" => ValaSyntaxKind::ElseKw,
                        "enum" => ValaSyntaxKind::EnumKw,
                        "ensures" => ValaSyntaxKind::EnsuresKw,
                        "errordomain" => ValaSyntaxKind::ErrordomainKw,
                        "extern" => ValaSyntaxKind::ExternKw,
                        "false" => ValaSyntaxKind::FalseKw,
                        "finally" => ValaSyntaxKind::FinallyKw,
                        "for" => ValaSyntaxKind::ForKw,
                        "foreach" => ValaSyntaxKind::ForeachKw,
                        "get" => ValaSyntaxKind::GetKw,
                        "if" => ValaSyntaxKind::IfKw,
                        "in" => ValaSyntaxKind::InKw,
                        "inline" => ValaSyntaxKind::InlineKw,
                        "interface" => ValaSyntaxKind::InterfaceKw,
                        "internal" => ValaSyntaxKind::InternalKw,
                        "is" => ValaSyntaxKind::IsKw,
                        "lock" => ValaSyntaxKind::LockKw,
                        "namespace" => ValaSyntaxKind::NamespaceKw,
                        "new" => ValaSyntaxKind::NewKw,
                        "null" => ValaSyntaxKind::NullKw,
                        "out" => ValaSyntaxKind::OutKw,
                        "override" => ValaSyntaxKind::OverrideKw,
                        "owned" => ValaSyntaxKind::OwnedKw,
                        "private" => ValaSyntaxKind::PrivateKw,
                        "protected" => ValaSyntaxKind::ProtectedKw,
                        "public" => ValaSyntaxKind::PublicKw,
                        "ref" => ValaSyntaxKind::RefKw,
                        "requires" => ValaSyntaxKind::RequiresKw,
                        "return" => ValaSyntaxKind::ReturnKw,
                        "set" => ValaSyntaxKind::SetKw,
                        "sizeof" => ValaSyntaxKind::SizeofKw,
                        "static" => ValaSyntaxKind::StaticKw,
                        "struct" => ValaSyntaxKind::StructKw,
                        "switch" => ValaSyntaxKind::SwitchKw,
                        "this" => ValaSyntaxKind::ThisKw,
                        "throw" => ValaSyntaxKind::ThrowKw,
                        "throws" => ValaSyntaxKind::ThrowsKw,
                        "true" => ValaSyntaxKind::TrueKw,
                        "try" => ValaSyntaxKind::TryKw,
                        "typeof" => ValaSyntaxKind::TypeofKw,
                        "unowned" => ValaSyntaxKind::UnownedKw,
                        "using" => ValaSyntaxKind::UsingKw,
                        "var" => ValaSyntaxKind::VarKw,
                        "virtual" => ValaSyntaxKind::VirtualKw,
                        "void" => ValaSyntaxKind::VoidKw,
                        "volatile" => ValaSyntaxKind::VolatileKw,
                        "weak" => ValaSyntaxKind::WeakKw,
                        "while" => ValaSyntaxKind::WhileKw,
                        "yield" => ValaSyntaxKind::YieldKw,
                        // 基本类型
                        "bool" => ValaSyntaxKind::BoolKw,
                        "char" => ValaSyntaxKind::CharKw,
                        "uchar" => ValaSyntaxKind::UcharKw,
                        "int" => ValaSyntaxKind::IntKw,
                        "uint" => ValaSyntaxKind::UintKw,
                        "short" => ValaSyntaxKind::ShortKw,
                        "ushort" => ValaSyntaxKind::UshortKw,
                        "long" => ValaSyntaxKind::LongKw,
                        "ulong" => ValaSyntaxKind::UlongKw,
                        "int8" => ValaSyntaxKind::Int8Kw,
                        "uint8" => ValaSyntaxKind::Uint8Kw,
                        "int16" => ValaSyntaxKind::Int16Kw,
                        "uint16" => ValaSyntaxKind::Uint16Kw,
                        "int32" => ValaSyntaxKind::Int32Kw,
                        "uint32" => ValaSyntaxKind::Uint32Kw,
                        "int64" => ValaSyntaxKind::Int64Kw,
                        "uint64" => ValaSyntaxKind::Uint64Kw,
                        "float" => ValaSyntaxKind::FloatKw,
                        "double" => ValaSyntaxKind::DoubleKw,
                        "string" => ValaSyntaxKind::StringKw,
                        _ => ValaSyntaxKind::Identifier,
                    },
                    None => ValaSyntaxKind::Identifier,
                };

                state.add_token(token_kind, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理标点符号和操作符
    fn lex_punctuation(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            let token_kind = match ch {
                '(' => {
                    state.advance(1);
                    ValaSyntaxKind::LeftParen
                }
                ')' => {
                    state.advance(1);
                    ValaSyntaxKind::RightParen
                }
                '[' => {
                    state.advance(1);
                    ValaSyntaxKind::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    ValaSyntaxKind::RightBracket
                }
                '{' => {
                    state.advance(1);
                    ValaSyntaxKind::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    ValaSyntaxKind::RightBrace
                }
                ':' => {
                    state.advance(1);
                    ValaSyntaxKind::Colon
                }
                ';' => {
                    state.advance(1);
                    ValaSyntaxKind::Semicolon
                }
                '.' => {
                    state.advance(1);
                    ValaSyntaxKind::Dot
                }
                ',' => {
                    state.advance(1);
                    ValaSyntaxKind::Comma
                }
                '?' => {
                    state.advance(1);
                    ValaSyntaxKind::Question
                }
                '!' => {
                    if let Some('=') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        ValaSyntaxKind::NotEq
                    }
                    else {
                        state.advance(1);
                        ValaSyntaxKind::Bang
                    }
                }
                '@' => {
                    state.advance(1);
                    ValaSyntaxKind::At
                }
                '#' => {
                    state.advance(1);
                    ValaSyntaxKind::Hash
                }
                '$' => {
                    state.advance(1);
                    ValaSyntaxKind::Dollar
                }
                '%' => {
                    if let Some('=') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        ValaSyntaxKind::PercentEq
                    }
                    else {
                        state.advance(1);
                        ValaSyntaxKind::Percent
                    }
                }
                '^' => {
                    state.advance(1);
                    ValaSyntaxKind::Caret
                }
                '&' => {
                    if let Some('&') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        ValaSyntaxKind::AndAnd
                    }
                    else {
                        state.advance(1);
                        ValaSyntaxKind::Ampersand
                    }
                }
                '*' => {
                    if let Some('=') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        ValaSyntaxKind::StarEq
                    }
                    else {
                        state.advance(1);
                        ValaSyntaxKind::Star
                    }
                }
                '+' => {
                    if let Some('+') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        ValaSyntaxKind::PlusPlus
                    }
                    else if let Some('=') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        ValaSyntaxKind::PlusEq
                    }
                    else {
                        state.advance(1);
                        ValaSyntaxKind::Plus
                    }
                }
                '-' => {
                    if let Some('-') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        ValaSyntaxKind::MinusMinus
                    }
                    else if let Some('=') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        ValaSyntaxKind::MinusEq
                    }
                    else if let Some('>') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        ValaSyntaxKind::Arrow
                    }
                    else {
                        state.advance(1);
                        ValaSyntaxKind::Minus
                    }
                }
                '=' => {
                    if let Some('=') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        ValaSyntaxKind::EqEq
                    }
                    else {
                        state.advance(1);
                        ValaSyntaxKind::Eq
                    }
                }
                '<' => {
                    if let Some('<') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        ValaSyntaxKind::LeftShift
                    }
                    else if let Some('=') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        ValaSyntaxKind::LessEq
                    }
                    else {
                        state.advance(1);
                        ValaSyntaxKind::LessThan
                    }
                }
                '>' => {
                    if let Some('>') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        ValaSyntaxKind::RightShift
                    }
                    else if let Some('=') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        ValaSyntaxKind::GreaterEq
                    }
                    else {
                        state.advance(1);
                        ValaSyntaxKind::GreaterThan
                    }
                }
                '/' => {
                    if let Some('=') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        ValaSyntaxKind::SlashEq
                    }
                    else {
                        state.advance(1);
                        ValaSyntaxKind::Slash
                    }
                }
                '\\' => {
                    state.advance(1);
                    ValaSyntaxKind::Backslash
                }
                '|' => {
                    if let Some('|') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        ValaSyntaxKind::OrOr
                    }
                    else {
                        state.advance(1);
                        ValaSyntaxKind::Pipe
                    }
                }
                '~' => {
                    state.advance(1);
                    ValaSyntaxKind::Tilde
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

    /// 处理普通文

    fn lex_text(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        while let Some(ch) = state.peek() {
            // 遇到特殊字符时停

            match ch {
                ' ' | '\t' | '\n' | '\r' | '(' | ')' | '[' | ']' | '{' | '}' | ':' | ';' | '.' | ',' | '?' | '!' | '@'
                | '#' | '$' | '%' | '^' | '&' | '*' | '+' | '-' | '=' | '<' | '>' | '/' | '\\' | '|' | '~' | '"' | '\'' => {
                    break;
                }
                _ => {
                    if ch.is_ascii_alphabetic() || ch.is_ascii_digit() || ch == '_' {
                        break; // 这些应该由其他规则处
                    }
                    state.advance(ch.len_utf8());
                }
            }
        }

        if state.get_position() > start_pos {
            state.add_token(ValaSyntaxKind::Text, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<ValaLanguage> for ValaLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<ValaSyntaxKind> {
        let mut state = LexerState::new(source);

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

            if self.lex_number(&mut state, source) {
                continue;
            }

            if self.lex_ident_or_keyword(&mut state, source) {
                continue;
            }

            if self.lex_punctuation(&mut state, source) {
                continue;
            }

            if self.lex_text(&mut state) {
                continue;
            }

            // 如果所有规则都不匹配，跳过当前字符并标记为错误
            let start_pos = state.get_position();
            if let Some(ch) = state.peek() {
                state.advance(ch.len_utf8());
                state.add_token(ValaSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(ValaSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
