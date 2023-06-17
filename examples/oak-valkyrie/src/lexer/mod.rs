use crate::{kind::ValkyrieSyntaxKind, language::ValkyrieLanguage};
use oak_core::{Lexer, LexerState, SourceText, lexer::LexOutput};

type State<'input> = LexerState<'input, ValkyrieLanguage>;

pub struct ValkyrieLexer<'config> {
    config: &'config ValkyrieLanguage,
}

impl<'config> ValkyrieLexer<'config> {
    pub fn new(config: &'config ValkyrieLanguage) -> Self {
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
            state.add_token(ValkyrieSyntaxKind::Whitespace, start_pos, state.get_position());
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
            state.add_token(ValkyrieSyntaxKind::Newline, start_pos, state.get_position());
            true
        }
        else if let Some('\r') = state.peek() {
            state.advance(1);
            if let Some('\n') = state.peek() {
                state.advance(1);
            }
            state.add_token(ValkyrieSyntaxKind::Newline, start_pos, state.get_position());
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
                state.add_token(ValkyrieSyntaxKind::LineComment, start_pos, state.get_position());
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

                state.add_token(ValkyrieSyntaxKind::BlockComment, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理字符串字面量
    fn lex_string(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some(quote) = state.peek() {
            if quote == '"' {
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
                        state.add_token(ValkyrieSyntaxKind::StringLiteral, start_pos, state.get_position());
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

                state.add_token(ValkyrieSyntaxKind::Error, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理字符字面

    fn lex_char(&self, state: &mut State) -> bool {
        let start_pos = state.get_position();

        if let Some('\'') = state.peek() {
            state.advance(1);
            let mut escaped = false;

            if let Some(ch) = state.peek() {
                if ch == '\\' {
                    escaped = true;
                    state.advance(1);
                    if let Some(_) = state.peek() {
                        state.advance(1);
                    }
                }
                else if ch != '\'' && ch != '\n' && ch != '\r' {
                    state.advance(ch.len_utf8());
                }
            }

            // 检查结束引

            if let Some('\'') = state.peek() {
                state.advance(1);
                state.add_token(ValkyrieSyntaxKind::CharLiteral, start_pos, state.get_position());
                return true;
            }

            // 未闭合的字符字面

            state.add_token(ValkyrieSyntaxKind::Error, start_pos, state.get_position());
            return true;
        }

        false
    }

    /// 处理数字字面

    fn lex_number(&self, state: &mut State, source: &SourceText) -> bool {
        let start_pos = state.get_position();

        if let Some(ch) = state.peek() {
            if ch.is_ascii_digit() {
                // 检查特殊进

                if ch == '0' {
                    if let Some(next_ch) = source.get_char_at(start_pos + 1) {
                        if next_ch == 'x' || next_ch == 'X' {
                            // 十六进制
                            state.advance(2);
                            while let Some(ch) = state.peek() {
                                if ch.is_ascii_hexdigit() || ch == '_' {
                                    state.advance(1);
                                }
                                else {
                                    break;
                                }
                            }
                            state.add_token(ValkyrieSyntaxKind::IntegerLiteral, start_pos, state.get_position());
                            return true;
                        }
                        else if next_ch == 'b' || next_ch == 'B' {
                            // 二进

                            state.advance(2);
                            while let Some(ch) = state.peek() {
                                if ch == '0' || ch == '1' || ch == '_' {
                                    state.advance(1);
                                }
                                else {
                                    break;
                                }
                            }
                            state.add_token(ValkyrieSyntaxKind::IntegerLiteral, start_pos, state.get_position());
                            return true;
                        }
                        else if next_ch == 'o' || next_ch == 'O' {
                            // 八进

                            state.advance(2);
                            while let Some(ch) = state.peek() {
                                if (ch >= '0' && ch <= '7') || ch == '_' {
                                    state.advance(1);
                                }
                                else {
                                    break;
                                }
                            }
                            state.add_token(ValkyrieSyntaxKind::IntegerLiteral, start_pos, state.get_position());
                            return true;
                        }
                    }
                }

                // 十进制数
                let mut is_float = false;
                while let Some(ch) = state.peek() {
                    if ch.is_ascii_digit() || ch == '_' {
                        state.advance(1);
                    }
                    else if ch == '.' && !is_float {
                        // 检查是否是浮点

                        if let Some(next_ch) = source.get_char_at(state.get_position() + 1) {
                            if next_ch.is_ascii_digit() {
                                is_float = true;
                                state.advance(1); // 跳过小数
                            }
                            else {
                                break; // 不是浮点数，可能是方法调
                            }
                        }
                        else {
                            break;
                        }
                    }
                    else if (ch == 'e' || ch == 'E') && (is_float || state.get_position() > start_pos + 1) {
                        // 科学计数

                        is_float = true;
                        state.advance(1);
                        if let Some(sign) = state.peek() {
                            if sign == '+' || sign == '-' {
                                state.advance(1);
                            }
                        }
                        while let Some(ch) = state.peek() {
                            if ch.is_ascii_digit() || ch == '_' {
                                state.advance(1);
                            }
                            else {
                                break;
                            }
                        }
                        break;
                    }
                    else {
                        break;
                    }
                }

                let token_kind = if is_float { ValkyrieSyntaxKind::FloatLiteral } else { ValkyrieSyntaxKind::IntegerLiteral };

                state.add_token(token_kind, start_pos, state.get_position());
                return true;
            }
        }

        false
    }

    /// 处理标识符和关键

    fn lex_identifier(&self, state: &mut State, source: &SourceText) -> bool {
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
                let token_kind = match text.unwrap_or("") {
                    // 模块和结
                    "mod" => ValkyrieSyntaxKind::ModKw,
                    "fn" => ValkyrieSyntaxKind::FnKw,
                    "struct" => ValkyrieSyntaxKind::StructKw,
                    "enum" => ValkyrieSyntaxKind::EnumKw,
                    "trait" => ValkyrieSyntaxKind::TraitKw,
                    "impl" => ValkyrieSyntaxKind::ImplKw,
                    "type" => ValkyrieSyntaxKind::TypeKw,

                    // 变量和常
                    "let" => ValkyrieSyntaxKind::LetKw,
                    "mut" => ValkyrieSyntaxKind::MutKw,
                    "const" => ValkyrieSyntaxKind::ConstKw,
                    "static" => ValkyrieSyntaxKind::StaticKw,

                    // 控制
                    "if" => ValkyrieSyntaxKind::IfKw,
                    "else" => ValkyrieSyntaxKind::ElseKw,
                    "match" => ValkyrieSyntaxKind::MatchKw,
                    "for" => ValkyrieSyntaxKind::ForKw,
                    "while" => ValkyrieSyntaxKind::WhileKw,
                    "loop" => ValkyrieSyntaxKind::LoopKw,
                    "break" => ValkyrieSyntaxKind::BreakKw,
                    "continue" => ValkyrieSyntaxKind::ContinueKw,
                    "return" => ValkyrieSyntaxKind::ReturnKw,

                    // 可见性和模块
                    "pub" => ValkyrieSyntaxKind::PubKw,
                    "use" => ValkyrieSyntaxKind::UseKw,
                    "as" => ValkyrieSyntaxKind::AsKw,
                    "in" => ValkyrieSyntaxKind::InKw,
                    "where" => ValkyrieSyntaxKind::WhereKw,

                    // 特殊标识
                    "self" => ValkyrieSyntaxKind::SelfKw,
                    "super" => ValkyrieSyntaxKind::SuperKw,
                    "crate" => ValkyrieSyntaxKind::CrateKw,

                    // 安全性和外部接口
                    "unsafe" => ValkyrieSyntaxKind::UnsafeKw,
                    "extern" => ValkyrieSyntaxKind::ExternKw,

                    // 引用和移
                    "ref" => ValkyrieSyntaxKind::RefKw,
                    "move" => ValkyrieSyntaxKind::MoveKw,
                    "box" => ValkyrieSyntaxKind::BoxKw,

                    // 异步编程
                    "async" => ValkyrieSyntaxKind::AsyncKw,
                    "await" => ValkyrieSyntaxKind::AwaitKw,

                    // 错误处理
                    "try" => ValkyrieSyntaxKind::TryKw,
                    "catch" => ValkyrieSyntaxKind::CatchKw,
                    "finally" => ValkyrieSyntaxKind::FinallyKw,

                    // 生成
                    "yield" => ValkyrieSyntaxKind::YieldKw,

                    // 宏和动
                    "macro" => ValkyrieSyntaxKind::MacroKw,
                    "dyn" => ValkyrieSyntaxKind::DynKw,

                    // 基本类型
                    "bool" => ValkyrieSyntaxKind::BoolKw,
                    "char" => ValkyrieSyntaxKind::CharKw,
                    "str" => ValkyrieSyntaxKind::StrKw,
                    "i8" => ValkyrieSyntaxKind::I8Kw,
                    "i16" => ValkyrieSyntaxKind::I16Kw,
                    "i32" => ValkyrieSyntaxKind::I32Kw,
                    "i64" => ValkyrieSyntaxKind::I64Kw,
                    "i128" => ValkyrieSyntaxKind::I128Kw,
                    "isize" => ValkyrieSyntaxKind::IsizeKw,
                    "u8" => ValkyrieSyntaxKind::U8Kw,
                    "u16" => ValkyrieSyntaxKind::U16Kw,
                    "u32" => ValkyrieSyntaxKind::U32Kw,
                    "u64" => ValkyrieSyntaxKind::U64Kw,
                    "u128" => ValkyrieSyntaxKind::U128Kw,
                    "usize" => ValkyrieSyntaxKind::UsizeKw,
                    "f32" => ValkyrieSyntaxKind::F32Kw,
                    "f64" => ValkyrieSyntaxKind::F64Kw,

                    // 布尔字面
                    "true" | "false" => ValkyrieSyntaxKind::BoolLiteral,

                    _ => ValkyrieSyntaxKind::Identifier,
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
                '+' => {
                    if let Some('+') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        ValkyrieSyntaxKind::PlusPlus
                    }
                    else if let Some('=') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        ValkyrieSyntaxKind::PlusEq
                    }
                    else {
                        state.advance(1);
                        ValkyrieSyntaxKind::Plus
                    }
                }
                '-' => {
                    if let Some('-') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        ValkyrieSyntaxKind::MinusMinus
                    }
                    else if let Some('=') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        ValkyrieSyntaxKind::MinusEq
                    }
                    else if let Some('>') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        ValkyrieSyntaxKind::Arrow
                    }
                    else {
                        state.advance(1);
                        ValkyrieSyntaxKind::Minus
                    }
                }
                '*' => {
                    if let Some('=') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        ValkyrieSyntaxKind::StarEq
                    }
                    else {
                        state.advance(1);
                        ValkyrieSyntaxKind::Star
                    }
                }
                '/' => {
                    if let Some('=') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        ValkyrieSyntaxKind::SlashEq
                    }
                    else {
                        state.advance(1);
                        ValkyrieSyntaxKind::Slash
                    }
                }
                '%' => {
                    if let Some('=') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        ValkyrieSyntaxKind::PercentEq
                    }
                    else {
                        state.advance(1);
                        ValkyrieSyntaxKind::Percent
                    }
                }
                '&' => {
                    if let Some('&') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        ValkyrieSyntaxKind::AndAnd
                    }
                    else {
                        state.advance(1);
                        ValkyrieSyntaxKind::Ampersand
                    }
                }
                '|' => {
                    if let Some('|') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        ValkyrieSyntaxKind::OrOr
                    }
                    else {
                        state.advance(1);
                        ValkyrieSyntaxKind::Pipe
                    }
                }
                '<' => {
                    if let Some('<') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        ValkyrieSyntaxKind::LeftShift
                    }
                    else if let Some('=') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        ValkyrieSyntaxKind::LessEq
                    }
                    else {
                        state.advance(1);
                        ValkyrieSyntaxKind::LessThan
                    }
                }
                '>' => {
                    if let Some('>') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        ValkyrieSyntaxKind::RightShift
                    }
                    else if let Some('=') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        ValkyrieSyntaxKind::GreaterEq
                    }
                    else {
                        state.advance(1);
                        ValkyrieSyntaxKind::GreaterThan
                    }
                }
                '=' => {
                    if let Some('=') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        ValkyrieSyntaxKind::EqEq
                    }
                    else {
                        state.advance(1);
                        ValkyrieSyntaxKind::Eq
                    }
                }
                '!' => {
                    if let Some('=') = source.get_char_at(start_pos + 1) {
                        state.advance(2);
                        ValkyrieSyntaxKind::NotEq
                    }
                    else {
                        state.advance(1);
                        ValkyrieSyntaxKind::Bang
                    }
                }
                '(' => {
                    state.advance(1);
                    ValkyrieSyntaxKind::LeftParen
                }
                ')' => {
                    state.advance(1);
                    ValkyrieSyntaxKind::RightParen
                }
                '{' => {
                    state.advance(1);
                    ValkyrieSyntaxKind::LeftBrace
                }
                '}' => {
                    state.advance(1);
                    ValkyrieSyntaxKind::RightBrace
                }
                '[' => {
                    state.advance(1);
                    ValkyrieSyntaxKind::LeftBracket
                }
                ']' => {
                    state.advance(1);
                    ValkyrieSyntaxKind::RightBracket
                }
                ';' => {
                    state.advance(1);
                    ValkyrieSyntaxKind::Semicolon
                }
                ',' => {
                    state.advance(1);
                    ValkyrieSyntaxKind::Comma
                }
                '.' => {
                    state.advance(1);
                    ValkyrieSyntaxKind::Dot
                }
                ':' => {
                    state.advance(1);
                    ValkyrieSyntaxKind::Colon
                }
                '?' => {
                    state.advance(1);
                    ValkyrieSyntaxKind::Question
                }
                '@' => {
                    state.advance(1);
                    ValkyrieSyntaxKind::At
                }
                '#' => {
                    state.advance(1);
                    ValkyrieSyntaxKind::Hash
                }
                '$' => {
                    state.advance(1);
                    ValkyrieSyntaxKind::Dollar
                }
                '^' => {
                    state.advance(1);
                    ValkyrieSyntaxKind::Caret
                }
                '\\' => {
                    state.advance(1);
                    ValkyrieSyntaxKind::Backslash
                }
                '~' => {
                    state.advance(1);
                    ValkyrieSyntaxKind::Tilde
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
            state.add_token(ValkyrieSyntaxKind::Text, start_pos, state.get_position());
            true
        }
        else {
            false
        }
    }
}

impl<'config> Lexer<ValkyrieLanguage> for ValkyrieLexer<'config> {
    fn lex(&self, source: &SourceText) -> LexOutput<ValkyrieSyntaxKind> {
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

            if self.lex_char(&mut state) {
                continue;
            }

            if self.lex_number(&mut state, source) {
                continue;
            }

            if self.lex_identifier(&mut state, source) {
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
                state.add_token(ValkyrieSyntaxKind::Error, start_pos, state.get_position());
            }
        }

        // 添加 EOF kind
        let eof_pos = state.get_position();
        state.add_token(ValkyrieSyntaxKind::Eof, eof_pos, eof_pos);

        state.finish()
    }
}
